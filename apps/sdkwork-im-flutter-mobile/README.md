# SDKWork IM Flutter Mobile

Flutter mobile application root for SDKWork IM chat.

## Features

- Appbase IAM deep-link auth (`sdkworkim://auth/callback`) or development credentials
- Inbox via `im_sdk_generated` `chat.inboxRetrieve`
- Conversation timeline and text send via REST (`conversationsMessagesList`, `conversationsMessagesCreate`)
- WebSocket CCP live updates via `im_sdk_composed` `connect()`:
  - Inbox refresh via user-scope `events.onScope`
  - Conversation timeline via `messages.onConversation`
  - Shared live hub for inbox + conversation subscriptions (no reconnect on navigation)
  - Hub disposed on sign-out and session reset via `resetSdkClients()`

## Development

```powershell
cd apps/sdkwork-im-flutter-mobile
flutter pub get
flutter run
```

Configure runtime env from:

- `config/app/runtime-env.development.example.json`
- `config/host/flutter.development.example.json`

## Verification

```powershell
flutter analyze
flutter test
pnpm run test:sdkwork-im-flutter-mobile-architecture-standard
```

## Application identity

- App ID: `sdkwork-im-flutter-mobile`
- Manifest: `sdkwork.app.config.json`

See [AGENTS.md](./AGENTS.md) for SDKWork agent entrypoint and spec index.
