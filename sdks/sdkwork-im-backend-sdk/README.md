# SDKWork IM Backend SDK

`sdkwork-im-backend-sdk` is the `/backend/v3/api` SDK family for backend management, operator,
control-plane, admin-console, and service-side Sdkwork IM integration.

It is intentionally separate from both public IM and app-development SDKs:

- `sdkwork-im-sdk` targets `/im/v3/api`.
- `sdkwork-im-app-sdk` targets `/app/v3/api`.
- `sdkwork-im-backend-sdk` targets `/backend/v3/api`.
- `sdkwork-im-backend-sdk` depends on `sdkwork-iam-backend-sdk` for appbase backend
  management capability and must not regenerate appbase-owned backend routes.

Control-plane and admin are modules in this workspace, not separate SDK families. The backend
authority explicitly owns:

- `/backend/v3/api/ops/*`
- `/backend/v3/api/audit/*`
- `/backend/v3/api/automation/*`
- `/backend/v3/api/control/*`
- `/backend/v3/api/admin/*`

Identity, token refresh, account, tenant, and organization context are supplied by the upstream
platform. This SDK only consumes propagated backend context and does not expose login or client
client-route lifecycle APIs.

## SDK Dependency Contract

The backend SDK is the Sdkwork IM backend/operator composition point, but its generated transport
remains scoped to Sdkwork IM-owned `/backend/v3/api` routes only.

- `sdkwork-iam-backend-sdk` remains the owner of appbase `/backend/v3/api` backend management
  capability.
- `sdkwork-im-backend-sdk` owns Sdkwork IM backend modules such as ops, audit, automation,
  protocol control, and admin API key management.
- Generated backend transport must not import, vendor, or regenerate
  `sdkwork-iam-backend-sdk`; consumers compose the appbase backend SDK at the backend SDK
  boundary.

Machine-readable contract fields:

| Field | `sdkwork-iam-backend-sdk` |
| --- | --- |
| `sdkDependencies[].workspace` | `sdkwork-iam-backend-sdk` |
| `sdkDependencies[].role` | `appbase-backend-management-capability` |
| `sdkDependencies[].required` | `true` |
| `sdkDependencies[].dependencyMode` | `consumer-sdk` |
| `sdkDependencies[].apiPrefix` | `/backend/v3/api` |
| `sdkDependencies[].generatedTransportImportPolicy` | `forbidden` |

Package-level dependency names:

| Language | Appbase backend SDK dependency |
| --- | --- |
| TypeScript | `@sdkwork/iam-backend-sdk` |
| Flutter | `sdkwork_iam_backend_sdk` |
| Rust | `sdkwork-iam-backend-sdk` |
| Java | `com.sdkwork:sdkwork-iam-backend-sdk` |
| C# | `SDKWork.Appbase.BackendSdk` |
| Swift | `sdkwork-iam-backend-sdk` |
| Kotlin | `com.sdkwork:sdkwork-iam-backend-sdk` |
| Go | `github.com/sdkwork/sdkwork-iam-backend-sdk` |
| Python | `sdkwork-iam-backend-sdk` |

## Contract Files

- `openapi/sdkwork-im-backend-api.openapi.yaml`
  Authority OpenAPI 3.x contract for `/backend/v3/api`.
- `openapi/sdkwork-im-backend-api.sdkgen.yaml`
  Generator-compatible derived input.

## Generation

Primary Node entrypoint:

```powershell
node .\bin\generate-sdk.mjs --language typescript
```

PowerShell:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript
```

Bash:

```bash
./bin/generate-sdk.sh --language typescript
```

Defaults:

- base URL: `http://127.0.0.1:18079`
- schema URL: `/backend/v3/openapi.json`
- API prefix: `/backend/v3/api`
- SDK name: `sdkwork-im-backend-sdk`
- SDK target/type: `backend`
- standard profile: `sdkwork-v3`

Generated output is written under language-specific `sdkwork-im-backend-sdk-*` directories. Do not
edit generated output by hand.

## Flutter Layered Boundary

Flutter keeps both generated and manual-owned layers:

- generated transport package:
  `sdkwork-im-backend-sdk-flutter/generated/server-openapi` (`im_backend_api_generated`)
- consumer-facing composed package:
  `sdkwork-im-backend-sdk-flutter/composed` (`im_backend_sdk`)

The composed package re-exports generated transport and provides `ImBackendSdkClient` plus semantic
modules (`ops`, `audit`, `automation`, `control`, `admin`). Keep HTTP transport ownership in
generated output and place manual ergonomics only in `composed`.

The repository-wide boundary materialization command is:

```powershell
node ..\materialize-im-v3-openapi-boundaries.mjs
```

Run it before generation when control-plane, admin, app-business, or backend grouping changes. It
consolidates control/admin authority into this workspace and keeps non-management HTTP APIs in the
app SDK family.

## Verification

```powershell
node .\bin\verify-sdk.mjs
```

The verifier checks the `/backend/v3/api` OpenAPI surface, dual-token `AuthToken` and `AccessToken`
security, problem-detail errors, generated language manifests, TypeScript
`SdkworkImBackendClient` plus `SdkworkBackendClient` compatibility alias surface parity, and
Flutter composed workspace presence/contracts.

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = template_only_pending_generation`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## SDKWork Documentation Contract

Domain: communication
Capability: im
Package type: sdk-family
Status: standardizing

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- `SdkworkImBackendClient`
- `SdkworkBackendClient`

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `node apps/scripts/validate-component-specs.mjs --apps-root apps --json`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
