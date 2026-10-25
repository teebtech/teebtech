#!/usr/bin/env bash
# ==============================================================================
# TeebTech Local Development Rebuild & Restart Workflow
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/common.sh
source "${SCRIPT_DIR}/common.sh"

check_workspace_root
check_prerequisites
ensure_runtime_dirs

# Preserve explicit environment variables if passed
PRE_DATABASE_URL="${DATABASE_URL:-}"

# Load .env if present
if [ -f ".env" ]; then
    # shellcheck disable=SC2046
    export $(grep -v '^#' .env | xargs -d '\n' 2>/dev/null || true)
fi

if [ -n "$PRE_DATABASE_URL" ]; then
    export DATABASE_URL="$PRE_DATABASE_URL"
else
    export DATABASE_URL="${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev}"
fi

usage() {
    echo -e "${BOLD}TeebTech Development Environment Manager${RESET}"
    echo ""
    echo "Usage: $0 <command> [target]"
    echo ""
    echo "Commands:"
    echo "  start [all|home|client|admin]    Build and start application(s) in background with health checks"
    echo "  restart [all|home|client|admin]  Rebuild and restart application(s) cleanly"
    echo "  stop [all|home|client|admin]     Gracefully stop application(s)"
    echo "  status                          Display running status, ports, and health checks"
    echo "  watch <home|client|admin>        Start interactive cargo-leptos watch with hot-reload"
    echo "  logs <home|client|admin>         Tail the development logs"
    echo ""
    echo "Examples:"
    echo "  $0 start all       # Build & start home, client, and admin applications"
    echo "  $0 restart client  # Rebuild & restart Client Portal"
    echo "  $0 watch home      # Start Home application in interactive hot-reload mode"
    echo "  $0 status          # Inspect health of all running services"
    exit 1
}

build_app_dev() {
    local app="$1"
    local project
    project=$(get_project_name "$app")

    log_info "Building $project (Development mode with Tailwind & WASM)..."
    if ! cargo leptos build --project "$project"; then
        log_error "Build failed for $project. Development process was NOT started."
        return 1
    fi

    verify_assets "$app"
}

start_app_dev() {
    local app="$1"
    local project
    project=$(get_project_name "$app")
    local port
    port=$(get_app_port "$app")
    local pid_file=".run/dev-${app}.pid"
    local log_file="logs/dev-${app}.log"

    check_database

    # Build first to verify code validity before touching running process
    if ! build_app_dev "$app"; then
        return 1
    fi

    # Gracefully stop previous process if running
    safe_stop_pid "$pid_file" "$port"

    log_info "Starting $project on http://127.0.0.1:${port}..."
    setsid env DATABASE_URL="$DATABASE_URL" "./target/debug/${project}" </dev/null > "$log_file" 2>&1 &
    local new_pid=$!
    echo "$new_pid" > "$pid_file"

    if ! wait_for_health "$project" "$port" 30; then
        log_error "Startup failed for $project. Inspecting last 20 lines of $log_file:"
        tail -n 20 "$log_file"
        safe_stop_pid "$pid_file" "$port"
        return 1
    fi

    log_success "$project is active (PID: $new_pid) -> http://127.0.0.1:${port}"
}

stop_app_dev() {
    local app="$1"
    local port
    port=$(get_app_port "$app")
    local pid_file=".run/dev-${app}.pid"

    log_info "Stopping development instance of $app..."
    safe_stop_pid "$pid_file" "$port"
    log_success "$app stopped."
}

