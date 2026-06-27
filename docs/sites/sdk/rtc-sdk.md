# RTC SDK

`sdkwork-rtc-sdk` is an independent RTC provider-standard SDK family. It is not generated from OpenAPI.
It is not generated from
OpenAPI and it must not be collapsed into `sdkwork-im-sdk`, `sdkwork-im-app-sdk`, or
`sdkwork-im-backend-sdk`.

The app and backend OpenAPI SDKs may expose HTTP routes related to provider health, callbacks, or
management. IM exposes call signaling through `sdk.calls`. The RTC SDK owns a different layer:
provider selection, provider package boundaries, native runtime bridges, capability negotiation,
and provider-neutral media runtime contracts.

## Owns

| Boundary | Standard |
| --- | --- |
| SDK workspace root | `../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk` |
| Authority snapshot | `../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/.sdkwork-assembly.json` |
| Architecture | Provider-standard runtime SDK |
| Default provider identity | `volcengine`, `rtc-volcengine`, `sdkwork-rtc-driver-volcengine` |
| Primary verification | `node ../../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs` |

## Responsibilities

- Provider catalog and provider selection rules.
- Provider package catalogs and package-boundary loading.
- Runtime bridge contracts for native SDK integration and native driver ownership.
- Capability catalog and capability negotiation.
- Shared provider runtime vocabulary and media bridge expectations.
- Provider activation metadata across official languages.
- Root public surface rules for provider-neutral and builtin-provider exports.

## Does Not Own

- `/im/v3/api/*` HTTP routes; use `sdkwork-im-sdk`.
- `/app/v3/api/*` HTTP routes; use `sdkwork-im-app-sdk`.
- `/backend/v3/api/*` HTTP routes; use `sdkwork-im-backend-sdk`.
- Admin or control-plane HTTP APIs.

## Verification

Run from the RTC SDK workspace:

```powershell
node .\bin\verify-sdk.mjs
```

Or from the repository root:

```powershell
node ../../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs
```

Use full smoke only when the required language toolchains are available:

```powershell
node ../../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\smoke-sdk.mjs
```

## Related Docs

- [SDK Overview](/sdk/index)
- [IM Calls API](/api-reference/im/calls)
- `../../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md`
