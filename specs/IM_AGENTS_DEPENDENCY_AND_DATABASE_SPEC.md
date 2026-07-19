# SDKWork IM To Agents Dependency And Database Specification

- Version: `1.0.0`
- Status: active architecture constraint; database contract `2.0.0` active
- Owner: `im-platform`
- Dependency: `sdkwork-agents`
- Related: `../../sdkwork-agents/specs/AGENTS_IM_DEPENDENCY_BOUNDARY_SPEC.md`,
  `../database/contract/schema.yaml`, `../specs/database-table-registry.json`

## 1. Mandatory Dependency Direction

The only allowed dependency direction is:

```text
sdkwork-im -> sdkwork-agents -> sdkwork-kernel
```

IM is the consumer and Agents is the provider. IM MAY consume only public Agents
facades and SDKs:

- `@sdkwork/agents-app-sdk` for authenticated application workflows;
- `@sdkwork/agents-backend-sdk` for trusted server/admin orchestration;
- `@sdkwork/agents-sdk` for approved open-api integrations;
- `sdkwork-agents-runtime-facade` for approved in-process Rust composition.
- `sdkwork-agents-gateway-assembly` for embedded application host composition.

IM MUST NOT import Agents generated transport internals, provider crates,
repositories, SQL, private modules, or kernel provider implementations. Agents
MUST NOT import IM. IM code and migrations MUST NOT write any `ai_agent_*` table.

## 2. Bounded Context Ownership

IM owns:

- conversations, groups, channels, membership, authorization, invitations,
  presence, read state, reactions, pins, threads, and realtime delivery;
- the IM-visible message timeline and its `message_seq` ordering;
- conversation agent assignments and assignment generation;
- correlation between IM messages and Agents sessions/turns;
- retry, dispatch, visible agent reply, and compensation state on the IM side.

Agents owns:

- agent identity, lifecycle, revision/provider/composition resolution;
- hosted agent execution sessions, execution messages, turns, usage, and audit;
- inference status, cancellation, provider failures, and model metadata.

IM stores stable Agents identifiers only. It never copies Agents manifests,
prompts, model credentials, provider configuration, full execution metadata, or
Agents transcript tables into IM.

## 3. Existing IM Authorities

The active `1.0.0` database contract already provides:

- `im_commit_journal` as the conversation event system of record;
- `conversation.created` and `conversation.agents_replaced` events for ordered
  agent assignment state;
- `im_outbox_events` for durable
  `conversation.agent_mention_dispatch_requested` publication;
- `im_conversation_messages` and `im_conversation_seq_counters` for the visible
  conversation transcript;
- `im_projection_conversation_summaries` for conversation summary projection.

These authorities remain. New tables MUST NOT create a second assignment event
log or a second IM message timeline.

## 4. Current Database Gaps

1. Agent assignments are recoverable from the journal but are not materialized
   as a first-class, index-backed read model. Querying assignment state through
   projection payload JSON cannot provide bounded agent/conversation lookups.
2. The dispatch outbox proves publication intent but does not own the durable
   lifecycle from source IM message through Agents session/turn to visible IM
   reply.
3. There is no explicit, idempotent conversation-agent to Agents-session binding.
4. Existing message/event contracts serialize some tenant, organization, and
   principal identifiers as decimal strings. The Agents integration adapter must
   validate and normalize those values before writing its BIGINT-owned tables.
   Any future in-place conversion of those existing columns remains a separate
   expand/contract migration.
5. The application is published as `STABLE 0.1.0`; the baseline is immutable for
   existing installations. New structures require paired, versioned migrations.

## 5. Target `2.0.0` Database Contract

The following tables are active IM authority. Paired migration `0005` creates
them with immutable provenance; contract `2.0.0` activates them after repository,
projector/worker, SDK, subject-range, PostgreSQL, and release evidence passed.
Activation does not rewrite the historical baseline because this module uses
`baseline-plus-migrations`.

### 5.0 Common Contract

New rows use application-allocated Snowflake `BIGINT id`, public `uuid`, target
`BIGINT tenant_id`/`organization_id`, and `TIMESTAMPTZ` instants. IM resource ids
such as `conversation_id`, `binding_id`, and `dispatch_id` are bounded stable
strings. Existing `message_id` and `message_seq` remain `BIGINT`. Stable Agents
ids (`agent_id`, `agent_revision_ref`, `agents_session_id`, `agents_turn_id`) are
opaque bounded strings serialized exactly as returned by the public Agents API.

