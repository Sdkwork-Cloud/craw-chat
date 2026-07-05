# Pagination Debt Register

Authority: `sdkwork-specs/PAGINATION_SPEC.md` v1.1.

**Status (2026-07-06):** P0/P1 interactive IM and PC/H5 client pagination debt **cleared**. Residual items are documented export/maintenance paths only.

## Closed in 2026-07-05 audit

| ID | Resolution |
| --- | --- |
| PAG-001 | `projection-service/src/inbox.rs` — incremental per-principal `BTreeMap` inbox index on projection writes |
| PAG-002 | `projection-service/src/contacts.rs` — per-owner contact scope index |
| PAG-003 | `rpc_dispatch.rs` — member directory window without `limit*4` over-fetch |
| PAG-004 | `ActorInboxRuntimeStore` per-actor conversation index |
| PAG-005 | `im-domain-core/conversation.rs` — roster `BTreeMap` window iteration |
| PAG-006 | `postgres-projection/timeline_store.rs` — keyset batched SQL restore |
| PAG-007 | OpenAPI/SDK `pageSize` wire alignment across IM services and PC/H5 clients |
| PAG-008 | `streaming-service` cursor list frames |
| PAG-009 | `space-service` cursor list handlers |

## Closed in 2026-07-05 remediation pass

| ID | Resolution |
| --- | --- |
| PAG-010 | `notification-service` — `list_notifications_page` with `SdkWorkCursorListQuery`, recipient `BTreeMap` index iteration, `SdkWorkPageData` + `pageInfo` |
| PAG-011 | `projection-service/src/interactions.rs` — maintained per-scope `pinned_messages_index`; list/window iterate index without per-request full collect |
| PAG-012 | PC `ContactService` — `getStarredContacts` / `getUserById` use `forEachCursorPage` instead of first-page-only `getContacts()` |
| PAG-013 | Console `GroupService` — cursor-mode inbox paging (`listGroupsPage`); removed client virtual offset assembly |
| PAG-014 | Social supplemental Postgres lists (`postgres/block.rs`, `postgres/direct_chat.rs`) — `SdkWorkCursorListQuery` offset mode with SQL `LIMIT/OFFSET` via `bounded_sql_list_page` |

## Closed in 2026-07-06 remediation pass

| ID | Resolution |
| --- | --- |
| PAG-015 | PC `ChatService.doCatchUpConversationMessages` — bounded to `MAX_CATCH_UP_MESSAGE_PAGES` (50 pages × default `pageSize` 20) |
| PAG-016 | H5 `fetchAllChatInboxEntries` removed; inbox uses `fetchChatInboxPage` only |
| PAG-017 | PC/H5 default `pageSize` aligned to `SDKWORK_DEFAULT_PAGE_SIZE` (20) across chat/contact/group/favorite services |
| PAG-018 | PC mail/shop/devices/orders — migrated to sibling PC packages (`sdkwork-mail-pc`, `sdkwork-shop-pc`, `sdkwork-aiot-pc`); IM packages are thin adapters only |
| PAG-019 | Console `RoleService` — `forEachCursorPage` bounded sync instead of `collectCursorPages` |
| PAG-020 | Social `postgres/user_search.rs` — per-result `find_by_pair` / `find_active_block` instead of 500-row friendship/block scans |

## Residual P2 (documented, non-blocking for pre-launch)

| Area | Notes |
| --- | --- |
| Inbox export helpers (`projection-service/src/inbox.rs`) | Bounded multi-page export (`INBOX_EXPORT_MAX_ITEMS = 10_000`), not interactive UI |
| Maintenance sweeps (`session-gateway/presence.rs`, `im-calls-service` expiration) | Batch jobs with early `take`; not interactive list APIs |
| Contact tag memory fallback (`contact_open_api_backend.rs`) | Dev/test memory path only; production uses Postgres `list_tags_by_owner` SQL pagination |
| Organization directory bulk sync | Bounded `forEachCursorPage` startup hydration with `mapAppSdkCursorPage` + `unwrapSdkWorkApiEnvelope`; interactive UI should migrate to explicit page APIs over time |

Rust HTTP query deserializers accept `limit` as an alias for `pageSize` until external integrators migrate. OpenAPI authority and generated IM SDK expose **`pageSize` only**.

## Verification

```bash
pnpm run check:pagination
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
cargo check -p streaming-service -p notification-service -p social-service
cargo test -p notification-service -p projection-service -p social-service -p automation-service
```

Expected: pagination and envelope checkers pass; targeted service checks compile and tests pass.
