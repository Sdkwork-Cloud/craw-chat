# SDKWork IM / RTC Complete Integration Guide

Last updated: `2026-04-21`

This document is the current complete integration guide for:

- `sdks/sdkwork-im-sdk`
- `../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk`

It is written against the checked-in code and the fresh verification snapshot from this workspace.
It focuses on the two currently executable baselines:

- TypeScript
- Flutter

The other language workspaces are standardized boundaries, but they are not the current runnable
application integration baselines.

## 1. Current Integration Baseline

### IM SDK

| Language | Consumer package | Status | Current integration reality |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/im-sdk` | executable | Full app-facing baseline, including HTTP modules, message-first API, `sdk.connect(...)`, `sdk.sync.catchUp(...)`, IM calls route surface, and `sdkwork-im-chat` CLI |
| Flutter | `im_sdk` | executable | App-facing baseline for HTTP modules, `sdk.connect(...)` live receive, WebSocket auth standardization, and IM calls route surface |
| Rust / Java / C# / Swift / Kotlin / Go / Python | language workspace specific | standardized only | Official workspace family exists, but not the primary app-consumer baseline for Sdkwork IM today |

### RTC SDK

| Language | Consumer package | Status | Current integration reality |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/rtc-sdk` | executable | Official web/browser runtime baseline, default provider `volcengine`, media runtime ready for IM-owned call signaling |
| Flutter | `rtc_sdk` | executable | Official mobile runtime baseline, default provider `volcengine`, media runtime ready for IM-owned call signaling |
| Rust / Java / C# / Swift / Kotlin / Go / Python | language workspace specific | standardized only | Reserved runtime-bridge boundary, not the current runnable call baseline |

### RTC provider baseline

The current official provider catalog is:

- Built-in now: `volcengine`, `aliyun`, `tencent`
- Package-boundary target now: `agora`, `zego`, `livekit`, `twilio`, `jitsi`
- Future SPI target now: `janus`, `mediasoup`

Current runnable default provider:

- `volcengine`

## 2. Architecture Positioning

### IM SDK positioning

`sdkwork-im-sdk` is the app-facing IM SDK family.

Its responsibility is:

- app HTTP API integration
- auth/session/presence/device/inbox/conversation/message/media/stream/rtc route surfaces
- TypeScript live receive runtime
- Flutter composed consumer package and live receive runtime
- app-facing IM-owned call session lifecycle and signal route access

It does not directly own vendor RTC media runtime behavior.

### RTC SDK positioning

`sdkwork-rtc-sdk` is the provider-standard RTC SDK family.

Its responsibility is:

- JDBC-style provider-neutral RTC standard
- `DriverManager / DataSource / Client` model
- unified provider selection and capability negotiation
- provider adapter boundary and provider package SPI
- one standard call/session orchestration layer
- composition with `sdkwork-im-sdk` for signaling

It does not reimplement vendor media engines.

### IM and RTC boundary

The current standard boundary is:

- `sdkwork-im-sdk` owns app-facing call signaling, IM-owned call session routes, participant credential issuance, and realtime or sync receive
- `sdkwork-rtc-sdk` owns provider-neutral media runtime contracts and provider/runtime bridges
- the vendor SDK owns actual media engine behavior
- the application owns runtime environment, credentials, and package installation

## 3. Package Matrix

### TypeScript packages

| SDK | Package | Notes |
| --- | --- | --- |
| IM | `@sdkwork/im-sdk` | Root single-package SDK |
| RTC | `@sdkwork/rtc-sdk` | Root provider-standard SDK |
| RTC vendor | `@volcengine/rtc` | Required for actual Volcengine web runtime |

Important current contract:

- `@sdkwork/rtc-sdk` declares `@sdkwork/im-sdk` and `@volcengine/rtc` as peer dependencies
- that means a consumer must install them explicitly when using the IM-signaled RTC call flow

### Flutter packages

