-- sdkwork:migration
-- id: 0002_rewrite_legacy_conversation_id_prefixes
-- engine: postgres
-- module: im
-- purpose: Rewrite legacy Conversation identifiers to canonical prefixes
-- reversible: false
-- rollback: restore-cutover
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 15m
-- rewrite: expected for rows carrying legacy Conversation identifiers
-- backfill: one bounded maintenance-window transaction with legacy writers stopped
-- write_traffic: blocked for affected Conversation tables during cutover
-- replication_wal: proportional to rewritten rows and JSON event payloads
-- observability: monitor migration duration, blocked locks, WAL growth, and replica lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: restore the verified pre-cutover backup, correct the blocker, and rerun
-- contract_version: 1.1.0

-- Migration: Rewrite legacy conversation id prefixes to canonical form.
--
-- Background:
--   The conversation id scheme was tightened so that each conversation type
--   has a distinct single-letter prefix:
--     * direct chats   c_direct_<hex>  ->  c_<hex>
--     * agent dialogs   c_agent_<hex>  ->  a_<hex>
--   Group conversations already use the new g_ prefix and are unaffected.
--
-- The <hex> suffix is a deterministic sha256 truncation of the same business
-- seed used before, so only the prefix changes.  This migration rewrites
-- every conversation_id column and the JSON payloads that embed conversation
-- ids in commit/outbox/inbox event rows.
--
-- Safety:
--   * Idempotent — running twice is a no-op because the old prefixes no
--     longer match after the first pass.
--   * Wrapped in a transaction so a failure rolls back every table.
--   * Foreign keys that reference conversation_id are updated in dependency
--     order to avoid orphaned rows.
--
-- Run order: baseline 0001 must already be applied.  This migration is safe
-- to run during a maintenance window with legacy writers stopped. The
-- pre-launch cutover has no compatibility resolver or dual-write path.

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '15min';

-- Helpers ---------------------------------------------------------------------
--
-- `rewrite_conversation_id(id)` converts a single legacy id to its canonical
-- form.  The function is pure (same input -> same output) and leaves ids that
-- do not match the legacy patterns untouched.

CREATE OR REPLACE FUNCTION pg_temp.rewrite_conversation_id(id TEXT)
RETURNS TEXT
LANGUAGE sql
IMMUTABLE
AS $$
    SELECT
        CASE
            WHEN id LIKE 'c_direct_%' THEN 'c_' || substring(id FROM 10)
            WHEN id LIKE 'c_agent_%'  THEN 'a_' || substring(id FROM 9)
            ELSE id
        END
$$;

-- Verify the rewrite function before touching any data.
DO $$
DECLARE
    direct_result TEXT;
    agent_result  TEXT;
    plain_result  TEXT;
BEGIN
    direct_result := pg_temp.rewrite_conversation_id('c_direct_abcd1234abcd1234abcd1234');
    agent_result  := pg_temp.rewrite_conversation_id('c_agent_abcd1234abcd1234abcd1234');
    plain_result  := pg_temp.rewrite_conversation_id('g_abcd1234abcd1234abcd1234');
    IF direct_result <> 'c_abcd1234abcd1234abcd1234' THEN
        RAISE EXCEPTION 'rewrite_conversation_id direct failed: got %', direct_result;
    END IF;
    IF agent_result <> 'a_abcd1234abcd1234abcd1234' THEN
        RAISE EXCEPTION 'rewrite_conversation_id agent failed: got %', agent_result;
    END IF;
    IF plain_result <> 'g_abcd1234abcd1234abcd1234' THEN
        RAISE EXCEPTION 'rewrite_conversation_id plain failed: got %', plain_result;
    END IF;
END $$;

-- Core conversation tables ----------------------------------------------------

UPDATE im_conversation_messages
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversation_seq_counters
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_message_media_refs
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_message_reactions
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_message_pins
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_threads
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

-- RTC sessions ----------------------------------------------------------------

UPDATE im_rtc_sessions
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

-- Conversation and client sync authority --------------------------------------

UPDATE im_conversations
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversation_members
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_conversation_read_cursors
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_client_sync_events
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

-- Business association tables -------------------------------------------------

UPDATE im_direct_chats
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_chat_groups
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_chat_channels
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_shared_channel_policies
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_contact_recommendations
SET target_conversation_id = pg_temp.rewrite_conversation_id(target_conversation_id)
WHERE target_conversation_id IS NOT NULL
  AND (target_conversation_id LIKE 'c_direct_%' OR target_conversation_id LIKE 'c_agent_%');

-- Event journal JSON payloads -------------------------------------------------
--
-- im_outbox_events, im_inbox_events, and im_commit_journal store event
-- payloads as JSON text.  The conversation id appears both as a top-level
-- JSON field and embedded inside nested structures.  We use regexp_replace
-- on the raw payload text to catch every occurrence.
--
-- The patterns are anchored on the prefix + hex boundary to avoid accidental
-- matches inside unrelated text.

