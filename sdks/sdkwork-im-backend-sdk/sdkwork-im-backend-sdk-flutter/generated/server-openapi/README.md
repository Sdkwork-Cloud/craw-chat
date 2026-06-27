# sdkwork-im-backend-sdk (Flutter)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  im_backend_api_generated: ^0.1.0
```

## Quick Start

```dart
import 'package:im_backend_api_generated/im_backend_api_generated.dart';

final client = SdkworkImBackendClient.withBaseUrl(baseUrl: 'http://127.0.0.1:18079');
client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');

// Use the SDK
final result = await client.admin.apiKeyGroupsList();
print(result);
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```dart
final client = SdkworkImBackendClient.withBaseUrl(baseUrl: 'http://127.0.0.1:18079');

// Set custom headers
client.setHeader('X-Custom-Header', 'value');
```

## API Modules

- `client.ops` - ops API
- `client.audit` - audit API
- `client.automation` - automation API
- `client.control` - control API
- `client.admin` - admin API

## Usage Examples

### ops
```dart
// Retrieve ops health
final result = await client.ops.healthRetrieve();
print(result);
```

### audit
```dart
// List audit records
final result = await client.audit.recordsList();
print(result);
```

### automation
```dart
// Retrieve automation governance
final result = await client.automation.governanceRetrieve();
print(result);
```

### control
```dart
// Read the control-plane protocol governance snapshot.
final result = await client.control.protocolGovernanceRetrieve();
print(result);
```

### admin
```dart
// listApiKeyGroups
final result = await client.admin.apiKeyGroupsList();
print(result);
```

## Error Handling

```dart
try {
  final result = await client.admin.apiKeyGroupsList();
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

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
