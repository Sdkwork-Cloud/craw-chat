# sdkwork-im-app-sdk (Python)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
pip install sdkwork-im-app-api-generated
```

## Quick Start

```python
from sdkwork_im_app_api_generated import SdkworkImAppClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18079",
)

client = SdkworkImAppClient(config)
client.set_auth_token("your-auth-token")
client.set_access_token("your-access-token")

# Use the SDK
result = client.notification.list()
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```python
from sdkwork_im_app_api_generated import SdkworkImAppClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18079",
)

client = SdkworkImAppClient(config)
client.set_header('X-Custom-Header', 'value')
```

## API Modules

- `client.automation` - automation API
- `client.notification` - notification API
- `client.portal` - portal API
- `client.provider` - provider API

## Usage Examples

### automation

```python
# Start an agent response stream
body = {
    'executionId': 'executionId',
    'streamId': 'streamId',
    'streamType': 'streamType',
    'conversationId': 'conversationId',
    'schemaRef': 'schemaRef',
    'memberId': 'memberId',
    'agent': {
        'agent_id': 'agent_id',
        'session_id': 'session_id',
        'metadata': {},
    },
}
result = client.automation.agent_responses.create(body)
print(result)
```

### notification

```python
# List notifications for the current principal
result = client.notification.list()
print(result)
```

### portal

```python
# Read the tenant portal sign-in snapshot
result = client.portal.access.retrieve()
print(result)
```

### provider

```python
# Retrieve media provider health
result = client.provider.media_health.retrieve()
print(result)
```

## Error Handling

```python
try:
    client.notification.list()
except Exception as error:
    print(f"Error: {error}")
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

> Configure Python package registry credentials before release publish.

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
