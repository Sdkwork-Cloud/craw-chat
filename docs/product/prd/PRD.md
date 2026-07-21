# Sdkwork IM PRD

Status: active
Owner: SDKWork maintainers
Application: chat
Updated: 2026-07-21
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

- **Call signaling lifecycle**: Owned by `im-calls-service` at `/im/v3/api/calls/sessions/*`. Full state machine `started -> accepted -> ended` plus `rejected` terminal state, with idempotency keys per mutation and monotonic signal sequence numbers.
- **Signaling endpoints**: `create`, `retrieve`, `invite`, `accept`, `reject`, `end`, `signals` (post relay), `credentials` (participant credential issuance with initiator/participant authorization gate).
- **Provider handoff**: RTC media runtime comes from `../sdkwork-rtc`; the IM service issues tenant-scoped credentials that the RTC media runtime validates. Call state and signaling events are durable (`im_rtc_sessions`, `im_rtc_signals` tables).
- **Boundary**: IM owns signaling; RTC owns media. The boundary is enforced by `pnpm test:rtc-signaling-boundary`.

### 3.2 Conversations

- **Direct chat**: 1:1 conversations with stable ID derivation and peer profile hydration.
- **Group chat**: Multi-member conversations with profile management, member roles, and announcements.
- **Agent dialog**: AI assistant conversations with standard agent ID format.
- **Enterprise chat**: Official enterprise communication channels.
- **Conversation preferences**: Pin, mute, mark unread, hide per user per conversation.
- **Managed group knowledge base**: Each Conversation group can lazily initialize one managed
  SDKWork Knowledgebase space. Group creation leaves it absent by default; the initial Owner can
  explicitly request one post-create initialization attempt from the create dialog, or initialize
  it later and retry from the Header or group-information entry. Once active, joined non-Guest
  Owners, Admins, and Members open the complete standalone Knowledgebase application with
  role-derived access; Guests and former members are denied. See
  [REQ-2026-0713-group-knowledgebase.md](../requirements/REQ-2026-0713-group-knowledgebase.md).

### 3.3 Realtime Infrastructure

- **WebSocket CCP protocol**: `auth.init` frame-based authentication, rejecting query tokens in production.
- **Scope subscriptions**: User-level and conversation-level realtime event streams.
- **Cluster routing**: Redis-backed cluster bus with node draining on graceful shutdown.
- **Connection recovery**: Automatic catch-up with checkpoint-based incremental fetch.

### 3.4 Security and Compliance

- **Multi-tenant isolation**: Composite keys `(tenant_id, organization_id)` with SQL CHECK constraints.
- **Gateway protection**: One edge per-IP limiter per gateway ingress, post-auth per-tenant limiting, sliding-window circuit breakers, and trusted-proxy IP extraction.
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
| STABLE | 0.1.0 | Reserved release metadata only; not published while commercial gates remain blocked |

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

As of 2026-07-21:

Overall status: **pre-GA release candidate, commercial sign-off blocked**. The application has not
launched. Direct distribution remains prohibited until real pre-release/capacity runs and complete
checksum, signature, SBOM, provenance, staging E2E, HA, and recovery evidence pass the release gate.

### Backend, API, and Admin

- OpenAPI authorities for `/im/v3/api`, `/app/v3/api`, and `/backend/v3/api` are checked in and drive generated SDK families. The backend SDK has been regenerated for TypeScript, Rust, Flutter, Java, C#, Python, Swift, Kotlin, and Go. Authority verification plus TypeScript, Rust, Flutter, Java, and Python publish checks pass; Swift, Kotlin, and Go remain unverified because their toolchains are not installed, and the generated C# family still emits 524 build warnings. Those gaps block commercial SDK sign-off.
- PostgreSQL migrations live under `database/migrations/` with framework contract tests (`pnpm run test:database-framework-standard`). IM core durable authority is PostgreSQL-only. The PC web runtime uses browser storage (IndexedDB / localStorage) for gateway webstore and sibling modules; the Tauri desktop runtime additionally owns a separate bounded, principal-scoped SQLite offline cache and pending-send queue that is never a server source of truth.
- Message history reads prefer PostgreSQL `message_store` when configured (in-memory cache is not authoritative in cloud service deployments).
- Audit list/export/verify paths fail-closed on PostgreSQL read errors (no silent empty lists).
- Ops lag surfaces start empty until governance/runtime wiring publishes real lag items (no synthetic zero-lag defaults).
- `distributed_runtime_service.proto` (RuntimeTopology, RouteLease, DomainEventRelay) remains **Phase 2 contract-only**; internal RPC host serves RoomOrchestration and MessageDispatch unary RPCs only.
- Admin/console surfaces live in `apps/sdkwork-im-pc` package families. Admin feature services use
  `@sdkwork/im-pc-admin-sdk`, which composes generated `@sdkwork/im-backend-sdk` and
  `@sdkwork/iam-backend-sdk` clients with the shared TokenManager. A generated method is not treated
  as proof that its production admin authority is deployed.
