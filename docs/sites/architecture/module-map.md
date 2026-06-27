# Module Map

Understanding the workspace module map is the fastest way to locate behavior and to see which
directories are stable enough to document as product surfaces.

## Top-level Directories

| Directory | Current responsibility |
| --- | --- |
| `adapters/` | Provider and storage adapters such as local disk, local memory, IoT access, IoT MQTT, object storage, and RTC providers |
| `crates/` | Shared contracts, CCP protocol crates, AppContext projection, runtime links, route ownership models, and domain primitives |
| `services/` | App runtime services, control-plane API, operator services, and business subsystems |
| `services/sdkwork-im-cloud-gateway` | Unified external entrypoint, aggregate OpenAPI export, service-schema proxies, rendered docs, and canonical `sdkwork-im-server` binary |
| `tools/` | Local verification tools such as `chat-cli` and smoke workflows |
| `bin/` | PowerShell, Bash, and CMD lifecycle wrappers for local development and operations |
| `deployments/` | Dockerfile, Compose profiles, environment templates, and bootstrap scripts |
| `sdks/` | App and admin SDK workspaces, generation wrappers, and release-catalog metadata |
| `docs/` | Historical docs plus the current VitePress site under `docs/sites` |
| `apps/` | Frontend workspace directories that currently exist but are not documented as mature product surfaces |

## Key Services

| Service | Responsibility |
| --- | --- |
| `sdkwork-im-cloud-gateway` | Unified application ingress, aggregate OpenAPI export, service-schema proxies, rendered docs, and canonical `sdkwork-im-server` binary |
| `comms-conversation-service` | Conversation, membership, message, and handoff behavior |
| `session-gateway` | client route heartbeat, presence, realtime route ownership, disconnect fences, and websocket handling |
| `projection-service` | Inbox, timeline, summary, and read-model projection support |
| `media-service` | Media upload lifecycle, lookup, attachment, and provider-aware download URLs |
| `streaming-service` | Stream sessions, frames, checkpoints, completion, and abort flow |
| `im_calls` / `calls` | IM-owned call lifecycle, signaling, credentials, and RTC media handoff |
| `notification-service` | Notification task submission and retrieval |
| `automation-service` | Automation execution submission and retrieval |
| `audit-service` | Audit record storage and export |
| `ops-service` | Health, cluster, lag, diagnostics, runtime-dir, and provider-binding views |
| `control-plane-api` | Protocol governance, provider governance, and node lifecycle API |

## Key Contract and Protocol Crates

| Crate group | Why it matters |
| --- | --- |
| `sdkwork-im-contract-*` | Business and transport contracts for IM open-platform surfaces |
| `sdkwork-im-ccp-*` | CCP binding, codec, control, core, and registry surfaces |
| `im-platform-contracts` | Provider registry, effective binding, and platform integration contracts |
| `im-storage-*` | Shared storage provider schema, validation, fallback resolution, audit, and snapshot persistence seams |
| `im-app-context` | Shared SDKWork AppContext projection parsing from trusted `x-sdkwork-*` headers |
| `sdkwork-im-runtime-*` | Runtime linking and route-ownership contracts |
| `im-domain-*` | Core domain and event-level models reused by services |

## What The Docs Deliberately Do Not Overstate

- `apps/sdkwork-im-admin` and `apps/sdkwork-im-portal` are not documented as complete products.
- SDK workspaces are documented separately from actual release status.
- Checked-in OpenAPI authority now exists for the app, admin, and management SDK workspaces.
- `sdkwork-im-cloud-gateway` | Unified external entrypoint, aggregate OpenAPI export, service-schema proxies, rendered docs, and canonical `sdkwork-im-server` binary.
- The admin control-plane TypeScript SDK is locally verified, but that does not imply that every
  browser `/backend/v3/api/admin/*` route has already been promoted into the formal control-plane authority.

That distinction matters: directory presence alone is not treated as product delivery.

## What To Read Next

- [SDK Overview](/sdk/index)
- [Architecture Overview](/architecture/overview)
- [Control Plane API Overview](/api-reference/control-plane-api)
