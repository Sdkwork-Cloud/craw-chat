# sdkwork-im-app-sdk (Kotlin)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to your `build.gradle.kts`:

```kotlin
implementation("com.sdkwork:im-app-api-generated:0.1.0")
```

Or with Gradle Groovy:

```groovy
implementation 'com.sdkwork:im-app-api-generated:0.1.0'
```

## Quick Start

```kotlin
import com.sdkwork.im.app.api.generated.SdkworkImAppClient
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.common.core.SdkConfig
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val config = SdkConfig(baseUrl = "http://127.0.0.1:18079")
    val client = SdkworkImAppClient(config)
    client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")

    // Use the SDK
    val result = client.notification.notificationsList()
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
val client = SdkworkImAppClient(config)
```

## API Modules

- `client.automation` - automation API
- `client.notification` - notification API
- `client.portal` - portal API
- `client.provider` - provider API

## Usage Examples

### automation

```kotlin
// Start an agent response stream
val body = StartAgentResponseRequest(
    executionId = "1",
    streamId = "1",
    streamType = "streamtype",
    conversationId = "1",
    schemaRef = "schemaref",
    memberId = "1",
    agent = AgentSubject()
)
val result = client.automation.agentResponsesCreate(body)
println(result)
```

### notification

```kotlin
// List notifications for the current principal
val result = client.notification.notificationsList()
println(result)
```

### portal

```kotlin
// Read the tenant portal sign-in snapshot
val result = client.portal.accessRetrieve()
println(result)
```

### provider

```kotlin
// Retrieve media provider health
val result = client.provider.mediaHealthRetrieve()
println(result)
```

## Error Handling

```kotlin
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    try {
        val result = client.notification.notificationsList()
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
