# Topology profiles

Machine contract: [../../specs/topology.spec.json](../../specs/topology.spec.json)  
Platform standard: [../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md](../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md)

## Profiles

| File | Profile id | Use |
| --- | --- | --- |
| `standalone.development.env` | `standalone.development` | Default dev (`pnpm dev`) |
| `standalone.test.env` | `standalone.test` | Standalone integration test |
| `standalone.staging.env` | `standalone.staging` | Standalone staging smoke |
| `standalone.production.env` | `standalone.production` | Standalone production |
| `cloud.development.env` | `cloud.development` | Cloud development integration |
| `cloud.test.env` | `cloud.test` | Cloud test integration |
| `cloud.staging.env` | `cloud.staging` | Cloud staging / pre-production |
| `cloud.production.env` | `cloud.production` | Cloud production |

## Standalone gateway

Standalone profiles embed IAM and IM application ingress through `sdkwork-api-im-standalone-gateway`
on `application.public-ingress`. Client and platform SDK URLs collapse to the same bind.
Startup also provisions IAM tenant application runtime `sdkwork-im-pc` for tenant `100001`
before credential-entry routes (login, registration, QR auth) are served.
The standalone gateway runs the current single-ingress application assembly, so
IM foundation routes such as conversation messages are served by embedded
handlers unless an explicit cloud upstream is configured.

| Command | Purpose |
| --- | --- |
| `pnpm gateway:run:standalone` | Run standalone gateway only |
| `pnpm gateway:build:standalone` | Build standalone gateway binary |

## Default development binds

| Surface | Env key | Standalone value |
| --- | --- | --- |
| Application ingress | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | `127.0.0.1:18079` |
| Application HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `http://127.0.0.1:18079` |
| Platform gateway (collapsed) | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `http://127.0.0.1:18079` |

In the default `standalone.development` profile, IM API, OpenAPI, health, readiness, IAM app-api, and embedded dependency routes all share `http://127.0.0.1:18079`. Process layout is an implementation detail behind the selected deployment profile and must not appear in profile ids or public pnpm scripts. Port `3900` can be used by a separate platform or edge gateway in other workspaces; verify the process identity before diagnosing IM behavior from `3900`.

For `sdkwork-api-im-standalone-gateway`, `/openapi/runtime-summary.json` must report
`runtimeMode: "unified"`. A standalone process that tries to proxy IM chat routes
to unconfigured internal HTTP upstreams is stale or mis-launched and can return
`50301 dependency_unavailable`.

Load order: `scripts/im-dev.mjs` and `scripts/gateway-standalone-run.mjs` merge the selected profile before spawning local processes.

## Public URL convention

The machine-readable authority is `specs/im-api-deployment.spec.json`. Cloud deployments keep
the application and API origins separate: `im-dev`/`api-dev`, `im-test`/`api-test`,
`im-staging`/`api-staging`, and `im`/`api` for production. Standalone deployments publish one
application origin and serve `/im/v3/api`, `/app/v3/api`, `/backend/v3/api`, and the realtime
WebSocket from that same origin. PC and H5 bootstrap must receive these roots through their
standard Vite topology keys; shared packages do not choose environment domains.

## Internal RPC endpoints

| Service | Bind env | Default bind |
| --- | --- | --- |
| session-gateway-rpc (gRPC Phase 1) | `SDKWORK_IM_SESSION_GATEWAY_RPC_BIND_ADDR` | `127.0.0.1:50051` |
| comms-conversation-rpc (gRPC Phase 1) | `SDKWORK_IM_COMMS_CONVERSATION_RPC_BIND_ADDR` | `127.0.0.1:50052` |
| comms-conversation-internal-rpc (gRPC Phase 1.5) | `SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_BIND_ADDR` | `127.0.0.1:50053` |

## Managed Group Knowledgebase Lifecycle RPC

The Conversation service reaches the sibling `sdkwork-knowledgebase` lifecycle host only through
the generated Knowledgebase RPC SDK. The trusted path uses mTLS and framework-verified signed
caller context from the `sdkwork-im` service identity; raw HTTP and manually assembled
authorization headers are not a supported fallback.

The IM outbound client configuration is complete only when every non-secret setting below and
exactly one caller-context signing-key source are present. A fully absent client is permitted only
in development/test where the runtime permits it; partial configuration is rejected in every
environment, and staging/production require the complete client.

| Env key | Purpose |
| --- | --- |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT` | Knowledgebase lifecycle RPC endpoint |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH` | CA certificate file for server verification |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH` | IM client certificate file for mTLS |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH` | IM client private-key file for mTLS |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN` | Expected Knowledgebase TLS server name |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY` | Direct base64url caller-context signing key |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE` | File containing the caller-context signing key |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS` | Positive lifetime for signed caller credentials |
| `SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS` | Positive lifecycle RPC timeout |

`SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY` and
`SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE` are mutually exclusive;
both absent and both present are invalid in a configured client. Do not place concrete values for
these keys in source-controlled profiles.

