# sdkwork-im-app-sdk (Swift)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/sdkwork/ImAppApiGenerated", from: "0.1.0")
]
```

## Quick Start

```swift
import AppSDK
import SDKworkCommon

let config = SdkConfig(baseUrl: "http://127.0.0.1:18079")
let client = SdkworkImAppClient(config: config)
client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")

// Use the SDK
let result = try await client.notification.notificationsList()
print(result)
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```swift
let config = SdkConfig(baseUrl: "http://127.0.0.1:18079")
let client = SdkworkImAppClient(config: config)

// Set custom headers
client.setHeader("X-Custom-Header", value: "value")
```

## API Modules

- `client.automation` - automation API
- `client.notification` - notification API
- `client.portal` - portal API
- `client.provider` - provider API

## Usage Examples

### automation

```swift
// Start an agent response stream
let body = StartAgentResponseRequest(
    executionId: "1",
    streamId: "1",
    streamType: "streamtype",
    conversationId: "1",
    schemaRef: "schemaref",
    memberId: "1",
    agent: AgentSubject()
)
let result = try await client.automation.agentResponsesCreate(body: body)
print(result)
```

### notification

```swift
// List notifications for the current principal
let result = try await client.notification.notificationsList()
print(result)
```

### portal

```swift
// Read the tenant portal sign-in snapshot
let result = try await client.portal.accessRetrieve()
print(result)
```

### provider

```swift
// Retrieve media provider health
let result = try await client.provider.mediaHealthRetrieve()
print(result)
```

## Error Handling

```swift
do {
    try await client.notification.notificationsList()
} catch {
    print("Error: \(error)")
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

> Set `SWIFT_RELEASE_TAG` (or `SDKWORK_RELEASE_TAG`) for tag-based release.

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
