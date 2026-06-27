# sdkwork-im-backend-sdk (Kotlin)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to your `build.gradle.kts`:

```kotlin
implementation("com.sdkwork:im-backend-api-generated:0.1.0")
```

Or with Gradle Groovy:

```groovy
implementation 'com.sdkwork:im-backend-api-generated:0.1.0'
```

## Quick Start

```kotlin
import com.sdkwork.im.backend.api.generated.SdkworkImBackendClient
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.common.core.SdkConfig
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val config = SdkConfig(baseUrl = "http://127.0.0.1:18079")
    val client = SdkworkImBackendClient(config)
    client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")

    // Use the SDK
    val result = client.admin.apiKeyGroupsList()
    println(result)
}
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```kotlin
val config = SdkConfig(baseUrl = "http://127.0.0.1:18079")
val client = SdkworkImBackendClient(config)
```

## API Modules

- `client.ops` - ops API
- `client.audit` - audit API
- `client.automation` - automation API
- `client.control` - control API
- `client.admin` - admin API

## Usage Examples

### ops

```kotlin
// Retrieve ops health
val result = client.ops.healthRetrieve()
println(result)
```

### audit

```kotlin
// List audit records
val result = client.audit.recordsList()
println(result)
```

### automation

```kotlin
// Retrieve automation governance
val result = client.automation.governanceRetrieve()
println(result)
```

### control

```kotlin
// Read the control-plane protocol governance snapshot.
val result = client.control.protocolGovernanceRetrieve()
println(result)
```

### admin

```kotlin
// listApiKeyGroups
val result = client.admin.apiKeyGroupsList()
println(result)
```

## Error Handling

```kotlin
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    try {
        val result = client.admin.apiKeyGroupsList()
        println(result)
    } catch (e: Exception) {
        println("Error: ${e.message}")
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

> Configure Gradle publishing credentials and optional `GRADLE_PUBLISH_TASK`.

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
