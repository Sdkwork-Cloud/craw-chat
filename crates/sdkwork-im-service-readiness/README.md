# sdkwork-im-service-readiness

## Purpose

Shared process-infrastructure composition for SDKWork IM HTTP services. It owns canonical IM/IAM
database connectivity checks, configured Redis checks, registration of process lifecycle signals,
and fail-closed composition of gateway-specific required checks.

It does not define HTTP routes. Hosts pass its `ReadinessCheck` result to
`sdkwork-web-bootstrap::ServiceRouterConfig`, which owns `/healthz`, `/livez`, `/readyz`, and
`/metrics`.

## Public API

- `resolve_im_service_readiness_check`: resolves canonical database, Redis, and registered process checks.
- `resolve_gateway_readiness_check_with_required_checks`: adds host-owned runtime dependencies.
- `compose_im_required_readiness_checks`: combines a non-empty required set and fails closed when empty.
- `register_im_process_boolean_readiness_check`: registers a worker lifecycle signal before resolution.
- `bootstrap_im_service_database_from_env`: requires and installs process-shared PostgreSQL pools
  before route assembly; missing configuration and SQLite fail before listener binding.

Readiness error strings are server-side operational context. The Web Framework returns the canonical
client-safe dependency-unavailable detail and never returns database URLs, credentials, SQL errors,
provider payloads, or internal topology information.

## Configuration

- `SDKWORK_IM_DATABASE_URL`
- `SDKWORK_IM_REDIS_ENABLED`
- `SDKWORK_IM_REDIS_URL`
- `SDKWORK_IM_DEPLOYMENT_PROFILE`
- IAM database configuration resolved by the IAM/session-gateway adapter

## Related Specs

- `../../../sdkwork-specs/COMPONENT_SPEC.md`
- `../../../sdkwork-specs/HEALTH_CHECK_SPEC.md`
- `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../../sdkwork-specs/SECURITY_SPEC.md`
- `../../../sdkwork-specs/DATABASE_SPEC.md`
- `../../../sdkwork-specs/RUST_CODE_SPEC.md`
- `../../../sdkwork-specs/TEST_SPEC.md`

## Verification

```bash
cargo test -p sdkwork-im-service-readiness
cargo test -p sdkwork-api-im-standalone-gateway
```
