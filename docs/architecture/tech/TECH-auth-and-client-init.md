> Migrated from `docs/sites/sdk/auth-and-client-init.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Auth and Client Init

This page explains the shared client bootstrap model across the TypeScript, Flutter, and Rust app
SDKs.

## Public Auth Model

The public IM consumer SDK contract is SDKWork appbase based.

- `sdkwork-appbase` owns login, IAM sessions, tenants, users, organizations, and dual-token validation.
- Sdkwork IM receives only verified `x-sdkwork-*` AppContext projection headers from the trusted edge.
- local Sdkwork IM deployments do not generate or verify Sdkwork IM-owned public tokens.

If you are documenting or implementing a public consumer path, route authentication through the
SDKWork app/auth client and do not create local HTTP auth wrappers.

## Preferred Client Surface

| Language | Preferred client surface | Auth update method |
| --- | --- | --- |
| TypeScript | `new ImSdkClient({ baseUrl, authToken })` | Recreate the client with the new `authToken`, or update the generated transport with `setAuthToken(token)` when working at transport level |
| Flutter | `ImSdkComposedClient` + `SdkworkImClient` | `transport.setAuthToken(token)` and `composed.authToken = token` |
| Rust | `ImSdkClient::new_with_base_url(...)` | `client.set_auth_token(token)` |

All three languages also expose the generated transport layer, but the preferred integration surface
is the official IM consumer client for each language. TypeScript now ships that public contract from
the root `@sdkwork/im-sdk` package.

## Shared Initialization Flow

1. resolve the app base URL
2. create the preferred app client
3. set or inject the SDKWork auth/access token pair through the generated transport
4. route work through the semantic modules
5. drop to the App API reference only when you need exact payload or operation detail

## TypeScript

```ts
import { ImSdkClient } from "@sdkwork/im-sdk";

const sdk = new ImSdkClient({
  baseUrl: "http://127.0.0.1:18079",
  authToken: process.env.SDKWORK_IM_TOKEN,
});

const live = await sdk.connect({
  clientRouteId: "device-web-01",
  subscriptions: {
    conversations: ["conversation-demo-01"],
  },
});
```

## Flutter

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

final connection = composed.connect(
  options: ImConnectOptions(
    subscriptions: ImConnectSubscriptions(
      conversations: ['conversation-demo-01'],
    ),
  ),
);
connection.messages.onConversation('conversation-demo-01', (_) {
  // refresh timeline
});
```

## Rust

```rust
use im_sdk::ImSdkClient;

let client = ImSdkClient::new_with_base_url("http://127.0.0.1:18079")?;
client.set_auth_token(token);

let presence = client.presence().current().await?;
```

## Ownership Boundary

- `generated/server-openapi` is generator-owned transport output
- TypeScript assembles that generated boundary into the root `@sdkwork/im-sdk` package under
  `src/generated/**`
- `composed` remains the manual-owned authoring layer or semantic reserve before publication
- if a generated behavior needs to change, change the contract or generator inputs and regenerate

## Realtime Transport Note

The current SDK family splits realtime into generated HTTP coordination and manual live runtimes:

- client route heartbeat, subscription sync, catch-up, and event acknowledgement remain exposed directly
- TypeScript and Flutter both ship composed live receive through `@sdkwork/im-sdk` and `im_sdk_composed`
- TypeScript and Flutter now both keep `ImWebSocketAuthOptions.automatic()` as the standard
  default
- TypeScript automatic auth resolves to query credential on the default browser `WebSocket` path and
  to header credential when a custom `webSocketFactory` is present
- Flutter automatic auth resolves to header credential on native runtimes and query credential fallback on
  Flutter Web
- TypeScript browser runtimes can still prefer `sdk.connect({ url })` when the gateway issues a
  fully pre-signed realtime URL instead of a credential exchange flow
- Flutter Web should prefer `credentialProvider` with a short-lived realtime ticket when the
  gateway supports browser-safe WebSocket token exchange
- Rust and the transport-standardized languages still remain HTTP-coordination-only today

## Related Pages

- [TypeScript Quick Start](/sdk/typescript-quick-start)
- [Flutter Quick Start](/sdk/flutter-quick-start)
- [Rust Quick Start](/sdk/rust-quick-start)
- [Module Map](/sdk/module-map)
