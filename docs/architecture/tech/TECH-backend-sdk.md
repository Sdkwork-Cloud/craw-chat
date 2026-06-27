> Migrated from `docs/sites/sdk/backend-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Backend SDK

`sdkwork-im-backend-sdk` is the only generated HTTP SDK family for backend management, operator,
control-plane, and admin APIs under `/backend/v3/api/*`.

Control and admin are modules inside this backend SDK family. They are not separate public SDK
families.

## Owns

| Boundary | Standard |
| --- | --- |
| SDK workspace root | `sdks/sdkwork-im-backend-sdk` |
| API prefix | `/backend/v3/api` |
| Schema discovery | `/backend/v3/openapi.json` |
| Authority snapshot | `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml` |
| Derived generator input | `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.sdkgen.yaml` |
| Primary generated TypeScript client | `SdkworkBackendClient` |

Current backend modules include:

| Module | Route prefix |
| --- | --- |
| Ops | `/backend/v3/api/ops/*` |
| Audit | `/backend/v3/api/audit/*` |
| Automation governance | `/backend/v3/api/automation/*` |
| Control-plane governance | `/backend/v3/api/control/*` |
| Admin | `/backend/v3/api/admin/*` |

## Does Not Own

- `/im/v3/api/*`; use `sdkwork-im-sdk`.
- `/app/v3/api/*`; use `sdkwork-im-app-sdk`.
- Non-management provider health, IoT protocol, app-facing notifications, app-facing automation
  execution, or app-facing RTC provider callbacks; these belong under `/app/v3/api/*`.
- RTC provider runtime and native driver contracts; use `sdkwork-rtc-sdk`.

## Control And Admin Standard

All control-plane and admin routes are generated from the backend authority snapshot:

- `/backend/v3/api/control/*` uses backend control modules for protocol registry, provider policy,
  social graph control, shared-channel runtime repair, and node lifecycle.
- `/backend/v3/api/admin/*` uses backend admin modules for API keys, provider/routing/catalog
  administration, billing/usage, storage, credentials, and gateway policy.

Do not introduce a new admin SDK family. If a backend route is missing from generated output, fix
the backend OpenAPI authority or materialization script and regenerate `sdkwork-im-backend-sdk`.

## Verification

Run from the repository root:

```powershell
node .\sdks\sdkwork-im-backend-sdk\bin\verify-sdk.mjs
```

Regenerate a language from OpenAPI inputs with:

```powershell
node .\sdks\sdkwork-im-backend-sdk\bin\generate-sdk.mjs --language typescript
```

The verifier enforces `/backend/v3/api/*` ownership, required ops/audit/automation/control/admin
paths, SDKWork dual-token security, generated output structure, and assembly metadata.

## Related API Docs

- [Backend API Overview](/api-reference/backend-api)
- [Control Module Overview](/api-reference/control-plane-api)
- [Admin Storage Contract](/reference/admin-storage-contract)
- [SDK Overview](/sdk/index)

