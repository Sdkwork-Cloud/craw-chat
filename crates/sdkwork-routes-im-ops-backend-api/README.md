# sdkwork-routes-im-ops-backend-api

HTTP route adapter for the IM operations backend API at `/backend/v3/api/ops`.

## Runtime composition

- `build_public_app_with_runtime` builds a standalone HTTP surface around an injected `Arc<OpsRuntime>`.
- `gateway_mount_with_runtime` mounts guarded business routes into the application gateway.
- The application assembly shares one Ops runtime with Ops, Portal, and Governance.
- When the embedded realtime plane is present, the assembly mirrors observed lifecycle, route ownership, and inbox diagnostics into that runtime once per second.

The mirror is bounded to 200 route records while retaining the exact total, and blocking diagnostic reads run outside Tokio worker threads. A diagnostic read failure immediately resets realtime inbox health to `unavailable`; stale health is never retained as current evidence.

## Truthful status

`OpsRuntime::from_env()` does not synthesize a node, service health, route count, or healthy lifecycle. Without an observed realtime bootstrap the cluster node list is empty and unavailable diagnostics remain unavailable. The embedded gateway publishes a node only after supplying its actual node ID.

## Configuration

- `SDKWORK_IM_PROFILE_ID`
- `SDKWORK_IM_REALTIME_NODE_ID`
- `SDKWORK_IM_OPS_SERVICE_BIND_ADDR`, with `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` as the bind fallback
- `SDKWORK_IM_OPS_MAX_IN_FLIGHT_REQUESTS`
- `SDKWORK_IM_OPS_MAX_REQUEST_BODY_BYTES`
- `SDKWORK_IM_DATABASE_URL` for database-backed operational actions such as retention purge

The public app exports `/healthz`, `/readyz`, `/livez`, `/metrics`, `/openapi.json`, and `/docs`. `/metrics` includes the shared HTTP metrics and retention-purge metrics.

## Verification

```bash
cargo test -p ops-service
cargo check -p sdkwork-routes-im-ops-backend-api
pnpm test:component-spec-consistency
pnpm test:web-framework-standard
```

Canonical standards: `../../../sdkwork-specs/API_SPEC.md`, `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`, and `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`.
