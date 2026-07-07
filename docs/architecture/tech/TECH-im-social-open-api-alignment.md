# IM Social Open API Alignment

> Owner: sdkwork-im maintainers  
> Status: current architecture (pre-launch, 2026-07)

## Authority Model

Social mutations are event-sourced through `SocialRuntime` and persisted to the shared IM commit journal when PostgreSQL is configured.

| Layer | Responsibility |
| --- | --- |
| `SocialRuntime` | Authoritative in-memory aggregate for friend requests, friendships, blocks, direct chats |
| `im_commit_journal` | Durable commit log (Postgres in production; file/memory in dev) |
| `projection-service` | Contact read model for `GET /im/v3/api/chat/contacts` |
| `social-postgres` materializer | Supplemental **read-only** tables (`im_friend_requests`, `im_friendships`, `im_user_blocks`, contact tags/preferences/recommendations); materialize-before-append on writes; journal replay on bootstrap heals drift |

Startup replay: `replay_social_journal_to_projection()` (embedded projection) and `replay_social_journal_to_postgres_read_model()` (supplemental Postgres) run from `build_social_runtime_from_env()` when Postgres journal + pool are configured.
| `im_contact_tags` / `im_contact_preferences` / `im_contact_recommendations` | Durable contact UI metadata (tags, star, remark, recommendations); Postgres-backed when IM DB is configured — not an in-memory production path |
| `conversation-runtime` | Direct chat conversation bind on friend accept (unified-process only) |
| `session-gateway` realtime plane | Push social domain events to connected clients |

Bootstrap entrypoint: `social_service::build_social_runtime_from_env()`.

Unified-process wiring: `sdkwork_im_gateway_assembly::ApplicationAssembly::wire_embedded_realtime_plane()`.

Direct-message access in unified-process uses `SocialRuntime::ensure_direct_message_allowed()` (refreshes journal authority + block check); gateway assembly must not call `pub(crate)` runtime internals.

## Response Envelope

Postgres supplemental HTTP handlers, user search, runtime open-api routes, contact tag/preference routes, and backend control-plane routes (`/backend/v3/api/control/social/*`) return `SdkWorkApiResponse` via `finish_api_json` / `envelope::finish_enveloped_json`:

- Single resource / command: `data.item` via `api_payload::resource_item` (`SdkWorkResourceData`)
- Lists: `data.items` + `data.pageInfo` via `api_payload::limited_list_page`, `api_payload::full_inventory_page`, or `sdkwork_utils_rust::cursor_list_page_data`

Event-sourced control-plane handlers use `Extension<WebRequestContext>` + `Extension<AppContext>` (injected by `WebFrameworkLayer` when mounted through `sdkwork-routes-im-social-*`).

Infra `/metrics` on the social-service process merges standard HTTP metrics with shared-channel sync counters (`im_shared_channel_sync_*`, `im_health_status`).

## Write Path

```
Client → /im/v3/api/social/* → SocialRuntime mutation
  → SocialPostgresMaterializer (materialize-before-append when Postgres pool configured)
    → single-commit: per-store writes
    → multi-commit batch (e.g. friend accept): one PostgreSQL transaction across supplemental tables
  → commit journal append (compensate supplemental writes on append failure)
  → projection apply (contacts list)
  → SocialRealtimeFanout when embedded session-gateway is co-located (direct push)
  → else `im_outbox_events` enqueue (`aggregate_type=social`) when Postgres outbox is wired (split-deploy)
  → optional DirectChatConversationBinder (conversation-runtime bind before direct_chat.bound)
  → in-memory state update
```

Materialization failures before journal append reject the write and increment `im_social_postgres_materialization_failures_total`. Journal append failures after successful materialize increment `im_social_postgres_journal_append_failures_after_materialize_total` and attempt compensate rollback.

## Read Path

List and count handlers acquire a read lock, refresh once from journal authority when needed, then serve from in-memory runtime state. Full journal replay on every query is not used.

Control-plane snapshot routes (`friend_request_snapshot`, `friendship_snapshot`) enforce participant ACL: only requester/target or friendship members may read.

Control-plane decline/cancel/remove/activate mutations bind `declinedByUserId` / `canceledByUserId` / `removedByUserId` / `initiatorUserId` to the authenticated actor.

## Open API Surfaces