Physical foreign keys are permitted only between IM-owned tables. Every such key
includes tenant/organization and the owning IM aggregate scope. There is no
foreign key to an `ai_agent_*` table, even when both modules share one PostgreSQL
cluster. Lifecycle history uses retention and explicit status; dispatch or
binding facts are not cascade-deleted when a conversation assignment changes.

### 5.1 `im_projection_conversation_agent`

Profile: `read_model`. Source lineage:
`im_commit_journal` conversation events. Rebuild strategy: truncate the scoped
projection and replay `conversation.created` plus `conversation.agents_replaced`
in aggregate-version order.

| Column | Logical type | Rule |
| --- | --- | --- |
| `id`, `uuid` | int64, public id | Standard SDKWork identifiers. |
| `tenant_id`, `organization_id` | int64 | Required subject scope. |
| `conversation_id` | string | IM conversation identity. |
| `agent_id` | string | Stable Agents resource id; reference only. |
| `agent_revision_ref` | string nullable | Optional pinned Agents revision. |
| `assignment_source` | enum | `default_policy` or `conversation_override`. |
| `assignment_generation` | int64 | Monotonic conversation assignment generation. |
| `position` | int32 | Stable display/dispatch order. |
| `enabled`, `status` | boolean, enum | Active assignment state; status is `active`, `disabled`, or `removed`. |
| `assigned_by`, `assigned_at` | int64, instant | Trusted mutation actor and event time; `0` is reserved for system/non-user actors whose principal ids are not numeric IAM user ids. |
| `source_event_id`, `source_aggregate_version` | string, int64 | Replay and stale-write guard; aggregate versions are zero-based and remain within signed-int64 range. |
| `payload_hash` | string | Canonical projected assignment hash. |
| `created_at`, `updated_at`, `retention_until` | instant | Projection lifecycle. |

Constraints and indexes:

- unique `(tenant_id, organization_id, conversation_id, agent_id)`;
- unique active `(tenant_id, organization_id, conversation_id, position)`;
- index conversation list `(tenant_id, organization_id, conversation_id,
  status, position, id)`;
- reverse lookup `(tenant_id, organization_id, agent_id, status,
  updated_at DESC, id DESC)`;
- projection writes reject an older source aggregate version.

### 5.2 `im_conversation_agent_binding`

Profile: `relation_entity`. System of record for the IM-owned correlation from a
conversation assignment generation to an Agents execution session.

Required columns:

```text
id, uuid, binding_id
tenant_id, organization_id
conversation_id
agent_id, agent_revision_ref
assignment_generation
agents_session_id
status                         # pending, active, failed, closed, superseded
idempotency_key, payload_hash
created_by, updated_by
last_used_at, closed_at
last_error_code, last_error_detail
version
created_at, updated_at, retention_until
```

Rules:

- unique `(tenant_id, organization_id, binding_id)`;
- unique `(tenant_id, organization_id, conversation_id, agent_id,
  assignment_generation)`;
- at most one active binding for `(tenant_id, organization_id,
  conversation_id, agent_id)`;
- `agents_session_id` is opaque and has no database FK to Agents;
- an active binding requires non-null `agents_session_id`; pending/failed rows may
  retain it only when Agents creation was reconciled by idempotency key;
- assignment removal closes/supersedes the binding but does not delete dispatch
  history or the Agents session directly;
- retries use the same idempotency key and payload hash.

Indexes:

- active resolution `(tenant_id, organization_id, conversation_id, agent_id,
  status, assignment_generation DESC, id DESC)`;
- Agents session reverse lookup `(tenant_id, organization_id,
  agents_session_id)` where non-null;
- stale lifecycle/retention `(tenant_id, organization_id, status,
  updated_at, retention_until, id)`.

Binding transitions are compare-and-set using `version`:

```text
pending -> active | failed
active  -> closed | superseded | failed
failed  -> pending | superseded
closed/superseded -> terminal
```

Only the worker that owns the matching assignment generation can activate a
binding. A late response for an older generation is stored for audit/reconciliation
but cannot replace the current active binding.

### 5.3 `im_agent_dispatch`

Profile: `operational_state` plus durable correlation. Write owner:
`comms-conversation-service`/agent dispatch worker.

Required columns:

