# Pagination Debt Register

Authority: `sdkwork-specs/PAGINATION_SPEC.md` v1.3 (§12 Pre-Launch Zero-Debt Rule).

**Status (2026-07-07):** All P0/P1/P2 pagination technical debt is **cleared**. The application is pre-launch with zero production users, so no compatibility handling, legacy aliases, or migration exceptions remain. Residual items are documented bounded exceptions per `PAGINATION_SPEC.md` §2.5 and §11 only.

## Closed in 2026-07-07 pre-launch zero-debt remediation pass

| ID | Resolution |
| --- | --- |
| PAG-041 | `sdkwork-utils-rust` — removed `limit` serde alias from `SdkWorkCursorListQuery`, `SdkWorkPageSizeQuery`, `SdkWorkSeqWindowQuery`; removed `limit` parameter from `CursorListPageParams::resolve` |
| PAG-042 | `sdkwork-rtc` — updated `resolved_list_page_size` call to match new 2-parameter `CursorListPageParams::resolve` signature |
| PAG-043 | `space-service/ban.rs` — OFFSET → keyset `(created_at DESC, ban_id DESC)` + `keyset_list_page` |
| PAG-044 | `space-service/channel.rs` — OFFSET → keyset `(created_at DESC, channel_id DESC)` + `keyset_list_page` |
| PAG-045 | `space-service/channel_access_rule.rs` — OFFSET → keyset `(created_at ASC, rule_id ASC)` + `keyset_list_page` |
| PAG-046 | `space-service/group.rs` — OFFSET → keyset `(created_at DESC, group_id DESC)` + `keyset_list_page` |
| PAG-047 | `space-service/group_member.rs` — OFFSET → keyset `(joined_at ASC, user_id ASC)` + `keyset_list_page` |
| PAG-048 | `space-service/invitation.rs` — OFFSET → keyset `(created_at DESC, invitation_id DESC)` + `keyset_list_page` |
| PAG-049 | `space-service/space.rs` — OFFSET → keyset `(created_at DESC, space_id DESC)` + `keyset_list_page` |
| PAG-050 | `space-service/space_member.rs` — OFFSET → keyset `(joined_at ASC, user_id ASC)` + `keyset_list_page` |
| PAG-051 | `space-service/list_query.rs` — removed deprecated `resolve_list_page`, `sql_fetch_limit`, `sql_fetch_offset` |
| PAG-052 | `space-service/api_payload.rs` — removed deprecated `limited_list_page`, `bounded_sql_list_page` |
| PAG-053 | `governance_store.rs` — updated `BanStore`, `ChannelAccessRuleStore`, `SpaceMemberStore`, `InvitationStore` traits + SQL + Postgres impls to keyset predicates |
| PAG-054 | `organization_store.rs` — updated `SpaceStore`, `GroupStore`, `GroupMemberStore`, `ChannelStore` traits + SQL + Postgres impls to keyset predicates |
| PAG-055 | `session-gateway/realtime_http_routes.rs` — removed `x-request-id` legacy header fallback from `resolve_trace_id` |
| PAG-056 | `PAGINATION_SPEC.md` §12 — added Pre-Launch Application Zero-Debt Rule |
| PAG-057 | `space-service/tests/http_smoke_test.rs` — updated all Noop store mock impls to match keyset trait signatures |
| PAG-058 | OpenAPI and generated SDK page-size wire — `PageSizeQuery` now uses HTTP query `page_size`; generated TypeScript, Flutter, Rust, Java, Kotlin, C#, Swift, Go, and Python transports serialize `page_size` while preserving language-level `pageSize` model/options fields |
| PAG-059 | `projection-service` inbox HTTP query contract — canonical URL query is `page_size`; forbidden aliases including `pageSize`, `limit`, `page_no`, `pageNo`, `per_page`, and `size` return `40003 INVALID_PARAMETER` |
| PAG-060 | `social-service/contact_open_api_backend.rs` — fixed memory fallback contact-tag keyset range bounds for `Reverse<String>` so empty or bounded tag lists do not panic or leave callers pending |

## Related API contract debt closed in 2026-07-07 pass

| ID | Resolution |
| --- | --- |
| API-001 | `social-service` open-api and backend-api handlers now follow SDKWork operation semantics: create returns `201`, delete returns `204` with an empty body, and list/retrieve/update/command reads return `200` envelopes |
| API-002 | `sdkwork-api-product-runtime` admin sandbox collection creates now return `201`; config upserts, validation commands, status commands, and runtime reload commands remain `200`; deletes remain `204` |
| API-003 | Portal home runtime tests now assert `SdkWorkApiResponse.data.item` with `code: 0` and `traceId`, and no longer preserve the retired embedded organization-directory payload |

## Closed in 2026-07-07 remediation pass (DDL + keyset + security + OOM)

| ID | Resolution |
| --- | --- |
| DDL-001 | PostgreSQL baseline DDL: removed duplicate Migration 001 table definitions that shadowed Migration 010 organization_id columns |
| DDL-002 | SQLite baseline DDL: generated SQLite-compatible syntax (TEXT for JSONB/TIMESTAMPTZ, json_valid() CHECK, removed DO $$/pg_constraint/USING GIN) |
| PAG-039 | `social-service/block.rs` — OFFSET → keyset `(created_at DESC, block_id DESC)` + `keyset_list_page` |
| PAG-040 | `social-service/direct_chat.rs` — OFFSET → keyset `(updated_at DESC, direct_chat_id DESC)` + `keyset_list_page` |
| SEC-001 | `projection-service/cursor_auth.rs` — added `_FILE` secret variant, removed hardcoded dev secret fallback, fail-closed on ephemeral generation failure |
| OOM-001 | `projection-service/timeline_tier.rs` — `load_full_timeline_for_restore` safety cap (10,000 entries) prevents OOM on long-lived conversations |
| PERF-001 | `sdkwork-comms-conversation-service/runtime.rs` — eviction uses `select_nth_unstable_by_key` (O(n)) instead of full sort (O(n log n)) |