| Route | Purpose |
| --- | --- |
| `GET/POST /im/v3/api/social/friend_requests` | List/create friend requests (`data.items` + `data.pageInfo` cursor mode) |
| `GET /im/v3/api/social/friend_requests/pending/count` | Pending request count (`data.item.count`) |
| `GET /im/v3/api/social/friendships` | List friendships (`SocialRuntime` cursor inventory; **runtime open-api only**) |
| `POST /im/v3/api/social/friendships/{friendshipId}/remove` | Remove friendship (event-sourced) |
| `POST /im/v3/api/social/user_blocks` | Domain block (not preference-only); exposed by IM OpenAPI and generated SDK as `social.userBlocks.create` |
| `DELETE /im/v3/api/social/user_blocks/{blockId}` | Domain block release; exposed by IM OpenAPI and generated SDK as `social.userBlocks.delete` |
| `GET /im/v3/api/social/users` | User search (Postgres supplemental read model when IM DB configured) |
| `GET /im/v3/api/social/user_blocks` | Block list (**supplemental Postgres mount only**; not in OpenAPI/SDK; internal/gateway read mount) |
| `GET /im/v3/api/social/direct_chats` | Direct chat inventory (**supplemental Postgres mount only**; not in OpenAPI/SDK) |
| `GET /im/v3/api/chat/contacts` | Projected contact list (friendships) |
| `/im/v3/api/social/contacts/*` | Tags and preferences — **keyset cursor** list for tags (`updated_at`, `tag_id`); Postgres-backed when IM DB available |
| `POST /im/v3/api/social/contacts/{targetUserId}/recommendations` | Durable recommendation rows in `im_contact_recommendations` (Postgres when IM DB configured) |

Contact tags, preferences, and recommendations **fail closed** in production/staging when the IM Postgres pool is unavailable (`503` / `contact_store_unavailable`). In-memory fallback is limited to `SDKWORK_IM_ENVIRONMENT=dev|test` and the Rust test harness.

## Unified Gateway Mount (standalone)

`sdkwork_im_gateway_assembly::assemble_application_router()` always merges:

1. `sdkwork_routes_im_social_open_api::build_runtime_public_app` — event-sourced mutations and authoritative list/count surfaces (`friend_requests`, `friendships`, `user_blocks` create/delete, contacts tags/preferences).
2. When IM Postgres is configured: `sdkwork_routes_im_social_open_api::gateway_mount` — supplemental **read-only** Postgres handlers (user search, block list, direct chat list/get, profile/settings). Supplemental routes **must not** duplicate runtime open-api method+path pairs (for example `GET /im/v3/api/social/friendships`).

Dev startup (`pnpm dev`, `pnpm gateway:run:standalone`) uses `scripts/dev/run-standalone-gateway-dev.mjs`: terminate stale Windows gateway processes, wait for executable unlock, `cargo build`, then run the binary (avoids `cargo run` exe lock failures on Windows).

## Realtime Events

Social commits publish scope events with `scopeType: user`, `scopeId: <userId>`:

| Event | Recipients |
| --- | --- |
| `friend_request.submitted` | requester + target |
| `friend_request.accepted` | requester + target |
| `friend_request.declined` | requester + target |
| `friend_request.canceled` | requester + target |
| `friend_request.expired` | requester + target |
| `friendship.activated` | both friendship members |
| `friendship.removed` | both former members |
| `user_block.blocked` | blocker + blocked user |
| `user_block.released` | blocker + blocked user |

Contact projection applies `user_block.blocked` / `user_block.released` asymmetrically: only the **blocker** contact row shows `relationshipState: blocked`; the blocked user's row stays `active` while server-side messaging remains fail-closed.

## Client Integration

- PC `ContactService.addToBlacklist` calls `social.userBlocks.create` then syncs contact preferences.
- Pending badge uses `social.friendRequests.pendingCount()` (`data.item.count`); client may fall back to full pending list only when the count endpoint is unavailable.
- Contact list continues to use `chat.contacts.list` (projection read model).
- Contact tags and friend requests use cursor-mode `SdkWorkPageData` (`data.items` + `data.pageInfo`); TypeScript composed facade exposes unwrapped page DTOs via `openapi-compat-types.ts`.
- Group invitation acceptance: `conversations.members.acceptInvitation` (`POST .../members/accept_invitation`) transitions `Invited → Joined`.

