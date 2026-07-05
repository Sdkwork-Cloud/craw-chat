# @sdkwork/im-pc-core

Domain: communication  
Capability: im-pc-core  
Package type: node-package

IM PC **host runtime**: session storage, IAM auth runtime, IM SDK client, realtime connection manager, and **session bridges** into sibling product PC packages.

Machine-readable contract: `specs/component.spec.json`. Canonical standards: `../../../../../sdkwork-specs/`.

## Host bridges vs domain ownership

| Module | Role |
| --- | --- |
| `session`, `appAuthRuntime`, `appSdkClient`, `imSdkClient` | IM-owned chat/auth runtime |
| `mailPcIntegration` | IM session → `@sdkwork/mail-pc-core` |
| `commercePcIntegration` | IM session → `@sdkwork/shop-pc-core` |
| `aiotPcIntegration` | IM session → `@sdkwork/aiot-pc-core` |
| `communityPcIntegration` | IM session → `@sdkwork/community-app-sdk` |
| `coursePcIntegration` | IM session → `@sdkwork/course-app-sdk` + `@sdkwork/course-pc-course` host ports |
| `drivePcIntegration`, `voicePcIntegration`, `knowledgebasePcIntegration`, … | Host port injection into sibling `*-pc-*` packages |

Product UI and domain services for mail, shop, orders, devices, and community **must not** live in this package. See `docs/architecture/tech/INTEGRATION-ADAPTER-REGISTER.md`.

Legacy re-export paths (e.g. `./sdk/communityAppSdkClient`) remain for compatibility; implementation lives in `*PcIntegration.ts`.

## Verification

- `node ../../../scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs`
- `node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace ../../../`
