# SDKWork IM RPC SDK

`sdkwork-im-rpc-sdk` is the canonical gRPC SDK family for Sdkwork IM communication services. It is additive to the existing HTTP SDK families and does not replace `sdkwork-im-sdk`, `sdkwork-im-app-sdk`, or `sdkwork-im-backend-sdk`.

## Current Capability Inventory

The current Sdkwork IM repository already has several capabilities that can be wrapped as RPC adapters once their runtime boundaries are wired to tonic or another approved gRPC server stack:

| Existing capability | Current owner | RPC service group | Notes |
| --- | --- | --- | --- |
| Conversation lifecycle, members, read cursors, profiles, inbox | `comms-conversation-service`, `projection-service`, `sdkwork-im-cloud-gateway` | `ConversationService` | App-facing commands and queries for conversations and membership. |
| Message posting, edits, recalls, reactions, pins, favorites | `comms-conversation-service`, `projection-service`, `sdkwork-im-cloud-gateway` | `MessageService` | Uses Drive-backed `MediaResource` for uploaded media instead of redefining upload. |
| Presence, realtime event list/ack/watch, route registration | `session-gateway`, `projection-service`, `web-gateway` | `PresenceService`, `RealtimeService` | Server-streaming methods are intended for backend/private clients and native hosts. |
| Contacts, friend requests, friendships, social graph | `projection-service`, `control-plane-api`, `social-service` | `ContactService`, `SocialService`, `SocialAdminService` | App RPC and backend admin RPC stay separated. |
| Durable streams, frames, checkpoints, completion, abort | `streaming-service`, `automation-service` | `StreamService` | Supports unary mutation plus server-streaming frame watch. |
| RTC/calls signaling and credentials | `sdkwork-im-cloud-gateway calls runtime`, `sdkwork-rtc-sdk` provider adapters | `CallService` | The RPC service exposes communication signaling; provider SDK remains separate. |
| Notifications and notification request fanout | `notification-service` | `NotificationService` | App-facing notification list/request/watch surface. |
| Agent response and tool-call automation | `automation-service` | `AutomationService` | Keeps streaming frames and tool-call lifecycle explicit. |
| Runtime health, lag, replay, diagnostics, cluster control | `ops-service`, `control-plane-api` | `CommunicationOpsService`, `RealtimeNodeAdminService`, `CommunicationControlService` | Backend-only surface for operators and trusted services. |
| Shared-channel sync repair, dead letter, route takeover | `control-plane-api` | `SocialRuntimeAdminService` | Backend-only distributed repair surface. |
| Route leases, topology, domain event relay | `session-gateway`, `control-plane-api`, future RPC runtime adapters | `RuntimeTopologyService`, `RouteLeaseService`, `DomainEventRelayService` | Internal-only service-to-service surface for flexible distributed deployment. |

## Service Catalog

App RPC package: `sdkwork.communication.app.v3`

- `PresenceService`: heartbeat, current presence, presence watch.
- `RealtimeService`: subscription sync, event ack/list/watch.
- `ConversationService`: conversations, members, profiles, preferences, read cursors, member directory, pins.
- `MessageService`: conversation messages, system-channel publish, edits, recalls, favorites, visibility, reactions, pin/unpin.
- `ContactService`: contact list, tags, recommendations, preferences.
- `SocialService`: social users, friend requests, friendships.
- `StreamService`: stream sessions, frames, checkpoint, complete, abort, frame watch.
- `CallService`: call sessions, invite/accept/reject/end, signals, provider credentials, signal watch.
- `NotificationService`: notification list, request, retrieve, watch.
- `AutomationService`: automation executions, agent responses, response frames, tool calls.

Backend RPC package: `sdkwork.communication.backend.v3`

- `CommunicationOpsService`: health, cluster, lag, replay, runtime directory, diagnostics, provider binding drift.
- `RealtimeNodeAdminService`: node activation, drain, route migration.
- `CommunicationControlService`: protocol governance/registry, provider policies, provider registry, provider bindings.
- `SocialAdminService`: direct chat bindings, external connections/member links, managed social resources, shared-channel policies, user blocks.
- `SocialRuntimeAdminService`: shared-channel pending/dead-letter/delivered inventory, repair, requeue, republish, takeover.
- `AuditAdminService`: audit record list/create/export.

Internal RPC package: `sdkwork.communication.internal.v1`

- `RuntimeTopologyService`: distributed node topology and capability discovery.
- `RouteLeaseService`: route lease claim/renew/release/list for sharded realtime deployments.
- `DomainEventRelayService`: internal event publish/ack/watch for cross-node replay and fanout.

## Flexible Distributed Deployment

This RPC SDK is designed for multiple deployment shapes:

| Mode | Shape | Expected RPC use |
| --- | --- | --- |
| single-process local mode | `sdkwork-im-server` with `unified-process` layout hosts app, runtime, realtime, and control behavior in one process. | RPC may bind to loopback for local tools and native desktop hosts. |
| split service mode | Conversation, realtime, streaming, notification, automation, ops, and control-plane processes run separately. | App/backend RPC clients call explicit service endpoints; metadata providers attach auth, access token, trace, deadline, and idempotency metadata. |
| sharded realtime mode | Multiple realtime/session nodes own route leases and fanout shards. | Internal RPC uses `RuntimeTopologyService`, `RouteLeaseService`, and `DomainEventRelayService` for node discovery, route ownership, event relay, and repair. |
| private integration mode | Other backend services integrate Sdkwork IM without browser HTTP SDKs. | Backend/internal RPC clients use TLS or mTLS, health checking, deadlines, and retry rules from the manifest. |