| SDK | Package | Current source location |
| --- | --- | --- |
| IM | `im_sdk` | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed` |
| IM generated transport | `im_sdk_generated` | `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi` |
| RTC | `rtc_sdk` | `../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-flutter` |
| RTC vendor | `volc_engine_rtc` | external vendor package |

Important current contract:

- `rtc_sdk` depends on `im_sdk`
- `im_sdk` depends on `im_sdk_generated` and `sdkwork_common_flutter`
- inside this repository, that dependency graph is resolved by the checked-in workspace structure
  plus `sdkwork-im-sdk-flutter/composed/pubspec_overrides.yaml`

## 4. Backend Preconditions

Before integrating either SDK into a real app, the backend side must already provide:

- a reachable Sdkwork IM app base URL
- bearer-token login or an already-issued access token
- IM conversation routes
- IM message routes
- IM realtime routes
- IM-owned call session routes under the IM API
- IM-owned call signal posting routes under the IM API
- RTC participant credential issuance

For the RTC full call flow, the backend must be able to support:

- create RTC session
- invite participants
- accept or reject session
- end session
- post typed session signals such as offer, answer, and ICE
- issue provider participant credential for vendor room join

For the actual Volcengine media join path, the app must also provide:

- vendor `appId`
- vendor participant token or IM-issued participant credential
- room id
- participant id

## 5. TypeScript IM Integration

### Install

If the packages are already published to your private registry, install by package name:

```bash
pnpm add @sdkwork/im-sdk
```

If you are integrating directly from the current repository checkout, use the local package root:

```bash
pnpm add file:../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript
```

### Initialize IM SDK

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: process.env.sdkwork_im_BASE_URL!,
  authToken: process.env.sdkwork_im_TOKEN,
});
```

If HTTP and WebSocket origins differ:

```ts
const sdk = new ImSdkClient({
  apiBaseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: process.env.sdkwork_im_TOKEN,
});
```

### Appbase-token-first integration

```ts
const appbaseBearerToken = await resolveAppbaseBearerToken();
sdk.auth.useToken(appbaseBearerToken);
```

### Conversation and message integration

Recommended message path:

1. Build message through `sdk.createXxxMessage(...)`
2. Send it through `sdk.send(...)`

Example:

```ts
const message = sdk.createTextMessage({
  conversationId: 'conversation-1',
  text: 'hello world',
  summary: 'Greeting',
});

await sdk.send(message);
```

### Realtime integration

TypeScript currently supports two receive modes:

- live push: `sdk.connect(...)`
- durable sync replay: `sdk.sync.catchUp(...)`

Example:

```ts
const live = await sdk.connect({
  clientRouteId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-1'],
    rtcSessions: ['rtc-1'],
  },
});

live.messages.onConversation('conversation-1', (message, context) => {
  console.log(message.type, context.sequence);
});

live.signals.onRtcSession('rtc-1', (signal, context) => {
  console.log(signal.signalType, context.scopeId);
});
```

### IM calls route surface through IM

`@sdkwork/im-sdk` exposes IM-owned call lifecycle routes:

```ts
const session = await sdk.calls.start({
  rtcSessionId: 'rtc-1',
  conversationId: 'conversation-1',
  rtcMode: 'group_call',
});

await sdk.calls.invite(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
});

await sdk.calls.sendSignal(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
  signalType: 'rtc.offer',
  payload: JSON.stringify({ sdp: 'v=0...' }),
});

const credential = await sdk.calls.issueParticipantCredential(session.rtcSessionId, {
  participantId: '1',
});
```

### TypeScript CLI smoke path

Current CLI entrypoint:

```bash
node ./sdkwork-im-sdk-typescript/bin/sdk-chat.mjs --help
```

Current verified help contract:

- `--base-url`
- `--api-base-url`
- `--websocket-base-url`
- `--token`
- `--tenant-id`
- `--login`
- `--password`
- `--conversation-id`
- `--create-conversation`
- `--conversation-type`
- `--device-id`
- `--client-kind`
- `--receive-mode <auto|live|catch-up|off>`
- `--catch-up-interval-ms`

## 6. Flutter IM Integration

### Add dependencies

In the current repository layout, the app-facing dependency pattern is:

```yaml
dependencies:
  im_sdk:
    path: ../sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed
```

### Initialize IM SDK

