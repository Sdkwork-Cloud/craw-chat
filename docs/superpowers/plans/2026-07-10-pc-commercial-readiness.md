# PC Commercial Readiness Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the SDKWork IM PC application and its required shared backend safe, bounded, standards-compliant, and verifiably ready for a production release candidate.

**Architecture:** Preserve the existing service, repository, adapter, SDK, and PC package boundaries. Correct data races and pagination at their authoritative stores, keep clients cursor-opaque, make desktop state principal-scoped and lease-based, and make gateways drain before termination. Treat PostgreSQL as the production source of truth and desktop SQLite as a bounded offline cache, not as a server database substitute.

**Tech Stack:** Rust, Tokio, SQLx/PostgreSQL, Tauri/SQLite, TypeScript/React, generated SDKWork TypeScript SDKs, Kubernetes, pnpm, Cargo.

**Execution mode:** Inline execution in the current workspace because repository instructions do not authorize sub-agent delegation and the worktree already contains user changes. Do not commit, reset, clean, or overwrite unrelated changes.

---

### Task 1: Make Outbox Relay Claims Domain-Scoped And Race-Safe

**Files:**
- Modify: `crates/im-platform-contracts/src/outbox_store.rs`
- Modify: `adapters/postgres-journal/src/outbox_store.rs`
- Modify: `crates/sdkwork-im-gateway-assembly/src/conversation_outbox_relay.rs`
- Modify: `crates/sdkwork-im-gateway-assembly/src/rtc_outbox_relay.rs`
- Modify: `crates/sdkwork-im-gateway-assembly/src/social_outbox_relay.rs`
- Modify: `crates/sdkwork-im-gateway-assembly/src/outbox_relay_common.rs`
- Test: adapter and relay unit/integration tests beside the implementations

- [x] Add a failing contract test proving each worker claims only its aggregate type.
- [x] Add a failing concurrent-claim test proving the same event cannot be claimed twice.
- [x] Add a failing conditional-transition test proving a stale worker cannot move a published event back to pending/failed.
- [x] Introduce a typed `OutboxClaim`/filter boundary and transactional claim operation with a bounded lease.
- [x] Make publish/fail transitions compare the claim token and current state.
- [x] Remove the normal-path behavior that marks another domain's event as failed.
- [x] Run `cargo test -p im-adapters-postgres-journal` and `cargo test -p sdkwork-im-gateway-assembly`.

### Task 2: Replace Sequence Arithmetic With Opaque Backward Message Cursors

