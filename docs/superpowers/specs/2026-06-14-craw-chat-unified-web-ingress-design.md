# 2026-06-14 Sdkwork IM Unified API Ingress Design

## 1. Goal

Make `sdkwork-api-cloud-gateway` the only unified API ingress for Sdkwork IM HTTP traffic, while preserving `sdkwork-im-server` as the owner of IM realtime transport, TCP/long-connection runtime behavior, and the product static shell/asset hosting boundary.

The design fixes the current split where Sdkwork IM exposes:

- a product web entrypoint
- an internal foundation gateway entrypoint
- multiple service upstreams that are presented as if they were independent public ingress points

The target architecture must remove the extra product-side proxy hop for normal HTTP traffic and make the ingress boundary explicit:

- `sdkwork-api-cloud-gateway` owns API ingress, CORS, request identity, docs, OpenAPI aggregation, and HTTP routing
- `sdkwork-im-server` owns IM realtime transport and any direct transport protocols that cannot be modeled as ordinary HTTP ingress

## 2. Current Problem

The current runtime still exposes a layered flow:

```text
renderer -> sdkwork-im-server:18079 -> internal proxy/router -> product services or sdkwork-api-cloud-gateway:3900
```

This creates three problems:

1. duplicate proxying for HTTP traffic
2. unclear ownership of upstream services
3. inconsistent behavior across applications if every app keeps its own outer web server

The `Upstream Status` output also makes foundation services look like independent product upstreams even when they are actually gateway-backed dependency surfaces.

## 3. Required Target Shape

### 3.1 API Ingress

`sdkwork-api-cloud-gateway` becomes the canonical external HTTP API entrypoint.

It owns:

- one public API ingress for HTTP
- `Authorization` and `Access-Token` request flow
- request identity and `requestId`
- app/base OpenAPI and docs
- browser CORS
- route aggregation
- direct proxying to product services and gateway-backed dependency surfaces

### 3.2 Realtime Transport

`sdkwork-im-server` remains externally reachable for realtime IM transport.

It owns:

- static site shell and frontend asset hosting
- WebSocket and TCP/long-connection transport
- realtime session bootstrap, if required by the IM runtime
- any transport-specific message framing or state handling that is not part of the normal web ingress contract

It does not own:

- generic HTTP ingress for app/backend/open API surfaces
- docs/OpenAPI aggregation
- request identity middleware for the general web app

## 4. Architecture Options

### Option A: Split ownership, no API proxy hop in `sdkwork-im-server` for normal HTTP

Recommended.

Flow:

```text
renderer -> sdkwork-api-cloud-gateway -> product services / foundation services
renderer -> sdkwork-im-server -> realtime transport only
```

Tradeoffs:

- lowest HTTP complexity
- single API ingress contract
- realtime transport stays specialized
- requires explicit boundary cleanup in startup scripts and observability

### Option B: Keep `sdkwork-im-server` as an outer proxy for API traffic, but forward to `sdkwork-api-cloud-gateway`

Not recommended.

Tradeoffs:

- keeps current shape mostly intact
- preserves an extra hop and extra configuration layer
- still makes the architecture hard to reason about
- still produces confusing upstream status output

### Option C: Force all API traffic and realtime through `sdkwork-api-cloud-gateway`

Not recommended for Sdkwork IM.

Tradeoffs:

- simplest externally
- but it degrades realtime ownership clarity and may increase latency/control complexity for IM transport

## 5. Data Flow

### 5.1 HTTP API

```text
browser / desktop renderer
  -> sdkwork-im-server static shell
  -> sdkwork-api-cloud-gateway for API traffic
  -> appbase app-api / product app-api / product backend-api / other dependency services
```

### 5.2 Realtime

```text
browser / desktop renderer
  -> sdkwork-im-server
  -> websocket / TCP transport runtime
```

### 5.3 Shared Foundation Services

Foundation dependencies such as appbase, Drive, and Notary remain gateway-backed dependency surfaces and should not appear as independent public API ingress points in the product runtime summary.

## 6. Observability And Status Output

The current `Upstream Status` section should be reinterpreted as a runtime ownership report, not a flat list of public ingress points.

Required behavior:

- direct product services remain listed as their own upstreams
- foundation gateway-backed services are grouped under a shared foundation gateway heading
- the summary must not imply that `sdkwork-iam-app-api`, `sdkwork-drive-app-api`, and `sdkwork-notary-app-api` are separate public ingress endpoints when they all resolve to the same shared gateway base URL

Expected shape:

```text
Upstream Status
  control-plane-api: http://127.0.0.1:18081
  conversation-runtime: http://127.0.0.1:18082
  Shared Foundation Gateway: http://127.0.0.1:3900 [sdkwork-iam-app-api, sdkwork-drive-app-api, sdkwork-notary-app-api]
```

## 7. Request Identity

Request identity must remain server-owned and be generated by the framework/runtime boundary.

Rules:

- `requestId` is not a client field
- `requestId` must be generated by the server request identity chain
- application and backend clients must not supply `X-Request-Id`
- gateway logging may preserve its own internal correlation fields, but it must not override the server-owned `requestId`

## 8. Migration Plan

### Phase 1

Move the product HTTP API ingress contract to `sdkwork-api-cloud-gateway`.

### Phase 2

Remove the extra `sdkwork-im-server -> sdkwork-api-cloud-gateway` HTTP proxy hop for normal API traffic.

### Phase 3

Keep `sdkwork-im-server` for static shell delivery, realtime transport, and any transport-specific APIs only.

### Phase 4

Update startup summaries and tests to group foundation gateway-backed services.

### Phase 5

Validate that the login/register/session path uses the real runtime context on the actual gateway path, not a product-side fallback.

## 9. Validation

Required checks:

- `pnpm dev:desktop` does not spawn duplicate `sdkwork-api-cloud-gateway` instances
- login/register flows use the real gateway-owned runtime context
- `requestId` differs across requests and is generated by the server identity chain
- `Upstream Status` groups gateway-backed foundation services
- realtime transport remains accessible through `sdkwork-im-server`
- product HTTP API ingress does not depend on a second product-side proxy hop

## 10. Decision

Choose `sdkwork-api-cloud-gateway` as the single HTTP API ingress.

Keep `sdkwork-im-server` for static site delivery, realtime transport, and transport-specific responsibilities.

Do not preserve a second generic HTTP API proxy layer inside `sdkwork-im-server`.
