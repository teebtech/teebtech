#!/usr/bin/env bash
# ==============================================================================
# TeebTech Shared Build, Process Management & Health Check Library
# ==============================================================================

set -euo pipefail

# ANSI Color Codes
BOLD="\033[1m"
GREEN="\033[32m"
BLUE="\033[34m"
CYAN="\033[36m"
YELLOW="\033[33m"
RED="\033[31m"
RESET="\033[0m"

log_info() {
    echo -e "${BLUE}[INFO]${RESET} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${RESET} ${BOLD}$1${RESET}"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${RESET} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${RESET} ${BOLD}$1${RESET}" >&2
}

# Ensure script is executed from the repository root
check_workspace_root() {
    if [ ! -f "Cargo.toml" ] || [ ! -d "apps" ] || [ ! -d "crates" ]; then
        log_error "This script must be executed from the TeebTech workspace root directory."
        exit 1
    fi
}

# Ensure runtime directories exist
ensure_runtime_dirs() {
    mkdir -p .run logs
}

# Verify essential system tool prerequisites
check_prerequisites() {
    local missing=0
    for cmd in cargo curl ss; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            log_error "Required prerequisite '$cmd' is not installed or not in PATH."
            missing=1
        fi
    done

    if ! cargo leptos --version >/dev/null 2>&1; then
        log_error "'cargo-leptos' is not installed. Please run 'cargo install cargo-leptos'."
        missing=1
    fi

    if [ "$missing" -eq 1 ]; then
        exit 1
    fi
}

# Verify PostgreSQL connectivity without exposing credentials
check_database() {
    local db_url="${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev}"
    local host="127.0.0.1"
    local port="5433"

    # Extract host and port if custom
    if [[ "$db_url" =~ @([^:/]+):([0-9]+) ]]; then
        host="${BASH_REMATCH[1]}"
        port="${BASH_REMATCH[2]}"
    fi

    if command -v pg_isready >/dev/null 2>&1; then
        if ! pg_isready -h "$host" -p "$port" -q; then
            log_error "Database is not reachable at $host:$port. Please verify the PostgreSQL service is running."
            exit 1
        fi
    else
        # Fallback TCP socket test
        if ! (exec 3<>/dev/tcp/"$host"/"$port") 2>/dev/null; then
            log_error "Database TCP socket is not reachable at $host:$port."
            exit 1
        fi
        exec 3<&-
        exec 3>&-
    fi
}

# Acquire deployment lock to prevent concurrent executions
acquire_lock() {
    local lock_name="$1"
    local lock_file=".run/${lock_name}.lock"
    ensure_runtime_dirs

    if [ -f "$lock_file" ]; then
        local pid
        pid=$(cat "$lock_file" 2>/dev/null || true)
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            log_error "Another deployment/restart process (PID $pid) is already running holding '$lock_file'."
            exit 1
        else
            log_warn "Removing stale lock file '$lock_file'."
            rm -f "$lock_file"
        fi
    fi

    echo "$$" > "$lock_file"
}

# Release deployment lock
release_lock() {
    local lock_name="$1"
    local lock_file=".run/${lock_name}.lock"
    rm -f "$lock_file" 2>/dev/null || true
}

# Get Application Port
get_app_port() {
    local app="$1"
    case "$app" in
        home|home-app) echo "3000" ;;
        client|client-app) echo "3002" ;;
        admin|admin-app) echo "3004" ;;
        *) echo "0" ;;
    esac
}

# Get Application Crate/Project Name
get_project_name() {
    local app="$1"
    case "$app" in
        home|home-app) echo "home-app" ;;
        client|client-app) echo "client-app" ;;
        admin|admin-app) echo "admin-app" ;;
        *) echo "$app" ;;
    esac
}

# Get Output / Site-Root Directory Name
get_output_name() {
    local app="$1"
    case "$app" in
        home|home-app) echo "home" ;;
        client|client-app) echo "client" ;;
        admin|admin-app) echo "admin" ;;
        *) echo "$app" ;;
    esac
}

# Stop a process gracefully using its PID file
safe_stop_pid() {
    local pid_file="$1"
    local port="$2"

    if [ -f "$pid_file" ]; then
        local pid
        pid=$(cat "$pid_file" 2>/dev/null || true)
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            log_info "Gracefully stopping process PID $pid..."
            kill -15 "$pid" 2>/dev/null || true

            # Wait up to 5 seconds for graceful shutdown
            local count=0
            while kill -0 "$pid" 2>/dev/null && [ "$count" -lt 10 ]; do
                sleep 0.5
                count=$((count + 1))
            done

            # Force terminate if still lingering
            if kill -0 "$pid" 2>/dev/null; then
                log_warn "Process $pid did not exit after 5s. Sending SIGKILL..."
                kill -9 "$pid" 2>/dev/null || true
                sleep 0.5
            fi
        fi
        rm -f "$pid_file"
    fi

    # Verify port is released
    if [ -n "$port" ] && [ "$port" != "0" ]; then
        local port_pids
        port_pids=$(ss -tulpn "sport = :$port" 2>/dev/null | grep -o 'pid=[0-9]*' | cut -d= -f2 || true)
        if [ -n "$port_pids" ]; then
            for p in $port_pids; do
                log_warn "Port $port still held by PID $p. Terminating..."
                kill -15 "$p" 2>/dev/null || true
                sleep 0.5
                if kill -0 "$p" 2>/dev/null; then
                    kill -9 "$p" 2>/dev/null || true
                fi
            done
        fi
    fi
}

# Wait for application health check to succeed
wait_for_health() {
    local app="$1"
    local port="$2"
    local max_retries="${3:-30}" # 30 * 0.5s = 15s

    local health_url="http://127.0.0.1:${port}/health"
    local root_url="http://127.0.0.1:${port}/"

    log_info "Awaiting health check on $health_url..."

    local retries=0
    while [ "$retries" -lt "$max_retries" ]; do
        local status
        status=$(curl -s -o /tmp/health_resp.txt -w "%{http_code}" "$health_url" 2>/dev/null || echo "000")

        if [ "$status" = "200" ]; then
            local root_status
            root_status=$(curl -s -o /dev/null -w "%{http_code}" "$root_url" 2>/dev/null || echo "000")
            if [ "$root_status" = "200" ]; then
                log_success "$app is healthy and serving HTTP 200 on port $port."
                return 0
            fi
        fi

        sleep 0.5
        retries=$((retries + 1))
    done

    log_error "Health check failed for $app on port $port after $((max_retries / 2)) seconds."
    return 1
}

# Verify generated static and WASM assets exist
verify_assets() {
    local app="$1"
    local output_name
    output_name=$(get_output_name "$app")
    local pkg_dir="target/site/${output_name}/pkg"

    if [ ! -d "$pkg_dir" ]; then
        log_error "Expected asset directory '$pkg_dir' does not exist."
        return 1
    fi

    local js_file="${pkg_dir}/${output_name}.js"
    local wasm_file="${pkg_dir}/${output_name}.wasm"
    local css_file="${pkg_dir}/${output_name}.css"

    if [ ! -f "$js_file" ] || [ ! -f "$wasm_file" ] || [ ! -f "$css_file" ]; then
        log_error "Missing required frontend assets in '$pkg_dir'."
        log_error "Files found: $(ls "$pkg_dir" 2>/dev/null || true)"
        return 1
    fi

    log_success "Frontend assets verified for $app (${output_name}.js, ${output_name}.wasm, ${output_name}.css)."
    return 0
}
