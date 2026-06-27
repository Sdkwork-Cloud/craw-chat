> Migrated from `docs/sites/sdk/app-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# App API SDK

`sdkwork-im-app-sdk` is the generated HTTP SDK family for app-business and non-management APIs
under `/app/v3/api/*`.

It is separate from `sdkwork-im-sdk`. Use `sdkwork-im-sdk` for standardized IM development and
semantic product integrations. Use `sdkwork-im-app-sdk` when you need route-aligned generated
transport for app-business APIs outside the standardized IM SDK surface.

## Owns

| Boundary | Standard |
| --- | --- |
| SDK workspace root | `sdks/sdkwork-im-app-sdk` |
| API prefix | `/app/v3/api` |
| Schema discovery | `/app/v3/openapi.json` |
| Authority snapshot | `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml` |
| Derived generator input | `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.sdkgen.yaml` |
| Flutter HTTP generator input | `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.flutter.sdkgen.yaml` |
| Primary generated TypeScript client | `SdkworkAppClient` |

The family currently includes app-business routes such as portal snapshots, app-facing device and
social routes, notifications, automation execution, media/provider health, principal-profile
provider health, IoT protocol and provider health, and app-facing RTC provider callbacks or health.

## Does Not Own

- `/im/v3/api/*`; use `sdkwork-im-sdk`.
- `/backend/v3/api/*`; use `sdkwork-im-backend-sdk`.
- `/backend/v3/api/control/*`; control is a backend module.
- `/backend/v3/api/admin/*`; admin is a backend module.
- RTC provider runtime and native driver contracts; use `sdkwork-rtc-sdk`.

## Language Workspaces

`sdkwork-im-app-sdk` materializes generated transport workspaces for TypeScript, Flutter, Rust,
Java, C#, Swift, Kotlin, Go, and Python. The TypeScript generated package exposes
`SdkworkAppClient`; other languages follow the same generated transport standard.

The generated SDK is intentionally transport-level. Do not add handwritten business facades inside
`generated/server-openapi`. If a semantic wrapper is needed later, keep it outside generated output
and keep OpenAPI as the source of truth.

## Assembly Metadata

The workspace assembly snapshot is `sdks/sdkwork-im-app-sdk/.sdkwork-assembly.json`.

Use it to verify `manifestPath`, `generatedAt`, language workspace names, generated package paths,
and release state. This family is generated transport first; do not invent a composed public layer
inside generated output just to hide a missing OpenAPI operation.

## Verification

Run from the repository root:

```powershell
node .\sdks\sdkwork-im-app-sdk\bin\verify-sdk.mjs
```

Regenerate a language from OpenAPI inputs with:

```powershell
node .\sdks\sdkwork-im-app-sdk\bin\generate-sdk.mjs --language typescript
```

The verifier enforces `/app/v3/api/*` ownership, SDKWork dual-token security, generated output
structure, assembly metadata, and the rule that the family must not contain backend, admin, or
control routes.

In short: this family must not contain backend, admin, or control routes.

## Related API Docs

- [App API Overview](/api-reference/app-api)
- [SDK Overview](/sdk/index)

