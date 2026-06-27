> Migrated from `docs/sites/sdk/language-support.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Language Support

Language support is measured by SDK family, authority boundary, generated or manual ownership, and
repo verification state. A checked-in workspace does not automatically imply registry publication.

## Current Verified Baseline

The current verified baseline is the four-family SDK model: IM standard, App API, Backend API, and
RTC provider runtime. Use this page to confirm which family owns a route or runtime boundary before
choosing a language package.

## How To Use This Page

Start from the authority boundary, not the target language. This page is the repo contract for the
current SDK surface: `/im/v3/api/*` belongs to `sdkwork-im-sdk`, `/app/v3/api/*` belongs to
`sdkwork-im-app-sdk`, `/backend/v3/api/*` belongs to `sdkwork-im-backend-sdk`, and provider-runtime
RTC integration belongs to `sdkwork-rtc-sdk`.

After the family is selected, choose the package or generated transport for the target language and
run that family's verification command before publishing or consuming artifacts. Release semantics
are workspace-based: `.sdkwork-assembly.json` records package ownership and release state, while
checked-in generated output does not by itself mean registry publication.

## Current SDK Families

| Family | Authority | Languages | Current role |
| --- | --- | --- | --- |
| `sdkwork-im-sdk` | `/im/v3/api/*` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Standardized IM development SDK; semantic TypeScript, Flutter, and Rust lanes plus generated transport lanes |
| `sdkwork-im-app-sdk` | `/app/v3/api/*` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | App-business generated HTTP SDK; non-management APIs outside IM standardization |
| `sdkwork-im-backend-sdk` | `/backend/v3/api/*` | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Backend management generated HTTP SDK; includes control and admin modules |
| `sdkwork-rtc-sdk` | Provider runtime standard | TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, Python | Independent RTC provider-standard SDK, not an OpenAPI-generated HTTP SDK |

## Package And Client Baseline

| Family | TypeScript | Flutter | Other languages |
| --- | --- | --- | --- |
| `sdkwork-im-sdk` | `@sdkwork/im-sdk`, `ImSdkClient` | `im_sdk_composed`, `im_sdk_generated`, `ImSdkComposedClient` | Rust ships `im-sdk`; Java, C#, Swift, Kotlin, Go, and Python keep generated transport plus composed reserves |
| `sdkwork-im-app-sdk` | Generated `SdkworkAppClient` | Generated app API package | Generated transport packages |
| `sdkwork-im-backend-sdk` | Generated `SdkworkBackendClient` | Generated backend API package | Generated transport packages |
| `sdkwork-rtc-sdk` | Provider-standard runtime and catalogs | Mobile/provider metadata and runtime boundary | Provider metadata, package boundary, and future runtime bridge scaffolds |

The generated HTTP SDK families all use `generated/server-openapi` as the generator-owned boundary.
Handwritten semantic code must stay outside generated output.

## Boundary Selection

- Use `sdkwork-im-sdk` when the API is standardized IM development under `/im/v3/api/*`.
- Use `sdkwork-im-app-sdk` when the API is non-management app-business HTTP under `/app/v3/api/*`.
- Use `sdkwork-im-backend-sdk` when the API is backend, operator, governance, control, or admin
  under `/backend/v3/api/*`.
- Use `sdkwork-rtc-sdk` when the task is provider selection, native RTC runtime integration,
  provider package loading, or capability negotiation.

There are no current standalone admin or control-plane SDK families. `/backend/v3/api/control/*`
and `/backend/v3/api/admin/*` are backend SDK modules.

## Verification Signals

| Family | Verification |
| --- | --- |
| `sdkwork-im-sdk` | `node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs` |
| `sdkwork-im-app-sdk` | `node ./sdks/sdkwork-im-app-sdk/bin/verify-sdk.mjs` |
| `sdkwork-im-backend-sdk` | `node ./sdks/sdkwork-im-backend-sdk/bin/verify-sdk.mjs` |
| `sdkwork-rtc-sdk` | `node ../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs` |

Use `.sdkwork-assembly.json` in each workspace for package-layer ownership, `manifestPath`,
generated output path, and release-state facts.

## What To Read Next

- [SDK Overview](/sdk/index)
- [TypeScript SDK](/sdk/typescript-sdk)
- [Flutter SDK](/sdk/flutter-sdk)
- [App API SDK](/sdk/app-sdk)
- [Backend SDK](/sdk/backend-sdk)
- [RTC SDK](/sdk/rtc-sdk)
- [Generator Boundary](/sdk/generator-boundary)

