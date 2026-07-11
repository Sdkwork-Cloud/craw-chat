# Group Multi-Agent Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Allow a group conversation to expose and manage one or more synthetic agent targets without creating IAM users or ordinary conversation memberships.

**Architecture:** Conversation state owns an ordered list of `ConversationAgentAssignment` values resolved from the confirmed default policy or an explicit group override. The assignments are synthetic virtual participants: they are returned with group information and accepted as structured mention targets, but they never enter normal member, read-cursor, inbox, notification, or presence projections. Pre-approval assignment durability uses the existing generic journal without schema changes; durable trigger receipts, database changes, public naming, generated SDKs, IAM, and Agents runtime integration remain behind explicit review gates.

**Tech Stack:** Rust domain/runtime services, authored OpenAPI, IM event journal/outbox, Rust integration tests.

---

### Task 0: Resolve dictionaries, authority, and approval boundaries

- [ ] Read and record evidence from `AGENTS.md`, `sdkwork.app.config.json`, `specs/README.md`, `specs/component.spec.json`, `specs/im-app-api-sdk-integration.spec.md`, `../sdkwork-specs/SOUL.md`, and the relevant `../sdkwork-specs/{CODE_STYLE,NAMING,RUST_CODE,RUST_RPC,API,WEB_FRAMEWORK,WEB_BACKEND,SDK,SDK_WORKSPACE_GENERATION,RPC,TEST,DATABASE,DATABASE_FRAMEWORK,SCHEMA_REGISTRY,MIGRATION,EVENT,IAM,SECURITY,PRIVACY}_SPEC.md` files as each phase becomes relevant.
- [x] Confirm the product decision from the preceding design discussion: every new group resolves at least one default agent target, while explicit group configuration may atomically replace it with `1..N` targets.
- [x] Keep agent targets separate from IAM users, service accounts, credentials, `ConversationMember`, member roles, read cursors, inbox identities, presence, and notification recipients.
- [ ] Obtain explicit review before public OpenAPI naming, database migrations, generated SDK ownership, IAM/auth behavior, or sibling repository edits.

### Task 1: Define the domain contract and validation

