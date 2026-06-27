# SDK Workspace Overview

## Purpose

Repository-owned SDK families, OpenAPI authorities, RPC manifests, generated transports, and
verification entrypoints for Sdkwork IM.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- SDK family directories, OpenAPI authorities, derived generator inputs, generated transports,
  composed facades, RPC manifests, and workspace verification scripts.
- Checked-in assembly metadata and family-local `specs/` contracts.

## Forbidden Content

- Hand-edited generated SDK output without a verified source contract refresh.
- Raw HTTP client wrappers that bypass generated SDK transport boundaries.
- Runtime caches or local generator control-plane artifacts under repository `.sdkwork/`.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/SDK_SPEC.md`
- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run the family verifier scripts documented below and governance checks from the repository root
after SDK workspace layout or authority changes:

```bash
pnpm test:topology-baggage
pnpm test:sdkwork-workspace-structure-standard
node sdks/test/verify-im-v3-sdk-family-contract.test.mjs
```

`sdks/` is the repository home for Sdkwork IM SDK workspaces. The directory is organized by public
consumer SDK family and by authoritative API boundary, not by historical generated-package dumps.

The current standard model has three OpenAPI-generated HTTP SDK families and one communication
gRPC/RPC SDK family checked into this repository, plus one RTC provider-standard SDK family that
lives only in the sibling `../sdkwork-rtc` repository:

- `sdkwork-im-sdk`
  IM standardized development SDKs for `/im/v3/api`; logical API authority:
  `sdkwork-im-open-api`.
- `sdkwork-im-app-sdk`
  App-business and non-management HTTP SDKs for `/app/v3/api`; app SDK composition depends on
  `sdkwork-im-sdk` and sibling `sdkwork-rtc-sdk`; logical API authority: `sdkwork-im-app-api`.
- `sdkwork-im-backend-sdk`
  Backend management, operator, control-plane, and admin SDKs for `/backend/v3/api`; logical API
  authority: `sdkwork-im-backend-api`.
- `sdkwork-im-rpc-sdk`
  gRPC/protobuf SDK family for distributed communication service integration. It maps the current
  IM app/backend/internal runtime capabilities into `sdkwork.communication.*` proto packages and is
  generated with `sdkgen generate --protocol rpc`.
- `sdkwork-rtc-sdk` (sibling repo)
  Provider-standard RTC SDKs for multi-provider audio/video runtime integration. Authority path:
  `../sdkwork-rtc/sdks/sdkwork-rtc-sdk`. **Must not** be materialized under this repository's
  `sdks/` directory.

## Current Repository Truth

For day-to-day engineering, the checked-in SDK workspaces and their `.sdkwork-assembly.json`
snapshots are the source of truth.

The three Sdkwork IM HTTP-contract SDK families are separated by target surface:

- `sdkwork-im-sdk` owns `sdkwork-im-open-api`, the IM standardized development API under
  `/im/v3/api/*`.
- `sdkwork-im-app-sdk` owns `sdkwork-im-app-api`, the app-business API under `/app/v3/api/*`, and
  declares SDK dependencies on `sdkwork-im-sdk` plus `sdkwork-rtc-sdk` for the full app integration
  stack.
- `sdkwork-im-backend-sdk` owns `sdkwork-im-backend-api`, the backend management API under
  `/backend/v3/api/*`.

The RTC workspace is intentionally separate from the OpenAPI-generated HTTP SDK families and from
this repository checkout:

- `sdkwork-rtc-sdk` lives in `../sdkwork-rtc/sdks/sdkwork-rtc-sdk` and owns provider standards,
  driver contracts, provider catalogs, runtime surface rules, and provider package boundaries.
- It is not a route-generated SDK workspace and must not be collapsed into app or backend generated
  transport packages or copied into `sdkwork-im/sdks/`.

The communication RPC workspace is intentionally separate from the OpenAPI-generated HTTP SDK
families:

- `sdkwork-im-rpc-sdk` owns proto contracts, the RPC manifest, and generated RPC SDK
  scaffolds for distributed backend, private, local, native host, and service-to-service
  integration.
- It does not replace `sdkwork-im-sdk`, `sdkwork-im-app-sdk`, or `sdkwork-im-backend-sdk`; browser
  and app UI packages continue to use those HTTP SDKs unless gRPC-Web is explicitly approved.

## API Boundary Rules

Every API must map to exactly one SDK family:

- IM standardized development API: `/im/v3/api/*` -> `sdkwork-im-open-api` ->
  `sdkwork-im-sdk`.
- App-business and non-management API: `/app/v3/api/*` -> `sdkwork-im-app-api` ->
  `sdkwork-im-app-sdk`.
- Backend management, operator, governance, control-plane, and admin API:
  `/backend/v3/api/*` -> `sdkwork-im-backend-api` -> `sdkwork-im-backend-sdk`.
- RTC provider/runtime standard: sibling `../sdkwork-rtc/sdks/sdkwork-rtc-sdk`, not an OpenAPI HTTP
  family and not a local `sdks/` workspace.

Backend management modules currently include:

- `/backend/v3/api/ops/*`
- `/backend/v3/api/audit/*`
- `/backend/v3/api/automation/*`
- `/backend/v3/api/control/*`
- `/backend/v3/api/admin/*`

App API absorbs HTTP surfaces that are not management-system APIs and are not part of the IM
standardized development API. Representative examples include provider health, IoT protocol, and
RTC provider callback or health routes after they are exposed under `/app/v3/api/*`.

## Language Baseline

The official OpenAPI-generated language set for the three HTTP SDK families is:

- TypeScript
- Flutter
- Rust
- Java
- C#
- Swift
- Kotlin
- Go
- Python

The official RPC SDK baseline language set for `sdkwork-im-rpc-sdk` is:

- TypeScript
- Go
- Java
- Python
- Rust

For OpenAPI-generated SDK families, generator-owned transport output is always under
`generated/server-openapi`. Manual-owned consumer-facing facades live under `composed` only when a
family has a semantic SDK layer.

Current Flutter semantic package mapping:

- `sdkwork-im-sdk-flutter/composed` -> `im_sdk`
- `sdkwork-im-app-sdk-flutter/composed` -> `im_app_sdk`
- `sdkwork-im-backend-sdk-flutter/composed` -> `im_backend_sdk`

The IM TypeScript line publishes the consumer package `@sdkwork/im-sdk`; its generated TypeScript
transport package uses `@sdkwork-internal/im-sdk-generated` as a workspace-internal identity only.
The IM Flutter line publishes `im_sdk` for consumers and keeps `im_sdk_generated` as the generated
transport package.

## Workspace Matrix

| Workspace | Audience | Languages | Primary package boundary |
| --- | --- | --- | --- |
| `sdkwork-im-sdk` | IM standardized development integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Semantic IM SDK package `@sdkwork/im-sdk` plus internal generated transports `@sdkwork-internal/im-sdk-generated` and `im_sdk_generated` |
| `sdkwork-im-app-sdk` | App developers and app-business integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Generated app transport plus Flutter composed `im_app_sdk` with semantic `ImAppSdkClient`; family-level dependencies on `sdkwork-im-sdk` and `sdkwork-rtc-sdk` |
| `sdkwork-im-backend-sdk` | Backend, operator, control-plane, and admin integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Generated backend transport plus Flutter composed `im_backend_sdk` with semantic `ImBackendSdkClient` |
| `sdkwork-im-rpc-sdk` | Distributed backend, private, local, native-host, and service-to-service communication integrations | TypeScript, Go, Java, Python, Rust | gRPC/protobuf SDK family generated from `proto/` and `rpc/sdkwork-im-rpc.manifest.json` |
| `sdkwork-rtc-sdk` (sibling) | RTC provider-standard integrations | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | `../sdkwork-rtc/sdks/sdkwork-rtc-sdk`; provider-standard packages and adapters, not OpenAPI-generated transport |

All current package lines remain `not_published` until a release freeze assigns publishable
versions. The release snapshot is recorded in
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- IM, App API, and Backend API HTTP SDK artifacts use `state = generated_pending_publication` and
  `generationStatus = generated`.
- RTC provider-runtime SDK artifacts use `state = template_only_pending_generation` and
  `generationStatus = template_only_pending_generation`.
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

## Standard Package Boundary

Every OpenAPI-generated SDK family follows the same boundary rules:

- Family naming, API authority naming, OpenAPI authority placement, derived generator input
  placement, and generated-output placement follow
  [`SDK_WORKSPACE_GENERATION_SPEC.md`](../../../specs/SDK_WORKSPACE_GENERATION_SPEC.md).
- The OpenAPI 3.x authority contract is checked into the workspace under `openapi/`.
- Generator-compatible derived contracts stay in the same workspace and remain traceable to the
  authority contract.
- Generated code is owned only under `generated/server-openapi`.
- Consumer-facing orchestration, ergonomic client facades, and manual integration helpers live only
  under `composed` when that SDK family has a semantic manual layer.
- Manual code must consume generated output through package root entrypoints only.
- Downstream code must not import generated private source paths such as
  `generated/server-openapi/src/*` or language-specific private internals.
- `/app/v3/api` and `/backend/v3/api` SDK families use the shared `sdkwork-v3` generation profile,
  dual-token `AuthToken` plus `AccessToken` security, and `application/problem+json` error
  responses.

`sdkwork-rtc-sdk` intentionally differs from the OpenAPI-first families:

- it is a provider-standard workspace, not a route-generated SDK workspace
- it standardizes `Driver`, `DriverManager`, `DataSource`, `capabilities`, and `unwrap()`
- it materializes provider adapters instead of generated transport packages

## Regeneration And Verification

The standard SDK boundary materialization entrypoint is:

```powershell
node .\sdks\materialize-im-v3-openapi-boundaries.mjs
```

That command consolidates backend control/admin authority into `sdkwork-im-backend-sdk`, keeps
non-management HTTP APIs in `sdkwork-im-app-sdk`, and refreshes derived OpenAPI inputs.

Each OpenAPI workspace then owns its own wrappers under `bin/`:

```powershell
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-app-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-backend-sdk\bin\verify-sdk.mjs
```

Run the RPC contract check before generating RPC SDK scaffolds:

```powershell
node .\scripts\dev\sdkwork-im-rpc-contract.test.mjs
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language typescript --package-name @sdkwork/im-rpc-sdk --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-go --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language go --package-name github.com/sdkwork/im-rpc-sdk-go --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-java --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language java --package-name com.sdkwork.im.rpc --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-python --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language python --package-name sdkwork_im_rpc_sdk --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language rust --package-name sdkwork-im-rpc-sdk-rust --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-go --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-java --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-python --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust --json
```

Use the RTC verifier for the independent provider-standard SDK:

```powershell
node ..\sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs
```

## Shared SDK Materialization Modes

Sdkwork IM consumes sibling SDK families through git materialization today; a published npm registry is not yet the default path.

| Mode | When | Entry |
| --- | --- | --- |
| `source` (default dev) | sibling repos already checked out under the workspace root | `SDKWORK_SHARED_SDK_MODE=source` + `pnpm sdk:shared:prepare` |
| `git` (CI / release) | clean agents without sibling checkouts | `node scripts/prepare-ci-dependencies.mjs --apply` and `node scripts/dev/prepare-shared-sdk-git-sources.mjs` |
| `registry` (future) | only after SDK families publish versioned npm packages | not enabled; do not bypass generated SDK boundaries with raw HTTP |

Pin authorities:

- CI workflow siblings: `sdkwork.workflow.json`
- PC/shared SDK git refs: `config/shared-sdk-release-sources.json`

Release status for generated families remains `not_published` until registry packages ship; see `artifacts/releases/sync-sdk-release-catalog.mjs`.

## Recommended Reading

- [`sdks/sdkwork-im-sdk/README.md`](./sdkwork-im-sdk/README.md)
- [`sdks/sdkwork-im-app-sdk/README.md`](./sdkwork-im-app-sdk/README.md)
- [`sdks/sdkwork-im-backend-sdk/README.md`](./sdkwork-im-backend-sdk/README.md)
- [`../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md`](../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md)
- [`docs/sites/sdk/index.md`](../docs/sites/sdk/index.md)
