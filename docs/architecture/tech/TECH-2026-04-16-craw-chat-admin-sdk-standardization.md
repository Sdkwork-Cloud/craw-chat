# Sdkwork IM Admin SDK Implementation Status

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-16
Specs: SDK_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, TYPESCRIPT_CODE_SPEC.md, TEST_SPEC.md

## 1. Implemented Boundary

The active TypeScript administration path is:

```text
apps/sdkwork-im-pc admin feature package
  -> @sdkwork/im-pc-admin-sdk
  -> @sdkwork/im-backend-sdk and @sdkwork/iam-backend-sdk
```

`@sdkwork/im-pc-admin-sdk` owns the generated client construction, global TokenManager integration,
SDKWork request-context interceptors, and backend SDK response helpers. The admin UI core depends on
that facade. Its `./sdk` exports are compatibility delegates and do not create a reverse dependency
from the facade to the UI package.

## 2. Contract Sources

- IM backend authority: `apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml`
- SDK family: `sdks/sdkwork-im-backend-sdk`
- PC facade component contract:
  `apps/sdkwork-im-pc/packages/sdkwork-im-pc-admin-sdk/specs/component.spec.json`
- Admin UI component contract:
  `apps/sdkwork-im-pc/packages/sdkwork-im-admin-core/specs/component.spec.json`

Generated files under `generated/server-openapi` are updated only through the SDK generator. The PC
facade is handwritten composition and does not duplicate generated route DTOs or transport logic.

## 3. Fail-Closed Behavior

- Missing backend base URL fails client construction instead of selecting an unrelated API surface.
- Missing production admin upstream returns dependency unavailable.
- The seeded admin sandbox is development-only. Production-like runtime startup fails when the
  sandbox flag is enabled.
- UI capabilities without a published API contract display an unavailable state and expose no fake
  mutation controls.
- Dashboard and billing reads use bounded page requests and real retry actions; no permanent client
  cache or fabricated trend/alert data is accepted.

## 4. Commercial Readiness State

The SDK consumption boundary is implemented, but the administration product is not yet commercially
ready solely because its TypeScript client compiles. Production release remains blocked until all
reachable admin capabilities have a deployed, durable backend authority and typed response schemas.

Current blockers include:

- billing and metering need a real durable production authority rather than the development sandbox;
- billing summary responses must replace `LooseJsonValue` with closed OpenAPI DTOs;
- production upstream availability, authorization, audit, rate limiting, SLOs, and recovery evidence
  must be exercised in release verification;
- unpublished announcements, settings, and integration operations must remain unavailable or be
  implemented contract-first before navigation is enabled.

## 5. Verification Evidence

The following commands are the current executable checks:

```powershell
node apps/sdkwork-im-pc/scripts/backend-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/appbase-backend-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/reachable-surface-fail-closed-contract.test.mjs
node scripts/dev/sdkwork-im-component-spec-consistency.test.mjs
node scripts/dependency-management-standard.test.mjs
node ../sdkwork-specs/tools/check-application-layering.mjs --root .
pnpm --dir apps/sdkwork-im-pc lint
```

The first six checks pass as of 2026-07-16. The final PC lint reaches linked sibling source and is
currently blocked only by three pre-existing `TS7030` errors in
`sdkwork-order/.../points-recharge-dialog.tsx`; it reports no new IM admin facade error.

