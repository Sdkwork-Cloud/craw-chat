# sdkwork-routes-im-governance-backend-api

HTTP route adapter for the IM governance control plane at `/backend/v3/api/control`.

## Runtime composition

- `build_public_app_with_governance_sinks` builds a standalone HTTP surface around an injected realtime cluster, Ops runtime, and Audit runtime.
- `gateway_mount_with_governance_sinks` mounts guarded control-plane routes into the application gateway.
- The application assembly injects the same realtime cluster used by Session Gateway and the same Ops/Audit instances used by their owning routes.
- Node drain, activation, route migration, and provider-policy operations therefore update the observed control-plane state and feed the shared operational/audit views.

The no-argument standalone builder creates an empty realtime cluster and environment-derived Ops/Audit runtimes. It does not claim knowledge of another process or cluster. Production Audit initialization requires durable PostgreSQL configuration and fails startup when it is unavailable.

## Configuration and limits

- `SDKWORK_IM_CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS`
- `SDKWORK_IM_CONTROL_PLANE_MAX_REQUEST_BODY_BYTES`
- `SDKWORK_IM_ENVIRONMENT` and `SDKWORK_IM_DATABASE_URL` for the Audit runtime
- Standard Ops profile, node, and bind environment keys for the Ops runtime

The public app exports `/healthz`, `/readyz`, `/livez`, `/metrics`, `/openapi.json`, `/backend/v3/api/control/openapi.json`, and `/docs`. Control-plane operations inherit backend dual-token IAM, permission checks, request limits, and SDKWork response/problem envelopes.

## Verification

```bash
cargo test -p governance-service
cargo check -p sdkwork-routes-im-governance-backend-api
pnpm test:component-spec-consistency
pnpm test:web-framework-standard
```

Canonical standards: `../../../sdkwork-specs/API_SPEC.md`, `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`, and `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`.
