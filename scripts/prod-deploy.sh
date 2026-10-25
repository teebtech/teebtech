#!/usr/bin/env bash
# ==============================================================================
# TeebTech Production Rebuild, Safe Staged Deployment & Restart Workflow
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/common.sh
source "${SCRIPT_DIR}/common.sh"

check_workspace_root
check_prerequisites
ensure_runtime_dirs

PRE_APP_ENV="${APP_ENV:-}"
PRE_DATABASE_URL="${DATABASE_URL:-}"

# Load production configuration if present, otherwise .env
if [ -f ".env.production" ]; then
    log_info "Loading production configuration from .env.production..."
    # shellcheck disable=SC2046
    export $(grep -v '^#' .env.production | xargs -d '\n' 2>/dev/null || true)
elif [ -f ".env" ]; then
    log_info "Loading configuration from .env..."
    # shellcheck disable=SC2046
    export $(grep -v '^#' .env | xargs -d '\n' 2>/dev/null || true)
fi

export APP_ENV="${PRE_APP_ENV:-${APP_ENV:-production}}"
export DATABASE_URL="${PRE_DATABASE_URL:-${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev}}"

usage() {
    echo -e "${BOLD}TeebTech Production Deployment & Restart Manager${RESET}"
    echo ""
    echo "Usage: $0 <command> [target]"
    echo ""
    echo "Commands:"
    echo "  deploy [all|home|client|admin]    Build release artifacts, safely replace running service & verify health"
    echo "  restart [all|home|client|admin]   Restart running production service without full rebuild"
    echo "  stop [all|home|client|admin]      Gracefully terminate production service"
    echo "  status                           Display production service health, PIDs, and build info"
    echo "  logs <home|client|admin>          Tail production service logs"
    echo ""
    echo "Examples:"
    echo "  $0 deploy all       # Full atomic release build & staged deployment across all portals"
    echo "  $0 deploy client    # Staged release deployment for Client Portal only"
    echo "  $0 status           # Audit health and memory of all production services"
    exit 1
}

build_app_prod() {
    local app="$1"
    local project
    project=$(get_project_name "$app")

    log_info "Executing production release build for $project (WASM optimization, JS minification, CSS build)..."

    # Capture start time for build metrics
    local start_time
    start_time=$(date +%s)

    if ! cargo leptos build --release --project "$project"; then
        log_error "Production release build failed for $project!"
        log_error "The currently running production version was PRESERVED and NOT modified."
        return 1
    fi

    local end_time
    end_time=$(date +%s)
    log_success "Production build completed for $project in $((end_time - start_time)) seconds."

    # Validate release binary artifact exists
    local bin_path="./target/release/${project}"
    if [ ! -f "$bin_path" ] || [ ! -x "$bin_path" ]; then
        log_error "Compiled release binary '$bin_path' not found or not executable."
        return 1
    fi

    verify_assets "$app"
}

deploy_app_prod() {
    local app="$1"
    local project
    project=$(get_project_name "$app")
    local port
    port=$(get_app_port "$app")
    local pid_file=".run/prod-${app}.pid"
    local log_file="logs/prod-${app}.log"
    local bin_path="./target/release/${project}"
    local backup_bin="./target/release/${project}.backup"

    check_database

    log_info "=== INITIATING STAGED PRODUCTION DEPLOYMENT: $project (Port $port) ==="

    # Phase 1: Build the new release version FIRST while preserving existing process
    if ! build_app_prod "$app"; then
        log_error "Deployment aborted during build phase. Zero-downtime safety preserved."
        return 1
    fi

    # Phase 2: Backup previous release binary if exists
    if [ -f "$bin_path" ]; then
        cp -p "$bin_path" "$backup_bin" 2>/dev/null || true
    fi

    # Phase 3: Gracefully terminate previous production instance
    log_info "Stopping previous production instance on port $port..."
    safe_stop_pid "$pid_file" "$port"

    # Phase 4: Launch updated release binary
    log_info "Starting updated production binary $bin_path..."
    setsid env APP_ENV="$APP_ENV" DATABASE_URL="$DATABASE_URL" "$bin_path" </dev/null > "$log_file" 2>&1 &
    local new_pid=$!
    echo "$new_pid" > "$pid_file"

    # Phase 5: Verification & Health Check with Rollback capability
    log_info "Verifying health check and SSR readiness (PID: $new_pid)..."
    if ! wait_for_health "$project" "$port" 30; then
        log_error "Deployment health check failed for $project! Inspecting failure log:"
        tail -n 25 "$log_file"

        log_warn "Initiating automatic rollback to previous release binary..."
        safe_stop_pid "$pid_file" "$port"

        if [ -f "$backup_bin" ]; then
            mv "$backup_bin" "$bin_path"
            setsid env APP_ENV="$APP_ENV" DATABASE_URL="$DATABASE_URL" "$bin_path" </dev/null > "$log_file" 2>&1 &
            local rollback_pid=$!
            echo "$rollback_pid" > "$pid_file"
            if wait_for_health "$project" "$port" 20; then
                log_success "Rollback successful. Previous release is running on PID $rollback_pid."
            else
                log_error "Rollback instance also failed health check. Manual intervention required."
            fi
        fi
        return 1
    fi

    local sha
    sha=$(sha256sum "$bin_path" | cut -d' ' -f1 | head -c 12)
    log_success "=== DEPLOYMENT SUCCESSFUL: $project (PID: $new_pid | Build SHA: $sha) -> http://127.0.0.1:${port} ==="
}

