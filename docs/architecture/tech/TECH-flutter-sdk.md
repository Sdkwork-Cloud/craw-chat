> Migrated from `docs/sites/sdk/flutter-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Flutter SDK

The checked-in Flutter IM consumer package is `im_sdk_generated`.
This page documents the generated Dart transport surface exactly as it exists today.

::: warning Current delivery status
The package names below describe repo package contracts, not current pub.dev availability. The
release catalog still marks `app-flutter` as `generated` and `not_published`.
:::

## Current Delivery Reality

The checked-in Flutter standard is:

- public consumer package: `im_sdk_generated`
- composed consumer package: `im_sdk_composed`
- primary public client: `ImTransportClient` (HTTP) and `ImSdkComposedClient` (CCP WebSocket realtime)
- generated transport boundary: `im_sdk_generated`
- route-aligned HTTP coverage for presence, realtime coordination, conversations, streams, social,
  and RTC
- CCP WebSocket live receive via `im_sdk_composed` (`connect()`, `events.onScope()`, `messages.onConversation()`)

## How To Use This Page

- Start with [Package Contract](#package-contract) to understand the generated transport boundary.
- Use [Current Surface Reality](#current-surface-reality) and [Current Parity Gap](#current-parity-gap)
  before promising TypeScript-style message builders; WebSocket live push is available through
  `im_sdk_composed`.
- Keep [Consumption Reality](#consumption-reality) and [Local Workspace Workflow](#local-workspace-workflow)
  open when you need local path overrides or handoff artifacts before publication.

## Package Contract

| Layer | Package | Entrypoint | Primary exports | Use when |
| --- | --- | --- | --- | --- |
| Generated transport | `im_sdk_generated` | `package:im_sdk_generated/im_sdk_generated.dart` | `ImTransportClient`, generated models, generated API groups | You want direct transport access and generated request or response models |
| Composed realtime | `im_sdk_composed` | `package:im_sdk_composed/im_sdk_composed.dart` | `ImSdkComposedClient`, `createImLiveConnection()`, CCP wire helpers | You want WebSocket live inbox and conversation updates aligned with TypeScript `@sdkwork/im-sdk` |

For Flutter app integrations in this repository snapshot, start from
`package:im_sdk_generated/im_sdk_generated.dart`.

## Consumption Reality

Treat the package names on this page as repo package contracts first. Until the release catalog
changes, the reliable Flutter consumption paths are:

- local workspace development against the checked-in generated package
- assembled handoff artifacts produced from the Flutter workspace with `sdk-assemble.ps1` or
  `sdk-assemble.sh`

Do not treat `im_sdk_generated` as a current pub.dev coordinate while `app-flutter` remains
`not_published` in the release catalog.

## Generated Transport Quick Start

```dart
import 'package:im_sdk_generated/im_sdk_generated.dart';

final client = ImTransportClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18079',
  authToken: 'sdkwork-appbase-credential',
);

final presence = await client.presence.meRetrieve();
final inbox = await client.chat.inboxRetrieve();
final events = await client.realtime.eventsList(20);
```

The checked-in generated Flutter client currently exports these route groups through
`ImTransportClient`:

- `presence`
- `realtime`
- `chat`
- `streams`
- `rtc`
- `social`

## Generated Chat Quick Start

```dart
import 'package:im_sdk_generated/im_sdk_generated.dart';

final client = ImTransportClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18079',
  authToken: 'sdkwork-appbase-credential',
);

// Tokens are issued by sdkwork-appbase and passed into Sdkwork IM.
final inbox = await client.chat.inboxRetrieve();

await client.chat.conversationsMessagesCreate(
  'conversation-1',
  PostMessageRequest(
    clientMsgId: 'client-1',
    text: 'hello world',
    summary: 'Greeting',
  ),
);

await client.streams.framesCreate(
  'stream-1',
  AppendStreamFrameRequest(
    payload: '{"text":"partial chunk"}',
  ),
);

await client.calls.sessionsRetrieve(
  'rtc-1',
);
```

The checked-in Flutter transport currently exposes:

- `authToken`
- `client.presence`
- `client.realtime`
- `client.chat`
- `client.streams`
- `client.calls`
- `client.social`

## Drive-Backed Media Messages

Use `sdkwork-drive` for file lifecycle work. After Drive returns a node reference, send the IM
message with `ContentPart.drive` as the `DriveReference` and `ContentPart.resource` as the
standardized `MediaResource` usage snapshot. The canonical Drive URI shape is
`drive://spaces/{spaceId}/nodes/{nodeId}`.

