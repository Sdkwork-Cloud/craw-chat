# sdkwork-im-standalone-gateway

Domain: communication  
Capability: im  
Package type: rust-service  
Status: active

Standalone IM gateway binary for local and packaged deployments. Composes `sdkwork-web-framework` ingress, embedded IAM app-api routes, IM route registry, and product runtime static assets through the current single-ingress topology.

Default standalone unified development binds IM application ingress to `http://127.0.0.1:18079`. Do not use `127.0.0.1:3900` as the IM gateway health/API authority unless the selected topology explicitly starts a shared platform API gateway on that port.

## Startup sequence

On boot the gateway:

1. Bootstraps IM database lifecycle
2. Bootstraps IAM schema through `sdkwork-iam-database-host`
3. Provisions tenant application runtime `sdkwork-im-pc` for tenant `100001`
4. Assembles embedded IAM and IM routers on one bind
5. Applies a single edge `HybridIpRateLimiter` after IM, IAM, and embedded dependency routers are merged

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

## Gateway composition

Standalone composition intentionally calls the cloud-gateway router builder variant that disables the inner per-IP limiter. The final standalone router then merges IM, embedded application, dependency, and IAM routes and applies one outer `HybridIpRateLimiter`.

This preserves a single edge rate-limit decision for the whole standalone ingress and prevents IM requests from being counted twice. Health and metrics probe paths (`/health`, `/healthz`, `/livez`, `/ready`, `/readyz`, `/metrics`) are exempt from IP rate limiting so Kubernetes, local dev scripts, and process supervisors can still observe liveness during traffic spikes.

`GET /openapi.json` is served by the shared gateway OpenAPI aggregator. In unified-process profiles the aggregator skips upstream schema fetches that would point back to the gateway's own aggregate endpoint, caches successful aggregate documents, and coalesces concurrent refreshes. Tune the cache TTL with `SDKWORK_IM_GATEWAY_OPENAPI_CACHE_TTL_SECS`.

## Public API

- Binary: `sdkwork-im-standalone-gateway`
- Config: gateway YAML/TOML resolved through `sdkwork-im-cloud-gateway-config` and `sdkwork-api-config`

## Configuration

Reads gateway bind URLs, upstream service endpoints, and static site directories from the resolved standalone gateway config file.

Direct binary execution, `pnpm dev`, and `pnpm gateway:run:standalone` route IM foundation APIs through embedded in-process handlers unless an explicit cloud upstream is configured.

Split upstream routing belongs to `sdkwork-im-cloud-gateway` and cloud topology profiles. If `/openapi/runtime-summary.json` reports `runtimeMode: "split"` for `sdkwork-im-standalone-gateway`, the running process is stale or not using the current binary.

## Verification

```bash
cargo build -p sdkwork-im-standalone-gateway
cargo test -p sdkwork-im-standalone-gateway -- --nocapture
cargo test -p sdkwork-im-iam-application-bootstrap
pnpm gateway:build:standalone
node scripts/dev/run-standalone-gateway-dev.mjs --config configs/sdkwork-im-standalone-gateway.development.toml
node scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
```