## Canonical Direct Chat IDs

Direct chat `conversationId` and `directChatId` are derived from the normalized actor pair via `im_domain_core::direct_chat::resolve_direct_chat_binding_ids()`. Social accept and conversation-service bind must use the same canonical IDs (not request-scoped synthetic IDs).

When social authority is durable (Postgres journal or file journal), friend accept **requires** `DirectChatConversationBinder` wiring; accept fails with `503` if the binder is missing.

## Block Lifecycle

- `POST /im/v3/api/social/user_blocks` — event-sourced block (`user_block.blocked`); `blockerUserId` 来自认证用户
- `DELETE /im/v3/api/social/user_blocks/{blockId}` — event-sourced release (`user_block.released`); 仅 blocker 可解封
- Supplemental Postgres DELETE unblock route removed (was bypassing journal authority)
- **Friendship cascade**: blocking with scope `all` or `friendship` atomically emits `friendship.removed` when an active friendship exists for the pair (same journal batch as `user_block.blocked`), archives active direct chats, and keeps contact projection consistent without stale active friendships while blocked
- **Message enforcement**: unified-process 优先 embedded `SocialRuntimeDirectMessageAccessGate`；split-deploy conversation 进程使用 `PostgresDirectMessageAccessGate`（查询 `im_user_blocks`）在 `post_message` Direct 场景拦截发送

## Message Post Atomic Write (Conversation)

When Postgres is configured, `build_conversation_runtime_from_env()` wires:

| Component | Role |
| --- | --- |
| `PostgresDurableMessagePostWriter` | Single transaction: `im_commit_journal` append + `im_conversation_messages` insert + optional `im_outbox_events` enqueue |
| `build_message_posted_outbox_record` | Enqueues outbox only when no in-process `RealtimeEventPublisher` (split-deploy) |
| `spawn_conversation_outbox_relay_from_env` | Drains `aggregate_type=conversation` outbox rows → `RealtimeDeliveryRuntime` (user scope, `conversation` subscription) |

Unified-process with embedded session-gateway uses direct `publish_message_posted_realtime`; outbox enqueue is skipped to avoid double delivery.

## Social Realtime Outbox Relay

When Postgres is configured, `build_social_runtime_from_env()` wires `PostgresOutboxStore` + Snowflake `IdGenerator` on `SocialRuntime`.

| Component | Role |
| --- | --- |
| `build_social_realtime_outbox_record` | Enqueues outbox when no in-process `SocialRealtimeFanout` (split-deploy social process) |
| `spawn_social_outbox_relay_from_env` | Drains `aggregate_type=social` rows → `RealtimeDeliveryRuntime` (`user` scope) |
| `SessionGatewaySocialRealtimeFanout` | Unified-process direct fanout; outbox enqueue skipped when fanout is registered |

Unified-process: `wire_social_runtime_embedded_plane` registers embedded fanout; social outbox relay still runs on session-gateway but only drains rows produced by split-deploy social workers.

Production fail-closed: set `SDKWORK_IM_REQUIRE_REALTIME_PUBLISHER=1` when split-deploy has no embedded fanout/publisher and no outbox → `post_message` and social commits that require realtime delivery return unavailable.

## Friend Request Rate Limit

- Env: `SDKWORK_IM_FRIEND_REQUEST_DAILY_LIMIT` (default **50** per requester per UTC day)
- Checked in `SocialRuntime.submit_friend_request` before journal append; quota increments only after durable commit succeeds (failed/idempotent retries do not consume quota)
- **Multi-instance authority**: when Postgres supplemental stores are configured, daily counts are read from `im_friend_requests.created_at` for the UTC day; process-local counters are used only for dev/memory journal paths
- HTTP `429` with `friend_request_rate_limited` and `Retry-After` (seconds until next UTC day)
- Gateway per-tenant rate limiting remains the outer abuse-protection layer for all social HTTP surfaces

## Control-Plane Friendship Activation

- `POST /backend/v3/api/control/social/friendships` (`activate_friendship`) is fail-closed by default
- Allowed when `SDKWORK_IM_SOCIAL_CONTROL_ACTIVATE_FRIENDSHIP=true` (migration/backfill) **or** an accepted friend request exists for the normalized pair (repair after partial materialization)
- Normal user consent path remains `accept_friend_request`, which materializes friendship and direct chat in one batch

