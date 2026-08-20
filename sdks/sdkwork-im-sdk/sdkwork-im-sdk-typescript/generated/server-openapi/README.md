# sdkwork-im-sdk

Professional TypeScript SDK for SDKWork API.

## Installation

```bash
npm install @sdkwork/im-sdk
# or
yarn add @sdkwork/im-sdk
# or
pnpm add @sdkwork/im-sdk
```

## Quick Start

```typescript
import { SdkworkImClient } from '@sdkwork/im-sdk';

const client = new SdkworkImClient({
  baseUrl: 'http://127.0.0.1:18089',
  timeout: 30000,
});

// Mode A: API Key (recommended for server-to-server calls)
client.setApiKey('your-api-key');

// Use the SDK
const result = await client.presence.me.retrieve();
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```typescript
const client = new SdkworkImClient({ baseUrl: 'http://127.0.0.1:18089' });
client.setApiKey('your-api-key');
// Sends: X-API-Key: <apiKey>
```

### Mode B: Dual Token

```typescript
const client = new SdkworkImClient({ baseUrl: 'http://127.0.0.1:18089' });
client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `setApiKey(...)` together with `setAuthToken(...)` + `setAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```typescript
import { SdkworkImClient } from '@sdkwork/im-sdk';

const client = new SdkworkImClient({
  baseUrl: 'http://127.0.0.1:18089',
  timeout: 30000, // Request timeout in ms
  headers: {      // Custom headers
    'X-Custom-Header': 'value',
  },
});
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

```typescript
// Retrieve current principal presence
const result = await client.presence.me.retrieve();
```

### realtime

```typescript
// List pending realtime events
const params = {
  page_size: 1,
  cursor: 'cursor',
};
const result = await client.realtime.events.list(params);
```

### calls

```typescript
// Create an IM call signaling session
const body = {
  rtcSessionId: 'rtcSessionId',
  conversationId: 'conversationId',
  rtcMode: 'rtcMode',
};
const result = await client.calls.sessions.create(body);
```

### social

```typescript
// Retrieve pending incoming friend request count
const result = await client.social.friendRequests.pending.count.retrieve();
```

### chat

```typescript
// Ensure the current user received the system-agent Welcome message
const result = await client.chat.me.welcome.ensure();
```

### streams

```typescript
// Open a stream
const body = {
  streamType: 'streamType',
  conversationId: 'conversationId',
};
const result = await client.streams.create(body);
```

### spaces

```typescript
// List spaces
const params = {
  page_size: 1,
  cursor: 'cursor',
};
const result = await client.spaces.list(params);
```

## Error Handling

```typescript
import { SdkworkImClient, NetworkError, TimeoutError, AuthenticationError } from '@sdkwork/im-sdk';

try {
  const result = await client.presence.me.retrieve();
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

TypeScript check and publish commands use pnpm to materialize workspace dependency versions in a temporary tarball. They reject local-only dependency protocols before npm publication and do not rewrite the source `package.json`.

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

> Set `NPM_TOKEN` (and optional `NPM_REGISTRY_URL`) before release publish.

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