The Knowledgebase lifecycle host is deployed and operated in the sibling Knowledgebase product. A
staging/production rollout requires its image, Service, network policy, issued CA/server/client
certificates, and persistent database and Drive storage. The host rejects unverified callers and
checks database, runtime, and Drive readiness before accepting lifecycle work; its
`SDKWORK_KNOWLEDGEBASE_DRIVE_STORAGE_ROOT` must be an explicit absolute persistent path in staging
and production. This repository cannot provide or guess the cross-namespace DNS name, Secret
names, certificate paths, image, or volume claim. Supply those environment-owned values through
the deployment process before activating group Knowledgebase initialization.

### session-gateway HA (optional)

| Env key | Purpose |
| --- | --- |
| `SDKWORK_IM_REALTIME_NODE_ID` | Realtime node identity for cluster routing |
| `SDKWORK_IM_REALTIME_CLUSTER_BUS_URL` | Redis pub/sub URL for cross-node route events |
| `SDKWORK_DATABASE_URL` | Postgres-backed realtime stores (**required** when cluster bus is enabled - fail-closed) |
| `SDKWORK_IM_REALTIME_MAX_WEBSOCKET_CONNECTIONS` | WebSocket connection ceiling |
| `SDKWORK_IM_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS` | HTTP in-flight request gate |
| `SDKWORK_IM_SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES` | Max HTTP request body size |
| `SDKWORK_IM_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS` | Total application drain deadline in seconds; default `45`, valid range `5`-`300`, invalid values fail startup |

> **HA fail-closed**: When `SDKWORK_IM_REALTIME_CLUSTER_BUS_URL` is set (multi-node topology),
> `SDKWORK_DATABASE_URL` must also be set to a shared Postgres instance. The bootstrap
> will reject startup if cluster bus is enabled without Postgres-backed disconnect fence
> storage, because in-memory fallback is unsafe across nodes.

On SIGTERM/SIGINT, the gateway first fails readiness and marks its realtime node
draining, then stops listeners and maintenance work, persists disconnect fences,
releases owned routes, cancels the Redis subscriber, and waits only within the
configured total deadline. The Kubernetes reference uses a 45-second application
deadline and `terminationGracePeriodSeconds: 75`, leaving 30 seconds for the
preStop signal, scheduler delay, and forced process termination.

### Gateway protection (rate limit + circuit breaker)

| Env key | Default | Purpose |
| --- | --- | --- |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM` | `600` | Max requests per minute per client IP |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST` | `50` | Token bucket burst capacity |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` | `5000` | Max tracked client IPs before eviction |
| `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD` | `10` | Consecutive 5xx failures before tripping |
| `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_RESET_SECS` | `30` | Seconds before half-open probe retry |
| `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES` | _(empty)_ | Comma-separated trusted proxy IPs for X-Forwarded-For |
| `SDKWORK_IM_GATEWAY_OPENAPI_CACHE_TTL_SECS` | `60` | Successful aggregate `/openapi.json` cache TTL; concurrent misses are coalesced |

Standalone applies one final edge `HybridIpRateLimiter` after IM, IAM, and embedded dependency routers are merged. The canonical infrastructure probe paths (`/healthz`, `/livez`, `/readyz`, `/metrics`) are exempt from IP rate limiting. Legacy `/health` and `/ready` aliases are not served.

`/openapi.json` skips configured upstreams whose `{baseUrl}/openapi.json` resolves to the current gateway aggregate endpoint. This prevents recursive OpenAPI aggregation, the request fan-out that caused API calls to remain pending after startup, and the secondary rate-limit/socket pressure that followed.

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

The process database pool (`SDKWORK_DATABASE_*`) enables `resolve_iam_auth_pool_from_env` for authoritative dual-token verification in session-gateway.

## Service persistence backends

| Service | Backend | Config switch | Production behavior |
| --- | --- | --- | --- |
| session-gateway | PostgreSQL + Redis (realtime stores, route store) | `SDKWORK_DATABASE_URL`, `SDKWORK_IM_REDIS_URL` | **Fail-closed** in production without Postgres pools and membership-gated realtime scopes |
| conversation-service | PostgreSQL normalized Conversation/Message authority + bounded cache | `SDKWORK_DATABASE_URL` | **Fail-closed** in production without the normalized repository; cache never determines correctness |
| audit-service | PostgreSQL (durable) | `SDKWORK_DATABASE_URL` | **Fail-closed panic** in production without durable Postgres storage |
| ops-service | In-memory diagnostics (transient by design) | None needed | Diagnostic views are rebuilt from live services; no persistence required |

## Verification

```bash
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root ../.. --spec specs/topology.spec.json
pnpm test:topology-baggage
pnpm test:sdkwork-im-pc-dev-command
node scripts/dev/sdkwork-im-topology-env-lint.test.mjs
node scripts/dev/sdkwork-im-k8s-secret-guard.test.mjs
```

The topology env lint guard rejects bare `\r` line breaks and concatenated `KEY=VALUE` entries in `.env` profiles (prevents silent `SDKWORK_IM_ENVIRONMENT` drops that disable production fail-closed guards). The K8s secret guard rejects legacy `CHANGE_ME` placeholders and ensures credential placeholders only live inside `Secret` resources, never `ConfigMap`.
