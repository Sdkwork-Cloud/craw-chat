# SDKWork IM Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-06-29
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md, SECURITY_SPEC.md, OPERATIONS_SPEC.md

## 1. System Overview

SDKWork IM is a multi-tenant, event-sourced instant messaging platform built on Rust microservices with Axum, featuring real-time WebSocket delivery, event journal persistence, and CQRS-style projection reads.

### Core Principles

- **Event Sourcing**: All state mutations flow through `im_commit_journal`; projections are derived read models.
- **Multi-Tenant Isolation**: Every organization-scoped table enforces `(tenant_id, organization_id)` composite keys with `NOT NULL DEFAULT '0'` and CHECK constraints preventing empty values.
- **Contract-First**: OpenAPI authorities under `apis/` drive SDK generation for 9 languages; no hand-written HTTP clients in consumers.
- **High Availability**: Gateway and session services support horizontal scaling; disconnect fence and presence state use Redis-backed storage in HA topologies.
- **Defense in Depth**: Trusted-proxy IP validation, per-service circuit breakers, bounded rate limiter memory, two-layer rate limiting (per-IP pre-auth + per-tenant post-auth), and Docker/Kubernetes `_FILE` secret injection.
- **Production Readiness**: Graceful shutdown with connection draining, Kubernetes health probes (liveness/readiness/startup), capacity management with multi-dimensional resource tracking.

### Topology

```
                    ┌─────────────────────────────────┐
                    │     Standalone / Cloud Gateway    │
                    │  (Axum + Rate Limit + Circuit     │
                    │   Breaker + CORS + ConnectInfo)    │
                    └──────┬──────────┬──────────┬─────┘
                           │          │          │
              ┌────────────┘          │          └────────────┐
              ▼                       ▼                       ▼
   ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
   │  Session Gateway  │  │  Comms Conv. Svc │  │  Social Service  │
   │  (WebSocket,      │  │  (Event Journal, │  │  (Contacts,      │
   │   Presence,       │  │   Projection,    │  │   Friend Reqs)   │
   │   Cluster Bus)    │  │   Recovery)      │  │                  │
   └────────┬─────────┘  └────────┬─────────┘  └────────┬─────────┘
            │                     │                     │
            ▼                     ▼                     ▼
   ┌──────────────────────────────────────────────────────────────┐
   │                     PostgreSQL / SQLite                       │
   │  im_commit_journal · im_outbox_events · im_inbox_events      │
   │  im_conversation_messages · im_conversation_seq_counters     │
   └──────────────────────────────────────────────────────────────┘
```

## 2. Service Architecture

### 2.1 Gateway Layer

| Service | Binary | Responsibility |
|---|---|---|
| `sdkwork-im-standalone-gateway` | `sdkwork-im-standalone-gateway` | Single-process deployment embedding IAM, session, and all IM routes on one bind. |
| `sdkwork-im-cloud-gateway` | `sdkwork-im-server` | Split-deploy proxy gateway with registry-driven upstream routing. |

**Gateway Protection**: Both gateway variants apply the following protection layers:

1. **Trusted-Proxy IP Extraction** (`SDKWORK_IM_GATEWAY_TRUSTED_PROXIES`): Only honours `X-Forwarded-For` / `X-Real-IP` when the direct TCP peer (via `ConnectInfo<SocketAddr>`) is in the configured trusted-proxy list. Prevents IP-spoofing bypass of rate limits. When no trusted proxies are configured, the direct peer IP is used exclusively.

2. **Rate Limiting (two layers)**:
   - **Layer 1 — per-IP token bucket** (default 600 RPM / 50 burst): Runs pre-auth, before IAM context resolution. Uses `DashMap` for lock-free concurrent access (P1-8 fix). Retry-after is dynamically calculated based on actual RPM: `ceil(60 / max_rpm)` seconds (P0-5 fix). Bounded eviction at `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` (default 5000) prevents unbounded memory growth from rotating client IPs. When real client IP cannot be determined (no trusted proxies, no ConnectInfo), a header-based hash generates a unique fallback IP to prevent all unknown-IP requests from sharing a single rate-limit bucket (P1-9 fix).
   - **Layer 2 — per-tenant token bucket** (default 60 000 RPM / 2 000 burst): Runs post-auth, after `AppContext` is resolved by the IAM interceptor chain. Each authenticated tenant has an independent bucket so that a noisy tenant on a shared NAT egress IP cannot exhaust the IP-level budget for other tenants. Configurable via `SDKWORK_IM_GATEWAY_TENANT_RATE_LIMIT_RPM`, `SDKWORK_IM_GATEWAY_TENANT_RATE_LIMIT_BURST`, `SDKWORK_IM_GATEWAY_TENANT_RATE_LIMIT_MAX_ENTRIES` (default 10 000). Unauthenticated public routes are governed solely by Layer 1.

