# Sdkwork IM Admin SDK Boundary Design

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-16
Specs: SDK_SPEC.md, SDK_WORKSPACE_GENERATION_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, BACKEND_UI_SPEC.md, SECURITY_SPEC.md

## 1. Decision

Sdkwork IM administration uses the application backend API family. The checked-in contract authority
is `apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml`, the generated transport
package is `@sdkwork/im-backend-sdk`, and the PC administration facade is
`@sdkwork/im-pc-admin-sdk`.

There is no active `apps/control-plane`, `sdkwork-control-plane-sdk`, `ControlPlaneSdkClient`, or
independent `/admin/im/v3` SDK family in this repository. Those names are not implementation targets
and must not be used by current documentation or consumers.

## 2. Ownership

| Concern | Authority |
| --- | --- |
| IM backend HTTP contract | `apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml` |
| Derived SDK input | `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml` |
| Generated TypeScript transport | `@sdkwork/im-backend-sdk` under `generated/server-openapi` |
| IAM backend administration | Dependency SDK `@sdkwork/iam-backend-sdk` |
| PC admin composition | `apps/sdkwork-im-pc/packages/sdkwork-im-pc-admin-sdk` |
| Admin UI shell | `apps/sdkwork-im-pc/packages/sdkwork-im-admin-core` |
| Production admin execution | Configured backend upstream through `SDKWORK_ADMIN_PROXY_TARGET` |
| Development-only contract sandbox | `sdkwork-api-product-runtime` with explicit `SDKWORK_ADMIN_SANDBOX=true` |

The PC facade owns base URL resolution, global TokenManager reuse, dual-token credential injection,
and SDKWork request-context interceptors. Feature packages consume the facade and never construct raw
HTTP requests or manual authentication headers.

`@sdkwork/im-admin-core/sdk/*` remains a compatibility export only. It delegates to
`@sdkwork/im-pc-admin-sdk` and does not own generated SDK construction.

## 3. Runtime Authenticity

The local admin sandbox is not a commercial control-plane implementation. It exists only for
development contract work and uses seeded or file-backed local state. Production-like environments
reject startup when `SDKWORK_ADMIN_SANDBOX` is enabled, even if the flag was set accidentally.

A production deployment must configure a real admin backend upstream. Missing upstreams fail with a
dependency-unavailable response; they must not fall back to seeded data. An OpenAPI path or generated
SDK method is not evidence that the backing billing, metering, tenant, or operations authority is
deployed.

## 4. Consumer Flow

```text
admin page
  -> admin feature service
  -> @sdkwork/im-pc-admin-sdk
  -> @sdkwork/im-backend-sdk or @sdkwork/iam-backend-sdk
  -> /backend/v3/api through the configured gateway/upstream
```

The generated transport remains generator-owned. Handwritten compatibility and browser-runtime
composition remain outside `generated/server-openapi`.

## 5. Capability Publication

- A capability may be navigable only when its generated method and production backend authority are
  both available.
- Unpublished settings, announcements, or integration capabilities render an explicit unavailable
  state without mutation controls.
- Billing SDK methods currently exist, but commercial billing readiness still depends on a real
  production billing authority and typed, non-`LooseJsonValue` response schemas.
- The development sandbox must never be used as billing, audit, metering, or tenant source of truth.

## 6. Security

- Backend-admin calls use the global TokenManager and generated SDK credential hooks.
- Consumers do not assemble `Authorization`, `Access-Token`, or `X-API-Key` headers.
- Route authorization remains server-owned; hiding navigation is not an authorization control.
- Production admin traffic must use the configured gateway/upstream, TLS, and normal backend-api
  request-context validation.

## 7. Verification

```powershell
node apps/sdkwork-im-pc/scripts/backend-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/appbase-backend-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/reachable-surface-fail-closed-contract.test.mjs
node ../sdkwork-specs/tools/check-application-layering.mjs --root .
pnpm --dir apps/sdkwork-im-pc lint
cargo test -p sdkwork-api-product-runtime
```

The PC lint command is considered green for this boundary only when it reports no IM-owned errors;
errors from linked sibling repositories remain separate workspace blockers and must be reported
verbatim.