Production distributed deployments should use TLS, service-to-service mTLS for internal RPC, standard gRPC health checking, and controlled reflection. Reflection is allowed in local/private development and must be disabled or access-controlled for public production.

## Generation

RPC SDK generation uses the SDKWork RPC generator path added to `@sdkwork/sdk-generator`:

All baseline SDKWork RPC languages have generated workspaces in this family:

| Language | Workspace | Package name | Generated output |
| --- | --- | --- | --- |
| TypeScript | `sdkwork-im-rpc-sdk-typescript` | `@sdkwork/im-rpc-sdk` | `generated/proto` |
| Go | `sdkwork-im-rpc-sdk-go` | `github.com/sdkwork/im-rpc-sdk-go` | `generated/proto` |
| Java | `sdkwork-im-rpc-sdk-java` | `com.sdkwork.im.rpc` | `generated/proto/java` |
| Python | `sdkwork-im-rpc-sdk-python` | `sdkwork_im_rpc_sdk` | `generated/proto` |
| Rust | `sdkwork-im-rpc-sdk-rust` | `sdkwork-im-rpc-sdk-rust` | `generated/proto` |

TypeScript scaffold dry-run:

```powershell
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate `
  --protocol rpc `
  --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript `
  --name SdkworkImRpc `
  --sdk-name sdkwork-im-rpc-sdk `
  --language typescript `
  --package-name @sdkwork/im-rpc-sdk `
  --dry-run `
  --no-sync-published-version
```

Go scaffold dry-run:

```powershell
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate `
  --protocol rpc `
  --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-go `
  --name SdkworkImRpc `
  --sdk-name sdkwork-im-rpc-sdk `
  --language go `
  --package-name github.com/sdkwork/im-rpc-sdk-go `
  --dry-run `
  --no-sync-published-version
```

Java scaffold dry-run:

```powershell
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate `
  --protocol rpc `
  --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-java `
  --name SdkworkImRpc `
  --sdk-name sdkwork-im-rpc-sdk `
  --language java `
  --package-name com.sdkwork.im.rpc `
  --dry-run `
  --no-sync-published-version
```

Python scaffold dry-run:

```powershell
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate `
  --protocol rpc `
  --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-python `
  --name SdkworkImRpc `
  --sdk-name sdkwork-im-rpc-sdk `
  --language python `
  --package-name sdkwork_im_rpc_sdk `
  --dry-run `
  --no-sync-published-version
```

Rust scaffold dry-run:

```powershell
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate `
  --protocol rpc `
  --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust `
  --name SdkworkImRpc `
  --sdk-name sdkwork-im-rpc-sdk `
  --language rust `
  --package-name sdkwork-im-rpc-sdk-rust `
  --dry-run `
  --no-sync-published-version
```

`sdkgen generate --protocol rpc` orchestrates SDKWork scaffold and Buf/protoc-compatible configuration. It does not replace Protocol Buffers, Buf, protoc, or language-specific gRPC plugins.

RPC SDK source workspaces use convention mode by default. The authoritative source evidence is the `sdkwork-im-rpc-sdk` family root, `rpc/sdkwork-im-rpc.manifest.json`, the proto source under `proto/`, each generated language workspace name, `rpc-methods.json`, and the native package manifest such as `package.json`, `go.mod`, `pom.xml`, `pyproject.toml`, or `Cargo.toml`.

Use `sdkgen inspect --protocol rpc` for every RPC SDK workspace. Inspect validates the convention evidence and reports `evidenceMode: "convention"` for normal source output. Release, CI, audit, or migration workflows may request optional generator evidence with `--emit-control-plane`; those files are derived by generator convention and are not normal source-control evidence for this RPC SDK family.

## HTTP SDK Compatibility

This RPC family preserves existing HTTP SDK ownership:

- `sdkwork-im-sdk` remains the HTTP/OpenAPI SDK family for `/im/v3/api`.
- `sdkwork-im-app-sdk` remains the HTTP/OpenAPI SDK family for `/app/v3/api`.
- `sdkwork-im-backend-sdk` remains the HTTP/OpenAPI SDK family for `/backend/v3/api`.

Frontend/browser UI should continue to use generated HTTP app/open/backend SDKs unless gRPC-Web is explicitly approved. RPC SDKs are intended for distributed backend services, local/private runtimes, native hosts, CLI tooling, and cross-language service integrations.

## Verification

Minimum verification for this contract slice:

```powershell
node .\scripts\dev\sdkwork-im-rpc-contract.test.mjs
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language typescript --package-name @sdkwork/im-rpc-sdk --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-go --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language go --package-name github.com/sdkwork/im-rpc-sdk-go --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-java --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language java --package-name com.sdkwork.im.rpc --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-python --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language python --package-name sdkwork_im_rpc_sdk --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js generate --protocol rpc --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json --proto-root .\proto --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust --name SdkworkImRpc --sdk-name sdkwork-im-rpc-sdk --language rust --package-name sdkwork-im-rpc-sdk-rust --dry-run --no-sync-published-version
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-go --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-java --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-python --json
node ..\sdkwork-sdk-generator\bin\sdkgen.js inspect --protocol rpc --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust --json
npx -y @bufbuild/buf@1.70.0 lint
cargo test -p sdkwork-im-rpc-service-rust
```

Language compile gates:

```powershell
cd .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript; npm run check; npm run build
cd .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-go; go test ./...
cd .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-java; mvn -q -DskipTests package
cd .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-python; python -m compileall src generated\proto
cd .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust; cargo check
```

Do not claim generated language clients compile until Buf/protoc execution and these language-specific compile checks have run.