## Related Domains

- **Space groups** (`space-service`): see [TECH-im-space-open-api-alignment.md](./TECH-im-space-open-api-alignment.md) for group CRUD authorization, conversation binding, channel access rules, and SdkWorkApiResponse envelopes. Space member, invitation, ban, and channel ACL handlers are backed by real Postgres stores (`PostgresSpaceMemberStore`, `PostgresInvitationStore`, `PostgresBanStore`, `PostgresChannelAccessRuleStore`); group member roster uses `PostgresGroupMemberStore`.
- **Conversation roster persistence**: membership and read-cursor mutations call `persist_aggregate_state()` / `best_effort_persist_aggregate_state()` when `PostgresAggregateStore` is configured via conversation-runtime bootstrap.
- **Projection personalization** (`projection-service`): conversation preferences and message favorites durable snapshots via `personalization_snapshot.rs` (metadata catalog `projection-personalization`); restored on bootstrap with other projection snapshots.
- **Streaming sessions** (`streaming-service`): stream session/frame state persists to `im_stream_sessions` / `im_stream_frames` via `PostgresStreamStateStore` when IM Postgres is configured (`streaming_service::build_runtime_from_env()`).

## Friend Request TTL

- New submissions carry `expiresAt` on `friend_request.submitted` (default **7 days**, env `SDKWORK_IM_FRIEND_REQUEST_TTL_SECONDS`).
- Background scheduler (`SDKWORK_IM_FRIEND_REQUEST_EXPIRATION_SCHEDULER_ENABLED`, interval `SDKWORK_IM_FRIEND_REQUEST_EXPIRATION_INTERVAL_SECONDS`) emits `friend_request.expired` commits for stale pending requests.
- Accept handler rejects expired pending requests (`friend_request_expired`) even if scheduler has not yet run.
- Wired in unified-process bootstrap (`ApplicationAssembly`) alongside shared-channel stale reclaim.
- Pending badge API: `GET /im/v3/api/social/friend_requests/pending/count` returns `{ count }` in `data.item`.
- User search `relationshipState`: `self`, `active`, `pending_incoming`, `pending_outgoing`, `none` (blocked users filtered from results).
- OpenAPI block/unblock use deterministic `blockId` / `eventId` seeds; duplicate block requests return the active record; release is idempotent when already released.
- Contact preferences `GET` syncs `isBlocked` from `UserBlock` (`all` scope), not preference-store drift alone.

## Deferred

- H5/Flutter contacts surface: `sdkwork-im-h5-chat` is inbox/conversation-only today; full `ContactService` parity with PC (`sdkwork-im-pc-chat`) is a separate product slice.

## Verification

```bash
cargo check -p social-service -p space-service -p projection-service -p streaming-service -p im-adapters-postgres-journal -p sdkwork-comms-conversation-service -p sdkwork-im-gateway-assembly
cargo test -p projection-service -p sdkwork-comms-conversation-service -p social-service -p sdkwork-im-gateway-assembly --lib
cargo test -p social-service friend_request_expiration friend_request_rate -- --nocapture
cargo test -p projection-service test_user_block_projection_marks_and_restores_friendship_contacts -- --exact
node sdks/sdkwork-im-sdk/bin/generate-sdk.mjs --language typescript
node scripts/dev/sdkwork-im-open-api-route-coverage.test.mjs
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
```

Production ops flags:

| Env | Purpose |
| --- | --- |
| `SDKWORK_IM_REQUIRE_REALTIME_PUBLISHER=1` | Fail `post_message` and social commits when neither embedded publisher/fanout nor outbox is configured |
| `SDKWORK_IM_FRIEND_REQUEST_DAILY_LIMIT` | Per-requester daily friend request cap (default 50) |
| `SDKWORK_IM_SOCIAL_CONTROL_ACTIVATE_FRIENDSHIP=true` | Allow control-plane `activate_friendship` without accepted-request evidence |
| `SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_*` | Split-deploy conversation message.posted relay tuning |
| `SDKWORK_IM_SOCIAL_OUTBOX_RELAY_*` | Split-deploy social commit realtime relay tuning |
| `SDKWORK_IM_REALTIME_FANOUT_RECIPIENT_BATCH_SIZE` | Chunk size for durable/ephemeral scope fanout (default 256) |