```dart
final drive = DriveReference(
  driveUri: 'drive://spaces/space_app_upload_demo/nodes/node_storefront_png',
  spaceId: 'space_app_upload_demo',
  nodeId: 'node_storefront_png',
  nodeVersion: '1',
);

final body = PostMessageRequest(
  text: 'Latest storefront concept',
  summary: 'Storefront concept',
  parts: <ContentPart>[
    ContentPart(
      kind: 'media',
      mediaRole: 'attachment',
      drive: drive,
      resource: MediaResource(
        id: drive.nodeId,
        kind: MediaKind.image,
        source: MediaSource.providerAsset,
        uri: drive.driveUri,
        fileName: 'storefront.png',
        mimeType: 'image/png',
        sizeBytes: fileBytes.length.toString(),
      ),
    ),
  ],
);

await sdk.conversations.postMessage('conversation-1', body);
```

The TypeScript composed SDK names the same high-level flow `createImageMessage(...)`; Flutter keeps
the same `DriveReference` and `MediaResource` contract while its message-first helpers continue to
catch up.

## Current Surface Reality

The checked-in Dart surface is intentionally narrower than the TypeScript SDK:

- `im_sdk_generated` exports the generated `ImTransportClient`; the checked-in route groups are
  `client.presence`, `client.realtime`, `client.chat`, `client.streams`, `client.calls`, and
  `client.social`.
- Tokens are issued by `sdkwork-appbase`; Flutter consumers pass them through `authToken` at
  construction time or update them with `client.setAuthToken(...)`.
- The generated transport exposes explicit HTTP coordination through
  `client.realtime.subscriptionsSync(...)`, `client.realtime.eventsList(...)`, and
  `client.realtime.eventsAck(...)`.
- `im_sdk_composed` ships `ImSdkComposedClient.connect(...)` with CCP WebSocket live delivery.
- The composed package does not yet ship `sdk.createXxxMessage()`, `sdk.send()`,
  or `sdk.decodeMessage()`.
- Text posting currently uses generated chat calls such as
  `client.chat.conversationsMessagesCreate(...)`.

## Realtime Coordination

```dart
final client = ImTransportClient.withBaseUrl(
  baseUrl: 'https://api.example.com',
  authToken: 'sdkwork-appbase-credential',
);

await client.presence.heartbeatCreate(
  PresenceHeartbeatRequest(clientRouteId: 'mobile-01'),
);

await client.realtime.subscriptionsSync(
  RealtimeSubscriptionSyncRequest(
    clientRouteId: 'mobile-01',
    conversations: <String>['conversation-1'],
  ),
);

final events = await client.realtime.eventsList(50);
```

Generated Flutter covers presence heartbeat, subscription sync, event polling, and ACK over HTTP.
For live push, use `im_sdk_composed`:

```dart
final composed = ImSdkComposedClient(
  transport: client,
  webSocketUrl: 'wss://api.example.com/im/v3/api/realtime/ws',
);
final connection = composed.connect(
  options: ImConnectOptions(
    subscriptions: ImConnectSubscriptions(
      conversations: ['conversation-1'],
      scopes: [
        ImRealtimeScopeSubscription(
          scopeType: 'user',
          scopeId: '1',
          eventTypes: inboxRealtimeEventTypes,
        ),
      ],
    ),
  ),
);
connection.messages.onConversation('conversation-1', (_) { /* refresh timeline */ });
connection.events.onScope('user', '1', (_) { /* refresh inbox */ });
```

## Module Coverage Map

| Concern | Generated transport | Composed client | Primary HTTP reference |
| --- | --- | --- | --- |
| SDKWork credential pass-through | `client.setAuthToken(...)` | `ImSdkComposedClient` uses transport auth | [Auth And Client Init](/sdk/auth-and-client-init) |
| Realtime presence | `client.presence`, `client.realtime` | `connect()`, lifecycle state | [Realtime And Presence](/api-reference/im/session-and-realtime) |
| Inbox and conversations | `client.chat` | `events.onScope()` (user scope inbox refresh) | [Conversations and Handoff](/api-reference/im/conversations) |
| Membership and read state | `client.chat` | scope events via `events.onScope()` | [Membership and Read State](/api-reference/im/membership-and-read-state) |
| Messages | `client.chat` | `messages.onConversation()` | [Messages](/api-reference/im/messages) |
| Media usage references | Drive transport plus generated message models | Not checked in | [Media](/api-reference/im/media) |
| Streams | `client.streams` | Not checked in | [Streams](/api-reference/im/streams) |
| Calls | `client.calls` | Not checked in | [Calls](/api-reference/im/calls) |

## Current Parity Gap

The Flutter SDK still trails the TypeScript SDK on message-first authoring:

- the composed runtime does not yet ship `sdk.createXxxMessage()`,
  `sdk.send()`, or `sdk.decodeMessage()`
- WebSocket live receive is available through `im_sdk_composed`; HTTP polling remains on
  `im_sdk_generated` for coordination and fallback
- if you need the richer message-first send surface today, use the TypeScript SDK until the
  Flutter semantic runtime catches up

This page intentionally documents the checked-in exported surface, not only the OpenAPI authority.

The corresponding HTTP surface is still documented in:

