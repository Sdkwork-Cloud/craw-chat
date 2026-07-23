# SDKWork IM To Agents Dependency And Database Contract

- Version: `2.2.0`
- Status: active
- Owner: `im-platform`
- Requirement: `REQ-2026-0722`
- Decision: `ADR-20260722-normalized-im-authority`
- Related: `../../sdkwork-agents/specs/AGENTS_IM_DEPENDENCY_BOUNDARY_SPEC.md`,
  `IM_DOMAIN_AND_PERSISTENCE_SPEC.md`, `../database/contract/schema.yaml`

## Dependency Direction

The only valid direction is:

```text
sdkwork-im -> sdkwork-agents -> sdkwork-kernel
```

IM is a consumer of public Agents contracts. Agents and Kernel must not import,
call, query, or link IM.

Approved integration surfaces are generated Agents SDK families, the public
`sdkwork-agents-runtime-facade`, and an approved public assembly. IM must not use
Agents generated transport internals, repositories, SQL, private crates, provider
implementations, or copied OpenAPI contracts. IM migrations must never create or
write an `ai_agent_*` table.

## Semantic Boundary

IM owns communication:

- Conversation identity, type, lifecycle, membership, authorization, invitation,
  and channel/thread semantics;
- visible Message history and strict per-Conversation `message_seq`;
- read cursors, reactions, pins, presence, routing, and realtime delivery;
- IM-side Agent assignment intent, dispatch lifecycle, visible reply Message, and
  stable correlation identifiers.

Agents owns execution:

- Agent Project, Session, Turn, SessionItem, Interaction, and Task;
- Agent identity, revision, composition, model/provider binding, tool execution,
  inference state, checkpoint, usage, and execution audit;
- execution transcript and provider failure details.

An IM Message and an Agents SessionItem are two different facts. When an IM user invokes an
Agent, IM records the user-visible Message, Agents records the Turn and SessionItems, and IM
records a new visible reply Message only after publication. Neither record is a
projection of the other and their deletion/retention lifecycles are independent.

## IM-Owned Agents Integration Tables

### `im_conversation_agent_assignments`

Canonical IM relation between a Conversation and an assigned Agent.

Required invariants:

- unique `(tenant_id, organization_id, conversation_id, agent_id)`;
- unique active position within a Conversation;
- monotonic `assignment_generation`;
- stable opaque `agent_id` and optional `agent_revision_ref` only;
- compare-and-set update guarded by source aggregate version;
- no copied Agent title, prompt, model, provider, manifest, or lifecycle state.

### `im_conversation_agent_binding`

Canonical IM correlation between one assignment generation and an Agents Session.
It stores `agents_session_id` as an opaque reference and has no foreign key to
Agents persistence.

Transitions:

```text
pending -> active | failed
active -> closed | superseded | failed
failed -> pending | superseded
closed | superseded -> terminal
```

Every transition uses optimistic `version`, a stable idempotency key, and a payload
hash. Assignment removal closes or supersedes the IM binding; it does not delete an
Agents Session.

### `im_agent_dispatch`

Canonical IM delivery workflow from one source IM Message to one Agent target and,
when successful, one visible reply IM Message.

It may store opaque `agents_session_id` and `agents_turn_id` references. It must not
store Agent SessionItems, reasoning, tool calls, model metadata, provider credentials, or
the Agents transcript.

Transitions:

```text
pending -> leased -> dispatched -> running -> succeeded
pending | leased | dispatched | running -> retry_wait | failed | cancelled
retry_wait -> leased | failed | cancelled
```

Worker claims use bounded batches, lease expiry, retry scheduling, and compare-and-
set updates. The reply Message is written through the canonical IM Message writer.

## Transaction Boundary

The IM mutation path is:

```text
authorize IM command
  -> mutate normalized IM tables
  -> append immutable IM audit/journal event
  -> enqueue integration/realtime outbox event
  -> commit one PostgreSQL transaction
  -> relay outbox
  -> invoke Agents through public SDK/facade
  -> persist stable Session/Turn correlation in IM
  -> publish a separate visible IM reply Message
```

The commit journal is evidence, not current-state authority. Startup and ordinary
queries must not require projector replay.

## Security And Subject Rules

- Tenant and organization come from verified request context, not payload fields.
- End-user actor IDs are validated positive signed-64-bit decimal values before
  binding to BIGINT columns; `0` is reserved for an explicitly trusted system actor
  only where the schema permits it.
- Agents resource IDs are bounded opaque strings and are never authorization
  evidence.
- All reverse lookups include tenant and organization scope.
- Errors and logs redact prompts, credentials, full Agent output, and private Message
  bodies.

## Release State

The canonical app manifest declares `publish.status = DRAFT`. Listed
`STABLE/0.1.0` artifact URLs are disabled and carry `releaseBuildDeferred = true`;
they describe a future release lane, not an installed immutable release. The
pre-launch contract is therefore corrected directly, with versioned and reversible
database migrations retained for local environment safety.

## Verification Gates

- dependency scan proves no Agents-to-IM Cargo, pnpm, HTTP, SDK, SQL, or OpenAPI edge;
- database scan proves no IM write to `ai_agent_*` and no active
  `im_projection_*` table;
- migration tests validate forward/down behavior, tenant isolation, unique
  assignments, dispatch claims, and stable identifier bounds;
- integration tests prove normalized state + journal + outbox atomicity;
- API/SDK contracts keep Conversation/Message separate from Session/Turn/SessionItem;
- canonical PRD and technical architecture link this contract and contain no stale
  projection-authority or already-released claim.