3. **Per-Service Circuit Breaker** (`CircuitBreakerRegistry`): Each upstream service has an independent circuit breaker. Failures in one service do not trip the breaker for others. HalfOpen state allows only a single probe request at a time. Configurable via `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD` (default 10) and `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_RESET_SECS` (default 30).

4. **CORS Production Safety**: Both gateways reject `allow_any_origin=true` in production. If no explicit origins are configured in production, safe defaults are applied.

5. **Body Size Limit**: Gateway proxy requests are capped at 5 MB (configurable via `SDKWORK_IM_GATEWAY_MAX_REQUEST_BODY_BYTES`, hard max 20 MB). Large file uploads should use presigned URL direct-to-storage, not gateway proxy.

### 2.2 Session Gateway

Manages WebSocket lifecycle, presence, and cluster routing:

- **CCP Protocol**: Dual-protocol WebSocket with `auth.init` frame authentication. Tokens are passed via `Authorization` and `Access-Token` headers in the auth frame, never in query parameters. Query-token mode is rejected in production.
- **Connection Limiting**: Semaphore-based concurrent WebSocket connection cap (`SDKWORK_IM_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS`). Max message size 512 KB, max frame size 256 KB.
- **Cluster Bus**: Inter-node presence sync via `SDKWORK_IM_REALTIME_CLUSTER_BUS_*` env vars. Redis-backed in HA; in-memory fallback for single-node dev.
- **Disconnect Fence**: Prevents stale session takeover during network partitions. Storage backend is configurable — Redis for HA, in-memory for dev. **P1-7 fix**: Added `expire_fences_older_than()` method to clean up fences older than N days, preventing storage膨胀 from long-term offline devices.
- **Heartbeat Mechanism**: Server-initiated heartbeat at configurable interval (default 30s) detects silent disconnects and enforces idle timeout (default 90s). This prevents zombie connections that would otherwise occupy route slots indefinitely (P0-2 fix). Configurable via `SDKWORK_IM_WEBSOCKET_HEARTBEAT_INTERVAL_SECS` and `SDKWORK_IM_WEBSOCKET_IDLE_TIMEOUT_SECS`.
- **Route Epoch Change Grace**: Increased from 25ms to 250ms to give clients more time to handle route migrations without missing state changes (P2-3 fix).

### 2.3 Comms Conversation Service

Event-sourced conversation engine:

- **Write Path**: Commands append to `im_commit_journal` via append-only journal with idempotency keys.
- **Read Path**: Projections serve materialized views from `im_conversation_messages` with `(tenant_id, organization_id, conversation_id)` composite indexes.
- **Recovery**: On startup, replays journal from last checkpoint to rebuild in-memory state. Checkpoint store is Redis-backed in HA.

### 2.4 Social Service

Contact directory and friend request management with `organization_id`-scoped queries.

### 2.5 Supporting Services

| Service | Role |
|---|---|
| `projection-service` | Builds and serves read-model projections from journal events. |
| `notification-service` | Push notification pipeline with outbox dispatch. |
| `automation-service` | Agent/automation response lifecycle. |
| `audit-service` | Compliance audit trail. |
| `governance-service` | Policy enforcement loop. |
| `im-calls-service` | RTC call signaling lifecycle (`create`/`retrieve`/`invite`/`accept`/`reject`/`end`/`signals`/`credentials`), credential issuance, provider handoff to `../sdkwork-rtc`. **Architecture**: Uses `DashMap` for lock-free concurrent session storage with epoch-based fencing (`RtcSession.epoch: u64`) to reject stale concurrent writes. Each state transition increments epoch atomically via `AtomicU64::fetch_add`. Persistence layer (`RtcStateStore.save_state`) compares epoch before merging: higher epoch wins, equal epochs merge monotonically. Participant authorization enforced per SECURITY_SPEC §4.2. |
| `streaming-service` | Media streaming. |
| `space-service` | Workspace/space management. |

## 3. Data Architecture

### 3.1 Event Journal

```sql
im_commit_journal (
    partition_key TEXT,          -- routing key for partitioned reads
    commit_offset BIGINT,        -- monotonic per-partition offset
    event_id      TEXT,          -- globally unique event ID
    tenant_id     TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0' CHECK (organization_id <> ''),
    aggregate_type TEXT,
    aggregate_id   TEXT,
    payload_json   JSONB,
    payload_hash   TEXT,
    occurred_at    TIMESTAMPTZ,
    -- PK: (partition_key, commit_offset)
    -- Indexes: (tenant_id, organization_id, aggregate_type, aggregate_id, aggregate_seq)
)
```