- Gateway chat routes resolve principal directories from environment (catalog path or dev/test allow-all); production forbids `SDKWORK_IM_ALLOW_ALL_PRINCIPALS`.
- Production topology requires `SDKWORK_IM_JWT_REQUIRE_JTI=true`, `SDKWORK_IM_JWT_REPLAY_REDIS_URL`, and Redis for replay protection.
- Production rejects the public dev/test JWT signing secret (`sdkwork-im-dev-jwt-secret-not-for-production-use`) at AppContext validation time (fail-closed).
- Audit, conversation journal, and RTC state stores fail-closed in production when durable backends are unavailable.
- Commit journal recovery and projection consumers replay in bounded batches (`COMMIT_JOURNAL_REPLAY_BATCH_LIMIT` = 200) via `CommitJournal::recorded_page` (PostgreSQL `LIMIT` keyset), preventing unbounded OOM on large journals.
- Application data streams are durably isolated by tenant and organization. Session creation, frame append, and state transitions use PostgreSQL transaction/CAS semantics; frame pages use server-side keyset `LIMIT`, and production processes do not retain or rewrite full stream histories in memory.
- Single-conversation journal recovery uses aggregate-scoped `CommitJournal::recorded_page_for_aggregate` (PostgreSQL `WHERE aggregate_id = $1`) instead of full-journal scan plus in-memory filter.
- Embedded projection apply after journal commit is fail-closed in production (`ContractError::Unavailable`); the cloud projection runtime remains the durable path.
- Portal dashboard/conversations/realtime snapshots expose typed `availability.state = unavailable`
  and omit metrics until an authoritative ops source reports real data.
- Gateway `realtime.events.list` returns `SdkWorkApiResponse` envelope; RPC cursor pagination sets `total_count = 0` when the total is unknown.
- Interactive list HTTP query parameters use canonical `page_size` and `cursor`; `pageSize` is SDK/model naming only and is rejected when sent as a URL query alias.
- Projection contacts, inbox, member-directory, pinned-message, and favorite-message lists reject numeric offset cursors in every environment and use opaque keyset cursors only; pre-launch offset compatibility code has been removed.
- Ops lag, provider-binding, and provider-binding-drift reads use bounded keyset pagination with `cursor` and `page_size` (1-200). Responses expose `data.items` and cursor `data.pageInfo`; lag `uint64` values are decimal strings at the JSON boundary. Unknown query aliases fail with HTTP 400.
- Governance-to-Ops node route mirroring reads a 200-entry internal keyset window rather than cloning every connection on the node. The authoritative route count is carried separately, so cluster totals remain exact and diagnostic bundles report truncation instead of presenting the window as the full set.
- Graceful realtime shutdown fences and releases routes in fixed 256-entry batches; runtime cleanup and Redis/PostgreSQL mirror scans use at most 1,000 routes per page. Control-plane node migration still performs an all-route runtime-state move and all-target mirror rewrite, so million-route migration is not a commercially supported operation.
- Provider-binding mirror rebuilds enumerate tenant overrides in 1,000-entry internal keyset pages and stream snapshots directly into the replacement index, avoiding simultaneous all-tenant ID and snapshot vectors.
- Notification and automation file stores are bounded single-node development/test facilities only. Production startup rejects them and requires PostgreSQL for both the projection store and commit journal. Local JSON stores are capped at 32 MiB and 50,000 records; they are not HA authorities.
- The automation process bounds executions (1,024 / 64 MiB), agent response streams (256 / 64 MiB), frames per response (1,024 / 16 MiB), and tool calls (1,024 / 64 MiB). Active work is never evicted; exhausted capacity fails closed with `automation_runtime_capacity_exhausted`, and in-memory state is committed only after journal append succeeds.
- Automation `/metrics` exports fixed-label capacity rejections, terminal evictions by resource/reason, current resident entries/estimated bytes, and journal append failures. It never places tenant, principal, execution, stream, or tool-call identifiers in metric labels.
- Social open-api handlers emit `SdkWorkApiResponse` / `ProblemDetail` envelopes via SDKWork web-framework response mapping; create routes return `201`, delete routes return `204`, and list/retrieve/update routes return `200`.
- The admin sandbox is a development/test-only contract surface. Production-like runtimes reject
  startup when it is enabled and require a real `SDKWORK_ADMIN_PROXY_TARGET`; file-backed sandbox
  state is never a billing, metering, audit, tenant, or storage authority.
