# SDKWork IM Database Inventory

Status: active
Owner: `im-platform`
Generated: yes
Generator: `docs/sites/scripts/generate-contract-inventories.mjs`
Source contract: `database/contract/table-registry.json`
Specs: `DATABASE_SPEC.md`, `DOCUMENTATION_SPEC.md`

This inventory contains only tables owned by SDKWork IM. IAM, Agents, Drive, Knowledgebase, RTC,
and other sibling databases are external dependencies and are intentionally excluded.

## Persistence Authority

- PostgreSQL is the durable IM authority for normalized Conversation, Message, Member, ReadCursor,
  social, realtime, signaling, and operational state.
- `im_commit_journal` is immutable audit/integration evidence, not a source for rebuilding current state.
- `im_outbox_events` and `im_inbox_events` provide transactional integration delivery.
- A business mutation, its journal evidence, and required outbox event commit in one transaction.
- Current state is read from typed normalized tables. No second Message timeline or persisted read-model
  authority is allowed.
- Cross-domain identifiers are opaque references without physical foreign keys to sibling databases.

## Registry Summary

- Schema version: `1`
- Module: `im`
- Contract version: `2.0.0`
- Lifecycle strategy: `baseline-plus-migrations`
- Registered IM tables: 63
- Runtime engines: `postgres`

## Table Inventory

### instant_messaging (38)

| Table | Profile | Write owner | Authority role | Migration / DDL source |
| --- | --- | --- | --- | --- |
| `im_agent_dispatch` | `operational_state` | `comms-conversation-service` | system of record | `database/migrations/postgres/0005_agents_integration_expand.up.sql` |
| `im_audit_records` | `audit_log` | `audit-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_automation_executions` | `automation_execution` | `automation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_client_sync_cursors` | `client_sync_cursor_authority` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_client_sync_events` | `client_sync_event_log` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_commit_journal` | `event_log` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_agent_assignments` | `conversation_agent_assignment` | `comms-conversation-service` | system of record | `database/migrations/postgres/0005_agents_integration_expand.up.sql` |
| `im_conversation_agent_binding` | `relation_entity` | `comms-conversation-service` | system of record | `database/migrations/postgres/0005_agents_integration_expand.up.sql` |
| `im_conversation_business_bindings` | `relation_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_handoffs` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_knowledge_space_link` | `relation_entity` | `comms-conversation-service` | owned relation / operational state | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_members` | `conversation_member_authority` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_messages` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_policies` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_read_cursors` | `read_cursor_authority` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversation_seq_counters` | `sequence_allocator` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_conversations` | `conversation_authority` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_group_knowledge_launch_tickets` | `operational_state` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_idempotency_keys` | `idempotency` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_inbox_events` | `inbox_event` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_message_media_refs` | `relation_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_notification_tasks` | `notification_task` | `notification-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_outbox_events` | `outbox_event` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_presence_states` | `presence_state` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_realtime_checkpoints` | `checkpoint` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_realtime_device_events` | `event_log` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_realtime_disconnect_fences` | `coordination_fence` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_realtime_subscription_scopes` | `fanout_index` | `session-gateway` | owned relation / operational state | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_realtime_subscriptions` | `subscription` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_registered_client_routes` | `realtime_route_authority` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_route_bindings` | `route_state` | `session-gateway` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_rtc_outbox_events` | `outbox_event` | `im-call-runtime` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_rtc_participant_credentials` | `credential_lifecycle` | `im-call-runtime` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_rtc_quality_reports` | `telemetry_log` | `im-call-runtime` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_rtc_sessions` | `rtc_session` | `im-call-runtime` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_rtc_signals` | `rtc_signal_log` | `im-call-runtime` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_stream_frames` | `stream_frame_log` | `streaming-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_stream_sessions` | `stream_session` | `streaming-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |

### messaging (4)

| Table | Profile | Write owner | Authority role | Migration / DDL source |
| --- | --- | --- | --- | --- |
| `im_message_pins` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_message_reactions` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_thread_subscriptions` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_threads` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |

### organization (8)

| Table | Profile | Write owner | Authority role | Migration / DDL source |
| --- | --- | --- | --- | --- |
| `im_ban_records` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_channel_access_rules` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_chat_channels` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_chat_groups` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_group_members` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_invitations` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_space_members` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_spaces` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |

### social (10)

| Table | Profile | Write owner | Authority role | Migration / DDL source |
| --- | --- | --- | --- | --- |
| `im_contact_preferences` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_contact_recommendations` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_contact_tags` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_direct_chats` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_external_connections` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_external_member_links` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_friend_requests` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_friendships` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_shared_channel_policies` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_user_blocks` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |

### user (3)

| Table | Profile | Write owner | Authority role | Migration / DDL source |
| --- | --- | --- | --- | --- |
| `im_conversation_settings` | `tenant_entity` | `comms-conversation-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_user_profiles` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |
| `im_user_settings` | `tenant_entity` | `social-service` | system of record | `database/ddl/baseline/postgres/0001_im_baseline.sql` |

## Contract Boundaries

Field definitions, indexes, constraints, retention, and migration ordering remain authoritative in
the registry-linked DDL and migration sources. The domain invariants are narrowed by
`specs/IM_DOMAIN_AND_PERSISTENCE_SPEC.md`; this generated file is a complete discovery inventory,
not a second schema definition.

## Regeneration And Verification

```bash
node docs/sites/scripts/generate-contract-inventories.mjs --write
node docs/sites/scripts/generate-contract-inventories.mjs --check
pnpm test:database-naming-standard
pnpm test:database-framework-standard
```
