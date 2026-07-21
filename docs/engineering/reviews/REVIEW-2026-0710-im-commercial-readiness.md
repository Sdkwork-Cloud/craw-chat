# SDKWork IM Commercial Readiness Review

Status: active
Owner: SDKWork maintainers
Review: REVIEW-2026-0710
Updated: 2026-07-21
Specs: `DOCUMENTATION_SPEC.md`, `QUALITY_GATE_SPEC.md`, `RELEASE_SPEC.md`, `PAGINATION_SPEC.md`

## Outcome

SDKWork IM is not ready for commercial production sign-off. Core messaging, PostgreSQL persistence, cursor pagination, desktop offline storage, session draining, and PC rendering have substantial implemented coverage, but known correctness, memory, authorization, HA, and release-evidence blockers remain. Passing static contract checks must not be treated as capacity or production evidence.

## Verified Improvements

- PostgreSQL atomic message, journal, and outbox persistence now binds all timestamps as `TIMESTAMPTZ`, rejects unique-key conflicts instead of silently dropping outbox work, and has a live commit/rollback/replay integration test.
- PC conversation rows use bounded virtual rendering with fixed 64 px geometry, server-cursor load controls, semantic list markup, and cross-window keyboard navigation. A 10,000-row browser fixture verifies mounted-row bounds and repeated-scroll heap retention.
- PC message and companion state now has a centralized bounded cache path under active concurrency remediation; this item remains open until account-switch, deletion-race, protected-overflow, and live-notification-index tests pass independent review.
- The workspace lockfile is mechanically synchronized with the declared workspace package manager and passes frozen-lockfile validation.
- The PRD now distinguishes browser storage, desktop SQLite offline cache, and PostgreSQL server authority consistently with the technical architecture.
- **[2026-07-11] Route-level RBAC implemented**: `apps/sdkwork-im-pc/src/bootstrap/routes.tsx` now wraps `/console/*` and `/admin/*` with `<RequirePermission anyOf={['control.read', 'control.write']}>` so a merely authenticated user can no longer reach privileged surfaces. Client secret fields are redacted and unsupported capability routes are disabled. Closes the "Admin authorization and secrets" High item below.
- **[2026-07-11] Projection tenant isolation fixed**: The PostgreSQL timeline/postgres projection port no longer hard-codes organization to `default`; every projection read/write is scoped by `(tenant_id, organization_id)` at the store boundary. Closes the "Projection tenant isolation" High item below.
- **[2026-07-11] Realtime and RTC memory caps implemented**: `spawn_realtime_maintenance_jobs` interval reduced from 5 minutes to 60 seconds. `CallingRuntime` enforces `RTC_SESSIONS_MAX_ENTRIES` (100 000) and `RTC_SIGNALS_MAX_PER_SESSION` (1 000) with stale-session reaping and fail-closed on capacity. `enforce_client_route_maps_capacity()` reclaims realtime client-route map overflows each maintenance tick. `signal_rate_by_sender` DashMap uses LRU eviction at `SDKWORK_IM_CALLING_SIGNAL_RATE_TRACKER_CACHE_MAX`. Partially addresses the "Deployment memory contract" High item below.
- **[2026-07-11] Topology env lint and K8s secret guard added**: `scripts/dev/sdkwork-im-topology-env-lint.test.mjs` rejects bare `\r` and concatenated `KEY=VALUE` entries that silently drop `SDKWORK_IM_ENVIRONMENT`. `scripts/dev/sdkwork-im-k8s-secret-guard.test.mjs` rejects legacy `CHANGE_ME` placeholders and ensures credential placeholders only live inside `Secret` resources. Partially addresses the "Deployment memory contract" High item below.
- **[2026-07-21] Ops and internal route reads are bounded**: Ops lag/provider-binding/drift endpoints use SDKWork keyset pagination; governance mirrors only a 200-route diagnostic window with an authoritative total. Graceful route drain uses 256-route batches, runtime cleanup and mirror scans use 1,000-route pages, and the in-memory migration implementation no longer clones every binding for rollback.
- **[2026-07-21] Notification and automation acceptance is truthful and bounded**: acceptance no longer fabricates provider dispatch or execution success. Production rejects file-backed stores, local files are capped at 32 MiB/50,000 records, automation resident state has entry/byte budgets, and low-cardinality capacity/journal metrics are exported.
- **[2026-07-21] Backend Ops OpenAPI and SDKs are authority-derived**: live Ops OpenAPI reuses the canonical backend authority, response envelopes and pagination gates pass, and generated backend SDK families preserve typed page payloads. TypeScript, Rust, Flutter, Java, and Python publish checks pass; C# warning cleanup and Swift/Kotlin/Go toolchain verification remain open.
- **[2026-07-21] Projection list offset compatibility removed**: contacts, inbox, member directory, pinned messages, and favorites reject numeric cursors in every environment and retain opaque keyset traversal only. PostgreSQL message search was already keyset-only.
- **[2026-07-21] Full commercial gate completed and failed closed**: functional PC/H5/Flutter, Playwright, production-security, SQLite, gateway, and Rust integration stages completed. Sign-off remains blocked by four evidence gates: non-direct Pre-Release evidence, backfilled Capacity evidence, `publish.status=DRAFT`, and no enabled release package in the app manifest.

## Commercial Blockers

