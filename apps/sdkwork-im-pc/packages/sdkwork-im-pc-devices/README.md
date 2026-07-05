# @sdkwork/im-pc-devices

Capability: im-pc-devices

Thin IM host adapter over canonical `@sdkwork/aiot-pc-console-device` in `../sdkwork-aiot`.

## Ownership

| Concern | Owner |
| --- | --- |
| Device console UI and AIoT app SDK integration | `sdkwork-aiot` |
| IM session bridge into AIoT PC runtime | `@sdkwork/im-pc-core` (`aiotPcIntegration`) |
| Gateway upstream `/app/v3/api/iot/*` | `sdkwork-api-cloud-gateway` |

Bootstrap: `apps/sdkwork-im-pc/src/bootstrap/aiotPc.ts` calls `bootstrapAiotPcForIm()` before render.

Authority: sibling `sdkwork-aiot` PC packages (`@sdkwork/aiot-pc-core`, `@sdkwork/aiot-pc-console-device`).