**Files:**
- Modify: `crates/im-domain-core/src/conversation.rs`
- Modify: `crates/im-domain-core/src/message.rs`
- Test: `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
- Test: `crates/im-domain-core/tests/model_contract_test.rs`

- [ ] Write failing tests for ordered `1..N` agent assignments, duplicate rejection, group-only assignment, and a structured agent mention with `kind=mention`, `targetKind=agent`, `targetId`, non-authoritative display text, and `assignmentGeneration`.
- [ ] Run the narrow Rust tests and verify that they fail because the types and validation do not exist.
- [ ] Add minimal domain types and validation with stable agent IDs, assignment generation, enabled state, and a first-class mention content part.
- [ ] Re-run the narrow Rust tests and verify they pass.

### Task 2: Add conversation runtime commands and synthetic projection behavior

**Files:**
- Modify: `services/sdkwork-comms-conversation-service/src/runtime.rs`
- Modify: `services/sdkwork-comms-conversation-service/src/runtime/creation.rs`
- Modify: `services/sdkwork-comms-conversation-service/src/runtime/recovery.rs`
- Test: `services/sdkwork-comms-conversation-service/tests/conversation_flow_test.rs`

- [ ] Write failing runtime tests for a group receiving its confirmed default synthetic target and an owner/admin atomically replacing the ordered set with several agents.
- [ ] Add negative tests proving assignments create no member, member count, role, read cursor, actor inbox, unread recipient, notification recipient, or presence subscription; human members still receive the group and its agent metadata.
- [ ] Run the focused tests and verify failure.
- [ ] Implement commands that only allow group owners/admins to set `1..N` assignments and keep synthetic agents outside membership APIs. New group creation resolves the mandatory default before commit and embeds the assignment plus policy version atomically in `conversation.created.v2`; later replacement emits a versioned assignment-set event through the existing generic journal with optimistic generation.
- [ ] Replay both events in `runtime/recovery.rs` without a schema migration. Define one fixed, versioned v1 compatibility default; never consult mutable current policy while replaying old `conversation.created.v1` streams.
- [ ] Add compatibility tests proving a v1 stream replays to the same fixed historical default even after current default policy changes, and assignment updates survive restart with the same generation.
- [ ] Re-run focused tests and verify passing behavior.

### Task 3: Review and author the public API contract

**Files:**
- Modify: `apis/open-api/im/sdkwork-im-im.openapi.yaml`
- Modify: `services/sdkwork-comms-conversation-service/src/runtime/http.rs`
- Test: `services/sdkwork-comms-conversation-service/tests/http_smoke_test.rs`
- Test: repository API contract validation.

- [ ] Obtain human review for the public resource and field names before editing OpenAPI.
- [ ] Prefer atomic resource replacement semantics (`PUT`/`PATCH`) over a vague command, with optimistic assignment generation.
- [ ] Write or extend contract tests for the approved group-agent resource, group detail field, and structured `mention` content part.
- [ ] Run the focused contract test and verify it fails.
- [ ] Add authored OpenAPI schemas and operations using SDKWork response envelopes and no generated-file edits.
- [ ] Run the focused contract test and verify it passes.

### Task 4: Validate mention targets without durable side effects

**Files:**
- Modify: `crates/im-domain-core/src/message.rs`
- Modify: `services/sdkwork-comms-conversation-service/src/runtime.rs`
- Modify: `services/sdkwork-comms-conversation-service/src/runtime/rpc_dispatch.rs`
- Modify: `services/sdkwork-comms-conversation-service/src/runtime/internal_rpc_dispatch.rs`
- Test: `crates/im-domain-core/tests/model_contract_test.rs`
- Test: `services/sdkwork-comms-conversation-service/tests/conversation_flow_test.rs`

- [ ] Write failing tests proving only an authenticated active ordinary member may target an enabled assigned agent.
- [ ] Repeated mentions of the same target resolve once and distinct enabled targets resolve once each. The client-supplied `assignmentGeneration` is compared with current state; an unknown, disabled, stale-generation, unassigned, cross-conversation, or cross-tenant target is rejected atomically. The server stamps authoritative generation into the future trigger event. Rendered text, edits, forwards, and replay remain accepted but non-triggering.
- [ ] Implement pure validation and canonical target resolution only. Do not add a durable receipt, outbox record, migration, or external invocation before approval.
- [ ] Preserve compatibility with existing messages and the existing `urn:sdkwork:sdkwork-im:message:agent` data schema. Internal RPC changes are serialization/classification only; they must not relax ordinary membership authorization or create a synthetic member.
- [ ] Treat agent-authored output as a non-triggering validator/replay property only. A trusted service-authored agent output ingress with service authentication and provenance belongs in Task 6.
- [ ] Record `git diff --name-only` and prove there are no sibling-repository, database migration, IAM/auth, or generated-output edits before requesting Task 5 approval.

### Task 5: Human approval gate

- [ ] Present the authored public naming proposal and the durable invocation persistence design for review.
- [ ] Decide whether the existing atomic message writer persists one aggregate invocation event containing unique targets or is extended to persist multiple outbox records; invocation durability must not depend on realtime publisher configuration.
- [ ] Obtain human approval before editing migration files, store contracts, PostgreSQL adapters, generated SDK ownership, IAM security/auth contracts, or sibling repositories.
- [ ] Before Task 6 starts, update this plan with the selected exact migration, table registry, store port, PostgreSQL adapter, durable writer, relay/consumer, authored contract, generated family, and integration-test file paths plus their exact verification commands.

### Task 6: Approved persistence and integration work

**Files:**
- Modify after approval: `database/**`, the applicable store ports/adapters, durable message writer, relay/consumer modules, and authored SDK authorities.
- Generate after approval: owned SDK outputs from authored contracts.

- [ ] Update database migration/table registries only after approval, with tenant-leading indexes, retention, replay, rollback, and live integration tests.
- [ ] Extend the atomic message transaction and outbox relay according to the approved one-event or multi-event design.
- [ ] Add recovery/replay and idempotency tests keyed by tenant, conversation, message, unique target, and assignment generation.
- [ ] Regenerate SDKs from authored authorities; never hand-edit generated output.

### Task 7: Final verification

- [ ] Run `cargo test -p im-domain-core --test conversation_domain_builder_test --test model_contract_test`.
- [ ] Run `cargo test -p sdkwork-comms-conversation-service --test conversation_flow_test --test http_smoke_test` and the focused projection tests.
- [ ] Run `node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .`.
- [ ] Run `node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .`.
- [ ] Run `pnpm test:rpc-contract` unconditionally.
- [ ] After approved persistence work, run `pnpm test:database-naming-standard`, the owned generated IM SDK drift check, and the selected PostgreSQL outbox integration test named in the Task 5 exit update.
- [ ] Verify `git diff --name-only` contains no unapproved sibling, migration, IAM, or generated-output edits.
- [ ] Report implemented behavior, deferred approval-boundary work, and verification evidence.