## Closed in 2026-07-07 remediation pass (concurrency + security + offline)

| ID | Resolution |
| --- | --- |
| CONC-001 | `session-gateway` realtime restore uses `lock_scope_sequence_maps` canonical order |
| CONC-002 | gRPC `list_events` / `ack_events` isolated via `spawn_blocking` |
| CONC-003 | `im-calls-service` handlers use `run_blocking_call` (`spawn_blocking`) |
| CONC-004 | `conversation-service` `post_message` uses `run_blocking_conversation` |
| CONC-005 | `session-gateway` production fail-closed without PG pool + membership gate |
| CONC-006 | `projection-service` inbox entry build snapshots stores per lock (no nested hold) |
| CONC-007 | `projection-service` embedded apply fail-closed in production-like env |
| CONC-008 | Desktop `offline_store` WAL + `BEGIN IMMEDIATE` transactions + pending send claim |
| CONC-009 | H5 IndexedDB offline queue + claim/lease + legacy sessionStorage migration |
| CONC-010 | Flutter `shared_preferences` v2 offline queue + claim/lease |
| CONC-011 | `session-gateway` cluster route notifier cleanup releases lock before route lookup |

## Closed in 2026-07-06 remediation pass (pageInfo envelope)

| ID | Resolution |
| --- | --- |
| PAG-025 | `projection-service` inbox/contacts/timeline/favorites/search — `SdkWorkPageData` + nested `pageInfo` |
| PAG-026 | `sdkwork-comms-conversation-service` members/history/inbox/pinned — `SdkWorkPageData` + `pageInfo` |
| PAG-027 | PC `ChatService.getChats()` — bounded `forEachCursorPage` sync (max 2000) |
| PAG-028 | PC `appSdkResponseHelpers.readCursorPageInfo` — `pageInfo` only (no root `hasMore` fallback) |
| PAG-029 | `message_realtime` fanout — `list_members_window` batched publish (1000/batch) |
| PAG-030 | `social-service/block.rs` — nested cross-instance file lock deadlock removed |
| PAG-031 | `notification_task_store` — batched SQL restore (`LIMIT 200`) + transactional save |
| PAG-032 | `audit-service` list — `SdkWorkPageData` + nested `pageInfo` (seq cursor) |
| PAG-033 | `session-gateway` HTTP realtime poll — `RealtimeEventsListData` with `pageInfo` |
| PAG-034 | `automation_execution_store` — transactional load-merge-upsert |
| PAG-035 | `aggregate_store` read cursors — batched SQL restore (`LIMIT 500`) |
| PAG-036 | PC `ChatService` timeline/members — `pageInfo` only (no root `hasMore`) |
| PAG-037 | `social-service` contact tags — SQL keyset `(updated_at DESC, tag_id DESC)` + signed HS256 cursor (no OFFSET) |
| PAG-038 | PC `ContactService.getTags()` — bounded `forEachCursorPage` sync (max 200 tags) |

## Closed in 2026-07-05 / 2026-07-06 (index + client)

See prior table entries PAG-001 through PAG-024 in git history; all P0/P1 interactive index and client cursor paths remain closed.

## Documented Bounded Exceptions (per PAGINATION_SPEC.md §2.5 and §11, not debt)

| Area | Justification |
| --- | --- |
| Inbox export helpers (`projection-service/src/inbox.rs`) | Bounded multi-page export (`INBOX_EXPORT_MAX_ITEMS = 10_000`), not interactive UI. Per §2.5, bounded batch export jobs MAY scan from maintained indexes. |
| Maintenance sweeps (`session-gateway/presence.rs`, `im-calls-service` expiration) | Batch jobs with early `take(batch_limit)` on sorted iterators. Per §2.5, background expiration jobs MAY scan bounded batches. |
| Organization directory bulk sync | Bounded `forEachCursorPage` startup hydration. Per §2.5, bounded batch reconciliation during startup is allowed. |

## Pre-Launch Zero-Debt Verification

Per `PAGINATION_SPEC.md` §12, this pre-launch application has:

- ✅ No legacy `pageSize` or `limit` wire aliases in HTTP query deserializers; `pageSize` remains language-level SDK/model naming only
- ✅ No numeric offset strings as `cursor` in interactive list APIs
- ✅ No `OFFSET` pagination on any interactive list API (all migrated to keyset)
- ✅ No deprecated pagination helper functions in production code paths
- ✅ No `x-request-id` header fallbacks for `traceId` in HTTP routes
- ✅ No migration exception entries in `MIGRATION_SPEC.md` for pagination
- ✅ All list responses include `PageInfo.mode` (offset or cursor)

## Verification

```bash
pnpm run check:pagination
node scripts/dev/align-im-openapi-page-size-wire.mjs
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node sdks/test/verify-im-v3-sdk-family-contract.test.mjs
node scripts/dev/sdkwork-im-pc-client-pagination-standard.test.mjs
cargo test -p projection-service --test http_smoke_test
cargo test -p social-service
cargo test -p sdkwork-api-product-runtime
cargo test -p space-service --test http_smoke_test
cargo check -p sdkwork-comms-conversation-service -p projection-service -p space-service
```

Expected: pagination, operation-pattern, and envelope checkers pass; targeted runtime tests assert SDKWork list envelopes, canonical `page_size` input, create `201`, delete `204`, and `ProblemDetail` errors.