restart_app_prod() {
    local app="$1"
    local project
    project=$(get_project_name "$app")
    local port
    port=$(get_app_port "$app")
    local pid_file=".run/prod-${app}.pid"
    local log_file="logs/prod-${app}.log"
    local bin_path="./target/release/${project}"

    if [ ! -f "$bin_path" ]; then
        log_error "Release binary '$bin_path' does not exist. Run '$0 deploy $app' first."
        return 1
    fi

    check_database
    log_info "Restarting $project without rebuild..."
    safe_stop_pid "$pid_file" "$port"

    setsid env APP_ENV="$APP_ENV" DATABASE_URL="$DATABASE_URL" "$bin_path" </dev/null > "$log_file" 2>&1 &
    local new_pid=$!
    echo "$new_pid" > "$pid_file"

    if ! wait_for_health "$project" "$port" 30; then
        log_error "Restart failed for $project."
        tail -n 20 "$log_file"
        return 1
    fi

    log_success "$project restarted successfully (PID: $new_pid)."
}

stop_app_prod() {
    local app="$1"
    local port
    port=$(get_app_port "$app")
    local pid_file=".run/prod-${app}.pid"

    log_info "Stopping production instance of $app..."
    safe_stop_pid "$pid_file" "$port"
    log_success "$app stopped."
}

show_status() {
    echo -e "${BOLD}--- TeebTech Production Environment Status ---${RESET}"
    printf "%-12s %-8s %-10s %-8s %-14s %-12s %-25s\n" "APPLICATION" "PORT" "STATUS" "PID" "BUILD SHA" "MEMORY" "HEALTH CHECK"
    echo "--------------------------------------------------------------------------------------------"

    for app in home client admin; do
        local project
        project=$(get_project_name "$app")
        local port
        port=$(get_app_port "$app")
        local pid_file=".run/prod-${app}.pid"
        local status="STOPPED"
        local pid="-"
        local mem="-"
        local health="-"
        local bin_path="./target/release/${project}"
        local sha="-"

        if [ -f "$bin_path" ]; then
            sha=$(sha256sum "$bin_path" 2>/dev/null | cut -d' ' -f1 | head -c 12 || echo "-")
        fi

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

        printf "%-12s %-8s %-19b %-8s %-14s %-12s %-34b\n" "$app" "$port" "$status" "$pid" "$sha" "$mem" "$health"
    done
    echo ""
}

tail_logs() {
    local app="$1"
    local log_file="logs/prod-${app}.log"
    if [ ! -f "$log_file" ]; then
        log_warn "Log file '$log_file' does not exist yet."
        exit 1
    fi
    log_info "Tailing production log $log_file (Ctrl+C to exit)..."
    tail -f "$log_file"
}

# Main Dispatcher
COMMAND="${1:-}"
TARGET="${2:-all}"

case "$COMMAND" in
    deploy)
        acquire_lock "prod_deploy"
        trap 'release_lock "prod_deploy"' EXIT
        if [ "$TARGET" = "all" ]; then
            deploy_app_prod home
            deploy_app_prod client
            deploy_app_prod admin
        else
            deploy_app_prod "$TARGET"
        fi
        ;;
    restart)
        acquire_lock "prod_deploy"
        trap 'release_lock "prod_deploy"' EXIT
        if [ "$TARGET" = "all" ]; then
            restart_app_prod home
            restart_app_prod client
            restart_app_prod admin
        else
            restart_app_prod "$TARGET"
        fi
        ;;
    stop)
        if [ "$TARGET" = "all" ]; then
            stop_app_prod home
            stop_app_prod client
            stop_app_prod admin
        else
            stop_app_prod "$TARGET"
        fi
        ;;
    status)
        show_status
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
