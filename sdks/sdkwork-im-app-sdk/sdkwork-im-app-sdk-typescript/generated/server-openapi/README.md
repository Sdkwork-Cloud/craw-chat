# sdkwork-im-app-sdk

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
npm install @sdkwork-internal/im-app-api-generated
# or
yarn add @sdkwork-internal/im-app-api-generated
# or
pnpm add @sdkwork-internal/im-app-api-generated
```

## Quick Start

```typescript
import { SdkworkImAppClient } from '@sdkwork-internal/im-app-api-generated';

const client = new SdkworkImAppClient({
  baseUrl: 'http://127.0.0.1:18079',
  timeout: 30000,
});

// Authentication
client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');

// Use the SDK
const result = await client.notifications.list();
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```typescript
import { SdkworkImAppClient } from '@sdkwork-internal/im-app-api-generated';

const client = new SdkworkImAppClient({
  baseUrl: 'http://127.0.0.1:18079',
  timeout: 30000, // Request timeout in ms
  headers: {      // Custom headers
    'X-Custom-Header': 'value',
  },
});
```

## API Modules

- `client.automation` - automation API
- `client.notifications` - notifications API
- `client.portal` - portal API
- `client.provider` - provider API

## Usage Examples

### automation

```typescript
// Start an agent response stream
const body = {
  executionId: 'executionId',
  streamId: 'streamId',
  streamType: 'streamType',
  conversationId: 'conversationId',
  schemaRef: 'schemaRef',
  memberId: 'memberId',
  agent: {
    agent_id: 'agent_id',
    session_id: 'session_id',
    metadata: {},
  },
};
const result = await client.automation.agentResponses.create(body);
```

### notifications

```typescript
// List notifications for the current principal
const result = await client.notifications.list();
```

### portal

```typescript
// Read the tenant portal sign-in snapshot
const result = await client.portal.access.retrieve();
```

### provider

```typescript
// Retrieve media provider health
const result = await client.provider.mediaHealth.retrieve();
```

## Error Handling

```typescript
import { SdkworkImAppClient, NetworkError, TimeoutError, AuthenticationError } from '@sdkwork-internal/im-app-api-generated';

try {
  const result = await client.notifications.list();
} catch (error) {
  if (error instanceof AuthenticationError) {
    console.error('Authentication failed:', error.message);
  } else if (error instanceof TimeoutError) {
    console.error('Request timed out:', error.message);
  } else if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
  } else {
    throw error;
  }
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

> Configure npm registry credentials before release publish.

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
