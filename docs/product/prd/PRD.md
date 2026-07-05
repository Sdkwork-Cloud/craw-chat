# Sdkwork IM PRD

Status: active
Owner: SDKWork maintainers
Application: chat
Updated: 2026-07-05
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- [PRD-01-baseline-audit.md](PRD-01-baseline-audit.md)
- [PRD-01-productdesignrequirementsscope.md](PRD-01-productdesignrequirementsscope.md)

## 1. Background And Problem

Sdkwork IM is an enterprise-oriented instant messaging platform with PC web/desktop client,
multi-tenant console/admin surfaces, Rust microservice backend, generated SDK contracts, and
SDKWork-standard deployment profiles (`standalone` / `cloud`).

Product detail lives in the linked PRD shards below.

## 2. Target Users

- **Enterprise employees**: Daily IM communication including 1:1 chat, group chat, file sharing, and voice/video calls.
- **Organization administrators**: User management, conversation governance, audit logging, and compliance configuration.
- **AI agent consumers**: Agent-assisted conversations with welcome messages and automated responses.
- **External contacts**: Federated communication with external users via direct chat binding.

## 3. Core Features

### 3.1 Messaging

- **Text, media, and structured messages**: Text, image, video, voice, file, link, card, applet, music, and video call messages.
- **Message lifecycle**: Send, edit, recall, delete, forward (including media forwarding via Drive reference reuse), and pin.
- **Reactions and replies**: Emoji reactions with interaction summaries, threaded replies with scroll-to-message.
- **Offline sync**: Incremental message synchronization using sequence checkpoints, with concurrency-limited batch processing.
- **Pagination**: Virtualized message list with on-demand older message loading via `loadMoreMessages`.

### 3.1b Voice/Video Calls

- **Call signaling lifecycle**: Owned by `im-calls-service` at `/im/v3/api/calls/sessions/*`. Full state machine `started → accepted → ended` plus `rejected` terminal state, with idempotency keys per mutation and monotonic signal sequence numbers.
- **Signaling endpoints**: `create`, `retrieve`, `invite`, `accept`, `reject`, `end`, `signals` (post relay), `credentials` (participant credential issuance with initiator/participant authorization gate).
- **Provider handoff**: RTC media runtime comes from `../sdkwork-rtc`; the IM service issues tenant-scoped credentials that the RTC media runtime validates. Call state and signaling events are durable (`im_rtc_sessions`, `im_rtc_signals` tables).
- **Boundary**: IM owns signaling; RTC owns media. The boundary is enforced by `pnpm test:rtc-signaling-boundary`.

### 3.2 Conversations

- **Direct chat**: 1:1 conversations with stable ID derivation and peer profile hydration.
- **Group chat**: Multi-member conversations with profile management, member roles, and announcements.
- **Agent dialog**: AI assistant conversations with standard agent ID format.
- **Enterprise chat**: Official enterprise communication channels.
- **Conversation preferences**: Pin, mute, mark unread, hide per user per conversation.

### 3.3 Realtime Infrastructure

- **WebSocket CCP protocol**: `auth.init` frame-based authentication, rejecting query tokens in production.
- **Scope subscriptions**: User-level and conversation-level realtime event streams.
- **Cluster routing**: Redis-backed cluster bus with node draining on graceful shutdown.
- **Connection recovery**: Automatic catch-up with checkpoint-based incremental fetch.

### 3.4 Security and Compliance

- **Multi-tenant isolation**: Composite keys `(tenant_id, organization_id)` with SQL CHECK constraints.
- **Gateway protection**: Two-layer rate limiting (per-IP pre-auth token bucket + per-tenant post-auth token bucket), sliding window circuit breakers, trusted-proxy IP extraction.
- **K8s security**: Restricted Pod Security Standards (runAsNonRoot, readOnlyRootFilesystem, seccomp RuntimeDefault, all capabilities dropped).
- **Supply chain**: SHA-256 checksums, Cosign/Sigstore code signing, SBOM generation.
- **Network isolation**: Default-deny egress with explicit CIDR allowlists for database, Redis, and external HTTPS.

