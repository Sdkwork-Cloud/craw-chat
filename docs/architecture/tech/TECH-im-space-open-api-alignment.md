# IM Space Open API Alignment

> Owner: sdkwork-im maintainers  
> Status: current architecture (pre-launch, 2026-07)

## Authority Model

PostgreSQL tables such as `im_spaces`, `im_chat_groups`, `im_space_members`, and `im_group_members` are the only current-state authority for Space and Group. `im_commit_journal` retains immutable audit evidence; it is not replayed to rebuild current state. Core normalized rows and their audit rows are written in one transaction.

| Layer | Responsibility |
| --- | --- |
| `space-service` | Open API handlers for `/im/v3/api/spaces/*` |
| `im-domain-events::space` | Versioned `space.*` and `group.*` commit envelopes |
| `SpaceCommitJournal` | Immutable audit evidence using the same process-wide PostgreSQL pool as normalized state |
| journal-owned normalized Space writer | Applies commit envelopes to authoritative `im_*` rows in the same PostgreSQL transaction |
| `organization_store` | Authoritative normalized state for spaces, groups, and channels |
| `governance_store` | Authoritative normalized state for space members, invitations, bans, and channel access rules |
| `conversation-runtime` | Group and system-channel conversation binding when the conversation service is available |

The PostgreSQL write path for Space and Group core mutations is:

```text
handler -> SpaceWriteAuthority
  -> acquire deterministic per-partition PostgreSQL advisory locks
  -> allocate contiguous aggregate sequences
  -> append one or more rows to im_commit_journal
  -> validate member capacity and write normalized rows
  -> commit audit Journal and normalized state in one PostgreSQL transaction
```

Any Journal insert, payload validation, capacity check, normalized-state mutation, or final commit failure rolls back the entire transaction. There is no alternate normalized writer or materialize-first compensation path. Failed atomic transactions increment `im_space_postgres_atomic_write_failures_total`.

The runtime constructs both adapters from the same process-wide PostgreSQL pool in `space_service::app_state_from_postgres_pool()`. It does not create a service-local pool or fall back to a second current-state authority.

Startup validates database authority and begins serving normalized PostgreSQL state directly. It never replays the Journal or reconstructs current state in process memory.

Channel, invitation, ban, and channel-access-rule stores are normalized PostgreSQL state; they are not projections or Journal-derived read models.

## Concurrency And Capacity

All partitions in a coordinated batch are locked in lexical order with transaction-scoped PostgreSQL advisory locks. This gives same-aggregate commands one serialization point and prevents lock-order cycles for future multi-aggregate batches.

Aggregate sequence allocation occurs after the locks are held. A new aggregate starts at stored sequence `1`; subsequent events receive contiguous values. Exact event-ID replay retains its stored sequence so immutable fingerprint validation remains idempotent.

Space and group member insertion performs the parent-row capacity check and member insert inside the same transaction as the Journal append. Concurrent commands for the same aggregate cannot both pass the capacity decision against a stale pre-commit state.

## Authorization

| Surface | Rule |
| --- | --- |
| Space create | Authenticated actor becomes owner |
| Space list | Owned spaces plus member spaces for the actor |
| Space get | Space membership required |
| Space update | Space owner or admin required |
| Space delete | Space owner required |
| Group CRUD | Space/group membership and manager checks according to the command |
| Group member remove | Self-leave is allowed except for the owner; managers may remove non-owner members |
| Group owner transfer | Current group owner only; target must already be a member |
| Channel list/get | Space membership required |
| Channel create/update/delete | Space owner or admin required |
| Channel access rules | Space manager required and the channel must belong to the space |
| Invitations and bans | Space manager/member checks according to the operation |

Banned users are rejected by the Space membership access checks through `ban_store.is_user_banned`.

## Conversation Binding

### Groups

1. Independent `group_id` and `conversation_id` Snowflake values are allocated.
2. The `group.created` Journal row, normalized group row, and owner member row commit atomically.
3. The binder creates the corresponding `group` conversation after the database transaction commits.
4. A binder failure triggers a Journal-backed `group.deleted` compensation command. Failure of that command is logged and returned as an error rather than reported as success.
5. Member add/remove and owner transfer synchronize the conversation-service roster through the configured binder.

The conversation binder is a cross-service boundary and is not part of the PostgreSQL transaction. Durable outbox or saga delivery is still required before group conversation creation, roster changes, and ownership transfer can claim atomic cross-service delivery.

### Self-Leave

Members may call `DELETE .../groups/{groupId}/members/{userId}` when `userId` matches the authenticated actor. The owner must transfer ownership before leaving.

### Owner Transfer

`POST /im/v3/api/spaces/{spaceId}/groups/{groupId}/transfer_owner` accepts `{ "newOwnerUserId": "<userId>" }`. The target user must already be a group member.

### Channels

Channel and system-channel conversation IDs are allocated independently. When PostgreSQL is configured but a required production conversation binder is missing, group/channel creation fails closed instead of reporting a synthetic bind.

## Response Envelope

Open API handlers serialize through the SDKWork response helpers:

- Single resource: `data.item` through `SdkWorkResourceData`.
- Lists: `data.items` plus cursor-mode `data.pageInfo` through `SdkWorkPageData`.
- SQL-backed lists: keyset predicates and bounded `LIMIT page_size + 1` reads.
- List input: `page_size` plus an opaque `cursor`; pre-launch compatibility aliases are not accepted.

The wire view types must remain aligned with `apis/open-api/im/sdkwork-im-im.openapi.yaml` and the generated SDK authority.

## Deferred Work

- Journal coverage for channel, invitation, ban, and channel-access-rule mutations.
- Channel roster synchronization beyond system-channel bootstrap.
- Durable outbox or saga delivery for group conversation creation, roster changes, and owner transfer.
- Live PostgreSQL concurrency certification in CI using `SDKWORK_IM_DATABASE_URL`.

PostgreSQL is the only server authority for the atomic Space/Group write path described here. Client-local storage does not implement or mirror this write authority.

## Verification

```bash
cargo test -p im-adapters-postgres-journal --lib
cargo test -p im-adapters-social-postgres --lib
cargo test -p space-service
cargo test -p im-adapters-postgres-journal --test append_live_integration_test --no-run
cargo clippy -p im-adapters-postgres-journal -p im-adapters-social-postgres -p space-service --all-targets -- -D warnings
node ../sdkwork-specs/tools/check-application-layering.mjs --root .
node ../sdkwork-specs/tools/check-rust-backend-composition.mjs --root .
```

The ignored live transaction test can be executed against a migrated disposable PostgreSQL database:

```bash
SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p im-adapters-postgres-journal --test append_live_integration_test coordinated_append_allocates_sequences_and_rolls_back_callback_failures -- --ignored --nocapture
```