### 3.2 Projection Tables

| Table | Purpose | Org-Scoped |
|---|---|---|
| `im_conversation_messages` | Message read model | Yes |
| `im_conversation_seq_counters` | Per-conversation sequence counter | Yes |
| `im_message_media_refs` | Media attachment references | Yes |

### 3.3 Multi-Tenant Isolation

All organization-scoped tables enforce:
1. `organization_id TEXT NOT NULL DEFAULT '0'` — column constraint
2. `CHECK (organization_id <> '')` — non-empty validation (migration 0005, idempotent)
3. Composite indexes prefixed with `(tenant_id, organization_id, ...)` — query performance
4. Application-level contract test (`sdkwork-im-multi-tenant-isolation-contract.test.mjs`) validates SQL queries include `organization_id` filtering

## 4. WebSocket / Realtime Architecture

### 4.1 Connection Lifecycle

1. Client connects to `wss://gateway/ws/v1/realtime`
2. Client sends `auth.init` frame with access token + device ID
3. Server validates token via IAM auth pool, resolves tenant + organization
4. Server sends `auth.ok` confirmation
5. Bidirectional message stream begins (CCP protocol)
6. Server-initiated heartbeat maintains connection liveness (P0-2 fix)

### 4.2 Token Handling

- Access tokens are passed in the `auth.init` frame, NOT in query parameters for production.
- Query-parameter token mode is **rejected in production** (`SDKWORK_IM_ENVIRONMENT=production`) with HTTP 401. It is permitted only in non-production environments for browser WebSocket compatibility.
- Token normalization accepts `Bearer <token>`, bare `<token>`, and URL-encoded forms.

### 4.3 Heartbeat and Keep-Alive

**Server-Initiated Heartbeat**: The gateway sends periodic heartbeat frames to maintain connection liveness and detect silent disconnects:

- **Heartbeat Interval**: Default 30 seconds, configurable via `SDKWORK_IM_WEBSOCKET_HEARTBEAT_INTERVAL_SECS`
- **Idle Timeout**: Default 90 seconds (3x heartbeat), configurable via `SDKWORK_IM_WEBSOCKET_IDLE_TIMEOUT_SECS`
- **Protocol**: 
  - CCP mode: `HeartbeatFrame` with sequence number
  - Legacy mode: WebSocket Ping/Pong
- **Idle Detection**: If no activity for `idle_timeout` duration, connection is closed with `idle_timeout` close frame
- **Activity Tracking**: Any message (incoming or outgoing) resets the activity timer

This prevents zombie connections that would otherwise occupy route slots and cause resource leaks.

### 4.4 Cluster Routing

In HA deployments, session gateway nodes share presence state via Redis cluster bus. The disconnect fence ensures that when a client reconnects to a different node, the old connection is properly closed before the new one is established.

## 5. Security Architecture

### 5.1 Authentication

- IAM-backed OAuth2 token validation
- Dual-token support: access token + refresh token
- Device binding: tokens are bound to device IDs for session tracking

### 5.2 Secret Management

- All secrets use the Docker/Kubernetes `_FILE` suffix pattern: `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET_FILE`, `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET_FILE`
- When `_FILE` env var is set, the secret is read from the referenced file path
- When only the direct env var is set, the value is used as the literal secret
- `_FILE` variant takes precedence over direct env var
- No placeholder secrets in production topology configurations

### 5.3 Supply Chain

- `checksumRequired: true` — all release artifacts must have SHA-256 checksums
- `signatureRequired: false` — code signing infrastructure pending (pre-launch)
- `sbomRequired: true` — SBOM generation in CI pipeline
- CI validation script rejects fake/placeholder checksums

### 5.4 Network Security

- **Trusted-Proxy IP Extraction**: `X-Forwarded-For` only honoured from trusted proxy IPs (configurable via `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES`). When no trusted proxies are configured and no `ConnectInfo` is available, a header-based hash generates a unique fallback IP to prevent all unknown-IP requests from sharing a single rate-limit bucket (P1-9 fix).
- **Rate limiting**: Per-IP token bucket at gateway layer with bounded memory. Uses `DashMap` for lock-free concurrent access (P1-8 fix). Dynamic retry-after calculation based on actual RPM (P0-5 fix).
- **Circuit breaker**: Per-upstream-service consecutive failure detection prevents cascade failures
- **CORS**: Explicit origin allowlist in production; `allow_any_origin` rejected in production
- **WebSocket auth**: `auth.init` frame-based authentication; query-token auth rejected in production
- **Anomaly Detection**: Configuration errors are handled gracefully with safe defaults rather than panics (P0-4 fix). Invalid `message_rate_threshold`, `failed_auth_threshold`, or `max_log_entries` values are logged as warnings and replaced with sensible defaults, ensuring service availability even with misconfiguration.
- **Idempotency**: Lock timeout enforcement ensures stale reservations are cleared after configured timeout (default 30s), preventing indefinite lockouts on retry failures.