### 3.5 Observability

- **Distributed tracing**: OpenTelemetry OTLP export to centralized collector.
- **Health probes**: `/healthz` (liveness) and `/readyz` (readiness) on every service.
- **Structured logging**: `tracing` crate with environment-configured log levels.

## 4. Non-Functional Requirements

| Category | Target | Implementation |
| --- | --- | --- |
| Availability | 99.9% uptime with 2 replicas per service | HPA + PDB + graceful shutdown |
| Latency | P99 < 200ms for message send/receive | Incremental sync, batch interaction summaries |
| Security | Restricted PSS compliance | securityContext, network policies, code signing |
| Scalability | Horizontal pod autoscaling | HPA templates per service |
| Deployability | Zero-downtime rolling updates | Readiness probes + termination grace period |

## 5. Release Channels

| Channel | Version | Status |
| --- | --- | --- |
| STABLE | 0.2.0 | Security hardening, K8s compliance, frontend performance |

## 6. Dependencies

- **PostgreSQL**: Primary event store and projection store (IM core runtime authority).
- **Desktop local storage**: Browser IndexedDB / localStorage for gateway webstore and sibling modules; not the IM commit journal.
- **Redis**: Cluster bus, route store, sequence allocator.
- **Object storage (S3)**: Media file storage via Drive SDK.
- **IAM**: Tenant and user identity via `iam_tenant`, `iam_user`.
- **Community**: Product logic in sibling `../sdkwork-community`; IM integrates via gateway proxy and `@sdkwork/im-pc-community` host adapter.
- **OpenTelemetry collector**: Distributed tracing and metrics.

## 7. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
| --- | --- | --- | --- |
| Database connection exhaustion | Medium | High | Connection pooling with configurable limits |
| WebSocket connection storms | Low | High | Rate limiting + circuit breaker on gateway |
| Cross-tenant data leakage | Low | Critical | Composite keys + SQL CHECK constraints |
| Message loss during failover | Medium | High | Commit journal + incremental checkpoint sync |

## 8. Commercial Readiness Status

As of 2026-06-30:

### Backend, API, and Admin

- OpenAPI authorities for `/im/v3/api`, `/app/v3/api`, and `/backend/v3/api` are checked in with generated TypeScript and Flutter SDK families.
- PostgreSQL migrations live under `database/migrations/` with framework contract tests (`pnpm run test:database-framework-standard`). IM core durable authority is PostgreSQL-only; desktop uses browser local storage (IndexedDB / localStorage) for gateway webstore and sibling modules, with no local SQL database file.
- Admin/console surfaces ship through `apps/sdkwork-im-pc` package families (`sdkwork-im-console-*`, `sdkwork-im-admin-*`) with generated backend SDK integration.
- Gateway chat routes resolve principal directories from environment (catalog path or dev/test allow-all); production forbids `SDKWORK_IM_ALLOW_ALL_PRINCIPALS`.
- Production rejects the public dev/test JWT signing secret (`sdkwork-im-dev-jwt-secret-not-for-production-use`) at AppContext validation time (fail-closed).
- Audit, conversation journal, and RTC state stores fail-closed in production when durable backends are unavailable.
- Social open-api handlers emit `SdkWorkApiResponse` / `ProblemDetail` envelopes via `finish_api_json`.
- `shutdown_signal()` handles SIGTERM and SIGINT on Unix for Kubernetes graceful drain.
- K8s deployments enforce Restricted Pod Security Standards with `securityContext`, `imagePullSecrets`, and `readOnlyRootFilesystem`.
- Network policies enforce default-deny egress with explicit CIDR allowlists.
- Release artifacts require SHA-256 checksums and Cosign/Sigstore code signing.

### Client Delivery Matrix

