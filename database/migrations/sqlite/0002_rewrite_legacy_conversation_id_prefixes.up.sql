-- Migration: Rewrite legacy conversation id prefixes to canonical form (SQLite).
--
-- See postgres/0002_rewrite_legacy_conversation_id_prefixes.up.sql for the full
-- background.  This SQLite variant performs the same prefix rewrite but uses
-- SQLite's REPLACE() and custom REGEXP function where available.  Because
-- SQLite does not support CREATE FUNCTION, the rewrite is expressed as
-- inline CASE expressions repeated per table.
--
-- The migration is wrapped in a transaction for atomicity.

BEGIN TRANSACTION;

-- Core conversation tables ----------------------------------------------------

UPDATE im_conversation_messages
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversation_seq_counters
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_message_media_refs
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_message_reactions
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_message_pins
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_threads
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

-- RTC sessions ----------------------------------------------------------------

UPDATE im_rtc_sessions
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

-- Projection tables -----------------------------------------------------------

UPDATE im_projection_timeline_entries
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversations
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversation_members
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversation_read_cursors
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_client_sync_events
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_projection_contacts
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_projection_direct_chat_bindings
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

-- Business association tables -------------------------------------------------

UPDATE im_direct_chats
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_chat_groups
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_chat_channels
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_shared_channel_policies
SET conversation_id =
    CASE
        WHEN conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(conversation_id, 9)
        WHEN conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(conversation_id, 8)
        ELSE conversation_id
    END
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_contact_recommendations
SET target_conversation_id =
    CASE
        WHEN target_conversation_id LIKE 'c_direct_%' THEN 'c_' || substr(target_conversation_id, 9)
        WHEN target_conversation_id LIKE 'c_agent_%'  THEN 'a_' || substr(target_conversation_id, 8)
        ELSE target_conversation_id
    END
WHERE target_conversation_id IS NOT NULL
  AND (target_conversation_id LIKE 'c_direct_%' OR target_conversation_id LIKE 'c_agent_%');

-- Event journal JSON payloads -------------------------------------------------
--
-- SQLite's built-in REPLACE() performs literal string replacement (no regex).
-- We apply two passes: one for c_direct_ -> c_ and one for c_agent_ -> a_.
-- This is safe because the prefixes are unique enough that accidental
-- matches in unrelated JSON text are extremely unlikely.

UPDATE im_outbox_events
SET payload_json = REPLACE(REPLACE(payload_json, 'c_direct_', 'c_'), 'c_agent_', 'a_')
WHERE payload_json LIKE '%c_direct_%' OR payload_json LIKE '%c_agent_%';

UPDATE im_inbox_events
SET payload_json = REPLACE(REPLACE(payload_json, 'c_direct_', 'c_'), 'c_agent_', 'a_')
WHERE payload_json LIKE '%c_direct_%' OR payload_json LIKE '%c_agent_%';

UPDATE im_commit_journal
SET payload_json = REPLACE(REPLACE(payload_json, 'c_direct_', 'c_'), 'c_agent_', 'a_')
WHERE payload_json LIKE '%c_direct_%' OR payload_json LIKE '%c_agent_%';

-- aggregate_id / partition_key / commit_offset in im_commit_journal -----------------

UPDATE im_commit_journal
SET aggregate_id =
    CASE
        WHEN aggregate_id LIKE 'c_direct_%' THEN 'c_' || substr(aggregate_id, 9)
        WHEN aggregate_id LIKE 'c_agent_%'  THEN 'a_' || substr(aggregate_id, 8)
        ELSE aggregate_id
    END
WHERE aggregate_id LIKE 'c_direct_%' OR aggregate_id LIKE 'c_agent_%';

UPDATE im_commit_journal
SET partition_key =
    CASE
        WHEN partition_key LIKE 'c_direct_%' THEN 'c_' || substr(partition_key, 9)
        WHEN partition_key LIKE 'c_agent_%'  THEN 'a_' || substr(partition_key, 8)
        ELSE partition_key
    END
WHERE partition_key LIKE 'c_direct_%' OR partition_key LIKE 'c_agent_%';

UPDATE im_commit_journal
SET commit_offset = REPLACE(REPLACE(commit_offset, 'c_direct_', 'c_'), 'c_agent_', 'a_')
WHERE commit_offset LIKE '%c_direct_%' OR commit_offset LIKE '%c_agent_%';

-- Idempotency keys ------------------------------------------------------------

UPDATE im_idempotency_keys
SET idempotency_key = REPLACE(REPLACE(idempotency_key, 'c_direct_', 'c_'), 'c_agent_', 'a_')
WHERE idempotency_key LIKE '%c_direct_%' OR idempotency_key LIKE '%c_agent_%';

COMMIT;
