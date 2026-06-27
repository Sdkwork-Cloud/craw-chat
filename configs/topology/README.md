# Topology profiles

Machine contract: [../../specs/topology.spec.json](../../specs/topology.spec.json)  
Platform standard: [../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md](../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md)

## Profiles

| File | Profile id | Use |
| --- | --- | --- |
| `standalone.unified-process.development.env` | `standalone.unified-process.development` | Default dev (`pnpm dev`) |
| `standalone.split-services.development.env` | `standalone.split-services.development` | Standalone split local integration |
| `standalone.unified-process.production.env` | `standalone.unified-process.production` | Standalone unified production |
| `standalone.split-services.production.env` | `standalone.split-services.production` | Standalone split production |
| `cloud.split-services.production.env` | `cloud.split-services.production` | Cloud production |
| `cloud.split-services.staging.env` | `cloud.split-services.staging` | Cloud staging / pre-production |

## Standalone gateway

Standalone profiles embed IAM and IM application ingress through `sdkwork-im-standalone-gateway`
on `application.public-ingress`. Client and platform SDK URLs collapse to the same bind.
Startup also provisions IAM tenant application runtime `sdkwork-im-pc` for tenant `100001`
before credential-entry routes (login, registration, QR auth) are served.

| Command | Purpose |
| --- | --- |
| `pnpm gateway:run:standalone` | Run standalone gateway only |
| `pnpm gateway:build:standalone` | Build standalone gateway binary |

## Default development binds

| Surface | Env key | Standalone unified value |
| --- | --- | --- |
| Application ingress | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | `127.0.0.1:18079` |
| Application HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `http://127.0.0.1:18079` |
| Platform gateway (collapsed) | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `http://127.0.0.1:18079` |

Load order: `scripts/im-dev.mjs` and `scripts/im-server-dev.mjs` merge the selected profile before spawning services.

## Split-services internal upstreams

| Service | Bind env | Default bind |
| --- | --- | --- |
| session-gateway (HTTP/WS) | `SDKWORK_IM_INTERNAL_SESSION_GATEWAY_BIND` | `127.0.0.1:18080` |
| session-gateway-rpc (gRPC Phase 1) | `SDKWORK_IM_SESSION_GATEWAY_RPC_BIND_ADDR` | `127.0.0.1:50051` |
| comms-conversation-rpc (gRPC Phase 1) | `SDKWORK_IM_COMMS_CONVERSATION_RPC_BIND_ADDR` | `127.0.0.1:50052` |
| comms-conversation-internal-rpc (gRPC Phase 1.5) | `SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_BIND_ADDR` | `127.0.0.1:50053` |

### session-gateway HA (optional)

| Env key | Purpose |
| --- | --- |
| `SDKWORK_IM_REALTIME_NODE_ID` | Realtime node identity for cluster routing |
| `SDKWORK_IM_REALTIME_CLUSTER_BUS_URL` | Redis pub/sub URL for cross-node route events |
| `SDKWORK_IM_DATABASE_URL` | Postgres-backed realtime stores (**required** when cluster bus is enabled — fail-closed) |
| `SDKWORK_IM_REALTIME_MAX_WEBSOCKET_CONNECTIONS` | WebSocket connection ceiling |
| `SDKWORK_IM_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS` | HTTP in-flight request gate |
| `SDKWORK_IM_SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES` | Max HTTP request body size |

> **HA fail-closed**: When `SDKWORK_IM_REALTIME_CLUSTER_BUS_URL` is set (multi-node topology),
> `SDKWORK_IM_DATABASE_URL` must also be set to a shared Postgres instance. The bootstrap
> will reject startup if cluster bus is enabled without Postgres-backed disconnect fence
> storage, because in-memory fallback is unsafe across nodes.

### Gateway protection (rate limit + circuit breaker)

| Env key | Default | Purpose |
| --- | --- | --- |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM` | `600` | Max requests per minute per client IP |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST` | `50` | Token bucket burst capacity |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` | `5000` | Max tracked client IPs before eviction |
| `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD` | `10` | Consecutive 5xx failures before tripping |
| `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_RESET_SECS` | `30` | Seconds before half-open probe retry |
| `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES` | _(empty)_ | Comma-separated trusted proxy IPs for X-Forwarded-For |

### session-gateway RPC Phase 1

| Env key | Purpose |
| --- | --- |
| `SDKWORK_IM_SESSION_GATEWAY_RPC_BIND_ADDR` | gRPC listener bind address |
| `SDKWORK_IM_SESSION_GATEWAY_RPC_PUBLIC_ENDPOINT` | Advertised gRPC endpoint for topology/gateway manifests |

### Realtime auth and AppContext hardening

| Env key | Purpose |
| --- | --- |
| `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE` | Require HMAC-signed AppContext projection headers on internal services |
| `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET` | Shared secret between gateway and internal services (literal value) |
| `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET_FILE` | Path to file containing the shared secret (Docker/K8s secrets pattern; takes precedence over direct env var) |
| `SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID` | Bootstrap tenant id for tenant-bound JWT verification at realtime boundaries |
| `SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID` | JWT header `kid` for bootstrap signing key (default `bootstrap`) |
| `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET` | HS256 secret when services validate dual tokens directly (literal value) |
| `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET_FILE` | Path to file containing the JWT signing secret (Docker/K8s secrets pattern; takes precedence over direct env var) |
| `SDKWORK_IM_GATEWAY_ALLOW_WEBSOCKET_QUERY_TOKENS` | Opt-in WebSocket query-string token auth (default `false`; rejected in production regardless) |
| `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES` | Comma-separated trusted proxy IPs for X-Forwarded-For validation |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` | Max tracked client IPs before forced eviction (default `5000`) |

IAM database pool (`SDKWORK_IM_DATABASE_*` / `SDKWORK_IM_DATABASE_URL`) enables `resolve_iam_auth_pool_from_env` for authoritative dual-token verification in session-gateway.

## Service persistence backends

| Service | Backend | Config switch | Production behavior |
| --- | --- | --- | --- |
| projection-service | Postgres (durable) + in-memory hot path | `SDKWORK_IM_DATABASE_URL` | Hard-fails without Postgres; snapshots restored on startup |
| audit-service | In-memory (dev/test only) | `SDKWORK_IM_DATABASE_URL` (planned) | Logs `error` when running in production without durable storage; all records lost on restart |
| ops-service | In-memory diagnostics (transient by design) | None needed | Diagnostic views are rebuilt from live services; no persistence required |

## Verification

```bash
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root ../.. --spec specs/topology.spec.json
pnpm test:topology-baggage
pnpm test:sdkwork-im-pc-dev-command
```