- `shutdown_signal()` handles SIGTERM and SIGINT on Unix for Kubernetes graceful drain.
- Kubernetes source templates include Restricted Pod Security contexts, image pull secrets,
  read-only root filesystems, zero-unavailable rolling updates, host/zone topology spread, complete
  PDB coverage, internal-service HPA coverage, and bounded termination windows. They are not applied
  directly: a clean-revision release materializer requires real OCI digests for all 13 services and
  emits a checksummed bundle. Real image-lock, registry, telemetry, rollout, rollback, and target-cluster
  evidence are still required for commercial sign-off.
- Release policy requires SHA-256 checksums, signing, SBOM, and provenance; those artifacts must be
  produced and verified for the actual release, not inferred from manifest flags.
- Managed group Knowledgebase production activation additionally requires the independently
  deployed Knowledgebase RPC host, durable database and Drive storage, approved network route,
  issued mTLS material, and runtime preflight. Endpoint, certificate, Secret, and storage-claim
  values are deployment-owned inputs and are not fabricated in IM source control.
- PC startup synchronization is intentionally bounded: it refreshes the offline message window
  only; it does not expose a fake global group-member sync or enumerate every group. Selected-group
  member state is loaded through the generated SDK with bounded cursor pagination when the group is
  opened or mutated.

### Client Delivery Matrix

| Surface | Root | Status | Notes |
| --- | --- | --- | --- |
| PC web/desktop | `apps/sdkwork-im-pc` | **Pre-GA remediation in progress** | Core chat uses generated/composed SDKs and server pagination. Startup sync is bounded to the offline message window; selected-group member hydration uses bounded SDK cursor pages. The Tauri offline store is principal-scoped and bounded, uses lease-fenced multi-batch sends, quarantines corrupt payloads, and never replaces PostgreSQL as the production source of truth. Route-level RBAC is implemented (`RequirePermission` gates `/console/*` and `/admin/*`) and realtime maps are capped. Production cluster, direct staging E2E/capacity, recovery, immutable artifact, signing, SBOM, and provenance evidence remain blockers. |
| Console/admin | `apps/sdkwork-im-pc` (`sdkwork-im-console-*`, `sdkwork-im-admin-*`) | **Pre-GA integration only** | Route-level RBAC, fail-closed unavailable states, secret redaction, and generated SDK composition are implemented. Commercial release also requires real durable admin/billing authorities, typed non-`LooseJsonValue` contracts, production upstream preflight, audit/SLO evidence, and signed release artifacts. |
| H5 mobile | `apps/sdkwork-im-h5` | **Pre-GA validation** | IAM `platform: "h5"`, server pagination, a virtualized message history window capped at 500 items, incremental WebSocket message sync, a bounded offline text queue, Drive app SDK integration, and user-visible retry are implemented. Real staging E2E, device matrix, capacity, security, and signed distribution evidence remain required. |
| Flutter mobile | `apps/sdkwork-im-flutter-mobile` | **Pre-GA validation** | Inbox/conversation REST, bounded cursor pagination, incremental WebSocket sync, bounded offline text queue, Drive facade, secure token storage, and retry UX are implemented. Real staging E2E, device matrix, capacity, security, and signed distribution evidence remain required. |

### Commerce and Extension Modules (pre-GA boundaries)

| Module | Status | Notes |
| --- | --- | --- |
| Orders | Merchant/consumer read + cancel/fulfill/pay via `@sdkwork/order-app-sdk` / `@sdkwork/shop-app-sdk` | No delete/create-from-console; completion follows fulfillment lifecycle |
| Shop | Catalog, cart, checkout, `orders.pay` | Favorites and consumer shipping-address APIs not in T1 contracts |
| Community | Feeds, comments, reactions, entry delete via `../sdkwork-community` (`@sdkwork/community-pc-community`); IM integrates through `@sdkwork/im-pc-community` host adapter + gateway proxy | Groups/news/docs/repos/resources tabs deferred until contracts ship |
| Calendar / Mail / Approval / Attendance / Gen-AI tabs | **Contract pending** | Hidden from commercial navigation (`CONTRACT_PENDING_MODULES`) |

