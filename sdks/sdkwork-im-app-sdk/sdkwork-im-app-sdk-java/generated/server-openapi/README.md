# sdkwork-im-app-sdk (Java)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to your `pom.xml`:

```xml
<dependency>
    <groupId>com.sdkwork</groupId>
    <artifactId>im-app-api-generated</artifactId>
    <version>0.1.0</version>
</dependency>
```

Or with Gradle:

```groovy
implementation 'com.sdkwork:im-app-api-generated:0.1.0'
```

## Quick Start

```java
import com.sdkwork.im.app.api.generated.SdkworkImAppClient;
import com.sdkwork.common.core.Types;
import com.sdkwork.im.app.api.generated.model.*;

public class Main {
    public static void main(String[] args) throws Exception {
        Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18079");
        SdkworkImAppClient client = new SdkworkImAppClient(config);
        client.setAuthToken("your-auth-token");
client.setAccessToken("your-access-token");

        // Use the SDK
        NotificationListResponse result = client.getNotification().notificationsList();
        System.out.println(result);
    }
}
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```java
Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18079");
SdkworkImAppClient client = new SdkworkImAppClient(config);

// Set custom headers
client.getHttpClient().setHeader("X-Custom-Header", "value");
```

## API Modules

- `client.getAutomation()` - automation API
- `client.getNotification()` - notification API
- `client.getPortal()` - portal API
- `client.getProvider()` - provider API

## Usage Examples

### automation

```java
// Start an agent response stream
StartAgentResponseRequest body = new StartAgentResponseRequest();
body.setExecutionId("1");
body.setStreamId("1");
body.setStreamType("streamtype");
body.setConversationId("1");
body.setSchemaRef("schemaref");
body.setMemberId("1");
body.setAgent(new AgentSubject());
StreamSession result = client.getAutomation().agentResponsesCreate(body);
System.out.println(result);
```

### notification

```java
// List notifications for the current principal
NotificationListResponse result = client.getNotification().notificationsList();
System.out.println(result);
```

### portal

```java
// Read the tenant portal sign-in snapshot
Map<String, Object> result = client.getPortal().accessRetrieve();
System.out.println(result);
```

### provider

```java
// Retrieve media provider health
Map<String, Object> result = client.getProvider().mediaHealthRetrieve();
System.out.println(result);
```

## Error Handling

```java
try {
    NotificationListResponse result = client.getNotification().notificationsList();
    System.out.println(result);
} catch (Exception e) {
    System.err.println("Error: " + e.getMessage());
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

> Use Maven `settings.xml` credentials and optional `MAVEN_PUBLISH_PROFILE`.

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
