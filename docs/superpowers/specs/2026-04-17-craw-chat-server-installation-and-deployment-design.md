# 2026-04-17 Sdkwork IM Server Installation And Deployment Design

## 1. Goal

Define a production-grade `sdkwork-im-server` installation, initialization, packaging, and service-management standard that works across Linux, macOS, and Windows. The standard must:

- treat the unified `web-gateway` as the default external entrypoint
- make PostgreSQL the default server-edition database baseline
- support externally managed PostgreSQL by configuration file without forcing database creation
- support both archive-style and native installer distribution formats
- standardize installation directories, runtime directories, configuration files, and service registration semantics
- preserve the existing `local-minimal` tooling as a development profile rather than the formal server product

## 2. Current State

The repository already contains:

- cross-platform `local-minimal` lifecycle scripts under `bin/`
- deployment and runtime directory standards under `docs/部署/`
- release bundle archive conventions under `artifacts/releases/`
- a growing split-service runtime with a dedicated `web-gateway` workstream

The current deployment tooling is useful but still optimized for local or minimal-node workflows:

- lifecycle scripts are named `*-local.*`
- templates are `local-minimal` and `local-default`
- runtime directories still assume the existing `.runtime/local-minimal` contract
- there is no dedicated `server` package/install surface yet
- there is no finalized cross-platform standard for `systemd`, `launchd`, or Windows Service registration
- PostgreSQL is the architecture baseline in docs, but not yet surfaced as the formal server-install contract

## 3. Design Principles

1. One product, many installer shells. Linux, macOS, and Windows packages must wrap the same canonical server payload instead of inventing platform-specific semantics.
2. Install and initialize are different phases. File installation, config generation, storage initialization, service registration, and startup must remain separately executable and auditable.
3. Configuration files are the source of truth. Interactive flows may help generate config, but the final state must always exist as explicit files under the instance config directory.
4. External PostgreSQL is the production standard. The server must connect to an already provisioned PostgreSQL instance through configuration only. Automatic database/schema bootstrap is optional, not mandatory.
5. Server edition owns a single external service identity. Operators should manage one formal product service, not a collection of internal business services.
6. Existing local workflows stay intact. `local-minimal` remains a development and troubleshooting entrypoint and is not retrofitted into the formal server package identity.

## 4. Product Shapes

### 4.1 Formal Product

`sdkwork-im-server`

- single external base URL
- default external entrypoint is `web-gateway`
- PostgreSQL is the default persistence baseline
- installed and managed as a system service

### 4.2 Quickstart Product

`sdkwork-im-server-quickstart`

- intended for evaluation, smoke verification, single-host trials, and demos
- may use Docker Compose or equivalent orchestration
- reuses the same configuration model, endpoint model, and health semantics as the formal product
- is not allowed to define a second, container-only configuration language

## 5. Distribution Standard

Every release first produces a canonical payload. All installer formats must be derived from that payload.

### 5.1 Canonical Payload Contents

- product binaries
- lifecycle scripts
- configuration templates
- PostgreSQL migration assets
- service templates
- OpenAPI authority snapshots
- install, upgrade, and rollback guides
- checksum manifest
- bundle manifest

### 5.2 Required Distribution Formats

- Linux: `.tar.gz`
- macOS: `.tar.gz`
- Windows: `.zip`

### 5.3 Native Installer Targets

- Linux: `.deb`, `.rpm`
- macOS: `.pkg`
- Windows: `.msi`

Archive packages are always the lowest-common-denominator official distribution. Native installers are formal wrappers around the same payload and may not change file naming, config semantics, or service identity.

## 6. Directory Layout Standard

The server uses a product directory plus per-instance directories. The default instance name is `default`.

### 6.1 Linux

- install root: `/opt/sdkwork-im`
- config root: `/etc/sdkwork-im/default`
- data root: `/var/lib/sdkwork-im/default`
- log root: `/var/log/sdkwork-im/default`
- run root: `/var/run/sdkwork-im/default`

### 6.2 macOS

- install root: `/usr/local/lib/sdkwork-im`
- config root: `/usr/local/etc/sdkwork-im/default`
- data root: `/usr/local/var/lib/sdkwork-im/default`
- log root: `/usr/local/var/log/sdkwork-im/default`
- run root: `/usr/local/var/run/sdkwork-im/default`

### 6.3 Windows

- install root: `C:\Program Files\SdkworkIm`
- config root: `C:\ProgramData\SdkworkIm\default\config`
- data root: `C:\ProgramData\SdkworkIm\default\data`
- log root: `C:\ProgramData\SdkworkIm\default\logs`
- run root: `C:\ProgramData\SdkworkIm\default\run`

### 6.4 Invariants

- upgrades may replace files under the install root
- upgrades may not overwrite user-owned config or runtime data without explicit operator action
- logs, PID files, secrets, and migration reports live outside the install root
- all lifecycle scripts resolve paths through the instance config rather than by hardcoded platform branches

## 7. Configuration Model