- [Portal Access](/api-reference/app/portal-access)
- [App API Overview](/api-reference/app-api)

If your product needs the richer message-first outbound surface, the current documented fallback is
the TypeScript SDK.

## Helper Builders

The composed Flutter package ships helper builders for common flows:

- `ImBuilders.textMessage()`
- `ImBuilders.textEdit()`
- `ImBuilders.textFrame()`
- `ImBuilders.jsonRtcSignal()`

These helpers are used internally by the composed modules and remain available when you want to mix
semantic helpers with explicit generated request types.

## Auth And Transport Rules

- Public auth is SDKWork appbase based.
- Prefer constructor `authToken` or `sdk.setAuthToken(...)` at the composed layer.
- `setAuthToken()` remains available on `ImTransportClient` and `SdkworkImClient` for low-level
  fallback control.
- The WebSocket endpoint is documented at the API layer and ships from `im_sdk_composed`
  (CCP subprotocol `sdkwork-im.ccp.ws.v1`).
- `ImWebSocketAuthOptions.automatic()` is the standard default.
- `ImWebSocketAuthOptions.headerBearer()` is preferred on native runtimes.
- Flutter Web should keep `automatic()` or use `queryBearer()` unless a custom `webSocketFactory`
  can attach authenticated upgrade headers.

## Assembly Metadata

The workspace root emits `.sdkwork-assembly.json` as the verified package-layer record for Flutter.

Use it when you need to trace package ownership instead of inferring from folder names:

- `manifestPath` maps the generated and composed package manifests into the assembly output
- `generatedAt` stays stable when assembly content is unchanged
- Flutter records the `generated` `im_sdk_generated` layer and the `composed` `im_sdk_composed` layer

## Local Workspace Workflow

The checked-in Flutter workspace is organized in two layers:

- `generated/server-openapi`
  Generator-owned transport package.
- `composed/im_sdk_composed`
  Manual-owned CCP WebSocket realtime package.

When you consume the checked-in Flutter SDK locally, keep the generated, composed, and override
layers aligned:

- `generated/server-openapi/pubspec.yaml` owns `im_sdk_generated`
- `composed/im_sdk_composed/pubspec.yaml` owns `im_sdk_composed` and depends on `im_sdk_generated`
- `composed/im_sdk_composed/pubspec_overrides.yaml` resolves `im_sdk_generated` locally inside the repo

Recommended local loop:

1. Run generation and verification from the Flutter workspace root.
2. Start IM consumer integration work from `composed/im_sdk_composed` and
   `package:im_sdk_composed/im_sdk_composed.dart`.
3. Drop to `generated/server-openapi` only when you intentionally want the standalone generated
   transport boundary or raw generated request and response models.
4. Mirror the same `dependency_overrides` structure in any local consumer app, adjusting the paths
   relative to that app.
5. Use `sdk-assemble.ps1` or `sdk-assemble.sh` when you need a packaged handoff before pub.dev
   publication exists.

From the Flutter workspace root:

```powershell
.\bin\sdk-gen.ps1
.\bin\sdk-verify.ps1
```

```powershell
powershell -ExecutionPolicy Bypass -File .\bin\sdk-gen.ps1
powershell -ExecutionPolicy Bypass -File .\bin\sdk-verify.ps1
```

```bash
./bin/sdk-gen.sh
./bin/sdk-verify.sh
```

The composed package keeps `pubspec.yaml` publish-friendly and resolves the generated package
locally through `pubspec_overrides.yaml`:

```yaml
dependency_overrides:
  im_sdk_generated:
    path: ../generated/server-openapi
```

In the checked-in workspace, the composed override file also resolves the shared common package:

```yaml
dependency_overrides:
  im_sdk_generated:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ../../../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-flutter
```

If you wire `im_sdk_composed` into another local Flutter app before publication, mirror that override
shape in the consuming app and adjust the paths relative to the app location.

## Verification

From the repository root, the family-level verification entrypoint is:

```bash
node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language flutter --with-dart
```

Use `verify-sdk.mjs --with-dart` when you want native Dart verification in addition to the default
source-level guards. Add `--language flutter` when you want to run only the Flutter lane.

On Windows, the native Dart verification path also falls back to
`bin/verify-flutter-dart-analysis.dart` when `dart analyze` cannot safely launch its helper
process in the current environment.

## Source-of-Truth Notes

- Authority contract: `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`
- Generated transport manifest: `sdkwork-im-sdk-flutter/generated/server-openapi/pubspec.yaml`
- Assembly metadata: `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`

## What To Read Next

- Read [App SDK](/sdk/app-sdk) for family-wide audience, release, and contract-source rules.
- Read [Language Support](/sdk/language-support) for the current TypeScript versus Flutter parity
  snapshot.
- Read [Auth And Client Init](/sdk/auth-and-client-init) when you need the underlying token
  handoff contract behind `client.setAuthToken(...)` and `authToken`.
