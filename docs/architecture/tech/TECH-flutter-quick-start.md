> Migrated from `docs/sites/sdk/flutter-quick-start.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Flutter Quick Start

## Audience

Use this page when you are integrating Sdkwork IM into a Flutter application and want the preferred
composed SDK layer.

## Package

- preferred composed package: `im_sdk_composed`
- generated transport package: `im_sdk_generated`
- workspace path: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/im_sdk_composed`
- reference app: `apps/sdkwork-im-flutter-mobile`

## Install

If you are consuming from a local repository checkout before public publication, wire the package as
a local path dependency:

```yaml
dependencies:
  im_sdk_generated:
    path: ../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi
  im_sdk_composed:
    path: ../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/im_sdk_composed
```

## Create a client

```dart
import 'package:im_sdk_generated/im_sdk_generated.dart';
import 'package:im_sdk_composed/im_sdk_composed.dart';

final transport = SdkworkImClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18079',
  authToken: token,
);
final composed = ImSdkComposedClient(
  transport: transport,
  websocketBaseUrl: 'ws://127.0.0.1:18079/im/v3/api/realtime/ws',
  authToken: token,
);
```

If your app uses a split realtime origin, set `websocketBaseUrl` on the composed client:

```dart
final composed = ImSdkComposedClient(
  transport: transport,
  websocketBaseUrl: 'wss://realtime.example.com/im/v3/api/realtime/ws',
  authToken: token,
);
```

## First read call

```dart
final inbox = await transport.chat.inboxRetrieve();
```

## First write call

```dart
await transport.chat.conversationsMessagesCreate(
  'conv-demo-01',
  PostMessageRequest(
    clientMsgId: 'client-1',
    text: 'hello from Flutter',
    summary: 'Greeting',
  ),
);
```

## First live receive

```dart
final connection = composed.connect(
  options: ImConnectOptions(
    subscriptions: ImConnectSubscriptions(
      conversations: ['conv-demo-01'],
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

connection.messages.onConversation('conv-demo-01', (_) {
  // refresh conversation timeline
});
connection.events.onScope('user', '1', (_) {
  // refresh inbox
});
```

Reuse one live connection for inbox and conversation subscriptions. The reference app
`apps/sdkwork-im-flutter-mobile` keeps a shared hub so navigation does not reconnect.

## Common module entrypoints

```dart
await transport.presence.meRetrieve();
await transport.chat.conversationsMessagesCreate(
  'conv-demo-01',
  PostMessageRequest(
    clientMsgId: 'client-2',
    text: 'hello from Flutter',
    summary: 'Greeting',
  ),
);
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Flutter SDK](/sdk/flutter-sdk)
- [Messages Module](/sdk/modules/messages)
