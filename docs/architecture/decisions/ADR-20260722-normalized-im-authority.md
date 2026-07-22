# ADR-20260722: Use Normalized IM State As The Only Business Authority

- Status: accepted
- Requirement: `REQ-2026-0722`
- Owner: `im-platform`
- Date: 2026-07-22
- Specs: `ARCHITECTURE_DECISION_SPEC.md`, `DOMAIN_SPEC.md`, `DATABASE_SPEC.md`,
  `EVENT_SPEC.md`, `MIGRATION_SPEC.md`, `DEPENDENCY_MANAGEMENT_SPEC.md`

## Context

The pre-launch IM implementation combines an append-only commit journal, an
asynchronous projector, normalized social tables, duplicated timeline/contact
tables, and generic metadata snapshots. Several resources can therefore be read
from two independently updated representations. Operational recovery requires
replay, while ordinary reads depend on freshness and projector availability.

IM also integrates Agents. Communication facts must remain distinct from Agent
execution facts: an IM Conversation and Message are not an Agent Session, Turn, or
Item. The established dependency direction is `sdkwork-im -> sdkwork-agents ->
sdkwork-kernel`.

The canonical app manifest is `DRAFT`; listed release packages are disabled and
deferred. No launched compatibility contract requires preserving the projection
architecture.

## Decision

1. Normalized IM state is the only business system of record.
2. The core aggregate is `Conversation -> Member / Message / ReadCursor`.
3. `im_commit_journal` is retained only as an immutable audit and integration-event
   record. It is not used to reconstruct required business state during normal
   startup or reads.
4. A state mutation, its journal/audit record, and its `im_outbox_events` record are
   committed atomically. Downstream delivery remains asynchronous and idempotent.
5. The duplicate timeline, contact, and direct-chat binding tables are removed:
   - timeline reads query `im_conversation_messages`;
   - contacts use bounded joins over canonical social tables;
   - direct-chat binding uses `im_direct_chats.conversation_id`.
6. Conversation summaries, members, and read cursors become canonical tables named
   `im_conversations`, `im_conversation_members`, and
   `im_conversation_read_cursors`.
7. Client synchronization records are operational delivery logs named
   `im_client_sync_events` and `im_client_sync_cursors`; they are not business
   read models and cannot override Conversation or Message state.
8. IM-owned Agent integration records are named for assignments, bindings, and
   dispatch. They contain opaque Agents IDs only. No IM table references an
   `ai_agent_*` table and no Agents component depends on IM.
9. Generic serialized business snapshots are retired. Typed normalized tables own
   preferences, favorites, visibility, interaction summaries, routing, and other
   queryable state.
10. Public communication APIs continue to use Conversation and Message terms.
    Agent execution APIs continue to use Session, Turn, and Item terms.

## Alternatives

### Keep CQRS projections

Rejected. It preserves duplicate authority, freshness semantics, replay coupling,
and an unnecessary projector failure mode for an application that has not shipped.

### Rename projection tables without changing authority

Rejected. Naming alone would hide the same dual-authority architecture.

### Put IM messages in Agents

Rejected. Human/channel communication and Agent execution have different
membership, delivery, retention, privacy, and lifecycle invariants.

### Put Agent sessions in IM

Rejected. It reverses the established dependency direction and duplicates the
Agents system of record.

## Consequences

- Command persistence becomes more explicit and transactionally reliable.
- Ordinary reads no longer wait for journal replay or projection freshness.
- The projector service, projection route crate, projection adapter, generic
  snapshots, replay checkpoints, and related deployment/configuration surfaces can
  be deleted after their consumers use normalized repositories.
- Database migration must validate row counts, stable IDs, tenant isolation, and
  checksums before old tables are dropped.
- Historical point-in-time events remain available for audit without controlling
  current business state.

## Verification

- Database contract scan rejects `im_projection_*`.
- Cargo/pnpm dependency scans enforce `IM -> Agents -> Kernel` and forbid the
  reverse edge.
- PostgreSQL integration tests prove state + journal + outbox atomicity and exact
  idempotent replay.
- Message-history tests prove reads come from `im_conversation_messages`.
- Migration tests compare scoped counts and reject ambiguous duplicate bindings.
- Canon documentation link and terminology checks pass.

## Supersedes / Superseded By

This decision supersedes the projection-authority portions of pre-launch technical
step documents and the projection sections of
`specs/IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md`. Historical ADRs remain as
decision history but do not override this accepted baseline.