show_status() {
    echo -e "${BOLD}--- TeebTech Development Environment Status ---${RESET}"
    printf "%-12s %-8s %-10s %-8s %-12s %-25s\n" "APPLICATION" "PORT" "STATUS" "PID" "MEMORY" "HEALTH CHECK"
    echo "-------------------------------------------------------------------------------"

    for app in home client admin; do
        local port
        port=$(get_app_port "$app")
        local pid_file=".run/dev-${app}.pid"
        local status="STOPPED"
        local pid="-"
        local mem="-"
        local health="-"

        if [ -f "$pid_file" ]; then
            local p
            p=$(cat "$pid_file" 2>/dev/null || true)
            if [ -n "$p" ] && kill -0 "$p" 2>/dev/null; then
                status="${GREEN}RUNNING${RESET}"
                pid="$p"
                mem=$(ps -o rss= -p "$p" 2>/dev/null | awk '{printf "%.1f MB", $1/1024}' || echo "-")
            fi
        fi

        # If not tracked by PID file, check if port is listening
        if [ "$status" = "STOPPED" ]; then
            local active_pid
            active_pid=$(ss -tulpn "sport = :$port" 2>/dev/null | grep -o 'pid=[0-9]*' | cut -d= -f2 | head -n 1 || true)
            if [ -n "$active_pid" ]; then
                status="${YELLOW}LISTENING${RESET}"
                pid="$active_pid"
                mem=$(ps -o rss= -p "$active_pid" 2>/dev/null | awk '{printf "%.1f MB", $1/1024}' || echo "-")
            fi
        fi

        if [ "$pid" != "-" ]; then
            local health_resp
            health_resp=$(curl -s "http://127.0.0.1:${port}/health" 2>/dev/null || true)
            if [[ "$health_resp" =~ healthy ]]; then
                health="${GREEN}OK (healthy)${RESET}"
            else
                health="${YELLOW}No /health response${RESET}"
            fi
        fi

        printf "%-12s %-8s %-19b %-8s %-12s %-34b\n" "$app" "$port" "$status" "$pid" "$mem" "$health"
    done
    echo ""
}

watch_app() {
    local app="$1"
    local project
    project=$(get_project_name "$app")
    local port
    port=$(get_app_port "$app")
    local pid_file=".run/dev-${app}.pid"

    check_database

    log_info "Ensuring port $port is free before launching watch mode..."
    safe_stop_pid "$pid_file" "$port"

    log_success "Launching interactive watch mode for $project..."
    echo -e "${CYAN}Press Ctrl+C to terminate.${RESET}"
    cargo leptos watch --project "$project"
}

tail_logs() {
    local app="$1"
    local log_file="logs/dev-${app}.log"
    if [ ! -f "$log_file" ]; then
        log_warn "Log file '$log_file' does not exist yet."
        exit 1
    fi
    log_info "Tailing $log_file (Ctrl+C to exit)..."
    tail -f "$log_file"
}

# Main Dispatcher
COMMAND="${1:-}"
TARGET="${2:-all}"

case "$COMMAND" in
    start)
        acquire_lock "dev"
        trap 'release_lock "dev"' EXIT
        if [ "$TARGET" = "all" ]; then
            start_app_dev home
            start_app_dev client
            start_app_dev admin
        else
            start_app_dev "$TARGET"
        fi
        ;;
    restart)
        acquire_lock "dev"
        trap 'release_lock "dev"' EXIT
        if [ "$TARGET" = "all" ]; then
            start_app_dev home
            start_app_dev client
            start_app_dev admin
        else
            start_app_dev "$TARGET"
        fi
        ;;
    stop)
        if [ "$TARGET" = "all" ]; then
            stop_app_dev home
            stop_app_dev client
            stop_app_dev admin
        else
            stop_app_dev "$TARGET"
        fi
        ;;
    status)
        show_status
        ;;
    watch)
        if [ "$TARGET" = "all" ]; then
            log_error "Watch mode requires a single target: home, client, or admin."
            exit 1
        fi
        watch_app "$TARGET"
        ;;
    logs)
        if [ "$TARGET" = "all" ]; then
            log_error "Logs command requires a target: home, client, or admin."
            exit 1
        fi
        tail_logs "$TARGET"
        ;;
    *)
        usage
        ;;
esac
