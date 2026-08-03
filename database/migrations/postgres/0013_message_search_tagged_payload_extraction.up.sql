-- sdkwork:migration
-- id: 0013_message_search_tagged_payload_extraction
-- engine: postgres
-- module: im
-- purpose: Extract message search text from the tagged ContentPart payload instead of the legacy flat text fields
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: function body replacement only; no column or index changes
-- backfill: existing rows are re-vectorized through the trigger by touching search_vector
-- write_traffic: message writes continue; the trigger recomputes search_vector on every insert/update
-- replication_wal: bounded to the row updates of the backfill
-- observability: monitor trigger creation and backfill duration
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: correct through a forward migration and restore only from verified normalized backups
-- idempotency: the trigger is recreated with IF-missing semantics via DROP/CREATE pairs inside the
--   transaction, so fresh installs bootstrap as a no-op and legacy databases transform in place

BEGIN;

-- The message payload contract stores text in tagged ContentPart entries
-- (`{"parts": [{"kind": "text", "text": "..."}]}`) plus a top-level summary;
-- the legacy trigger only read flat top-level text fields and produced empty
-- search vectors for every typed message.
CREATE OR REPLACE FUNCTION im_messages_search_trigger() RETURNS trigger AS $$
DECLARE
    raw_text text;
BEGIN
    raw_text := COALESCE(NEW.payload_json->>'summary', '') || ' ' ||
                COALESCE(NEW.payload_json->>'text', '') || ' ' ||
                COALESCE(NEW.payload_json->>'caption', '') || ' ' ||
                COALESCE(NEW.payload_json->>'description', '') || ' ' ||
                COALESCE(ARRAY_TO_STRING(ARRAY(
                    SELECT part->>'text'
                    FROM jsonb_array_elements(NEW.payload_json->'parts') AS part
                    WHERE part->>'kind' = 'text'
                ), ' '), '');

    -- Use zhparser if available, otherwise fall back to simple
    -- (zhparser must be installed and 'chinese_zh' config created)
    BEGIN
        NEW.search_vector := to_tsvector('chinese_zh', raw_text);
    EXCEPTION WHEN OTHERS THEN
        -- Fallback: simple config (no CJK segmentation, but works for ASCII)
        NEW.search_vector := to_tsvector('simple', raw_text);
    END;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS im_messages_search_update ON im_conversation_messages;
CREATE TRIGGER im_messages_search_update
    BEFORE INSERT OR UPDATE ON im_conversation_messages
    FOR EACH ROW EXECUTE FUNCTION im_messages_search_trigger();

-- Re-vectorize existing rows through the trigger.
UPDATE im_conversation_messages SET search_vector = search_vector;

COMMIT;
