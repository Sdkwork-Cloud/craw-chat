# sdkwork-im-sdk (C#)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
dotnet add package Sdkwork.Im.Sdk.Generated
```

Or add to your `.csproj`:

```xml
<PackageReference Include="Sdkwork.Im.Sdk.Generated" Version="0.1.0" />
```

## Quick Start

```csharp
using Sdkwork.Im.Sdk.Generated.Models;
using Sdkwork.Im.Sdk.Generated;
using SDKwork.Common.Core;

var config = new SdkConfig("http://127.0.0.1:18079");
var client = new SdkworkImClient(config);
client.SetAuthToken("your-auth-token");
client.SetAccessToken("your-access-token");

var result = await client.Presence.MeRetrieveAsync();
Console.WriteLine(result);
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```csharp
var config = new SdkConfig("http://127.0.0.1:18079");
var client = new SdkworkImClient(config);

// Set custom headers
client.SetHeader("X-Custom-Header", "value");
```

## API Modules

- `client.Presence` - presence API
- `client.Realtime` - realtime API
- `client.Calls` - calls API
- `client.Social` - social API
- `client.Chat` - chat API
- `client.Streams` - streams API
- `client.Spaces` - spaces API

## Usage Examples

### presence

```csharp
// Retrieve current principal presence
var result = await client.Presence.MeRetrieveAsync();
Console.WriteLine(result);
```

### realtime

```csharp
// List pending realtime events
var query = new Dictionary<string, object>
{
    ["limit"] = 1,
    ["cursor"] = "cursor",
};
var result = await client.Realtime.EventsListAsync(query);
Console.WriteLine(result);
```

### calls

```csharp
// Create an IM call signaling session
var body = new CreateRtcSessionRequest
{
    RtcSessionId = "1",
    ConversationId = "1",
    RtcMode = "rtcmode",
};
var result = await client.Calls.SessionsCreateAsync(body);
Console.WriteLine(result);
```

### social

```csharp
// List contact tags
var query = new Dictionary<string, object>
{
    ["limit"] = 1,
    ["cursor"] = "cursor",
};
var result = await client.Social.ContactsTagsListAsync(query);
Console.WriteLine(result);
```

### chat

```csharp
// List IM contacts
var query = new Dictionary<string, object>
{
    ["limit"] = 1,
    ["cursor"] = "cursor",
};
var result = await client.Chat.ContactsListAsync(query);
Console.WriteLine(result);
```

### streams

```csharp
// Open a stream
var body = new OpenStreamRequest
{
    StreamType = "streamtype",
    ConversationId = "1",
};
var result = await client.Streams.CreateAsync(body);
Console.WriteLine(result);
```

### spaces

```csharp
// List spaces
var result = await client.Spaces.ListAsync();
Console.WriteLine(result);
```

## Error Handling

```csharp
try
{
    await client.Presence.MeRetrieveAsync();
}
catch (HttpRequestException ex)
{
    Console.WriteLine($"Error: {ex.Message}");
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

> Configure NuGet registry credentials before release publish.

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
