# sdkwork-im-sdk (Rust)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
cargo add im-sdk-generated
```

## Quick Start

```rust
use im_sdk_generated::{SdkworkImClient, SdkworkConfig};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SdkworkImClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;
    client.set_auth_token("your-auth-token");
client.set_access_token("your-access-token");

    let result = client.presence().me_retrieve().await?;
    println!("{result:?}");
    Ok(())
}
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```rust
let client = SdkworkImClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;
client.set_header("X-Custom-Header", "value");
```

## API Modules

- `client.presence()` - presence API
- `client.realtime()` - realtime API
- `client.calls()` - calls API
- `client.social()` - social API
- `client.chat()` - chat API
- `client.streams()` - streams API
- `client.spaces()` - spaces API

## Usage Examples

### presence

```rust
// Retrieve current principal presence
let result = client.presence().me_retrieve().await?;
println!("{result:?}");
```

### realtime

```rust
use std::collections::HashMap;
// List pending realtime events
let mut query = HashMap::new();
query.insert("limit".to_string(), serde_json::json!(1));
query.insert("cursor".to_string(), serde_json::json!("cursor"));
let result = client.realtime().events_list(Some(&query)).await?;
println!("{result:?}");
```

### calls

```rust
use im_sdk_generated::*;
// Create an IM call signaling session
let body = CreateRtcSessionRequest {
    rtc_session_id: "1".to_string(),
    conversation_id: Some("1".to_string()),
    rtc_mode: "rtcmode".to_string(),
    ..Default::default()
};
let result = client.calls().sessions_create(&body).await?;
println!("{result:?}");
```

### social

```rust
use std::collections::HashMap;
// List contact tags
let mut query = HashMap::new();
query.insert("limit".to_string(), serde_json::json!(1));
query.insert("cursor".to_string(), serde_json::json!("cursor"));
let result = client.social().contacts_tags_list(Some(&query)).await?;
println!("{result:?}");
```

### chat

```rust
use std::collections::HashMap;
// List IM contacts
let mut query = HashMap::new();
query.insert("limit".to_string(), serde_json::json!(1));
query.insert("cursor".to_string(), serde_json::json!("cursor"));
let result = client.chat().contacts_list(Some(&query)).await?;
println!("{result:?}");
```

### streams

```rust
use im_sdk_generated::*;
// Open a stream
let body = OpenStreamRequest {
    stream_type: "streamtype".to_string(),
    conversation_id: Some("1".to_string()),
    ..Default::default()
};
let result = client.streams().create(&body).await?;
println!("{result:?}");
```

### spaces

```rust
// List spaces
let result = client.spaces().list().await?;
println!("{result:?}");
```

## Error Handling

```rust
use im_sdk_generated::{SdkworkImClient, SdkworkConfig};


let client = SdkworkImClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;

let outcome: Result<(), _> = async {
    client.presence().me_retrieve().await?;
    Ok(())
}.await;

match outcome {
    Ok(()) => println!("request completed"),
    Err(error) => eprintln!("request failed: {error}"),
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

> Set cargo registry credentials before `cargo publish` and use `--dry-run` first.

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
