# TeebTech Enterprise Platform

A scalable, modular, multi-app SaaS platform architecture built with **Rust**, **Leptos 0.8**, and **Axum**.

---

## Workspace Structure

```text
teebtech/
├── apps/                    # Fullstack Leptos Applications
│   ├── home/                # Public Landing & Marketing (Port 3000)
│   ├── client/              # Customer / Tenant Dashboard (Port 3002)
│   └── admin/               # Backoffice Administration (Port 3004)
│
├── crates/                  # Backend & Domain Business Logic
│   ├── domain/              # Pure domain models & repository interfaces
│   ├── application/         # CQRS use cases, DTOs & service traits
│   ├── infrastructure/      # Telemetry, email & external integrations
│   ├── auth/                # Identity & authentication services
│   ├── authorization/       # RBAC & permission evaluation policies
│   ├── database/            # Database pool & repository implementations
│   └── shared/              # Primitives, Error types & Result aliases
│
├── packages/                # Frontend Shared Libraries
│   ├── ui/                  # Reusable Leptos components & layout shells
│   ├── design-system/       # Visual tokens & theme constants
│   └── config/              # Centralized environment configuration
│
├── migrations/              # Database migration files
├── docs/                    # Architecture documentation & guides
└── scripts/                 # Developer tooling & automation
```

---

## Quick Start & Developer Commands

### Development Workflow (`./scripts/dev.sh`)

Manage local development environments with background process supervision, PID tracking, and `/health` checks:

```bash
# Start all applications in the background with health verification
./scripts/dev.sh start all

# Rebuild and restart a specific portal (e.g. Client Portal)
./scripts/dev.sh restart client

# Inspect running PIDs, memory consumption, listening ports, and health status
./scripts/dev.sh status

# Interactive hot-reload development for an application
./scripts/dev.sh watch home
./scripts/dev.sh watch client
./scripts/dev.sh watch admin

# Stop development services
./scripts/dev.sh stop all
```

### Production Staged Deployment Workflow (`./scripts/prod-deploy.sh`)

Zero-downtime-capable atomic release builds with pre-flight asset validation, graceful process replacement, health checking, and automatic rollback on failure:

```bash
# Staged production deployment for all applications
./scripts/prod-deploy.sh deploy all

# Staged deployment for a single application
./scripts/prod-deploy.sh deploy client

# Inspect production service health, PIDs, and release binary build SHAs
./scripts/prod-deploy.sh status

# Fast production restart without full rebuild
./scripts/prod-deploy.sh restart client

# Stop production instances
./scripts/prod-deploy.sh stop all
```

### Workspace Verification & Tests

```bash
# Workspace compilation check
cargo check --workspace --all-targets --all-features

# Run all workspace unit tests
cargo test --workspace

# Run full zero-trust navigation and route protection test matrix (41 tests)
./scripts/test_portal_navigation.sh

# Run end-to-end workspace verification suite
./scripts/check-workspace.sh
```

---

## License

This project is licensed under either the MIT or Apache-2.0 license at your option.