### Operations and Evidence

- CI `im-commercial-gates.yml` runs `pnpm verify`, `pnpm check:commercial-readiness`, Playwright Chromium install, and cloud-service tests on `main`.
- Pre-Release and Capacity tier indexes are both `evidence_collected_gate_blocked`. Populated doc-captured slots are retained only as historical engineering evidence; commercial sign-off requires direct runs in the declared pre-release and `capacity-dedicated` profiles.
- The 2026-07-21 full `pnpm run check:commercial-readiness` run completed its functional build, smoke, security, SQLite, Playwright, gateway, and Rust integration stages, then correctly failed four release-evidence gates: Pre-Release evidence is not direct-run sign-off evidence, Capacity evidence is backfilled rather than capacity-dedicated, `publish.status` is `DRAFT`, and the app manifest has no enabled release package. These are real release blockers, not test harness failures.
- Notification request acceptance is implemented and durable, but end-to-end push delivery is **not implemented**. The repository has no authoritative device-token registration/routing store, durable provider worker claim/lease, retry/dead-letter pipeline, provider receipt projection, or invalid-token retirement flow. A request remains `requested`/accepted; it must not be reported as `dispatched` until a real provider receipt is committed. This is a release blocker.
- Kubernetes templates cover the 13 active gateway, realtime, conversation, governance, notification,
  projection, media, streaming, audit, automation, social, space, and ops services. Duplicate pre-release
  contact/interaction compatibility services have been removed from source, builds, and deployment inventory.
- Staging topology profile: `cloud.staging`.
- Customer operations and data protection guides: `docs/product/compliance/`.
- Observability runbook: `deployments/observability/README.md`.

### Remaining Enterprise Rollout Items

- Implement the authoritative device-registration and push-provider delivery plane, including tenant/organization isolation, durable claim leases, bounded retries/backoff, dead letters, provider receipts, token retirement, readiness, and real metrics.
- Implement the general automation target executor through the approved `sdkwork-agents` facade. Agent response start may move an execution to `Running` and a real response completion may move it to `Succeeded`; request acceptance alone remains `Requested`.
- Add durable automation response/tool-call projections, claim/lease recovery, event-sequence recovery after restart, and an outbox/materializer contract for journal/projection partial failure. Current process-local active streams are not HA-safe and remain a release blocker.
- Replace the in-memory provider policy history/full tenant-override snapshot authority with a durable, quota-governed store. Paged Ops mirror rebuild removes transient duplication but does not make an unbounded in-memory policy history commercially safe.
- Resolve database drift before production migration approval. The current development database reports 8 pending migrations and 72 error-level drift differences; do not auto-apply migrations to hide drift.
- Run isolated live PostgreSQL concurrency tests for first insert races, monotonic terminal state/retry updates, tenant/organization negative isolation, and notification cursor continuity under concurrent inserts.
- Replace all-route runtime-state migration and all-target Redis/PostgreSQL mirror rewrites with a bounded durable migration job that has claim fencing, resumable progress, compensating recovery, readiness, and low-cardinality metrics.
- Staging-backed Playwright runs against real cloud-service topology (mock-based chat e2e ships in CI today).
- Multi-region DR automation and published SDK artifact registry (git materialization remains the default today).
- Dedicated staging/capacity topology runs to replace doc-captured Step-11 backfill before formal GA sign-off.
- Desktop-parity offline cache for H5/Flutter (PC desktop has a bounded principal-scoped SQLite cache; mobile clients queue text sends only).
- H5/Flutter RTC calls, reactions, threads, and rich media beyond image attachments.
- Implement or formally defer `distributed_runtime_service.proto` streaming RPC hosts (Phase 2).
- PC production build currently succeeds but reports static/dynamic import overlap and several
  1MB+ chunks; chunk-boundary and dependency-loading optimization remains a pre-GA performance task
  and is not represented as a completed capacity claim.
- Voice market: `@sdkwork/voice-pc-market` lists `audio_assets` via SDK in production; pilot preview via `VITE_SDKWORK_VOICE_MARKET_PILOT` (clone UI pilot-only).
- Voice speech: `@sdkwork/voice-pc-speech` submits TTS through `voice.speech.create` with configurable defaults (`VITE_SDKWORK_VOICE_SPEECH_DEFAULT_MODEL` / `_VOICE`).


## 9. Open Questions
