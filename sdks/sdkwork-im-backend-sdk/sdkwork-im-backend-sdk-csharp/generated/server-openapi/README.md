# sdkwork-im-backend-sdk (C#)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
dotnet add package Sdkwork.Im.BackendApi.Generated
```

Or add to your `.csproj`:

```xml
<PackageReference Include="Sdkwork.Im.BackendApi.Generated" Version="0.1.0" />
```

## Quick Start

```csharp
using Sdkwork.Im.BackendApi.Generated.Models;
using Sdkwork.Im.BackendApi.Generated;
using SDKwork.Common.Core;

var config = new SdkConfig("http://127.0.0.1:18079");
var client = new SdkworkImBackendClient(config);
client.SetAuthToken("your-auth-token");
client.SetAccessToken("your-access-token");

var result = await client.Admin.BillingEventsSummaryRetrieveAsync();
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
var client = new SdkworkImBackendClient(config);

// Set custom headers
client.SetHeader("X-Custom-Header", "value");
```

## API Modules

- `client.Ops` - ops API
- `client.Audit` - audit API
- `client.Automation` - automation API
- `client.Control` - control API
- `client.Admin` - admin API

## Usage Examples

### ops

```csharp
// Retrieve ops health
var result = await client.Ops.HealthRetrieveAsync();
Console.WriteLine(result);
```

### audit

```csharp
// Export audit bundle
var result = await client.Audit.ExportRetrieveAsync();
Console.WriteLine(result);
```

### automation

```csharp
// Retrieve automation governance
var result = await client.Automation.GovernanceRetrieveAsync();
Console.WriteLine(result);
```

### control

```csharp
// Read the control-plane protocol governance snapshot.
var result = await client.Control.ProtocolGovernanceRetrieveAsync();
Console.WriteLine(result);
```

### admin

```csharp
// getBillingEventSummary
var result = await client.Admin.BillingEventsSummaryRetrieveAsync();
Console.WriteLine(result);
```

## Error Handling

```csharp
try
{
    await client.Admin.BillingEventsSummaryRetrieveAsync();
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
