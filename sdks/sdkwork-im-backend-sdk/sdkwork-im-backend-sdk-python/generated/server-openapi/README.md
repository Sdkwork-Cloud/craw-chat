# sdkwork-im-backend-sdk (Python)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
pip install sdkwork-im-backend-api-generated
```

## Quick Start

```python
from sdkwork_im_backend_api_generated import SdkworkImBackendClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18079",
)

client = SdkworkImBackendClient(config)
client.set_auth_token("your-auth-token")
client.set_access_token("your-access-token")

# Use the SDK
result = client.admin.api_key_groups.list()
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```python
from sdkwork_im_backend_api_generated import SdkworkImBackendClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18079",
)

client = SdkworkImBackendClient(config)
client.set_header('X-Custom-Header', 'value')
```

## API Modules

- `client.ops` - ops API
- `client.audit` - audit API
- `client.automation` - automation API
- `client.control` - control API
- `client.admin` - admin API

## Usage Examples

### ops

```python
# Retrieve ops health
result = client.ops.health.retrieve()
print(result)
```

### audit

```python
# List audit records
result = client.audit.records.list()
print(result)
```

### automation

```python
# Retrieve automation governance
result = client.automation.governance.retrieve()
print(result)
```

### control

```python
# Read the control-plane protocol governance snapshot.
result = client.control.protocol_governance.retrieve()
print(result)
```

### admin

```python
# listApiKeyGroups
result = client.admin.api_key_groups.list()
print(result)
```

## Error Handling

```python
try:
    client.admin.api_key_groups.list()
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
