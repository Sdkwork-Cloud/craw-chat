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
-- to run on a live database during a maintenance window; concurrent writers
-- that still emit legacy ids will be handled by the backwards-compatible
-- resolver functions in projection-service until the next deploy.

BEGIN;

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
            WHEN id LIKE 'c_direct_%' THEN 'c_' || substring(id FROM 9)
            WHEN id LIKE 'c_agent_%'  THEN 'a_' || substring(id FROM 8)
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

-- Projection tables -----------------------------------------------------------

UPDATE im_projection_timeline_entries
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_projection_conversation_summaries
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_projection_conversation_members
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_projection_read_cursors
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

UPDATE im_projection_client_route_sync_feeds
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_projection_contacts
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id IS NOT NULL
  AND (conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%');

UPDATE im_projection_direct_chat_bindings
SET conversation_id = pg_temp.rewrite_conversation_id(conversation_id)
WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';

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

UPDATE im_outbox_events
SET payload_json = regexp_replace(
        regexp_replace(
            payload_json,
            '"conversationId"\s*:\s*"c_direct_([0-9a-f]+)"',
            '"conversationId":"c_\1"',
            'g'
        ),
        '"conversationId"\s*:\s*"c_agent_([0-9a-f]+)"',
        '"conversationId":"a_\1"',
        'g'
    )
WHERE payload_json LIKE '%c_direct_%' OR payload_json LIKE '%c_agent_%';

UPDATE im_inbox_events
SET payload_json = regexp_replace(
        regexp_replace(
            payload_json,
            '"conversationId"\s*:\s*"c_direct_([0-9a-f]+)"',
            '"conversationId":"c_\1"',
            'g'
        ),
        '"conversationId"\s*:\s*"c_agent_([0-9a-f]+)"',
        '"conversationId":"a_\1"',
        'g'
    )
WHERE payload_json LIKE '%c_direct_%' OR payload_json LIKE '%c_agent_%';

UPDATE im_commit_journal
SET payload_json = regexp_replace(
        regexp_replace(
            payload_json,
            '"conversationId"\s*:\s*"c_direct_([0-9a-f]+)"',
            '"conversationId":"c_\1"',
            'g'
        ),
        '"conversationId"\s*:\s*"c_agent_([0-9a-f]+)"',
        '"conversationId":"a_\1"',
        'g'
    )
WHERE payload_json LIKE '%c_direct_%' OR payload_json LIKE '%c_agent_%';

-- The aggregate_id and partition_key columns in im_commit_journal also carry the
-- conversation id for conversation-scoped events.

UPDATE im_commit_journal
SET aggregate_id = pg_temp.rewrite_conversation_id(aggregate_id)
WHERE aggregate_id LIKE 'c_direct_%' OR aggregate_id LIKE 'c_agent_%';

UPDATE im_commit_journal
SET partition_key = pg_temp.rewrite_conversation_id(partition_key)
WHERE partition_key LIKE 'c_direct_%' OR partition_key LIKE 'c_agent_%';

-- The commit_offset column uses a composite format
-- (tenant_id#conversation_id) but the conversation id portion may still
-- contain legacy prefixes.  Rewrite the conversation id segment only.

UPDATE im_commit_journal
SET commit_offset = regexp_replace(
        regexp_replace(
            commit_offset,
            '#c_direct_([0-9a-f]+)',
            '#c_\1',
            'g'
        ),
        '#c_agent_([0-9a-f]+)',
        '#a_\1',
        'g'
    )
WHERE commit_offset LIKE '%#c_direct_%' OR commit_offset LIKE '%#c_agent_%';

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
    FROM im_projection_conversation_summaries
    WHERE conversation_id LIKE 'c_direct_%' OR conversation_id LIKE 'c_agent_%';
    IF stale_count > 0 THEN
        RAISE EXCEPTION 'im_projection_conversation_summaries still has % legacy conversation ids', stale_count;
    END IF;

    SELECT COUNT(*) INTO stale_count
    FROM im_commit_journal
    WHERE aggregate_id LIKE 'c_direct_%'
       OR aggregate_id LIKE 'c_agent_%'
       OR partition_key LIKE 'c_direct_%'
       OR partition_key LIKE 'c_agent_%';
    IF stale_count > 0 THEN
        RAISE EXCEPTION 'im_commit_journal still has % legacy conversation ids in aggregate/scope', stale_count;
    END IF;
END $$;

-- Cleanup ---------------------------------------------------------------------

DROP FUNCTION pg_temp.rewrite_conversation_id(TEXT);

COMMIT;
