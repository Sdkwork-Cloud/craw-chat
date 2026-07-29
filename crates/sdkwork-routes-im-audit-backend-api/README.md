# sdkwork-routes-im-audit-backend-api

HTTP route adapter for the IM audit backend API at `/backend/v3/api/audit`.

## Runtime composition

- `build_public_app_with_runtime` builds a standalone HTTP surface around an injected `Arc<AuditRuntime>`.
- `gateway_mount_with_runtime` mounts only the guarded business routes for a composition host.
- The application assembly injects one shared audit runtime into Audit, Portal, and Governance so their reads and writes observe the same ledger.
- The service layer owns business rules and persistence. This route crate owns only route metadata, Axum mounting, and Web Framework integration.

The public app exports `/healthz`, `/readyz`, `/livez`, `/metrics`, `/openapi.json`, and `/docs`. Readiness reports the actual dependency state and may return `503`.

## Persistence and configuration

- `SDKWORK_IM_ENVIRONMENT=dev|test` selects the in-memory development/test process backend.
- Production is the default environment and requires `SDKWORK_IM_DATABASE_URL`; missing or invalid durable storage fails startup.
- Production PostgreSQL URLs must satisfy the audit service TLS policy.
- `SDKWORK_IM_AUDIT_MAX_IN_FLIGHT_REQUESTS`, `SDKWORK_IM_AUDIT_MAX_CONCURRENT_SCANS`, and `SDKWORK_IM_AUDIT_MAX_REQUEST_BODY_BYTES` bound request and scan concurrency.

SQLite/PostgreSQL selection for application-owned stores is not implemented in this route crate and must not be inferred from its API.

## Security

Audit operations inherit backend dual-token IAM enforcement and SDKWork response/problem envelopes from the Web Framework wrapper. Raw credential parsing and persistence access are forbidden here.

## Verification

```bash
cargo test -p audit-service --test http_smoke_test
cargo check -p sdkwork-routes-im-audit-backend-api
pnpm test:component-spec-consistency
pnpm test:web-framework-standard
```

Canonical standards: `../../../sdkwork-specs/API_SPEC.md`, `../../../sdkwork-specs/WEB_BACKEND_SPEC.md`, and `../../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`.
