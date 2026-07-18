# Profiles and Environment

Sdkwork IM development and production routing are owned by topology v4. Use
`specs/topology.spec.json` and `etc/topology/*.env` as the only profile authority.

## Development Profiles

| Profile id | Command | Purpose |
| --- | --- | --- |
| `standalone.development` | `pnpm dev`, `pnpm dev:browser`, `pnpm dev:desktop` | Default PostgreSQL standalone development stack |
| `standalone.staging` | pre-production rehearsal | Standalone release rehearsal |
| `standalone.production` | private install templates | Standalone production bind + URL contract |
| `cloud.development` | cloud integration dev | Cloud integration with platform gateway |
| `cloud.staging` | cloud rehearsal | SaaS pre-production rehearsal |
| `cloud.production` | `pnpm build` | SaaS production (`im.sdkwork.com`, `api.sdkwork.com`) |

See [Production Domain Binding](/deployment/production-domain-binding) for public URL keys.

## Application and Platform Surfaces

| Surface | Server env | Client env |
| --- | --- | --- |
| IM HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` |
| IM WebSocket | `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` |
| Platform gateway | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` |
| Ingress bind | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | — |

## Server-Only Dev

`pnpm dev:server` starts `scripts/im-server-dev.mjs`, which runs the server-only development
stack for the selected topology v4 profile. Public profile ids do not encode process layout.

## Packaged Server Deployment

Production server installs use:

- `deployments/templates/server.env.example`
- `deployments/templates/chat.toml.example`
- `/etc/sdkwork/chat/server.env`

Do not use retired pre-topology-v4 profile names or legacy per-profile runtime config trees under
`.runtime/`.

## Authentication Boundary

`sdkwork-appbase` owns login, IAM sessions, dual-token validation, users, tenants, organizations,
and the authoritative IAM context. Public clients send `Authorization: Bearer <auth-token>` and
`Access-Token: <access-token>` only.

For trusted gateway or service-to-service traffic, the gateway validates appbase dual tokens,
drops any client-supplied identity projection, and signs the private forwarded AppContext projection
with `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET`. Protected service routes should run with
`SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`.

## Security Hardening Variables

| Variable | Purpose |
| --- | --- |
| `SDKWORK_IM_BROWSER_ORIGINS` | Comma-separated explicit browser origins allowed to call the public app routes. |
| `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE` | Set to `true` so standalone services reject unsigned AppContext projection headers. |
| `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET` | Non-public HMAC secret shared only between the trusted gateway and internal services. |
| `SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID` | Tenant id bound to the bootstrap IAM signing key used to verify dual-token JWTs at realtime boundaries. |
| `SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID` | JWT header `kid` for the bootstrap signing key (defaults to `bootstrap`). |
| `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET` | HS256 secret for tenant-bound JWT verification when services validate dual tokens directly. |

## Realtime Session Gateway

