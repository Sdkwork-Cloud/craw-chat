# SDKWork IM Commercial Readiness Review

Status: active
Owner: SDKWork maintainers
Review: REVIEW-2026-0710
Updated: 2026-07-10
Specs: `DOCUMENTATION_SPEC.md`, `QUALITY_GATE_SPEC.md`, `RELEASE_SPEC.md`, `PAGINATION_SPEC.md`

## Outcome

SDKWork IM is not ready for commercial production sign-off. Core messaging, PostgreSQL persistence, cursor pagination, desktop offline storage, session draining, and PC rendering have substantial implemented coverage, but known correctness, memory, authorization, HA, and release-evidence blockers remain. Passing static contract checks must not be treated as capacity or production evidence.

## Verified Improvements

- PostgreSQL atomic message, journal, and outbox persistence now binds all timestamps as `TIMESTAMPTZ`, rejects unique-key conflicts instead of silently dropping outbox work, and has a live commit/rollback/replay integration test.
- PC conversation rows use bounded virtual rendering with fixed 64 px geometry, server-cursor load controls, semantic list markup, and cross-window keyboard navigation. A 10,000-row browser fixture verifies mounted-row bounds and repeated-scroll heap retention.
- PC message and companion state now has a centralized bounded cache path under active concurrency remediation; this item remains open until account-switch, deletion-race, protected-overflow, and live-notification-index tests pass independent review.
- The workspace lockfile is mechanically synchronized with the declared workspace package manager and passes frozen-lockfile validation.
- The PRD now distinguishes browser storage, desktop SQLite offline cache, and PostgreSQL server authority consistently with the technical architecture.

## Commercial Blockers

| Severity | Area | Evidence | Required action |
| --- | --- | --- | --- |
| Critical | Durable idempotency | Conversation post and mutation idempotency results are process-local; `im_idempotency_keys` is not the transactional authority. | Approve and implement durable claim/result transactions, replica-safe conflict semantics, retention, and recovery tests. |
| Critical | Projection memory and HA | `message_conversation_index` and `ReceivedMessageIndex` grow with lifetime messages. Replicas replay from no durable checkpoint, advance the page cursor even when an event fails to apply, retry failed snapshot persistence only when another new event arrives, and can overwrite newer snapshots without event-version fencing. | Stop cursor advancement on apply failure; persist retry state independently of new events; add durable lookup/read-count fallbacks, bounded checkpointed replay, consumer lease/fence, monotonic/versioned snapshot writes, metrics, and soak evidence before evicting companion indexes. |
| Critical | Global journal replay order | Global replay keysets on `(partition_key, commit_offset)`. A later event appended to a lexically earlier partition can sort behind the saved cursor and never be returned. | Human data-contract review: introduce a globally monotonic replay coordinate or equivalent durable change feed, migrate/checkpoint consumers, and add late-earlier-partition live tests. |
| High | Projection tenant isolation | PostgreSQL timeline persistence hard-codes organization `default`; the store port omits organization scope. | Human review: change the public port, define existing-row migration/backfill, then add cross-organization live tests. |
| High | Aggregate correctness and concurrency | Runtime hydration consumes only the first member/read-cursor page. Snapshot persistence performs serial autocommit writes; stale replicas can regress read state or membership and partial failures can tear the snapshot. | Page or stream complete hydration; introduce versioned conditional writes and one aggregate transaction. Migration/contract review is required for persisted versions. |
| High | Unbounded projection SQL | Production snapshot paths still call timeline loads without `LIMIT`. | Replace with bounded keyset windows/streaming and prove peak memory is independent of total history. |
| High | Admin authorization and secrets | `/admin/*` and `/console/*` have authentication gates but no verified permission guard; settings surfaces can render a server `secretKey`; an unavailable settings route remains reachable. | Human security review: define permission codes, enforce route/data authorization, remove client secret fields, and disable unsupported routes before delivery. |
| High | Desktop disk lifecycle | Tauri SQLite has per-principal TTL/row/logical-byte limits, but no global multi-principal sweep, physical page budget, or thresholded vacuum/WAL truncation. | Approve retention policy, add global stale-scope cleanup and physical file maintenance, then test account churn and crash recovery. |
| High | Deployment memory contract | Commercial deployment validation requires conversation count/byte limits in environment examples, all topology profiles, and Kubernetes ConfigMaps. | Human deployment review is required before changing production/staging configuration. |
| High | Release and capacity evidence | No staging-backed scale run proves target concurrency. Active PC artifacts lack complete signing/checksum/SBOM/provenance evidence and reviewed release media/version alignment. | Produce real artifacts and staging load/HA/DR evidence; placeholder or document-only evidence must continue to fail closed. |
| Medium | Search pagination debt | PostgreSQL search still accepts legacy numeric offset cursors and uses offset SQL. | Remove the pre-launch compatibility branch and retain opaque keyset cursors only; rerun pagination and search continuity tests. |

## Required Verification Before Sign-Off

1. Run API operation, response-envelope, pagination, database, security, deployment, documentation, SDK, and architecture gates from a frozen workspace.
2. Run live PostgreSQL isolation, transaction, outbox, idempotency, aggregate-concurrency, and projection-recovery tests against the migration-complete schema.
3. Run PC production-build Playwright authorization, large-list, offline/account-switch, message-action, and secret-redaction suites on desktop and mobile viewport classes.
4. Execute staging capacity, long-duration soak, rolling restart, node loss, Redis interruption, PostgreSQL failover, and projection catch-up scenarios with RSS, allocator, queue, pool, event-loop, latency, and error-budget evidence.
5. Run `pnpm check:commercial-readiness`; do not approve release while any gate, evidence index, signing requirement, or reviewed blocker above remains open.

## Decision Requests

- Approve the durable idempotency transaction and retention model.
- Approve organization-scoped projection port and data migration.
- Approve versioned aggregate membership/read-cursor persistence and migration.
- Approve admin permission codes, route-removal policy, and client secret redaction.
- Approve desktop global retention/physical file policy.
- Approve production topology memory-limit values before Kubernetes and production profile changes.
