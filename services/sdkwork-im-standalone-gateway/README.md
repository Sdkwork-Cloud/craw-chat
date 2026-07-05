# sdkwork-im-standalone-gateway

Domain: communication  
Capability: im  
Package type: rust-service  
Status: active

Standalone IM gateway binary for local and packaged deployments. Composes `sdkwork-web-framework` ingress, embedded IAM app-api routes, IM route registry, and product runtime static assets without the full split-service topology.

## Startup sequence

On boot the gateway:

1. Bootstraps IM database lifecycle
2. Bootstraps IAM schema through `sdkwork-iam-database-host`
3. Provisions tenant application runtime `sdkwork-im-pc` for tenant `100001`
4. Assembles embedded IAM and IM routers on one bind

## Development

Preferred local entrypoints:

```bash
pnpm dev
pnpm gateway:run:standalone
```

Both invoke `scripts/dev/run-standalone-gateway-dev.mjs`, which:

1. Terminates stale `sdkwork-im-standalone-gateway.exe` processes (Windows)
2. Waits for the dev executable to unlock when a prior process still holds the file
3. Runs `cargo build -p sdkwork-im-standalone-gateway`
4. Executes the built binary with `--config <standalone-gateway.toml>`

This avoids Windows `cargo run` failures (`拒绝访问` / os error 5) when an old gateway binary is still running.

Isolated cargo target directory (dev default): `.runtime/cargo-target/sdkwork-im-standalone-gateway-dev`

## Public API

- Binary: `sdkwork-im-standalone-gateway`
- Config: gateway YAML/TOML resolved through `sdkwork-im-cloud-gateway-config` and `sdkwork-api-config`

## Configuration

Reads gateway bind URLs, upstream service endpoints, and static site directories from the resolved standalone gateway config file.

## Verification

```bash
cargo build -p sdkwork-im-standalone-gateway
cargo test -p sdkwork-im-iam-application-bootstrap
pnpm gateway:build:standalone
node scripts/dev/run-standalone-gateway-dev.mjs --config configs/sdkwork-im-standalone-gateway.development.toml
node scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
```
