# sdkwork-routes-im-portal-app-api

HTTP route adapter for IM portal snapshots at `/app/v3/api/portal`.

## Runtime composition

- `build_public_app_with_runtime` builds a standalone HTTP surface around an injected `Arc<PortalRuntime>`.
- `gateway_mount_with_runtime` mounts guarded portal routes into the application gateway.
- `PortalRuntime` reads from injected Ops and Audit runtimes; it is not an independent persistence authority.
- The application assembly injects the same Ops and Audit instances used by their owning routes, so portal snapshots do not read disconnected process-local copies.
- The standalone builder creates its own Ops and Audit runtimes from environment configuration for a dedicated portal host.

The public app exports `/healthz`, `/readyz`, `/livez`, `/metrics`, `/openapi.json`, and `/docs`. Readiness reflects actual dependency state. Missing production Audit storage fails startup through `AuditRuntime::from_env()`.

## Configuration and limits

- `SDKWORK_IM_ENVIRONMENT` and `SDKWORK_DATABASE_URL` govern the injected/default Audit runtime.
- Ops profile and bind values use the standard Ops environment keys.
- `SDKWORK_IM_PORTAL_MAX_IN_FLIGHT_REQUESTS` bounds concurrent portal requests.

Portal routes inherit app dual-token IAM enforcement and SDKWork response envelopes from the Web Framework wrapper. Business logic, raw HTTP clients, credential parsing, and storage access are forbidden in this crate.

## Verification

```bash
cargo test -p portal-service --test http_smoke_test
cargo check -p sdkwork-routes-im-portal-app-api
pnpm test:component-spec-consistency
pnpm test:web-framework-standard
```

Canonical standards: `../../../sdkwork-specs/API_SPEC.md`, `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`, and `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`.