```dart
import 'package:im_sdk/im_sdk.dart';

final sdk = ImSdkClient.create(
  baseUrl: 'https://api.example.com',
  authToken: '<token>',
);
```

### Basic message integration

Flutter currently uses route-aligned posting instead of the TypeScript-style message-first root API:

```dart
await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
);
```

### Realtime reality on Flutter today

Flutter now ships:

- `sdk.connect(...)`
- a delivered WebSocket adapter in the manual-owned `im_sdk` package
- `sdk.realtime.replaceSubscriptions(...)`
- `sdk.realtime.catchUpEvents(...)`
- `sdk.realtime.ackEvents(...)`

That means Flutter realtime is now WebSocket-first for live receive, while explicit HTTP
coordination remains available for recovery, catch-up, and manual control.

### WebSocket auth standard on Flutter

The checked-in Flutter standard is:

- `ImWebSocketAuthOptions.automatic()` is the default
- Flutter mobile and desktop resolve `automatic()` to upgrade-header bearer auth
- Flutter Web resolves `automatic()` to query bearer auth because the default browser connector
  cannot attach custom upgrade headers
- `ImWebSocketAuthOptions.headerBearer()` remains the preferred explicit mode on native runtimes
- `ImWebSocketAuthOptions.queryBearer()` is the preferred explicit mode for browser-compatible
  fallback when you cannot provide a custom `webSocketFactory`
- `credentialProvider` is the preferred browser-safe extension point when your gateway can exchange
  the main access token for a short-lived realtime ticket or query credential
- `ImWebSocketAuthOptions.none()` is reserved for trusted local or custom-factory scenarios only
- query bearer auth must be used only over `wss://` in production and should prefer short-lived
  access tokens or gateway-exchanged realtime tokens
- custom gateways can replace the default connector through `webSocketFactory`

### Local dependency resolution note

`im_sdk` is publish-friendly, but local workspace integration currently relies on:

- `sdkwork-im-sdk-flutter/composed/pubspec_overrides.yaml`
- local resolution of `im_sdk_generated`
- local resolution of `sdkwork_common_flutter`

Inside this repository, that layout is already verified.
Outside this repository, if you consume `im_sdk` separately, you must provide a resolvable source
for `im_sdk_generated` and `sdkwork_common_flutter`, either by:

- publishing them to your private registry
- preserving the same workspace-relative path structure
- adding your own dependency override strategy

## 7. TypeScript RTC Integration

### Install

If packages are published:

```bash
pnpm add @sdkwork/im-sdk @sdkwork/rtc-sdk @volcengine/rtc
```

If you are integrating directly from the current repository checkout:

```bash
pnpm add file:../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript
pnpm add file:../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript
pnpm add @volcengine/rtc
```

### Media-only integration

Use this when your app already owns session orchestration:

```ts
import {
  createRtcCallTrackId,
  RtcDataSource,
  createBuiltinRtcDriverManager,
} from '@sdkwork/rtc-sdk';

const dataSource = new RtcDataSource({
  driverManager: createBuiltinRtcDriverManager(),
  nativeConfig: {
    appId: 'volc-app-id',
  },
});

const rtcClient = await dataSource.createClient();
```

### RTC media flow with IM-owned call signaling

Use this when the application combines IM-owned call signaling with the vendor media runtime:

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';
import {
  RtcDataSource,
  createBuiltinRtcDriverManager,
} from '@sdkwork/rtc-sdk';

const imSdk = new ImSdkClient({
  baseUrl: 'https://sdkwork-im.example.com',
  authToken: 'app-token',
});

const session = await imSdk.calls.start({
  rtcSessionId: 'rtc-session-1',
  conversationId: 'conversation-1',
  rtcMode: 'video_call',
});

await imSdk.calls.invite(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
});

await imSdk.calls.sendSignal(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
  signalType: 'rtc.offer',
  payload: JSON.stringify({ sdp: 'v=0...' }),
});

const credential = await imSdk.calls.issueParticipantCredential(session.rtcSessionId, {
  participantId: '1',
});

const rtcDataSource = new RtcDataSource({
  driverManager: createBuiltinRtcDriverManager(),
  nativeConfig: {
    appId: 'volc-app-id',
  },
});

