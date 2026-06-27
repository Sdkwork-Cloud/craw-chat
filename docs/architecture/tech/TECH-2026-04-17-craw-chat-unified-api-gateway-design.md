> Migrated from `docs/superpowers/specs/2026-04-17-craw-chat-unified-api-gateway-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-17 Sdkwork IM Unified API Gateway Design

## 1. Goal

Introduce a built-in single-port web gateway for all current Sdkwork IM server-style APIs. The gateway becomes the only standard external entrypoint, while independent services remain available as internal upstreams. The design must standardize:

- single-port access for HTTP and WebSocket traffic
- service-level and aggregate OpenAPI 3.1 contracts
- real-time schema discovery and refresh
- startup output that prints gateway endpoints, schema endpoints, and upstream status
- a stable authority-contract flow for docs and SDK generation

## 2. Current State

The workspace currently exposes multiple standalone HTTP services:

- `session-gateway` on `127.0.0.1:18080`
- `control-plane-api` on `127.0.0.1:18081`
- `conversation-runtime` on `127.0.0.1:18082`
- `projection-service` on `127.0.0.1:18083`
- `streaming-service` on `127.0.0.1:18084`
- `im-call-runtime` on `127.0.0.1:18085`
- `media-service` on `127.0.0.1:18086`
- `notification-service` on `127.0.0.1:18087`
- `automation-service` on `127.0.0.1:18088`
- `audit-service` on `127.0.0.1:18089`
- `local-minimal-node` on `127.0.0.1:18090`
- `ops-service` on `127.0.0.1:18091`

`local-minimal-node` currently aggregates a large app-facing surface and already exports `GET /im/v3/openapi.json`, but the checked-in app contract is no longer a reliable live representation of all real routes. The workspace does not yet have a dedicated gateway/web-host equivalent to `sdkwork-api-router`.

## 3. Design Principles

1. Do not rename business paths only to satisfy gateway concerns.
2. Separate runtime ownership from external visibility.
3. Make the route registry the single source of truth for path ownership.
4. Generate live OpenAPI contracts from the running service surface, not by hand.
5. Keep `local-minimal-node` as an embedded/local profile, not the long-term contract authority.
6. Treat WebSocket and realtime routes as first-class gateway citizens.

## 4. Runtime Model

The system will support two runtime modes with the same external shape:

- `split`
  - all services run independently on loopback
  - the gateway proxies to service upstreams
  - this is the primary production and standard development mode
- `embedded`
  - local one-process or tightly merged mode remains possible
  - the gateway still owns the external base URL and schema URLs
  - this is a convenience profile, not the authority-contract source

External clients must only rely on the gateway base URL, never on direct service binds.

## 5. Target Components

### 5.1 New Components

- `services/web-gateway`
  - single external web host
  - HTTP proxy, WebSocket proxy, health, docs, aggregate schemas
- `crates/sdkwork-im-api-registry`
  - service registry, route registry, ownership rules, visibility rules
- `crates/sdkwork-im-openapi`
  - service schema indexing, aggregate OpenAPI generation, docs HTML helpers
- `crates/sdkwork-im-cloud-gateway-config`
  - bind addresses, upstream config, schema refresh policy, runtime mode
- `crates/sdkwork-im-cloud-gateway-observability`
  - startup summary, route summary, schema status reporting

### 5.2 Existing Components To Preserve

- all current standalone services remain the business owners of their routes
- `local-minimal-node` remains available for embedded/local workflows
- `sdks/sdkwork-im-sdk` continues to consume the public app authority contract

## 6. API Layering

The gateway exposes four contract layers:

1. `aggregate`
   - full single-port contract for the real external surface
   - used for gateway docs, debugging, contract review
2. `public app`
   - formal public contract for `sdkwork-im-sdk`
3. `control`
   - formal admin/control contract for admin SDK and control-plane tools
4. `service`
   - internal service contracts used for registry verification and aggregate generation

## 7. Service Inventory And Ownership

### 7.1 Primary Service Ownership

- `session-gateway`
  - `/im/v3/api/presence/*`
  - `/im/v3/api/realtime/*`
- `control-plane-api`
  - `/backend/v3/api/control/*`
- `conversation-runtime`
  - write-side conversation operations
  - membership mutations
  - message mutations
  - handoff mutations
- `projection-service`
  - read-side conversation operations
  - `/im/v3/api/chat/contacts`
  - `/im/v3/api/chat/inbox`
- `streaming-service`
  - `/im/v3/api/streams/*`
- `im-call-runtime`
  - `/im/v3/api/calls/*`
- `media-service`
  - `/im/v3/api/media/*`
- `notification-service`
  - `/im/v3/api/notifications*`
- `automation-service`
  - `/im/v3/api/automation/*`
- `audit-service`
  - `/backend/v3/api/audit/*`
- `ops-service`
  - `/backend/v3/api/ops/*`
- `app aggregate owner`
  - sdkwork-iam/token APIs
  - `/app/v3/api/portal/*`
  - `/backend/v3/api/control/social/*`
  - `/app/v3/api/iot/*`
  - `/backend/v3/api/iot/*`
  - short-term provider-health routes that have not yet been split

### 7.2 Method-Level Ownership

Ownership is method-specific. The same path may be owned by different services for different HTTP methods. For example:

- `GET /im/v3/api/chat/conversations/{conversationId}/messages` -> `projection-service`
- `POST /im/v3/api/chat/conversations/{conversationId}/messages` -> `conversation-runtime`

The gateway must not resolve such cases by path prefix alone.

## 8. Route Registry Standard

Each route registry entry must include:

- `serviceId`
- `methods`
- `pathPattern`
- `visibility`
- `sdkTarget`
- `operationGroup`
- `upstream`
- `authPolicy`
- `protocol`
- `lifecycle`

Recommended OpenAPI extensions:

- `x-sdkwork-service`
- `x-sdkwork-visibility`
- `x-sdkwork-sdk-targets`
- `x-sdkwork-upstream-path`
- `x-sdkwork-protocol`

Required invariants:

- only one owner per `method + external path`
- registry is authoritative for gateway routing
- registry is authoritative for aggregate OpenAPI inclusion
- service contracts may not expose undeclared aggregate operations
- contract drift must fail CI

## 9. OpenAPI 3.1 Standard

All new live contracts should use `OpenAPI 3.1.0`.

### 9.1 Live Endpoints

Gateway endpoints:

- `/openapi.json`
- `/openapi/index.json`
- `/im/v3/openapi.json`
- `/openapi/sdkwork-im-control.openapi.json`
- `/openapi/services/{serviceId}.openapi.json`
- `/docs`
- `/docs/services/{serviceId}`

Direct service endpoints should converge on:

- `/openapi.json`
- `/docs`

### 9.2 Repo Authority Snapshots

- `openapi/aggregate/sdkwork-im-cloud-gateway.openapi.json`
- `openapi/aggregate/openapi-index.json`
- `openapi/public/sdkwork-im-im.openapi.yaml`
- `openapi/public/sdkwork-im-control.openapi.json`
- `openapi/services/{serviceId}.openapi.json`

For compatibility with existing SDK workflows, the app authority snapshot may also be mirrored into:

- `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`

The long-term source of truth remains `openapi/*`, not `sdks/*`.

## 10. Realtime And WebSocket Standard

WebSocket routes are part of the gateway contract, not a special side channel.

Current required route:

- `GET /im/v3/api/realtime/ws`

The gateway must support:

- HTTP upgrade handling
- auth header and query forwarding
- subprotocol forwarding
- close-code preservation
- websocket route health visibility

OpenAPI documents should describe the upgrade endpoint and carry protocol metadata through vendor extensions when message-level schemas cannot be fully expressed through core OpenAPI alone.

## 11. Schema Refresh And Drift Control

Three refresh modes:

- `startup`
- `manual`
- `watch`

Environment defaults:

- development: `startup + watch`
- CI and production: `startup + manual`

Strictness modes:

- `strict`
  - missing required service health or schema blocks startup
- `best-effort`
  - gateway starts in degraded mode and reports missing schemas clearly

Required drift checks:

- registry vs service live contract
- aggregate contract vs registered operations
- public app snapshot vs generated public app contract
- control snapshot vs generated control contract

## 12. Startup Output Standard

Startup output must include:

- runtime mode
- bind summary
- unified gateway base URL
- health and readiness URLs
- aggregate and service schema URLs
- docs URLs
- upstream health status

Example sections:

- `Mode`
- `Bind Summary`
- `Unified Access`
- `OpenAPI 3.1 Schemas`
- `Upstream Status`

## 13. Docs And SDK Governance

The enforced pipeline becomes:

`routes -> live service schemas -> aggregate/index -> authority snapshots -> docs/sdk`

Rules:

- docs site reads authority snapshots and schema index
- public SDK generation reads only the public app authority contract
- admin SDK generation reads only the control authority contract
- service contracts are not direct public SDK inputs by default

## 14. Rollout Strategy

### Phase A

Define registry, schema index, naming conventions, startup output conventions.

### Phase B

Add live OpenAPI 3.1 export to each standalone service.

### Phase C

Introduce the single-port `web-gateway` with HTTP and WebSocket proxying.

### Phase D

Generate aggregate OpenAPI and authority snapshots from live contracts.

### Phase E

Retire `local-minimal-node` as the implied contract authority and keep it only as an embedded runtime profile.

## 15. Success Criteria

- one documented external base URL for all server APIs
- one aggregate live OpenAPI 3.1 contract
- one schema index for all service contracts
- stable public app and control authority contracts
- direct service schemas available for debugging and internal contract review
- startup logs print gateway, schema, docs, and upstream endpoints
- SDK and docs consume the same authority contracts

