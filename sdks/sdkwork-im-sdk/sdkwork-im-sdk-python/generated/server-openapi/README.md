# sdkwork-im-sdk (Python)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
pip install sdkwork-im-sdk-generated
```

## Quick Start

```python
from sdkwork_im_sdk_generated import SdkworkImClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18079",
)

client = SdkworkImClient(config)
client.set_auth_token("your-auth-token")
client.set_access_token("your-access-token")

# Use the SDK
result = client.presence.me.list()
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```python
from sdkwork_im_sdk_generated import SdkworkImClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18079",
)

client = SdkworkImClient(config)
client.set_header('X-Custom-Header', 'value')
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

```python
# Retrieve current principal presence
result = client.presence.me.list()
print(result)
```

### realtime

```python
# List pending realtime events
params = {
    'limit': 1,
    'cursor': 'cursor',
}
result = client.realtime.events.list(params)
print(result)
```

### calls

```python
# Create an IM call signaling session
body = {
    'rtcSessionId': 'rtcSessionId',
    'conversationId': 'conversationId',
    'rtcMode': 'rtcMode',
}
result = client.calls.sessions.create(body)
print(result)
```

### social

```python
# List contact tags
params = {
    'limit': 1,
    'cursor': 'cursor',
}
result = client.social.contacts.tags.list(params)
print(result)
```

### chat

```python
# List IM contacts
params = {
    'limit': 1,
    'cursor': 'cursor',
}
result = client.chat.contacts.list(params)
print(result)
```

### streams

```python
# Open a stream
body = {
    'streamType': 'streamType',
    'conversationId': 'conversationId',
}
result = client.streams.create(body)
print(result)
```

### spaces

```python
# List spaces
result = client.spaces.list()
print(result)
```

## Error Handling

```python
try:
    client.presence.me.list()
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

- HTTP/OpenAPI generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- HTTP/OpenAPI generation also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- HTTP/OpenAPI apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put HTTP/OpenAPI hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across HTTP/OpenAPI regenerations.
- If an HTTP/OpenAPI generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
- RPC SDK source workspaces use convention-first evidence by default: RPC SDK family naming, language workspace naming, `rpc/*.manifest.json`, proto source references, generated client source, and native package manifests.
- Use `sdkgen inspect --protocol rpc` to verify RPC convention evidence. Request persisted generator evidence only with `--emit-control-plane` for release, CI, audit, or migration workflows; evidence paths are derived by generator convention.