const rtcClient = await rtcDataSource.createClient();
```

### Current TypeScript IM calls signaling mapping

The current IM calls facade contract is:

- `sdk.calls.start(...)` -> create an IM-owned call signaling session
- `sdk.calls.invite(...)` -> invite participant
- `sdk.calls.accept(...)` -> accept session
- `sdk.calls.reject(...)` -> reject session
- `sdk.calls.end(...)` -> end session
- `sdk.calls.sendSignal(...)` -> send offer, answer, and ICE through IM
- `sdk.calls.issueParticipantCredential(...)` -> issue vendor join credential after IM authorization
- `sdk.connect(...).signals.onRtcSession(...)` -> subscribe RTC session signal stream
- `sdk.createSignalMessage(...)` + `sdk.send(...)` -> publish conversation-scoped invite message

### Required runtime inputs

For the current Volcengine web runtime baseline, the app must provide:

- `nativeConfig.appId`
- RTC session id
- room id
- participant id
- provider token or IM-issued participant credential

## 8. Flutter RTC Integration

### Add dependencies

In the current repository layout, the verified dependency pattern is:

```yaml
dependencies:
  rtc_sdk:
    path: ../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-flutter
  im_sdk:
    path: ../sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed
  volc_engine_rtc: ^3.60.3
```

### Media-only integration

```dart
import 'package:rtc_sdk/rtc_sdk.dart';

final dataSource = RtcDataSource(
  options: const RtcDataSourceOptions(
    nativeConfig: RtcVolcengineFlutterNativeConfig(
      appId: 'volc-app-id',
    ),
  ),
);
```

### RTC media flow with IM-owned call signaling

```dart
import 'package:im_sdk/im_sdk.dart';
import 'package:rtc_sdk/rtc_sdk.dart';

final session = await imSdk.calls.start(
  rtcSessionId: 'rtc-session-1',
  conversationId: 'conversation-1',
  rtcMode: 'video_call',
);

await imSdk.calls.invite(
  session.rtcSessionId,
  signalingStreamId: 'rtc-signal-1',
);

await imSdk.calls.sendSignal(
  session.rtcSessionId,
  signalType: 'rtc.offer',
  payload: '{"sdp":"v=0..."}',
  signalingStreamId: 'rtc-signal-1',
);

final credential = await imSdk.calls.issueParticipantCredential(
  session.rtcSessionId,
  participantId: '1',
);

