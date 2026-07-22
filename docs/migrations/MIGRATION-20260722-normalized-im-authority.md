# MIGRATION-20260722: Normalize IM Persistence Authority

- ID: `MIG-2026-0722`
- Owner: `im-platform`
- Status: active
- Requirement: `REQ-2026-0722`
- Type: mixed database/package/runtime/documentation
- Strategy: pre-launch cutover with reversible table renames and validated contract
- Compatibility window: 2026-07-22 through first commercial release

## Producers

- `sdkwork-comms-conversation-service`
- `social-service`
- `session-gateway`
- IM Agent dispatch worker

## Consumers

- IM App/Open/Backend API assemblies
- PC, H5, Flutter, and server SDK consumers
- realtime relay and operations services

## Mapping

| Retired authority | Canonical target |
| --- | --- |
| `im_projection_conversation_summaries` | `im_conversations` |
| `im_projection_conversation_members` | `im_conversation_members` |
| `im_projection_read_cursors` | `im_conversation_read_cursors` |
| `im_projection_registered_client_routes` | `im_registered_client_routes` during route-store consolidation |
| `im_projection_client_route_sync_feeds` | `im_client_sync_events` |
| `im_projection_client_route_sync_checkpoints` | `im_client_sync_cursors` |
| `im_projection_conversation_agent` | `im_conversation_agent_assignments` |
| `im_projection_timeline_entries` | removed; use `im_conversation_messages` |
| `im_projection_contacts` | removed; bounded canonical social joins |
| `im_projection_direct_chat_bindings` | removed; use `im_direct_chats.conversation_id` |
| `im_projection_metadata_snapshots` | retired after typed state backfill |

## Cutover

1. Stop IM writes for the scoped maintenance window.
2. Create an encrypted PostgreSQL backup and record schema, per-tenant row counts,
   checksums, and migration history.
3. Reject ambiguous direct-chat mappings where one direct chat or conversation maps
   to multiple active records.
4. Rename canonical state tables in place so stable rows and indexes are preserved.
5. Backfill `im_conversations` from existing summary rows and validate every member,
   message, and read cursor references an existing conversation.
6. Verify the persisted timeline equals `im_conversation_messages`; abort on any
   non-identical message identity or sequence before dropping the duplicate table.
7. Verify contact and direct-chat derived results against canonical social tables.
8. Switch repositories and runtime composition to normalized state reads/writes.
9. Run API, SDK, standalone, cloud, realtime, PostgreSQL, and recovery smoke tests.
10. Drop duplicate tables only after all checks pass.

No projection table, compatibility view, dual write, or shadow schema remains after
cutover.

## Rollback

Rollback is supported until duplicate tables are dropped:

1. Stop writes.
2. Run the paired down migration, which renames canonical tables back only when no
   conflicting target exists.
3. Restore the previous application build and rerun scoped checksum verification.

After duplicate-table deletion, rollback requires restoring the encrypted backup.
That destructive contract step requires separate human approval and a successful
restore rehearsal.

## Verification

- Contract and migration SQL execute twice on a disposable PostgreSQL database.
- Forward and down migrations preserve scoped row counts and primary keys.
- Orphan, duplicate, sequence, and tenant/organization isolation checks return zero.
- No authored DDL, repository SQL, active spec, or canonical document contains an
  active `im_projection_*` authority.
- `sdkwork-agents` has no Cargo, pnpm, HTTP, SQL, or SDK dependency on IM.