**Files:**
- Modify: `apis/open-api/im/sdkwork-im-im.openapi.yaml`
- Modify: message service/repository contracts and `adapters/postgres-journal/src/message_store.rs`
- Regenerate: owning `sdks/sdkwork-im-sdk` TypeScript generated transport through `sdkgen`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts`
- Test: message repository, conversation flow, SDK generation, and PC pagination contract tests

- [x] Add a failing repository test for latest-page then older-page continuity under concurrent new inserts.
- [x] Add a failing PC test proving cursors are passed through unchanged and never parsed.
- [x] Define `cursor` + `page_size` with opaque, versioned, URL-safe cursor semantics and stable descending keyset order.
- [x] Return each page in UI chronological order while continuing backward from the oldest loaded message.
- [x] Remove `afterSeq`, numeric cursor compatibility, and client sequence arithmetic because the application is pre-launch.
- [x] Regenerate owner SDKs and verify generation is idempotent.
- [x] Run API operation, envelope, route collision, pagination, SDK, repository, and PC type checks.

### Task 3: Isolate And Bound Desktop Offline Storage

**Files:**
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/src/offline_store.rs`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineChatCache.ts`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineSendQueue.ts`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts`
- Test: Tauri offline store and PC SDK boundary tests

- [x] Add failing cross-principal isolation tests for conversations, messages, cursors, and pending sends.
- [x] Add a failing crash-recovery test for expired send claims.
- [x] Add a failing test for more than one batch of pending sends.
- [x] Version the pre-launch SQLite schema with tenant, organization, principal kind, and principal id scope.
- [x] Store claim time and lease expiry; reclaim expired claims and acknowledge/release only matching claim ids.
- [x] Flush in bounded batches until empty with cancellation/backoff and no UI-thread blocking.
- [x] Add TTL, row-count, byte-budget, and logout/account-switch purge policy while preserving unsent rows.
- [x] Run Cargo, PC TypeScript, offline cache, and secure-session tests.

### Task 4: Align Session Fencing And Graceful Drain

**Files:**
- Modify: `services/session-gateway/src/cluster/disconnect.rs`
- Modify: `services/session-gateway/src/presence.rs` only if shared semantics require extraction
- Modify: `services/session-gateway-bin/src/main.rs`
- Modify: `deployments/kubernetes/cloud/session-gateway/deployment.yaml`
- Modify: cloud topology/HPA/PDB configuration as required
- Test: `services/session-gateway/tests/cluster_routing_test.rs` and shutdown tests

- [x] Preserve the existing failing same-session fence integration test as RED.
- [x] Add tests for different session, principal kind, stale fence, and idempotent reconnect.
- [x] Centralize the fence decision so disconnect and presence paths use one policy.
- [x] Add cross-platform SIGINT/SIGTERM shutdown and a bounded drain lifecycle.
- [x] Stop readiness/new upgrades, mark the node draining, stop consumers/listeners in dependency order, and wait with a deadline before aborting.
- [x] Replace sleep-only preStop with readiness/drain-aware termination and adequate grace period.
- [x] Run the full session-gateway test suite and gateway HA/security deployment checks.

### Task 5: Bound Server And Projection Memory

**Files:**
- Modify: `services/sdkwork-comms-conversation-service/src/runtime.rs`
- Modify: `crates/im-domain-core/src/message.rs`
- Modify: `adapters/postgres-journal/src/aggregate_store.rs`
- Modify: `adapters/postgres-projection/src/metadata_store.rs`
- Modify: `projection-service/src/bootstrap.rs`
- Modify: `projection-service/src/snapshot.rs`
- Test: runtime eviction, repository pagination, projection bootstrap, and soak/capacity tests

- [ ] Add failing tests for message locator and idempotency-cache bounds after eviction/high-volume writes.
- [ ] Add failing repository tests showing member/read-cursor loads are paged or streamed.
- [ ] Add a failing projection bootstrap test proving startup memory is independent of total historical scopes.
- [ ] Replace unbounded maps with measured TTL/LRU or persisted lookup paths and remove all companion state on eviction.
- [ ] Page/stream aggregate hydration with explicit high-cardinality behavior.
- [ ] Restore projections lazily or in bounded checkpointed batches; never collect every scope before processing.
- [ ] Add metrics for entries, bytes, evictions, restore backlog, RSS, allocator pressure, and slow scans.
- [ ] Run unit, integration, pagination, performance, and bounded soak tests.

### Task 6: Bound PC Conversation Rendering And Cache State

**Files:**
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatList.tsx`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts`
- Test: PC list virtualization, cache eviction, and Playwright large-list scenarios

- [ ] Add failing tests for a 10,000-conversation inbox and cache companion-state eviction.
- [ ] Virtualize the conversation list using the repository's established virtualizer dependency.
- [ ] Keep stable row dimensions and accessible keyboard/focus behavior.
- [ ] Enforce one centralized cache budget across every message load/write path and remove related pagination/view state on eviction.
- [ ] Run PC typecheck, virtualization tests, Playwright desktop screenshots, and memory sampling.

### Task 7: Close API, Admin, And Message-Action Gaps

**Files:**
- Modify: room-create OpenAPI/service/repository implementation and regenerate owner SDK
- Modify: `apps/sdkwork-im-pc/src/bootstrap/routes.tsx`
- Modify: affected `sdkwork-im-pc-admin-*` and `sdkwork-im-pc-console-*` packages
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-console-settings/src/ConsoleSettings.tsx`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts`
- Test: room boundary, RBAC routes, secret redaction, media forward/retry, and UI states

- [ ] Add RED tests proving room ids are server-generated and duplicate requests are idempotent.
- [ ] Add RED tests proving non-admin users cannot mount admin routes and permission-denied pages do not render privileged labels/data.
- [ ] Remove server-secret retrieval/display contracts from all client surfaces.
- [ ] Implement reachable admin/console services through generated SDK methods or remove the route/menu from delivered capabilities until the contract exists.
- [ ] Complete supported media forward/retry semantics and explicitly disable unsupported actions before submission.
- [ ] Run API/SDK, security, PC architecture, and Playwright authorization tests.

### Task 8: Production Deployment, Supply Chain, And Capacity Evidence

**Files:**
- Modify: `deployments/kubernetes/cloud/**`
- Modify: `sdkwork.workflow.json`, package/release scripts, manifests, and lockfile only through canonical tools
- Modify: root and PC `sdkwork.app.config.json` plus package versions
- Modify: capacity profiles, dashboards, alerts, runbooks, and release evidence templates

- [ ] Add RED static tests for connection-aware HPA, multi-zone spread, PDB, termination drain, and resource limits.
- [ ] Set a capacity model whose configured replica/connections ceiling exceeds the PRD target with headroom.
- [ ] Add active connection, upgrade rejection, event-loop lag, queue depth, DB pool, Redis, RSS, and latency scaling signals.
- [ ] Restore lockfile consistency through pnpm; do not hand-edit lockfile entries.
- [ ] Generate real checksums, signatures, SBOM, provenance, and attestations from build artifacts; placeholders must fail release assessment.
- [ ] Replace generated placeholder media with reviewed PC release assets before publication evidence can pass.
- [ ] Run commercial readiness and production deployment gates; record external capacity/signing work as release blockers until real evidence exists.

### Task 9: Canonical Documentation And Final Verification

**Files:**
- Modify: `docs/product/prd/PRD.md` and active PRD shards
- Modify: `docs/architecture/tech/TECH_ARCHITECTURE.md` and relevant ADRs
- Modify: database, deployment, security, operations, capacity, migration, and release runbooks
- Remove/update: stale statements contradicted by the implemented system

- [ ] Update requirements and ADRs before closing each architecture/security/migration decision.
- [ ] Document PostgreSQL as production truth and desktop SQLite as a bounded principal-scoped offline cache.
- [ ] Align one version authority and remove stale production-readiness claims until their gates pass.
- [ ] Document cursor semantics without exposing cursor internals, shutdown ordering, recovery, RPO/RTO, and capacity assumptions.
- [ ] Run narrow tests after every task, then `cargo fmt --check`, relevant Clippy/tests, PC typecheck/build/Playwright, API/envelope/pagination/SDK/composition/security/deployment/database/documentation checks, and commercial readiness.
- [ ] Re-run the original audit checklist and leave no known unresolved item marked complete without executable evidence.
