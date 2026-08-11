# sdkwork-im-sdk-flutter

Flutter SDK boundary for `sdkwork-im-sdk`.

## Packages

| Path | Role |
| --- | --- |
| `generated/server-openapi` | OpenAPI-generated HTTP client (`im_sdk_generated`) |
| `composed/im_sdk_composed` | Composed CCP WebSocket realtime client (`im_sdk_composed`) |

Generated HTTP output must stay separate from composed realtime orchestration. Do not hand-edit files under `generated/server-openapi/lib`.

## Composed realtime

`im_sdk_composed` mirrors the TypeScript `@sdkwork/im-sdk` realtime surface:

- CCP WebSocket subprotocol `sdkwork-im.ccp.ws.v1`
- `ImSdkComposedClient.connect()` → `ImLiveConnection`
- `events.onScope()` for user-scope inbox live refresh
- `messages.onConversation()` for conversation live updates

Verification:

```powershell
cd sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/im_sdk_composed
flutter test
```

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = template_only_pending_generation`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

The release catalog remains the machine-readable source of truth:
`sdk-release-catalog.json`.