final dataSource = RtcDataSource(
  options: const RtcDataSourceOptions(
    nativeConfig: RtcVolcengineFlutterNativeConfig(
      appId: 'volc-app-id',
    ),
  ),
);
```

### Current Flutter IM calls signaling reality

Flutter RTC media runtime composes with IM-owned call signaling through:

- `sdk.calls.*` lifecycle routes
- `sdk.connect(...)`
- `sdk.realtime.replaceSubscriptions(...)`
- `sdk.realtime.ackEvents(...)`
- an IM realtime dispatcher in the IM SDK layer

The checked-in Flutter call path is therefore WebSocket-push-backed for live signaling, with ACK
state still committed through the HTTP realtime ACK route.

### Required runtime inputs

For the current Volcengine Flutter runtime baseline, the app must provide:

- `RtcVolcengineFlutterNativeConfig.appId`
- room id
- participant id
- vendor token or IM-issued participant credential

## 9. Verification Commands

### Verified in this workspace on `2026-04-21`

RTC fresh verification:

```powershell
node ../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\smoke-sdk.mjs
```

Observed result:

- full RTC regression passed
- TypeScript call smoke passed
- Flutter smoke passed in `analysis-backed` mode

IM fresh verification:

```powershell
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-sdk\sdkwork-im-sdk-typescript\bin\sdk-chat.mjs --help
```

Observed result:

- root IM verification passed
- TypeScript chat CLI help passed

### Recommended local commands

RTC:

```powershell
node ..\sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs
node ../../../sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\smoke-sdk.mjs
```

IM:

```powershell
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs
node .\sdks\sdkwork-im-sdk\bin\sdk-chat.mjs --language typescript --help
node .\sdks\sdkwork-im-sdk\sdkwork-im-sdk-typescript\bin\sdk-chat.mjs --help
```

## 10. Current Issues That Affect Integration

### Issue 1: Flutter WebSocket auth must match the runtime

Current reality:

- Flutter now ships `sdk.connect(...)`
- native Flutter runtimes can attach bearer headers during the upgrade
- Flutter Web cannot attach custom upgrade headers through the default connector and therefore
  uses query bearer auth when `ImWebSocketAuthOptions.automatic()` is selected

Impact:

- if your gateway rejects query bearer auth, Flutter Web live receive will require a custom
  `webSocketFactory`
- if your gateway logs full query strings, query bearer auth increases token-exposure risk unless
  you use short-lived tokens or gateway token exchange
- Flutter mobile and desktop should stay on header bearer auth by default

Severity:

- medium for Flutter Web integration if gateway policy is not aligned
- low for Flutter mobile and desktop when header bearer auth is supported

### Issue 2: TypeScript browser live receive still depends on WebSocket auth strategy

Current reality:

- TypeScript supports `sdk.connect(...)`
- plain browser `WebSocket` cannot attach `Authorization` headers by default

Impact:

- if your realtime gateway requires bearer auth in the WebSocket upgrade header, browser live receive
  will not work out of the box
- this affects IM live receive
- this also affects RTC incoming session signal receive when you rely on `sdk.connect(...)`

What you must provide:

- an auth-capable gateway strategy
- or a runtime-specific `webSocketFactory`
- or a backend-compatible upgrade strategy such as cookie or tokenized URL

Severity:

- high for browser live receive integration if the gateway is not already compatible

### Issue 3: Flutter RTC smoke is `analysis-backed`, not true vendor-runtime execution

Current reality:

- Flutter RTC verification passed
- but the smoke path is analyze-backed because `volc_engine_rtc` is not CLI-runnable in the current
  Dart VM toolchain

Impact:

- CI can verify source and integration surface
- CI cannot fully execute the vendor runtime through the current CLI smoke path

Severity:

- medium for maintainers and CI
- low for real device runtime integration

### Issue 4: Flutter local package resolution is workspace-dependent

Current reality:

- `im_sdk` depends on `im_sdk_generated` and `sdkwork_common_flutter`
- local development in this repo uses `pubspec_overrides.yaml`

Impact:

- if you copy only one Flutter package out of this workspace, dependency resolution will break
- you must preserve the workspace structure or provide your own published package source

Severity:

- high for external isolated integration
- low inside this repository layout

### Issue 5: Other languages are not the current runnable application baseline

Current reality:

- RTC runnable baseline today is TypeScript and Flutter only
- IM practical app-consumer baseline today is TypeScript and Flutter

Impact:

- do not plan near-term business integration on RTC Java, Go, Python, Swift, Kotlin, C#, or Rust
  runtime bridges
- use those workspaces only as standard boundaries unless you implement the missing runtime layer

Severity:

- high for roadmap planning
- not a blocker for current TypeScript or Flutter integration

### Issue 6: `sdkwork-im-sdk` workspace is currently dirty

Current reality:

- fresh `node .\bin\verify-sdk.mjs` passed
- but the repository currently contains tracked local modifications

Impact:

- integration verification is green
- release freeze, tagging, or publication should not happen until those modifications are reviewed
  and stabilized

Severity:

- low for local integration
- medium for release management

## 11. Recommended Integration Order

Recommended implementation order:

1. Integrate IM auth and token management first.
2. Integrate IM conversation and message send path second.
3. Integrate IM realtime receive path next.
4. Integrate RTC media-only path after IM is stable.
5. Integrate RTC full call flow on top of IM signaling last.

Recommended language priority:

1. TypeScript first, because it has the richest current IM baseline and full live receive support.
2. Flutter second, with the explicit understanding that live receive is now WebSocket-first but
   message-first authoring still trails TypeScript.

## 12. Direct Source References

RTC references:

- `../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/usage-guide.md`
- `../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/typescript-volcengine-runtime-usage.md`
- `../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/flutter-volcengine-runtime-usage.md`

IM references:

- `sdks/sdkwork-im-sdk/README.md`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/README.md`
