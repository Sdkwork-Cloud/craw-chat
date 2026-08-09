-- sdkwork:migration
-- id: 0001_organization_id_not_null
-- engine: postgres
-- module: sdkwork-im
-- purpose: Enforce organization_id NOT NULL DEFAULT on all tables in the
--   consolidated baseline. NULL rows (pre-standard data anomalies) are
--   backfilled with the platform sentinel before NOT NULL is set, and
--   NOT NULL columns without an explicit default receive the sentinel
--   default, keeping existing deployments consistent with fresh baseline
--   installs.
-- reversible: false
-- rollback: forward-fix (sentinel backfill is the canonical fix; NULL
--   organization rows are data anomalies)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

UPDATE im_conversation_messages SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_messages ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_messages ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_seq_counters SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_seq_counters ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_seq_counters ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_message_media_refs SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_message_media_refs ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_message_media_refs ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_outbox_events SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_outbox_events ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_outbox_events ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_inbox_events SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_inbox_events ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_inbox_events ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_commit_journal SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_commit_journal ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_commit_journal ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_idempotency_keys SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_idempotency_keys ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_idempotency_keys ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_realtime_device_events SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_realtime_device_events ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_realtime_device_events ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_realtime_checkpoints SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_realtime_checkpoints ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_realtime_checkpoints ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_realtime_subscriptions SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_realtime_subscriptions ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_realtime_subscriptions ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_realtime_subscription_scopes SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_realtime_subscription_scopes ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_realtime_subscription_scopes ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_presence_states SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_presence_states ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_presence_states ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_route_bindings SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_route_bindings ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_route_bindings ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_realtime_disconnect_fences SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_realtime_disconnect_fences ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_realtime_disconnect_fences ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_rtc_sessions SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_rtc_sessions ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_rtc_sessions ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_rtc_signals SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_rtc_signals ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_rtc_signals ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_audit_records SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_audit_records ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_audit_records ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_notification_tasks SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_notification_tasks ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_notification_tasks ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_automation_executions SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_automation_executions ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_automation_executions ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversations SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversations ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversations ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_policies SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_policies ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_policies ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_business_bindings SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_business_bindings ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_business_bindings ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_handoffs SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_handoffs ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_handoffs ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_members SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_members ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_members ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_read_cursors SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_read_cursors ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_read_cursors ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_registered_client_routes SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_registered_client_routes ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_registered_client_routes ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_client_sync_events SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_client_sync_events ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_client_sync_events ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_client_sync_cursors SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_client_sync_cursors ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_client_sync_cursors ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_stream_sessions SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_stream_sessions ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_stream_sessions ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_stream_frames SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_stream_frames ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_stream_frames ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_friend_requests SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_friend_requests ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_friend_requests ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_friendships SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_friendships ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_friendships ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_user_blocks SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_user_blocks ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_user_blocks ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_direct_chats SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_direct_chats ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_direct_chats ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_external_connections SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_external_connections ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_external_connections ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_external_member_links SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_external_member_links ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_external_member_links ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_shared_channel_policies SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_shared_channel_policies ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_shared_channel_policies ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_spaces SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_spaces ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_spaces ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_contact_tags SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_contact_tags ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_contact_tags ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_contact_preferences SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_contact_preferences ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_contact_preferences ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_contact_recommendations SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_contact_recommendations ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_contact_recommendations ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_space_members SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_space_members ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_space_members ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_chat_groups SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_chat_groups ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_chat_groups ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_group_members SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_group_members ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_group_members ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_chat_channels SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_chat_channels ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_chat_channels ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_channel_access_rules SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_channel_access_rules ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_channel_access_rules ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_message_reactions SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_message_reactions ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_message_reactions ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_message_pins SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_message_pins ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_message_pins ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_threads SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_threads ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_threads ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_thread_subscriptions SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_thread_subscriptions ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_thread_subscriptions ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_user_profiles SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_user_profiles ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_user_profiles ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_user_settings SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_user_settings ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_user_settings ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_settings SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_settings ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_settings ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_invitations SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_invitations ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_invitations ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_ban_records SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_ban_records ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_ban_records ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_rtc_outbox_events SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_rtc_outbox_events ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_rtc_outbox_events ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_rtc_quality_reports SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_rtc_quality_reports ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_rtc_quality_reports ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_rtc_participant_credentials SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_rtc_participant_credentials ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_rtc_participant_credentials ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_knowledge_space_link SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_conversation_knowledge_space_link ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_conversation_knowledge_space_link ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_group_knowledge_launch_tickets SET organization_id = '0' WHERE organization_id IS NULL;
ALTER TABLE im_group_knowledge_launch_tickets ALTER COLUMN organization_id SET DEFAULT '0';
ALTER TABLE im_group_knowledge_launch_tickets ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_agent_assignments SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE im_conversation_agent_assignments ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE im_conversation_agent_assignments ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_conversation_agent_binding SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE im_conversation_agent_binding ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE im_conversation_agent_binding ALTER COLUMN organization_id SET NOT NULL;

UPDATE im_agent_dispatch SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE im_agent_dispatch ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE im_agent_dispatch ALTER COLUMN organization_id SET NOT NULL;

COMMIT;