```text
id, uuid, dispatch_id
tenant_id, organization_id
conversation_id
source_message_id BIGINT, source_message_seq BIGINT
agent_id, agent_revision_ref, assignment_generation
binding_id, agents_session_id, agents_turn_id
status                         # pending, leased, dispatched, running,
                               # completed, failed, cancelled, dead_letter
idempotency_key, payload_hash
attempt_count, max_attempts
lease_owner, lease_expires_at, next_attempt_at
last_error_code, last_error_detail
requested_by
reply_message_id BIGINT, reply_message_seq BIGINT
created_at, updated_at, started_at, completed_at, cancelled_at
retention_until
```

Constraints and indexes:

- unique `dispatch_id` inside tenant/organization scope;
- unique `(tenant_id, organization_id, conversation_id, source_message_id,
  agent_id, assignment_generation)`;
- source timeline FK uses `(tenant_id, organization_id, conversation_id,
  source_message_seq)` against the existing `im_conversation_messages` primary
  key; service validation also requires the stored `source_message_id` to match;
- reply timeline FK uses the same scoped key when a reply exists, and service
  validation requires `reply_message_id` to match and be different from the
  source message;
- binding FK is `(tenant_id, organization_id, binding_id)` and points only to
  `im_conversation_agent_binding`;
- worker index `(tenant_id, organization_id, status, next_attempt_at,
  lease_expires_at, id)` for `FOR UPDATE SKIP LOCKED`;
- turn lookup `(tenant_id, organization_id, agents_turn_id)` where non-null;
- reply lookup `(tenant_id, organization_id, conversation_id,
  reply_message_seq)` where non-null.

Status transitions:

```text
pending -> leased -> dispatched -> running -> completed
leased/dispatched/running -> failed -> pending     # retryable
pending/leased/dispatched/running/failed -> cancelled
failed -> dead_letter                              # attempts exhausted/policy
completed/cancelled/dead_letter -> terminal
```

Claiming is an atomic lease compare-and-set. `attempt_count` increments once per
external Agents attempt, `max_attempts` is positive and bounded, and only an
unexpired lease owner may mutate a non-terminal attempt. Lease expiry returns the
row to reconciliation; it does not prove that Agents failed. Completion requires
both a reconciled Agents turn and an IM reply committed through the normal IM
message sequence allocator. All terminal writes clear lease fields.

`last_error_detail` is bounded and sanitized. No prompt body, full Agents
transcript, credentials, manual auth headers, raw tokens, or provider response is
stored in dispatch state.

## 6. Commit And Dispatch Flow

1. Conversation creation/replacement commits assignment events to
   `im_commit_journal` and an IM outbox event in the same transaction.
2. The projection service materializes
   `im_projection_conversation_agent` from committed event order.
3. A committed IM message mentioning an assigned agent creates one
   `im_agent_dispatch` per target and the existing agent-dispatch outbox record
   atomically. `dispatch_id` is the Agents idempotency key basis.
4. The worker resolves or creates `im_conversation_agent_binding` through a
   public Agents SDK, then sends the turn using the stored Agents session id.
5. On success, IM commits the visible agent reply through the normal
   `im_conversation_messages` sequence allocator and updates the dispatch with
   the returned Agents turn id and IM reply id/sequence.
6. On timeout, the worker reads the Agents turn by stable id/idempotency before
   retrying. It never assumes timeout means failure.
7. Terminal failure remains visible as dispatch state and may produce a normal
   IM system/error message according to product policy.

The worker authenticates to the in-process Agents facade as the fixed service
principal `service.sdkwork-im.agent-dispatch`. `requested_by` remains the
end-user on-behalf-of/audit subject through `owner_user_id`; it MUST NOT be
wrapped in an `AgentsChatActor` or granted `ai.agents.manage`. Roles for the
service principal are assembly-owned constants and cannot originate from an IM
message, dispatch row, browser, or API request.

Before re-executing a dispatch that already has `agents_session_id`, and after
any indeterminate completion error, the worker calls the Agents turn lookup with
the full tenant/organization/owner/agent/session/idempotency scope. The state
machine then applies:

```text
completed           -> atomically commit the persisted Agents response as IM reply
requested|running   -> keep dispatch running and schedule reconciliation
failed|cancelled    -> fail according to bounded retry/terminal policy
not found           -> retry complete_turn with the same idempotency key
lookup error        -> schedule reconciliation; do not consume failure budget
```

Reconciliation claims for `leased`/`dispatched`/`running` rows do not increment
`attempt_count`; only claims from `pending`/`failed` represent a new external
Agents execution attempt. A deferred running row clears `lease_owner`, fences
the next claim with `lease_expires_at=next_attempt_at`, retains any known
`agents_turn_id`, and records only the bounded
`agents_turn_indeterminate` diagnostic.

