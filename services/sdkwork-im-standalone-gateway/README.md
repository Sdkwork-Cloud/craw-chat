# sdkwork-im-standalone-gateway

Domain: communication
Capability: im
Package type: rust-service
Status: active

Standalone IM gateway binary for local and packaged deployments. Composes `sdkwork-web-framework` ingress, embedded IAM app-api routes, IM route registry, and product runtime static assets without the full split-service topology.

On startup the gateway:

1. Bootstraps IM database lifecycle
2. Bootstraps IAM schema through `sdkwork-iam-database-host`
3. Provisions tenant application runtime `sdkwork-im-pc` for tenant `100001`
4. Assembles embedded IAM and IM routers on one bind

## Public API

- Binary: `sdkwork-im-standalone-gateway`
- Config: gateway YAML/TOML resolved through `sdkwork-im-cloud-gateway-config` and `sdkwork-api-config`.

## Configuration

Reads gateway bind URLs, upstream service endpoints, and static site directories from the resolved standalone gateway config file.

## Verification

- `cargo build -p sdkwork-im-standalone-gateway`
- `cargo test -p sdkwork-im-iam-application-bootstrap`
- `pnpm gateway:build:standalone`
- `node scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs`