## 6. Deployment Architecture

### 6.1 Deployment Profiles

| Profile | Description | Use Case |
|---|---|---|
| `standalone` | Single-process, all services embedded | Development, small team |
| `cloud` | Split-deploy, horizontally scalable | Production, enterprise |

### 6.2 Environment Topology

Static topology configuration in `configs/topology/` maps upstream service URLs. In Phase 2, this will be replaced by `sdkwork-discovery` service discovery.

### 6.3 Database

- **PostgreSQL**: Production (schema in `database/ddl/baseline/postgres/`)
- **SQLite**: Development (`database/ddl/baseline/sqlite/`)
- Migrations in `database/migrations/postgres/` (0001–0005)
- All migrations are idempotent and safe to re-execute

## 7. Observability

- **Tracing**: `tracing` crate with `tracing-subscriber` env-filter
- **Structured Events**: All gateway events use `target: "sdkwork.im.gateway"` with structured fields
- **Health Checks**: `/healthz` endpoint on gateway
- **Startup Summary**: Gateway prints route registry and configuration summary on boot
- **Circuit Breaker Observability**: Per-service breaker state available via `CircuitBreakerRegistry::state_for(service_id)`

## 8. Architecture Decision Index

| ADR | Title | Status |
|---|---|---|
| ADR-20260619 | IM RPC Discovery Integration Deferred | Active |
| Migration 0003 | Organization scope for commit journal | Applied |
| Migration 0004 | Organization ID default zero alignment | Applied |
| Migration 0005 | Organization ID non-empty CHECK constraint (idempotent) | Applied |

## 9. Verification

| Check | Command | Scope |
|---|---|---|
| Multi-tenant isolation | `node scripts/dev/sdkwork-im-multi-tenant-isolation-contract.test.mjs` | SQL query org_id filtering |
| Gateway rate limit | `cargo test -p sdkwork-im-cloud-gateway gateway_protection` | Token bucket, circuit breaker, trusted proxy |
| Database naming | `pnpm test scripts/dev/sdkwork-im-database-naming-standard.test.mjs` | DDL convention compliance |
| Runtime ID | `pnpm test scripts/dev/sdkwork-im-runtime-id-standard.test.mjs` | Snowflake ID format |
| Full verify | `pnpm verify` | All checks |

## 10. Gateway Protection Configuration Reference

| Variable | Default | Description |
|---|---|---|
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM` | `600` | Max requests per minute per client IP |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST` | `50` | Burst capacity (token bucket size) |
| `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` | `5000` | Max tracked client IPs before forced eviction |
| `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD` | `10` | Consecutive failures before tripping |
| `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_RESET_SECS` | `30` | Seconds before half-open retry |
| `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES` | _(empty)_ | Comma-separated trusted proxy IPs |
| `SDKWORK_IM_GATEWAY_ALLOW_WEBSOCKET_QUERY_TOKENS` | `false` | Allow WebSocket query-token auth (non-production only) |
| `SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET_FILE` | _(empty)_ | Path to file containing HMAC signing secret |
| `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET_FILE` | _(empty)_ | Path to file containing JWT signing secret |
| `SDKWORK_IM_WEBSOCKET_HEARTBEAT_INTERVAL_SECS` | `30` | WebSocket heartbeat interval (P0-2 fix) |
| `SDKWORK_IM_WEBSOCKET_IDLE_TIMEOUT_SECS` | `90` | WebSocket idle timeout before disconnect (P0-2 fix) |
| `SDKWORK_IM_GATEWAY_POOL_MAX_IDLE_PER_HOST` | `50` | HTTP connection pool max idle per host (P1-10 fix) |
| `SDKWORK_IM_GATEWAY_POOL_IDLE_TIMEOUT_SECS` | `90` | HTTP connection pool idle timeout (P1-10 fix) |

## 11. Domain Core Modules (v0.3+)

The `im-domain-core` crate provides foundational domain logic with full test coverage (73 tests passing).

