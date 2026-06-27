# sdkwork-im-sdk (Go)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
go get github.com/sdkwork/im-sdk-generated
```

## Quick Start

```go
package main

import (
    "fmt"
    "github.com/sdkwork/im-sdk-generated"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"

)

func main() {
    cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18079")
    client := github.com/sdkwork/im-sdk-generated.NewSdkworkImClientWithConfig(cfg)
    client.SetAuthToken("your-auth-token")
client.SetAccessToken("your-access-token")
    
    // Use the SDK
    result, err := client.Presence.MeRetrieve()
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
client := github.com/sdkwork/im-sdk-generated.NewSdkworkImClientWithConfig(cfg)

// Set custom headers
client.SetHeader("X-Custom-Header", "value")
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

```go
// Retrieve current principal presence
result, err := client.Presence.MeRetrieve()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### realtime

```go
// List pending realtime events
params := map[string]interface{}{
    "limit": 1,
    "cursor": "cursor",
}
result, err := client.Realtime.EventsList(params)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### calls

```go
// Create an IM call signaling session
body := sdktypes.CreateRtcSessionRequest{
    RtcSessionId: "rtcSessionId",
    ConversationId: "conversationId",
    RtcMode: "rtcMode",
}
result, err := client.Calls.SessionsCreate(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### social

```go
// List contact tags
params := map[string]interface{}{
    "limit": 1,
    "cursor": "cursor",
}
result, err := client.Social.ContactsTagsList(params)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### chat

```go
// List IM contacts
params := map[string]interface{}{
    "limit": 1,
    "cursor": "cursor",
}
result, err := client.Chat.ContactsList(params)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### streams

```go
// Open a stream
body := sdktypes.OpenStreamRequest{
    StreamType: "streamType",
    ConversationId: "conversationId",
}
result, err := client.Streams.Create(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### spaces

```go
// List spaces
result, err := client.Spaces.List()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

## Error Handling

```go
_, err := client.Presence.MeRetrieve()
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

- HTTP/OpenAPI generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- HTTP/OpenAPI generation also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- HTTP/OpenAPI apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put HTTP/OpenAPI hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across HTTP/OpenAPI regenerations.
- If an HTTP/OpenAPI generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
- RPC SDK source workspaces use convention-first evidence by default: RPC SDK family naming, language workspace naming, `rpc/*.manifest.json`, proto source references, generated client source, and native package manifests.
- Use `sdkgen inspect --protocol rpc` to verify RPC convention evidence. Request persisted generator evidence only with `--emit-control-plane` for release, CI, audit, or migration workflows; evidence paths are derived by generator convention.