The source IM message, one row per target in `im_agent_dispatch`, and the existing
dispatch outbox event are committed atomically. Projector checkpoints remain in
the existing projection/checkpoint authority; the assignment projection does not
invent a second event offset store. Visible reply insertion, reply sequence
allocation, dispatch completion, and reply outbox publication are also one IM
transaction.

## 6.1 Query Surfaces

| IM capability | Database authority |
| --- | --- |
| Show assigned agents for a conversation | `im_projection_conversation_agent` by conversation/position |
| Find conversations assigned to an agent | projection reverse lookup by agent/status |
| Resolve/create Agents session | `im_conversation_agent_binding` current generation |
| Show pending/running/failed agent activity | `im_agent_dispatch` by source message/status |
| Deduplicate mention retries | dispatch unique source-message/agent/generation key |
| Reconcile Agents timeout | dispatch `agents_turn_id`/idempotency plus binding session id |
| Render visible agent answer | existing `im_conversation_messages` timeline |
| Realtime fanout/read state/reactions | existing IM authorities; never Agents tables |

All lists use database-side scope/filter/order/limit. Conversation agent lists use
`position, id`; worker lists use `next_attempt_at, lease_expires_at, id`; reverse
lookups use `updated_at DESC, id DESC`. Application code must not load journal or
dispatch history and paginate in memory.

## 7. Subject ID Activation

The three integration tables use SQL `BIGINT` from their first migration for
`tenant_id`, `organization_id`, end-user actor ids, message ids, sequences, and
Snowflake ids. They have no legacy TEXT integration columns, so introducing
shadow columns, dual-writing duplicate columns, backfilling, or dropping a
legacy column would create a fictitious migration rather than preserve data.

The actual compatibility boundary is the existing IM message/event contract,
which serializes some subject ids as canonical decimal strings. Before an Agents
integration write, the PostgreSQL adapter MUST parse those values as unsigned
decimal, reject values above signed `int64`, require positive tenant, message,
and end-user subject ids, and allow organization `0` only where the database
contract allows it. Migration `0006` reinforces the boundary with scope/sign
CHECK constraints. Direct `u64` store commands receive the same signed-range
validation before any `as i64` conversion.

If a future migration converts an existing IM-owned TEXT column in place, it
MUST use the full preflight, shadow-column, dual-write, validated backfill,
equivalent-index, compatibility-window, and reviewed contract/drop sequence.
That rule does not authorize a module-wide subject rewrite as part of the
Agents integration contract.

## 8. SDK And Trust Boundary

- Component contracts use the canonical plural families
  `sdkwork-agents-app-sdk`, `sdkwork-agents-backend-sdk`, and
  `sdkwork-agents-sdk`.
- Browser/user-facing IM code consumes `@sdkwork/agents-app-sdk` through IM core
  composition only. It does not import the backend SDK.
- Trusted dispatch workers consume an approved server/backend Agents facade and
  propagate trusted actor context according to Agents HTTP trust rules.
- The trusted actor is the fixed IM dispatch service principal. The original
  user is propagated only as the scoped `owner_user_id` on-behalf-of/audit
  subject; IM never fabricates privileged roles for that user.
- IM never sends client-selected tenant scope or manual authentication headers.
- Credentials and token managers remain runtime/bootstrap concerns and are not
  persisted in the three integration tables.

## 9. Migration And Verification Gates

The active baseline MUST NOT be edited to simulate rollout. Implementation uses
paired PostgreSQL migrations after existing `0004` and updates the database
manifest/registries only when runtime code exists.

Required activation evidence:

```powershell
pnpm db:validate
pnpm test:contract:database
pnpm test:database-naming-standard
pnpm check:dependency-management
pnpm test:component-spec-consistency
pnpm check:app-sdk-consumer-imports
cargo test -p im-adapters-postgres-journal
cargo test -p im-adapters-postgres-journal --test agent_integration_live_test -- --ignored --nocapture
cargo test -p im-adapters-postgres-journal --test agent_integration_migration_live_test -- --ignored --nocapture
```

Tests must cover assignment replay, generation conflicts, one dispatch per
message/agent/generation, binding idempotency, lease recovery, Agents timeout
reconciliation, reply sequence allocation, tenant/organization isolation,
retention, and absence of cross-module SQL writes.
