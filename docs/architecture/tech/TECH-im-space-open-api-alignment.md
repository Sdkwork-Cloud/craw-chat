# IM Space Open API Alignment

> Owner: sdkwork-im maintainers  
> Status: current architecture (pre-launch, 2026-07)

## Authority Model

Space and group **mutations** are event-sourced through the IM commit journal (`im_commit_journal`); supplemental PostgreSQL tables (`im_spaces`, `im_chat_groups`, `im_space_members`, `im_group_members`, …) are **read models** materialized by `SpacePostgresMaterializer`.

| Layer | Responsibility |
| --- | --- |
| `space-service` | Open API handlers for `/im/v3/api/spaces/*` |
| `im-domain-events::space` | `space.*` / `group.*` commit envelope types |
| `SpaceCommitJournal` | Postgres (production), file (`SDKWORK_IM_RUNTIME_DIR`), or memory (dev/test) |
| `SpacePostgresMaterializer` | Maps journal events → supplemental `im_*` tables (failures increment `im_space_postgres_materialization_failures_total`; journal remains authoritative) |
| `organization_store` | Durable read model for spaces, groups, channels |
| `governance_store` | Durable read model for space members, invitations, bans, channel access rules |
| `conversation-runtime` | Group + system-channel conversation bind (unified-process when Postgres + conversation runtime are both available) |

Write path (space/group core mutations):

```
handler → SpacePostgresMaterializer (materialize-before-append)
  → single-commit: per-store writes
  → multi-commit batch (e.g. space create): one PostgreSQL transaction across supplemental tables
  → append journal
  → compensate supplemental writes when journal append fails
```

Startup replay: when Postgres journal is active, `replay_space_journal_to_read_model()` idempotently rebuilds supplemental stores from `journal.recorded()`.

Channel, invitation, ban, and channel-access-rule mutations remain direct supplemental writes (phase 2 for journal coverage).

Bootstrap entrypoint: `space_service::app_state_from_postgres_pool()`.

Unified-process wiring: `sdkwork_im_gateway_assembly::wire_space_conversation_binders()`.

## Authorization

| Surface | Rule |
| --- | --- |
| Space create | Authenticated actor becomes owner |
| Space list | Owned spaces + member spaces for actor |
| Space get | `require_space_member` |
| Space update | `require_space_manager` (owner/admin) |
| Space delete | `require_space_owner` |
| Group CRUD | `require_space_member` / `require_space_manager` / `require_group_member` / `require_group_manager` |
| Group member remove | Self-leave when `userId == actor` (owner must transfer first); managers may remove non-owner members |
| Group owner transfer | `POST .../groups/{groupId}/transfer_owner` — current owner only; PG + conversation roster stay aligned |
| Channel list/get | `require_space_member` |
| Channel create/update/delete | `require_space_manager` |
| Channel access rules | `require_space_manager` + channel belongs to space |
| Invites / bans | `require_space_manager` / `require_space_member` as applicable |

Banned users are rejected by `require_space_member` via `ban_store.is_user_banned`.

## Conversation Binding

### Groups

1. `group_id` snowflake is allocated.
2. `conversation_id = group_id` (matches PC client convention).
3. Binder creates a `group` conversation before PG insert.
4. Group row and owner member row insert in a single Postgres transaction (`GroupStore::insert_with_owner_member`).
5. Member add/remove syncs non-owner roster into conversation-service.
6. Owner transfer updates `im_chat_groups.owner_user_id`, demotes/promotes member roles in one transaction (`GroupStore::transfer_owner`), then syncs conversation ownership via binder.

### Self-leave

Members may `DELETE .../groups/{groupId}/members/{userId}` when `userId` matches the authenticated actor. The group owner cannot self-leave until ownership is transferred.

### Owner transfer

`POST /im/v3/api/spaces/{spaceId}/groups/{groupId}/transfer_owner` with `{ "newOwnerUserId": "<userId>" }` returns the updated group in `data.item`. The target user must already be a group member.

### Channels

1. `channel_id` snowflake is allocated.
2. `conversation_id = channel_id`.
3. Binder creates a `system_channel` conversation (`CreateSystemChannelCommand`) before PG insert.

When Postgres is configured but conversation binders are missing, group/channel create **fail fast** instead of leaving orphan rows.

## Response Envelope

All Open API handlers return `SdkWorkApiResponse` via `finish_api_json`:

- Single resource: `data.item` via `api_payload::resource_item` (`SdkWorkResourceData`)
- Lists: `data.items` + `data.pageInfo` (`SdkWorkPageData`, offset-mode `pageSize` + numeric `nextCursor` via `sdkwork-utils-rust`)
  - SQL-backed lists: `api_payload::bounded_sql_list_page` after `LIMIT pageSize+1 OFFSET cursor` fetches (`list_query::sql_fetch_limit` / `sql_fetch_offset`)
  - `list_spaces`: merges owned + member spaces in memory, sorts by `createdAt` desc, then `api_payload::limited_list_page`
  - Query wire: `SdkWorkCursorListQuery` (`pageSize` + `cursor`; legacy `limit` alias accepted in Rust handlers)

Wire view types (`SpaceView`, `SpaceGroupView`, `SpaceChannelView`, `SpaceChannelAccessRuleView`, …) match `sdkwork-im-im.openapi.yaml` schemas.

## Channel Access Rules

Persisted in `im_channel_access_rules` via `PostgresChannelAccessRuleStore`.

- `ruleType`: `allow` | `deny`
- `permission`: `view` | `send` | `manage`

## Supplemental Postgres Routes

Read-only supplemental handlers (`sdkwork-routes-im-social-open-api`) expose list/get/search/profile/settings surfaces backed by `im_*` tables materialized from the commit journal. **Mutations are fail-closed** on supplemental handlers; clients must use event-sourced `/im/v3/api/social/*` (open API) or `/backend/v3/api/control/social/*` (control plane).

## Deferred

- Channel/invitation/ban event sourcing (journal coverage for governance tables).
- Channel roster sync beyond system-channel bootstrap (e.g. auto-subscribe space members).

List endpoints use offset-mode `pageSize` with numeric `nextCursor` continuation (OpenAPI `LimitQuery` + `CursorQuery`; see `PAGINATION-DEBT-REGISTER.md` PAG-009). `list_spaces` merges owned + member spaces, sorts by `createdAt` desc, then pages in memory.

SDK generation merges `sdks/sdkwork-im-sdk/openapi/im-spaces-paths.fragment.yaml` into the IM OpenAPI mirror; list pagination parameters must be kept in sync with `apis/open-api/im/sdkwork-im-im.openapi.yaml`.

## Verification

```bash
cargo check -p im-domain-events -p im-adapters-social-postgres -p space-service -p sdkwork-im-gateway-assembly
cargo test -p space-service --test http_smoke_test
cargo test -p im-adapters-social-postgres space_created_event_materializes
cargo test -p space-service journal_bootstrap
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
```
