> Migrated from `docs/sites/sdk/index.md` on 2026-06-24.
> Owner: SDKWork maintainers

# SDK Overview

The current SDK system has exactly four public SDK families. They are grouped by API authority and
runtime responsibility, not by historical workspace names.

This compatibility matrix is the current source of truth for SDK family ownership.

| SDK family | Owns | API authority | Current role |
| --- | --- | --- | --- |
| `sdkwork-im-sdk` | IM standardized development SDK | `/im/v3/api/*` | Product-facing IM SDK with semantic TypeScript, Flutter, and Rust lanes plus generated transport for other languages |
| `sdkwork-im-app-sdk` | App-business and non-management HTTP SDK | `/app/v3/api/*` | Generated transport SDK for app-business APIs outside the IM standardized surface |
| `sdkwork-im-backend-sdk` | Backend management, operator, control, and admin SDK | `/backend/v3/api/*` | The only backend/admin/control HTTP SDK family |
| `sdkwork-rtc-sdk` | Provider-neutral RTC runtime and provider packages | no OpenAPI route family | Independent RTC provider-standard SDK, not an OpenAPI-generated HTTP SDK |

Retired standalone control-plane and admin workspace names are not current public SDK families.
Control-plane and admin APIs are backend modules inside `sdkwork-im-backend-sdk`.

## API To SDK Map

| API group | SDK family | Rule |
| --- | --- | --- |
| `/im/v3/api/*` | `sdkwork-im-sdk` | Use for standardized IM development: conversations, messages, realtime, media, streams, IM call signaling, portal snapshots, and IM runtime helpers. |
| `/app/v3/api/*` | `sdkwork-im-app-sdk` | Use for app-business and non-management HTTP APIs that are not part of the IM standardized SDK. Provider health, IoT protocol, notifications, automation execution, and app-facing RTC provider callbacks belong here. |
| `/backend/v3/api/*` | `sdkwork-im-backend-sdk` | Use for ops, audit, automation governance, control-plane governance, node operations, and every admin route. |
| RTC provider runtime | `sdkwork-rtc-sdk` | Use for provider selection, provider package loading, native driver/runtime bridge contracts, and call runtime abstractions. |

There is no separate admin SDK family and no separate control-plane SDK family. If the route starts
with `/backend/v3/api/control/*` or `/backend/v3/api/admin/*`, it belongs to
`sdkwork-im-backend-sdk`.

## Choose By Scenario

| Need | Start with |
| --- | --- |
| Rich IM product integration in browser or Node.js | [TypeScript SDK](/sdk/typescript-sdk) |
| Flutter IM product integration | [Flutter SDK](/sdk/flutter-sdk) |
| Rust IM integration | [Rust SDK](/sdk/rust-sdk) |
| Generated app-business transport for `/app/v3/api/*` | [App API SDK](/sdk/app-sdk) |
| Backend, ops, control, or admin transport for `/backend/v3/api/*` | [Backend SDK](/sdk/backend-sdk) |
| RTC provider runtime and native driver boundary | [RTC SDK](/sdk/rtc-sdk) |
| Generated/manual ownership rules | [Generator Boundary](/sdk/generator-boundary) |
| Language maturity and package state | [Language Support](/sdk/language-support) |

## Current Workspace Truth

The checked-in workspaces and their `.sdkwork-assembly.json` snapshots are the source of truth for
repo-local generation state:

| Workspace | Current authority | Verification entry |
| --- | --- | --- |
| `sdks/sdkwork-im-sdk` | `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml` | `node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs` |
| `sdks/sdkwork-im-app-sdk` | `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml` | `node ./sdks/sdkwork-im-app-sdk/bin/verify-sdk.mjs` |
| `sdks/sdkwork-im-backend-sdk` | `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml` | `node ./sdks/sdkwork-im-backend-sdk/bin/verify-sdk.mjs` |
| `../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk` | `../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/.sdkwork-assembly.json` | `node ../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs` |

The OpenAPI-generated families share the SDKWork dual-token standard and generate from OpenAPI 3.x.
The RTC SDK is intentionally separate: it owns provider catalogs, provider package boundaries,
provider selection, native driver expectations, media runtime rules, and runtime bridge
contracts.

## Language Matrix

| Family | Languages | Primary current surface |
| --- | --- | --- |
| `sdkwork-im-sdk` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | `@sdkwork/im-sdk` for TypeScript, `im_sdk` for Flutter, `im-sdk` for Rust, generated transport for other languages |
| `sdkwork-im-app-sdk` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Generated clients such as `SdkworkAppClient` targeting `/app/v3/api/*` |
| `sdkwork-im-backend-sdk` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Generated clients such as `SdkworkBackendClient` targeting `/backend/v3/api/*` |
| `sdkwork-rtc-sdk` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Provider-standard metadata in every language, executable runtime baseline where implemented |

Generated symbols must be consumed through package root entrypoints only. Do not import private
`generated/server-openapi/src/*` paths from application code or docs snippets.

## Boundary Rules

- `sdkwork-im-sdk` must not absorb backend/admin/control APIs.
- `sdkwork-im-app-sdk` must not expose `/backend/v3/api/*`, `/im/v3/api/*`, `/admin/*`, or `/control/*` routes.
- `sdkwork-im-backend-sdk` must own all admin and control APIs, and must not absorb app-business provider or IoT APIs.
- `sdkwork-rtc-sdk` must remain independent from OpenAPI-generated HTTP SDK families.
- New non-management HTTP APIs outside IM standardization go to `/app/v3/api/*` and `sdkwork-im-app-sdk`.
- New management, governance, operator, or admin APIs go to `/backend/v3/api/*` and `sdkwork-im-backend-sdk`.

## What To Read Next

- [TypeScript SDK](/sdk/typescript-sdk)
- [Flutter SDK](/sdk/flutter-sdk)
- [App API SDK](/sdk/app-sdk)
- [Backend SDK](/sdk/backend-sdk)
- [RTC SDK](/sdk/rtc-sdk)
- [Language Support](/sdk/language-support)
- [Generator Boundary](/sdk/generator-boundary)
- [API Reference](/api-reference/index)

