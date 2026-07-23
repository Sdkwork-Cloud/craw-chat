# sdkwork-im-app-sdk (Rust)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
cargo add sdkwork-im-app-api-generated
```

## Quick Start

```rust
use sdkwork_im_app_api_generated::{SdkworkImAppClient, SdkworkConfig};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SdkworkImAppClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;
    client.set_auth_token("your-auth-token");
client.set_access_token("your-access-token");

    let result = client.portal().access_retrieve().await?;
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
let client = SdkworkImAppClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;
client.set_header("X-Custom-Header", "value");
```

## API Modules

- `client.automation()` - automation API
- `client.notifications()` - notifications API
- `client.portal()` - portal API
- `client.provider()` - provider API
- `client.chat()` - chat API

## Usage Examples

### automation

```rust
use sdkwork_im_app_api_generated::*;
// Start an agent response stream
let body = StartAgentResponseRequest {
    execution_id: "1".to_string(),
    stream_id: "1".to_string(),
    stream_type: "streamtype".to_string(),
    conversation_id: "1".to_string(),
    schema_ref: Some("schemaref".to_string()),
    member_id: Some("1".to_string()),
    agent: AgentSubject::default(),
    ..Default::default()
};
let result = client.automation().agent_responses_create(&body).await?;
println!("{result:?}");
```

### notifications

```rust
use std::collections::HashMap;
// List notifications for the current principal
let mut query = HashMap::new();
query.insert("page_size".to_string(), serde_json::json!(1));
query.insert("cursor".to_string(), serde_json::json!("cursor"));
let result = client.notifications().list(Some(&query)).await?;
println!("{result:?}");
```

### portal

```rust
// Read the tenant portal access snapshot
let result = client.portal().access_retrieve().await?;
println!("{result:?}");
```

### provider

```rust
// Retrieve media provider health
let result = client.provider().media_health_retrieve().await?;
println!("{result:?}");
```

### chat

```rust
// Retrieve the group knowledgebase link
let conversation_id = "1";
let result = client.chat().conversations_knowledgebase_retrieve(conversation_id).await?;
println!("{result:?}");
```

## Error Handling

```rust
use sdkwork_im_app_api_generated::{SdkworkImAppClient, SdkworkConfig};


let client = SdkworkImAppClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;

let outcome: Result<(), _> = async {
    client.portal().access_retrieve().await?;
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
