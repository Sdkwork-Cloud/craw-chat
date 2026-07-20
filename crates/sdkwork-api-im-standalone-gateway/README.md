# sdkwork-api-im-standalone-gateway

## Purpose

Provides a thin standalone HTTP host for the canonical `sdkwork-api-im-assembly` router. It is
useful for isolated API assembly checks and deployments that require only the IM application API
surface.

The full SDKWork IM standalone product gateway remains
`crates/sdkwork-api-im-standalone-gateway`. That service owns IAM bootstrap, embedded dependency
routes, database lifecycle, realtime wiring, static product surfaces, and the complete
single-ingress application runtime.

## Owner

SDKWork IM maintainers.

## Public API

- Binary: `sdkwork-api-im-standalone-gateway`
- Router authority: `sdkwork-api-im-assembly`
- Bind environment variable: `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND`

The bind defaults to `127.0.0.1:8080` when the environment variable is absent. Deployment and
application lifecycle plans must provide an explicit source `etc` profile instead of relying on
that development fallback.

## Allowed Content

- Process bootstrap for the canonical IM API assembly
- Shared `sdkwork-web-bootstrap` tracing, health, and listener integration
- Host-boundary verification

## Forbidden Content

- Product business handlers or copied route implementations
- IAM, dependency API, database, migration, worker, or static-site orchestration
- A competing application topology, workflow, or deployment manifest
- Secrets or environment-specific runtime values in source control

## Security

This host does not create signing material, credentials, or deployment evidence. Production
network policy, TLS termination, identity bootstrap, and secret injection belong to the selected
deployment environment and full application gateway plan.

## Related Specs

- `../../../sdkwork-specs/API_ASSEMBLY_SPEC.md`
- `../../../sdkwork-specs/APPLICATION_GATEWAY_SPEC.md`
- `../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`
- `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../../sdkwork-specs/RUST_CODE_SPEC.md`
- `../../../sdkwork-specs/TEST_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-api-im-standalone-gateway
```
