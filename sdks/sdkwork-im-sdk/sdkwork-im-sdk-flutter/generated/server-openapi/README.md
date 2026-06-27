# sdkwork-im-sdk (Flutter)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  im_sdk_generated: ^0.1.0
```

## Quick Start

```dart
import 'package:im_sdk_generated/im_sdk_generated.dart';

final client = SdkworkImClient.withBaseUrl(baseUrl: 'http://127.0.0.1:18079');
client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');

// Use the SDK
final result = await client.presence.meRetrieve();
print(result);
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```dart
final client = SdkworkImClient.withBaseUrl(baseUrl: 'http://127.0.0.1:18079');

// Set custom headers
client.setHeader('X-Custom-Header', 'value');
```

## API Modules

- `client.presence` - presence API
- `client.realtime` - realtime API
- `client.calls` - calls API
- `client.social` - social API
- `client.chat` - chat API
- `client.streams` - streams API
- `client.spaces` - spaces API

## Usage Examples

### presence
```dart
// Retrieve current principal presence
final result = await client.presence.meRetrieve();
print(result);
```

### realtime
```dart
// List pending realtime events
final params = <String, dynamic>{
  'limit': 1,
  'cursor': 'cursor',
};
final result = await client.realtime.eventsList(params);
print(result);
```

### calls
```dart
// Create an IM call signaling session
final body = CreateRtcSessionRequest(
  rtcSessionId: '1',
  conversationId: '1',
  rtcMode: 'rtcmode',
);
final result = await client.calls.sessionsCreate(body);
print(result);
```

### social
```dart
// List contact tags
final params = <String, dynamic>{
  'limit': 1,
  'cursor': 'cursor',
};
final result = await client.social.contactsTagsList(params);
print(result);
```

### chat
```dart
// List IM contacts
final params = <String, dynamic>{
  'limit': 1,
  'cursor': 'cursor',
};
final result = await client.chat.contactsList(params);
print(result);
```

### streams
```dart
// Open a stream
final body = OpenStreamRequest(
  streamType: 'streamtype',
  conversationId: '1',
);
final result = await client.streams.create(body);
print(result);
```

### spaces
```dart
// List spaces
final result = await client.spaces.list();
print(result);
```

## Error Handling

```dart
try {
  final result = await client.presence.meRetrieve();
  print(result);
} catch (e) {
  print('Error: $e');
}
```

## Publishing

This SDK includes cross-platform publish scripts in `bin/`:
- `bin/publish-core.mjs`
- `bin/publish.sh`
- `bin/publish.ps1`

### Check

```bash
./bin/publish.sh --action check
```

### Publish

```bash
./bin/publish.sh --action publish --channel release
```

```powershell
.\bin\publish.ps1 --action publish --channel test --dry-run
```

> Ensure `dart pub publish --dry-run` passes before release publish.

## License

MIT

## Regeneration Contract

- HTTP/OpenAPI generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- HTTP/OpenAPI generation also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- HTTP/OpenAPI apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put HTTP/OpenAPI hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across HTTP/OpenAPI regenerations.
- If an HTTP/OpenAPI generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
- RPC SDK source workspaces use convention-first evidence by default: RPC SDK family naming, language workspace naming, `rpc/*.manifest.json`, proto source references, generated client source, and native package manifests.
- Use `sdkgen inspect --protocol rpc` to verify RPC convention evidence. Request persisted generator evidence only with `--emit-control-plane` for release, CI, audit, or migration workflows; evidence paths are derived by generator convention.