The instance config directory must provide a stable file layout:

```text
<instance-config-root>/
  server.yaml
  server.env
  conf.d/
    10-network.yaml
    20-storage.yaml
    30-observability.yaml
  storage/
    postgresql.yaml
  secrets/
    postgresql.password
    app-secret.key
  install.json
```

### 7.1 Responsibilities

- `server.yaml`
  - primary instance-level server configuration
- `server.env`
  - scalar overrides for service wrappers and process launchers
- `conf.d/*.yaml`
  - optional layered overrides, loaded lexicographically
- `storage/postgresql.yaml`
  - PostgreSQL-specific connection, pool, schema, and migration policy
- `secrets/*`
  - file-based secret material
- `install.json`
  - generated installation metadata and last-known initialization status

### 7.2 Public Endpoint Model

The config must support explicit endpoint separation:

- `baseUrl`
- `apiBaseUrl`
- `websocketBaseUrl`
- `docsBaseUrl`

This avoids forcing SDKs and docs to infer WebSocket URLs from an HTTP base URL.

### 7.3 Config Resolution Order

1. CLI parameters
2. environment variables
3. `conf.d/*.yaml`
4. primary YAML files
5. built-in defaults

## 8. PostgreSQL Standard

PostgreSQL is the default server-edition database baseline.

### 8.1 Supported Operating Modes

- `external-managed`
  - database and account already exist
  - server may verify and optionally apply migrations
- `external-governed`
  - database governance is external
  - server verifies only and does not alter schema
- `bootstrap`
  - server may initialize schema and, where authorized, create the database

### 8.2 Required File-Based Configuration

The server must support fully file-driven PostgreSQL configuration, including the case where the database is already installed and provisioned by another team.

Required categories:

- host
- port
- database
- username
- password file
- ssl mode
- schema name
- migration policy
- connection pool settings

### 8.3 Initialization Modes

- `verify-only`
- `bootstrap-schema`
- `create-db-and-schema`

The initialization report must state:

- connection success/failure
- effective target database
- migration mode used
- schema version found
- schema version applied
- whether the server is safe to start

## 9. Lifecycle Command Standard

The formal server command surface is:

- `install-server.*`
- `init-config-server.*`
- `init-storage-server.*`
- `verify-server.*`
- `install-service-server.*`
- `uninstall-service-server.*`
- `start-server.*`
- `stop-server.*`
- `restart-server.*`
- `status-server.*`

All platforms must expose equivalent semantics through PowerShell, CMD wrappers, and Bash where applicable.

## 10. Service Standard

The formal product service identity is one operator-facing service:

- Linux: `sdkwork-im-server.service`
- macOS: `com.sdkwork.im.server`
- Windows: `SdkworkImServer`

### 10.1 Service Host

Windows should use a dedicated service-host wrapper instead of treating a foreground console binary as a first-class Windows Service implementation.

### 10.2 Service Guarantees

- read config from the instance config directory
- fail fast on missing critical dependencies
- write logs to the instance log directory
- write runtime state to the instance run directory
- print a startup summary including:
  - version
  - bind address
  - base URL
  - API URL
  - WebSocket URL
  - `/healthz`
  - `/readyz`
  - `/openapi.json`
  - `/docs`
  - current schema version

## 11. Installation Flow

Formal server installation uses six explicit stages:

1. `install`
2. `init-config`
3. `init-storage`
4. `verify`
5. `install-service`
6. `start`

This split is required for repeatability, auditability, and controlled failure handling.

## 12. Upgrade And Rollback Standard

### 12.1 Upgrade

Upgrades must:

- replace product files under the install root
- preserve config, data, logs, and runtime roots
- run explicit verification before restart
- run explicit storage migration when required
- record install and migration metadata for audit

### 12.2 Rollback

The standard only guarantees application-version rollback by default. Database schema rollback is only supported when the migration set explicitly declares reversibility.

Bundle manifests must state whether schema rollback is supported.

## 13. Integration With Existing Repository Standards

The new server standard builds on existing repo conventions:

- `bin/` remains the lifecycle script home
- `deployments/templates/` evolves into shared server template assets
- `deployments/docker-compose/` continues to host quickstart packaging
- `artifacts/releases/` remains the authority archive boundary for release bundles
- `docs/部署/` remains the operator-facing deployment documentation home

`local-*` scripts remain supported. The new `server-*` scripts are additive and define the formal server-edition contract.

## 14. Implementation Scope For The First Landing

The first landing should establish the contract and core tooling rather than every native installer:

1. create the spec and plan
2. add server deployment tests alongside the existing deployment script contract tests
3. add shared server profile/path helpers
4. add server configuration and PostgreSQL templates
5. add cross-platform `install-server`, `init-config-server`, `init-storage-server`, and `verify-server` scripts
6. add initial `systemd` template and service-install wrappers
7. update deployment docs and release bundle docs

Native `.deb`, `.rpm`, `.pkg`, and `.msi` generation can then build on the same stable payload and script contract.
