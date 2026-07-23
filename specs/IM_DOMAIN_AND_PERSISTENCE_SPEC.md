# SDKWork IM Domain And Persistence Contract

- Version: `2.2.0`
- Status: active
- Owner: `im-platform`
- Requirement: `REQ-2026-0722`
- Decision: `ADR-20260722-normalized-im-authority`

## Domain Vocabulary

IM uses `Conversation -> Message -> Member -> ReadCursor`.

Agents uses `Project -> Session -> Turn -> SessionItem -> Interaction`.

Kernel uses `Run -> Step -> ToolExecution`.

UI copy may use the word "chat", but database, API, SDK, DTO, event, and service
contracts must use the owning domain vocabulary. An IM Message is never an Agents
SessionItem, and an IM Conversation is never an Agents Session.

## Ownership

IM owns Conversation identity and lifecycle, membership, visible Message history,
message sequence, read state, reactions, pins, threads, presence, routing, realtime
delivery, and IM-side Agent assignment/dispatch correlation.

IM does not own Agent identity, Session, Turn, SessionItem, Interaction, inference,
tool execution, provider configuration, prompt, model, or Agent transcript data.

## Canonical Persistence

The normalized core authorities are:

- `im_conversations`
- `im_conversation_members`
- `im_conversation_messages`
- `im_conversation_read_cursors`
- `im_conversation_seq_counters`
- typed Message interaction and Conversation preference tables
- `im_outbox_events` and `im_inbox_events` for integration delivery
- `im_commit_journal` for immutable audit/integration history only

Rules:

- No active table may use the `im_projection_` prefix.
- No second Message timeline may be persisted.
- No generic JSON snapshot may own queryable status, membership, preferences,
  visibility, interaction, routing, or idempotency state.
- Cache entries are disposable and never participate in correctness or recovery.
- Current state is read from normalized tables, not rebuilt from the journal.
- Every mutation writes normalized state first-class and atomically records its
  journal/audit event and transactional outbox event when publication is required.
- Cross-domain references are stable IDs with no database foreign key.

## Query And Performance Contract

- All tenant data is scoped by `tenant_id` and `organization_id` in predicates,
  primary/unique constraints, and hot-path indexes.
- Lists use keyset cursors and fetch at most 201 rows for a maximum 200-item page.
- Message history orders by `message_seq`; no timestamp ordering substitutes for
  the per-Conversation sequence.
- Membership and read-state checks are bounded index lookups.
- Contact lists use bounded SQL joins over canonical social tables and do not load
  all relationships into process memory.

## Reliability Contract

- Retried commands use a stable idempotency key and request hash.
- State, journal/audit, and outbox changes share one PostgreSQL transaction.
- Relay workers claim outbox rows with bounded batches and `FOR UPDATE SKIP LOCKED`.
- Failed delivery does not roll back already committed business state; the outbox
  retains retry/dead-letter evidence.
- Startup does not require full journal replay.

## Forbidden Dependencies

- Agents to IM, direct or transitive application dependency.
- Direct access to Agents repositories, SQL, private crates, or generated transport
  internals.
- IM writes to `ai_agent_*` tables.
- Physical foreign keys from IM to another domain database.
- Raw HTTP or manual credentials where an approved SDK/facade exists.

## Verification

The repository must statically reject new `im_projection_*` DDL or repository SQL,
the reverse Agents dependency, duplicate timeline persistence, and unbounded reads.
