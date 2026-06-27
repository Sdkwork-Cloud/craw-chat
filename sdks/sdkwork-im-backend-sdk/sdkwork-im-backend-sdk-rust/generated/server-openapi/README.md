# sdkwork-im-backend-sdk (Rust)

Generated SDKWork v3 dual-token transport SDK.

## Installation

```bash
cargo add sdkwork-im-backend-api-generated
```

## Quick Start

```rust
use sdkwork_im_backend_api_generated::{SdkworkImBackendClient, SdkworkConfig};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SdkworkImBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;
    client.set_auth_token("your-auth-token");
client.set_access_token("your-access-token");

    let result = client.admin().api_key_groups_list().await?;
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
let client = SdkworkImBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;
client.set_header("X-Custom-Header", "value");
```

## API Modules

- `client.ops()` - ops API
- `client.audit()` - audit API
- `client.automation()` - automation API
- `client.control()` - control API
- `client.admin()` - admin API

## Usage Examples

### ops

```rust
// Retrieve ops health
let result = client.ops().health_retrieve().await?;
println!("{result:?}");
```

### audit

```rust
// List audit records
let result = client.audit().records_list().await?;
println!("{result:?}");
```

### automation

```rust
// Retrieve automation governance
let result = client.automation().governance_retrieve().await?;
println!("{result:?}");
```

### control

```rust
// Read the control-plane protocol governance snapshot.
let result = client.control().protocol_governance_retrieve().await?;
println!("{result:?}");
```

### admin

```rust
// listApiKeyGroups
let result = client.admin().api_key_groups_list().await?;
println!("{result:?}");
```

## Error Handling

```rust
use sdkwork_im_backend_api_generated::{SdkworkImBackendClient, SdkworkConfig};


let client = SdkworkImBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18079"))?;

let outcome: Result<(), _> = async {
    client.admin().api_key_groups_list().await?;
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

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