| Surface | Root | Status | Notes |
| --- | --- | --- | --- |
| PC web/desktop | `apps/sdkwork-im-pc` | **Production pilot ready** | Core chat SDK-backed with inbox/groups/contacts pagination (`list*Page`, `getGroupById`); bounded sync via `forEachCursorPage`; shell exposes only `COMMERCIAL_RUNTIME_MODULES` with verified SDK read/write surfaces |
| Console/admin | `apps/sdkwork-im-pc` (`sdkwork-im-console-*`, `sdkwork-im-admin-*`) | **Production pilot ready** | Admin overview wired to backend ops/audit SDKs |
| H5 mobile | `apps/sdkwork-im-h5` | **Production pilot ready** | IAM `platform: "h5"`, inbox pagination + virtualized timeline, incremental WebSocket sync, Drive image upload |
| Flutter mobile | `apps/sdkwork-im-flutter-mobile` | **Production pilot ready** | Inbox + conversation REST, incremental WebSocket timeline sync, scroll-up pagination, Drive image upload; tokens in `flutter_secure_storage` |

### Commerce and Extension Modules (pre-GA boundaries)

| Module | Status | Notes |
| --- | --- | --- |
| Orders | Merchant/consumer read + cancel/fulfill/pay via `@sdkwork/order-app-sdk` / `@sdkwork/shop-app-sdk` | No delete/create-from-console; completion follows fulfillment lifecycle |
| Shop | Catalog, cart, checkout, `orders.pay` | Favorites and consumer shipping-address APIs not in T1 contracts |
| Community | Feeds, comments, reactions, entry delete via `../sdkwork-community` (`@sdkwork/community-pc-community`); IM integrates through `@sdkwork/im-pc-community` host adapter + gateway proxy | Groups/news/docs/repos/resources tabs deferred until contracts ship |
| Calendar / Mail / Approval / Attendance / Gen-AI tabs | **Contract pending** | Hidden from commercial navigation (`CONTRACT_PENDING_MODULES`) |

### Operations and Evidence

- CI `im-commercial-gates.yml` runs `pnpm verify`, `pnpm check:commercial-readiness`, Playwright Chromium install, and split-service tests on `main`.
- Pre-Release and Capacity tier evidence indexes both require `evidence_collected_gate_passed`; doc-captured backfill boundaries are declared in each index `boundary` field.
- Push delivery supports FCM HTTP v1 OAuth (`SDKWORK_IM_FCM_CREDENTIALS_PATH`) with legacy server-key fallback, and APNs HTTP/2 JWT (`SDKWORK_IM_APNS_*`) for iOS device tokens.
- Kubernetes reference manifests cover gateway, realtime, conversation, governance, notification, projection, media, streaming, audit, automation, social, space, contact, interaction, and ops services with Ingress, PDB, HPA, ConfigMap, Secret, and NetworkPolicy templates.
- Staging topology profile: `cloud.split-services.staging`.
- Customer operations and data protection guides: `docs/product/compliance/`.
- Observability runbook: `deployments/observability/README.md`.

### Remaining Enterprise Rollout Items

- Staging-backed Playwright runs against real split-service topology (mock-based chat e2e ships in CI today).
- Multi-region DR automation and published SDK artifact registry (git materialization remains the default today).
- Dedicated staging/capacity topology runs to replace doc-captured Step-11 backfill before formal GA sign-off.
- H5/Flutter RTC calls, reactions, threads, and rich media beyond image attachments.
- Voice market: `@sdkwork/voice-pc-market` lists `audio_assets` via SDK in production; pilot preview via `VITE_SDKWORK_VOICE_MARKET_PILOT` (clone UI pilot-only).
- Voice speech: `@sdkwork/voice-pc-speech` submits TTS through `voice.speech.create` with configurable defaults (`VITE_SDKWORK_VOICE_SPEECH_DEFAULT_MODEL` / `_VOICE`).


## 9. Open Questions
