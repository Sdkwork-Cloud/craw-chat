# sdkwork-im-sdk (Java)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to your `pom.xml`:

```xml
<dependency>
    <groupId>com.sdkwork</groupId>
    <artifactId>im-sdk-generated</artifactId>
    <version>0.1.0</version>
</dependency>
```

Or with Gradle:

```groovy
implementation 'com.sdkwork:im-sdk-generated:0.1.0'
```

## Quick Start

```java
import com.sdkwork.im.sdk.generated.SdkworkImClient;
import com.sdkwork.common.core.Types;
import com.sdkwork.im.sdk.generated.model.*;

public class Main {
    public static void main(String[] args) throws Exception {
        Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18079");
        SdkworkImClient client = new SdkworkImClient(config);
        client.setAuthToken("your-auth-token");
client.setAccessToken("your-access-token");

        // Use the SDK
        PresenceView result = client.getPresence().meRetrieve();
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
SdkworkImClient client = new SdkworkImClient(config);

// Set custom headers
client.getHttpClient().setHeader("X-Custom-Header", "value");
```

## API Modules

- `client.getPresence()` - presence API
- `client.getRealtime()` - realtime API
- `client.getCalls()` - calls API
- `client.getSocial()` - social API
- `client.getChat()` - chat API
- `client.getStreams()` - streams API
- `client.getSpaces()` - spaces API

## Usage Examples

### presence

```java
// Retrieve current principal presence
PresenceView result = client.getPresence().meRetrieve();
System.out.println(result);
```

### realtime

```java
// List pending realtime events
Map<String, Object> params = new LinkedHashMap<>();
params.put("limit", 1);
params.put("cursor", "cursor");
RealtimeEventsResponse result = client.getRealtime().eventsList(params);
System.out.println(result);
```

### calls

```java
// Create an IM call signaling session
CreateRtcSessionRequest body = new CreateRtcSessionRequest();
body.setRtcSessionId("1");
body.setConversationId("1");
body.setRtcMode("rtcmode");
RtcSessionMutationResponse result = client.getCalls().sessionsCreate(body);
System.out.println(result);
```

### social

```java
// List contact tags
Map<String, Object> params = new LinkedHashMap<>();
params.put("limit", 1);
params.put("cursor", "cursor");
ContactTagsResponse result = client.getSocial().contactsTagsList(params);
System.out.println(result);
```

### chat

```java
// List IM contacts
Map<String, Object> params = new LinkedHashMap<>();
params.put("limit", 1);
params.put("cursor", "cursor");
ContactsResponse result = client.getChat().contactsList(params);
System.out.println(result);
```

### streams

```java
// Open a stream
OpenStreamRequest body = new OpenStreamRequest();
body.setStreamType("streamtype");
body.setConversationId("1");
StreamView result = client.getStreams().create(body);
System.out.println(result);
```

### spaces

```java
// List spaces
SpaceListResponse result = client.getSpaces().list();
System.out.println(result);
```

## Error Handling

```java
try {
    PresenceView result = client.getPresence().meRetrieve();
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

- HTTP/OpenAPI generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- HTTP/OpenAPI generation also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- HTTP/OpenAPI apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put HTTP/OpenAPI hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across HTTP/OpenAPI regenerations.
- If an HTTP/OpenAPI generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
- RPC SDK source workspaces use convention-first evidence by default: RPC SDK family naming, language workspace naming, `rpc/*.manifest.json`, proto source references, generated client source, and native package manifests.
- Use `sdkgen inspect --protocol rpc` to verify RPC convention evidence. Request persisted generator evidence only with `--emit-control-plane` for release, CI, audit, or migration workflows; evidence paths are derived by generator convention.