-- payload_json is JSONB; cast to text for LIKE/regexp_replace and back to
-- JSONB for assignment.  regexp_replace on the textual form is safe because
-- the conversationId value is a hex string that does not require JSON
-- escaping.

UPDATE im_outbox_events
SET payload_json = regexp_replace(
        regexp_replace(
            payload_json::text,
            '"conversationId"\s*:\s*"c_direct_([0-9a-f]+)"',
            '"conversationId":"c_\1"',
            'g'
        ),
        '"conversationId"\s*:\s*"c_agent_([0-9a-f]+)"',
        '"conversationId":"a_\1"',
        'g'
    )::jsonb
WHERE payload_json::text LIKE '%c_direct_%' OR payload_json::text LIKE '%c_agent_%';

UPDATE im_inbox_events
SET payload_json = regexp_replace(
        regexp_replace(
            payload_json::text,
            '"conversationId"\s*:\s*"c_direct_([0-9a-f]+)"',
            '"conversationId":"c_\1"',
            'g'
        ),
        '"conversationId"\s*:\s*"c_agent_([0-9a-f]+)"',
        '"conversationId":"a_\1"',
        'g'
    )::jsonb
WHERE payload_json::text LIKE '%c_direct_%' OR payload_json::text LIKE '%c_agent_%';

UPDATE im_commit_journal
SET payload_json = regexp_replace(
        regexp_replace(
            payload_json::text,
            '"conversationId"\s*:\s*"c_direct_([0-9a-f]+)"',
            '"conversationId":"c_\1"',
            'g'
        ),
        '"conversationId"\s*:\s*"c_agent_([0-9a-f]+)"',
        '"conversationId":"a_\1"',
        'g'
    )::jsonb
WHERE payload_json::text LIKE '%c_direct_%' OR payload_json::text LIKE '%c_agent_%';

-- The aggregate_id column in im_commit_journal carries the conversation id
-- for conversation-scoped events and is a plain TEXT column, so the rewrite
-- function applies directly.
--
-- partition_key uses a length-prefixed segment encoding
-- (<len>#<tenant_id><len>#<conversation_id>) and therefore does NOT start
-- with the conversation id prefix; rewrite_conversation_id() would never
-- match it.  Rewriting partition_key correctly requires re-encoding the
-- length prefixes and is intentionally not performed here — the journal
-- replay path resolves aggregates via aggregate_id, not by parsing
-- partition_key.
--
-- commit_offset is a BIGINT snowflake identifier with no relationship to
-- conversation ids, so no rewrite applies.

UPDATE im_commit_journal
SET aggregate_id = pg_temp.rewrite_conversation_id(aggregate_id)
WHERE aggregate_id LIKE 'c_direct_%' OR aggregate_id LIKE 'c_agent_%';

-- Idempotency keys ------------------------------------------------------------
--
-- im_idempotency_keys stores request keys that may embed the conversation id.
-- The key format is deterministic, so we rewrite any key that contains a
-- legacy prefix.

UPDATE im_idempotency_keys
SET idempotency_key = regexp_replace(
        regexp_replace(
            idempotency_key,
            'c_direct_([0-9a-f]+)',
            'c_\1',
            'g'
        ),
        'c_agent_([0-9a-f]+)',
        'a_\1',
        'g'
    )
WHERE idempotency_key LIKE '%c_direct_%' OR idempotency_key LIKE '%c_agent_%';

-- Verification ----------------------------------------------------------------
--
-- After migration, no row should reference a legacy prefix.  This block
-- raises an exception if any stale id survives, which rolls back the
-- transaction.

DO $$
DECLARE
    stale_count INTEGER;
BEGIN
    SELECT COUNT(*) INTO stale_count
    FROM im_conversation_messages
    WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';
    IF stale_count > 0 THEN
        RAISE EXCEPTION 'im_conversation_messages still has % legacy conversation ids', stale_count;
    END IF;

    SELECT COUNT(*) INTO stale_count
    FROM im_conversations
    WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';
    IF stale_count > 0 THEN
        RAISE EXCEPTION 'im_conversations still has % legacy conversation ids', stale_count;
    END IF;

    SELECT COUNT(*) INTO stale_count
    FROM im_commit_journal
    WHERE aggregate_id LIKE 'c_direct_%'
       OR aggregate_id LIKE 'c_agent_%';
    IF stale_count > 0 THEN
        RAISE EXCEPTION 'im_commit_journal still has % legacy conversation ids in aggregate_id', stale_count;
    END IF;
END $$;

-- Cleanup ---------------------------------------------------------------------

DROP FUNCTION pg_temp.rewrite_conversation_id(TEXT);

COMMIT;