| Variable | Purpose |
| --- | --- |
| `SDKWORK_IM_INTERNAL_SESSION_GATEWAY_BIND` | Internal HTTP/WebSocket bind for `session-gateway`. |
| `SDKWORK_IM_REALTIME_NODE_ID` | Realtime node identity used by cluster routing. Required when `SDKWORK_IM_REALTIME_ROUTE_STORE_URL` or `SDKWORK_IM_REALTIME_CLUSTER_BUS_URL` is set; the default `session_gateway_local_1` is rejected in cluster mode. |
| `SDKWORK_IM_REALTIME_CLUSTER_BUS_URL` | Redis pub/sub URL for multi-node route handoff. |
| `SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET` | HMAC secret for signed cluster route events when Redis cluster bus or route store is enabled. |
| `SDKWORK_IM_DATABASE_URL` | Postgres-backed realtime stores for HA deployments. |
| `SDKWORK_IM_REALTIME_MAX_WEBSOCKET_CONNECTIONS` | WebSocket connection ceiling per node. |
| `SDKWORK_IM_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS` | HTTP overload gate for public realtime routes. |
| `SDKWORK_IM_SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES` | Maximum accepted HTTP body size for session-gateway. |
| `SDKWORK_IM_SESSION_GATEWAY_RPC_BIND_ADDR` | gRPC bind for `session-gateway-rpc` Phase 1 host. |
| `SDKWORK_IM_SESSION_GATEWAY_RPC_PUBLIC_ENDPOINT` | Advertised gRPC endpoint for internal callers. |
| `SDKWORK_IM_GATEWAY_EMBED_REALTIME_PLANE` | When `true`/`1`, embed session-gateway in the gateway process for single-node dev or HA drills. |
| `SDKWORK_IM_REALTIME_TCP_BIND_ADDR` | Optional TCP link listener (`ccp/tcp/1`). |
| `SDKWORK_IM_REALTIME_UDP_BIND_ADDR` | Optional UDP datagram listener (`ccp/udp/1`). |
| `SDKWORK_IM_REALTIME_QUIC_BIND_ADDR` | Optional QUIC listener (`ccp/quic/1`); requires TLS cert/key env vars. |
| `SDKWORK_IM_REALTIME_QUIC_TLS_CERT_PATH` | PEM certificate for QUIC listener. |
| `SDKWORK_IM_REALTIME_QUIC_TLS_KEY_PATH` | PEM private key for QUIC listener. |
| `SDKWORK_IM_REALTIME_ROUTE_STORE_URL` | Redis-backed route store (tiered with Postgres when both set). |
| `SDKWORK_IM_REALTIME_MAX_LINK_CONNECTIONS` | Per-node link transport connection ceiling. |
| `SDKWORK_IM_LIVE_ROOM_MESSAGE_RATE_LIMIT` | Per-user live-room message posts per second (default `5`, max `60`). |
| `SDKWORK_IM_REQUIRE_REALTIME_PUBLISHER` | When `1`/`true`, fail closed when conversation `post_message` or social commits require realtime delivery but neither embedded fanout/publisher nor outbox enqueue is configured. |
| `SDKWORK_IM_REALTIME_FANOUT_RECIPIENT_BATCH_SIZE` | Chunk size for durable/ephemeral scope fanout (default `256`). |
| `SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_POLL_MS` | Poll interval for conversation outbox relay on session-gateway (default `50`). |
| `SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_TENANT_ID` | Optional scope pin for conversation outbox relay tests. |
| `SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID` | Optional scope pin for conversation outbox relay tests. |
| `SDKWORK_IM_SOCIAL_OUTBOX_RELAY_POLL_MS` | Poll interval for social outbox relay on session-gateway (default `50`). |
| `SDKWORK_IM_SOCIAL_OUTBOX_RELAY_TENANT_ID` | Optional scope pin for social outbox relay tests. |
| `SDKWORK_IM_SOCIAL_OUTBOX_RELAY_ORGANIZATION_ID` | Optional scope pin for social outbox relay tests. |
| `SDKWORK_IM_RTC_OUTBOX_RELAY_POLL_MS` | Poll interval for RTC outbox relay on session-gateway. |
| `SDKWORK_IM_RTC_OUTBOX_RELAY_TENANT_ID` | Optional scope pin for RTC outbox relay tests. |
| `SDKWORK_IM_RTC_OUTBOX_RELAY_ORGANIZATION_ID` | Optional scope pin for RTC outbox relay tests. |

Development orchestration (`pnpm dev`, `gateway:run:standalone`) injects safe local defaults for
`SDKWORK_IM_REALTIME_NODE_ID` and `SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET` when Redis route store or
cluster bus URLs are configured and `SDKWORK_IM_ENVIRONMENT` is not `production`. Production
deployments must set explicit values.

WebSocket clients must negotiate subprotocol `sdkwork-im.ccp.ws.v1`. The legacy `legacy.json` subprotocol remains accepted for compatibility but is deprecated and emits server-side warnings.

Desktop auth tokens on Tauri are stored in the OS keyring (`com.sdkwork.im-pc` / `session:v1`), not in webview `sessionStorage`.