| Severity | Area | Evidence | Required action |
| --- | --- | --- | --- |
| Critical | Durable idempotency | Conversation post and mutation idempotency results are process-local; `im_idempotency_keys` is not the transactional authority. | Approve and implement durable claim/result transactions, replica-safe conflict semantics, retention, and recovery tests. |
| Critical | Projection memory and HA | `message_conversation_index` and `ReceivedMessageIndex` grow with lifetime messages. Replicas replay from no durable checkpoint, advance the page cursor even when an event fails to apply, retry failed snapshot persistence only when another new event arrives, and can overwrite newer snapshots without event-version fencing. | Stop cursor advancement on apply failure; persist retry state independently of new events; add durable lookup/read-count fallbacks, bounded checkpointed replay, consumer lease/fence, monotonic/versioned snapshot writes, metrics, and soak evidence before evicting companion indexes. |
| Critical | Global journal replay order | Global replay keysets on `(partition_key, commit_offset)`. A later event appended to a lexically earlier partition can sort behind the saved cursor and never be returned. | Human data-contract review: introduce a globally monotonic replay coordinate or equivalent durable change feed, migrate/checkpoint consumers, and add late-earlier-partition live tests. |
| High | Aggregate correctness and concurrency | Runtime hydration consumes only the first member/read-cursor page. Snapshot persistence performs serial autocommit writes; stale replicas can regress read state or membership and partial failures can tear the snapshot. | Page or stream complete hydration; introduce versioned conditional writes and one aggregate transaction. Migration/contract review is required for persisted versions. |
| High | Unbounded projection SQL | Production snapshot paths still call timeline loads without `LIMIT`. | Replace with bounded keyset windows/streaming and prove peak memory is independent of total history. |
| High | Desktop disk lifecycle | Tauri SQLite has per-principal TTL/row/logical-byte limits, but no global multi-principal sweep, physical page budget, or thresholded vacuum/WAL truncation. | Approve retention policy, add global stale-scope cleanup and physical file maintenance, then test account churn and crash recovery. |
| High | Deployment memory contract | Commercial deployment validation requires conversation count/byte limits in environment examples, all topology profiles, and Kubernetes ConfigMaps. | Human deployment review is required before changing production/staging configuration. **[2026-07-11 PARTIALLY ADDRESSED]** RTC/realtime in-memory DashMap caps and maintenance-interval reduction to 60 seconds are implemented. Topology env lint and K8s secret guard scripts are added. Topology/K8s memory-limit values in production profiles still require human deployment review. |
| High | Release and capacity evidence | No staging-backed scale run proves target concurrency. Active PC artifacts lack complete signing/checksum/SBOM/provenance evidence and reviewed release media/version alignment. | Produce real artifacts and staging load/HA/DR evidence; placeholder or document-only evidence must continue to fail closed. |
| Critical | Notification delivery plane | Notification request acceptance is durable, but device-token registration/routing, provider worker claim leases, retry/dead-letter handling, receipts, and invalid-token retirement are absent. | Implement the authoritative delivery plane and keep accepted requests in `requested` until a provider receipt is committed. |
| Critical | Automation execution and HA | General target execution, durable response/tool-call projections, worker claim leases, restart recovery, and atomic journal/materializer recovery are absent. | Implement the approved executor boundary and durable fenced recovery before enabling production automation. |
| High | Route migration scalability and consistency | Shutdown drain is batched, but control-plane migration still moves all runtime state before one route-store commit; Redis/PostgreSQL adapters then rewrite every target route and partial persistence failures can leave tiers divergent. | Define and review a bounded durable migration job with fenced batches, resumable progress, compensation, and cross-tier convergence tests. |
| High | Provider policy authority | Policy history and effective override snapshots are process-local full collections without durable ownership or quotas. | Add a durable quota-governed repository and bounded keyset history API; do not treat the paged Ops mirror as authority. |
| High | Database migration state | Local PostgreSQL status reports 8 pending migrations and 72 error-level drift differences. | Classify drift, review the migration plan, and prove upgrade/rollback/idempotence on disposable PostgreSQL before applying production changes. |
| Medium | SDK and frontend performance evidence | C# generated code emits hundreds of warnings; Swift/Kotlin/Go are unverified locally; PC/H5 builds report import overlap and multiple roughly 1 MiB or larger chunks. | Fix generator/source authority warnings, verify missing language toolchains in CI, and introduce real lazy boundaries with bundle budgets. |

## Required Verification Before Sign-Off

1. Run API operation, response-envelope, pagination, database, security, deployment, documentation, SDK, and architecture gates from a frozen workspace.
2. Run live PostgreSQL isolation, transaction, outbox, idempotency, aggregate-concurrency, and projection-recovery tests against the migration-complete schema.
3. Run PC production-build Playwright authorization, large-list, offline/account-switch, message-action, and secret-redaction suites on desktop and mobile viewport classes.
4. Execute staging capacity, long-duration soak, rolling restart, node loss, Redis interruption, PostgreSQL failover, and projection catch-up scenarios with RSS, allocator, queue, pool, event-loop, latency, and error-budget evidence.
5. Run `pnpm check:commercial-readiness`; do not approve release while any gate, evidence index, signing requirement, or reviewed blocker above remains open.

## Decision Requests

- Approve the durable idempotency transaction and retention model.
- Approve versioned aggregate membership/read-cursor persistence and migration.
- Approve desktop global retention/physical file policy.
- Approve production topology memory-limit values before Kubernetes and production profile changes.
- Approve the durable route-migration job and Redis/PostgreSQL convergence contract.
- Approve the canonical tenant-root organization key migration from legacy `default` aliases to numeric `0` before persisted route keys are changed.