### 11.1 Security Layer

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `security` | Tenant isolation, permission validation, signal replay protection | `TenantIsolationValidator`, `SecurityContext`, `SignalReplayProtector` |
| `audit` | Security event logging | `AuditEvent`, `AuditEventBuilder` |
| `rate_limiter` | Token bucket rate limiting with tenant isolation | `DomainRateLimiter`, `TokenBucket`, `RateLimitError` |
| `idempotency` | Exactly-once processing semantics | `IdempotencyGuard`, `IdempotencyKey`, `IdempotencyState` |

### 11.2 Observability & Operations

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `logging/redactor` | Log sanitization (JWT, Bearer, Email, IP, Access Token) | `LogRedactor`, `RedactionPattern` |
| `lifecycle` | Graceful shutdown and health probes | `GracefulShutdown`, `HealthCheckProbes`, `ServiceState` |
| `capacity` | Multi-dimensional resource tracking | `CapacityManager`, `ResourceQuota`, `ResourceUsage` |

### 11.3 Data Management

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `retention` | Data retention policies | `RetentionClass`, `RetentionPolicy` |

### 11.4 Connection Quality (v0.4+)

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `connection_quality` | Adaptive heartbeat, network metrics, reconnect backoff with jitter | `NetworkMetrics`, `ConnectionQuality`, `AdaptiveHeartbeatPolicy`, `AtomicNetworkMetrics` |

**Key Features**:
- **Jitter-based Reconnect Backoff**: Decorrelated jitter algorithm prevents thundering herd effect when multiple clients disconnect simultaneously (P1-1 fix). Formula: `delay = base * random(1, 2^attempt)` with 60s cap.
- **Adaptive Heartbeat**: Dynamically adjusts interval based on network quality (RTT, loss rate, jitter)
- **Quality Score Calculation**: Composite score from RTT (40%), loss rate (40%), jitter (20%)
- **Connection Quality Levels**: Excellent (>0.9), Good (0.7-0.9), Poor (0.5-0.7), Critical (<0.5)

### 11.5 Presence System (v0.4+)

Extended presence status beyond simple Online/Offline:

| Status | Description | Push QoS |
|--------|-------------|----------|
| `Online` | User actively available | 3 (immediate push) |
| `Away` | User idle or stepped away | 2 (normal push) |
| `Busy` | Do-not-disturb mode | 1 (high-priority only) |
| `Invisible` | Appears offline but connected | 2 (normal push) |
| `Offline` | Disconnected | 0 (queue for later) |

### 11.6 RTC Signaling (v0.4+)

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `rtc` | RTC session lifecycle, signal rate tracking | `RtcSessionState`, `SignalRateTracker`, `RtcSession` |

**Signal Rate Tracker**: Implements sliding window algorithm to prevent signal flooding:
- Window size: 60 seconds default
- Max signals: 100 per window default
- Two-bucket sliding window for accurate rate calculation
- Prevents "boundary problem" of fixed window rate limiters
| `rtc` | RTC session state management | `RtcSessionState`, `StateRecord` |

### 11.4 Test Coverage Summary

```
test result: ok. 73 passed; 0 failed; 0 ignored
├── audit: 3 tests
├── logging/redactor: 15 tests
├── retention: 8 tests
├── room: 2 tests
├── rtc: 2 tests
├── security: 8 tests
├── rate_limiter: 6 tests
├── idempotency: 7 tests
├── lifecycle: 13 tests
└── capacity: 9 tests
```

## 12. Database Migrations

| Migration | Purpose | Status |
|-----------|---------|--------|
| 0001-0005 | Baseline schema | Applied |
| 0006_index_optimization | Composite indexes for query performance | Pending |

### 12.1 Index Optimization (Migration 0006)

Optimizes frequently queried columns with composite indexes:
- `im_commit_journal_tenant_org_idx`: `(tenant_id, organization_id)`
- `im_commit_journal_sequence_idx`: `(sequence_number DESC)`
- `im_outbox_events_created_idx`: `(created_at DESC)`
- `im_conversation_messages_conversation_idx`: `(conversation_id, sequence_number DESC)`
- See full DDL in `database/migrations/postgres/0006_index_optimization.up.sql`

## 13. Production Deployment Checklist

- [ ] Configure `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES` for your load balancer IPs
- [ ] Set up Kubernetes health probes using `/healthz`, `/readyz`, `/startupz`
- [ ] Configure `ResourceQuota` limits per tenant based on subscription tier
- [ ] Enable audit logging to external SIEM
- [ ] Set up capacity monitoring dashboards
- [ ] Configure graceful shutdown timeout (default: 30s)
- [ ] Review rate limit configuration for expected traffic patterns

