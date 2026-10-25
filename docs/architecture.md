# TeebTech Enterprise Architecture & Boundary Specification

This document defines the architectural rules, layers, and dependency invariants for the TeebTech Rust Workspace.

---

## 1. System Layers & Member Classification

```text
+-----------------------------------------------------------------------+
| 1. DELIVERY LAYER (Apps)                                              |
|    - apps/home    (Public Marketing, SSR + Hydration)                 |
|    - apps/client  (Customer SaaS Portal, SSR + Hydration)             |
|    - apps/admin   (Backoffice Console, SSR + Hydration)               |
+-----------------------------------------------------------------------+
        |                     |                       |
        v                     v (ssr feature only)    v
+-------------------+ +--------------------+ +-------------------------+
| 2. FRONTEND SHARED| | 4. SECONDARY       | | 3. APPLICATION &        |
|    PACKAGES       | |    ADAPTERS (Server) | |    SECURITY             |
|    - packages/ui  | |    - database      | |    - application        |
|    - design-system| |    - infrastructure| |    - auth               |
|    - config       | +--------------------+ |    - authorization      |
+-------------------+           |            +-------------------------+
        |                       v                         |
        |             +--------------------+              |
        +------------>| 5. PURE DOMAIN &   |<-------------+
                      |    PRIMITIVES      |
                      |    - domain        |
                      |    - shared        |
                      +--------------------+
```

---

## 2. Layer Definitions & Allowed Dependencies

### Layer 1: Delivery Layer (`apps/`)
* **Role**: Composition roots and protocol adapters (HTTP routing, SSR rendering, WASM hydration entrypoints).
* **Allowed Dependencies**:
  * Frontend packages: `packages/ui`, `packages/design-system`, `packages/config`
  * Core application: `crates/application`, `crates/domain`, `crates/shared`
  * Security abstractions: `crates/auth`, `crates/authorization` (in `client` and `admin`)
  * Secondary adapters (Gated strictly under `ssr` feature): `crates/database`, `crates/infrastructure`
* **Forbidden**:
  * Direct dependencies on peer apps (e.g. `home` must never depend on `client`).
  * Unconditional linkage of server-only crates into the client WASM hydration path.

### Layer 2: Frontend Shared Layer (`packages/`)
* **Role**: Visual design system, reusable Leptos components, and public client configuration.
* **Allowed Dependencies**:
  * `packages/ui` may depend on: `packages/design-system`, `leptos`
  * `packages/design-system` may depend on: `serde`
  * `packages/config` may depend on: `serde`, `thiserror`
* **Forbidden**:
  * Dependencies on backend crates (`crates/database`, `crates/infrastructure`, `crates/auth`, `crates/authorization`).
  * `packages/ui` must NOT depend on application-level config or shared error crates.
  * Server secrets must NEVER be placed in public client-visible structs in `packages/config`.

### Layer 3: Application & Security Layer (`crates/application`, `crates/auth`, `crates/authorization`)
* **Role**: Use case orchestration, CQRS command/query handlers, authentication contracts, authorization policies.
* **Allowed Dependencies**:
  * `crates/domain`, `crates/shared`, `serde`, `thiserror`, `tracing`
* **Forbidden**:
  * Dependencies on secondary adapters (`crates/database`, `crates/infrastructure`).
  * Dependencies on web/delivery frameworks (`axum`, `leptos`).
  * Direct coupling between `crates/authorization` and `crates/auth` unless explicitly required by interface definitions.

### Layer 4: Secondary Adapters (`crates/database`, `crates/infrastructure`)
* **Role**: Concrete external implementations (Postgres/SQLx pools, SMTP clients, cloud telemetry, object storage).
* **Allowed Dependencies**:
  * Inward dependencies on domain/application ports and primitives: `crates/shared`, `thiserror`, `tracing`
* **Forbidden**:
  * **Adapter-to-Adapter Coupling**: `crates/database` MUST NOT depend on `crates/infrastructure`.
  * Concrete adapters must not be depended upon by `crates/domain` or `crates/application`.

### Layer 5: Pure Domain Core (`crates/domain`, `crates/shared`)
* **Role**: Enterprise business entities, domain models, value objects, domain repository interfaces, and universal primitives.
* **Allowed Dependencies**:
  * `crates/domain` depends ONLY on `crates/shared`, `thiserror`, and optional `serde`.
  * `crates/shared` depends ONLY on `thiserror` and optional `serde`/`tracing`.
* **Forbidden (Strict Invariants)**:
  * ZERO dependencies on `axum`, `leptos`, database drivers, HTTP clients, browser APIs, or external infrastructure.

---

## 3. Configuration Security Boundary

1. **`PublicAppConfig`**:
   * Safe for distribution to the browser via WebAssembly hydration bundles.
   * Contains only public metadata (environment name, public API base URLs, feature flags).
2. **`ServerAppConfig`**:
   * Restricted strictly to non-WASM targets (`#[cfg(all(feature = "server", not(target_arch = "wasm32")))]`).
   * Contains sensitive credentials, connection strings, and cryptographic signing keys.
   * Must never be serialized into HTML responses or accessible by frontend components.
