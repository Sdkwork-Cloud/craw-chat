# PC Integration Adapter Register

Authority: `sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`, `sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`.

**Status (2026-07-06):** End-user sidebar domains mail / shop / orders / devices / community are **thin IM adapters**; UI, services, and SDK wiring live in sibling product repos. IM owns chat, session, IAM runtime, and host bridges only.

## Migrated — thin adapter (closed)

| IM package | Canonical product | Session bridge | Notes |
| --- | --- | --- | --- |
| `@sdkwork/im-pc-mail` | `@sdkwork/mail-pc-mail` | `mailPcIntegration` | `bootstrapImMailPcHost` wires `@sdkwork/mail-pc-core` client |
| `@sdkwork/im-pc-shop` | `@sdkwork/shop-pc-consumer` | `commercePcIntegration` + `bootstrapImShopPcHost` | Re-exports `ShopView` only |
| `@sdkwork/im-pc-orders` | `@sdkwork/shop-pc-orders` | `commercePcIntegration` | Re-exports orders surfaces only |
| `@sdkwork/im-pc-devices` | `@sdkwork/aiot-pc-console-device` | `aiotPcIntegration` | Embeds `SdkworkDevicePage` only |
| `@sdkwork/im-pc-community` | `@sdkwork/community-pc-community` | `communityPcIntegration` + `bootstrapImCommunityPcHost` | Host adapter injects SDK port + toast/avatar |
| Course (sidebar) | `@sdkwork/course-pc-course` | `coursePcIntegration` | Shell lazy-loads canonical course package; **no** `im-pc-course` package |

**Removed from IM (must not reintroduce):**

- `im-pc-core/src/sdk/{catalog,shop,order,aiot}AppSdkClient.ts`
- `im-pc-{shop,orders,mail,devices}/src/services/*Service.ts`
- `im-pc-devices/src/components/{BindAgentModal,DeviceDetailPanel}.tsx`
- `im-pc-shop/src/components/*` (ShopHome, CheckoutView, CashierView, …)
- `packages/sdkwork-im-pc-course` (use `@sdkwork/course-pc-course` from shell lazy loader instead)

## Host bridge only (IM-owned, intentional)

These files sync IM session → sibling `*-pc-core` or inject host ports. They must **not** duplicate product UI or domain services.

| Bridge | Delegates to |
| --- | --- |
| `mailPcIntegration` | `@sdkwork/mail-pc-core` |
| `commercePcIntegration` | `@sdkwork/shop-pc-core` |
| `aiotPcIntegration` | `@sdkwork/aiot-pc-core` |
| `communityPcIntegration` | `@sdkwork/community-app-sdk` via IM session interceptors |
| `coursePcIntegration` | `@sdkwork/course-pc-course` host ports + IM session-scoped `@sdkwork/course-app-sdk` client |
| `drivePcIntegration` | `@sdkwork/drive-pc-drive` host ports (client still IM-local pending drive session bridge) |
| `coursePcIntegration` | `@sdkwork/course-pc-core` host ports |
| `voicePcIntegration` | `@sdkwork/voice-pc-core` host ports |
| `knowledgebasePcIntegration` | `@sdkwork/knowledgebase-pc-core` host ports |
| `notaryPcIntegration` | `@sdkwork/notary-pc-core` host ports |
| `membershipPcIntegration` | `@sdkwork/membership-app-sdk` (shim removal pending) |

IM chat runtime (intentional, not debt): `imSdkClient`, `appSdkClient`, `session`, `appAuthRuntime`, `pcRealtimeConnectionManager`, `appbaseAppSdkClient`.

## Residual P2 (documented, non-blocking pre-launch)

| Area | Owner | Action |
| --- | --- | --- |
| Course console admin | `sdkwork-course` | Move `ConsoleCourse` + `CourseConsoleService` out of `im-console-core` |
| Drive upload client | `sdkwork-drive` | Replace IM `driveAppSdkClient` with `@sdkwork/drive-pc-core` session bridge |
| Membership transport shim | `sdkwork-mall` | Remove `im-pc-membership-transport`; consume composed `@sdkwork/membership-app-sdk` |
| Console shop placeholder | `sdkwork-shop` | Replace `im-console-shop` empty state when `@sdkwork/shop-pc-console-*` ships |
| Fail-closed stubs | IM | calendar / approvals / attendance / reports / enterprise / *-gen modules stay fail-closed until domain SDK contracts exist |

## Verification

```bash
node scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs
node scripts/dev/sdkwork-im-pc-sidebar-module-sdk-boundary.test.mjs
node scripts/dev/sdkwork-im-pc-client-pagination-standard.test.mjs
node scripts/dev/sdkwork-im-pc-device-agent-binding-standard.test.mjs
node apps/sdkwork-im-pc/scripts/mail-app-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/commerce-app-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/community-app-sdk-integration-contract.test.mjs
node apps/sdkwork-im-pc/scripts/course-app-sdk-integration-contract.test.mjs
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
```

Expected: all contract tests pass; grep for deleted IM-local service paths returns no matches outside negative assertions.
