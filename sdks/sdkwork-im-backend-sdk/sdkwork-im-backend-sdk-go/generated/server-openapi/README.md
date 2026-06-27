# sdkwork-im-backend-sdk (Go)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
go get github.com/sdkwork/im-backend-api-generated
```

## Quick Start

```go
package main

import (
    "fmt"
    "github.com/sdkwork/im-backend-api-generated"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"

)

func main() {
    cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18079")
    client := github.com/sdkwork/im-backend-api-generated.NewSdkworkImBackendClientWithConfig(cfg)
    client.SetAuthToken("your-auth-token")
client.SetAccessToken("your-access-token")
    
    // Use the SDK
    result, err := client.Admin.ApiKeyGroupsList()
    if err != nil {
        panic(err)
    }
    fmt.Println(result)
}
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```go
cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18079")
client := github.com/sdkwork/im-backend-api-generated.NewSdkworkImBackendClientWithConfig(cfg)

// Set custom headers
client.SetHeader("X-Custom-Header", "value")
```

## API Modules

- `client.Ops` - ops API
- `client.Audit` - audit API
- `client.Automation` - automation API
- `client.Control` - control API
- `client.Admin` - admin API

## Usage Examples

### ops

```go
// Retrieve ops health
result, err := client.Ops.HealthRetrieve()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### audit

```go
// List audit records
result, err := client.Audit.RecordsList()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### automation

```go
// Retrieve automation governance
result, err := client.Automation.GovernanceRetrieve()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### control

```go
// Read the control-plane protocol governance snapshot.
result, err := client.Control.ProtocolGovernanceRetrieve()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### admin

```go
// listApiKeyGroups
result, err := client.Admin.ApiKeyGroupsList()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

## Error Handling

```go
_, err := client.Admin.ApiKeyGroupsList()
if err != nil {
    // Handle error
    fmt.Println("Error:", err)
    return
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

> Set `GO_RELEASE_TAG` (or `SDKWORK_RELEASE_TAG`) and push tag if needed.

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
