-- sdkwork:migration
-- id: 0004_im_conversation_messages_search_vector
-- engine: postgres
-- module: sdkwork-im
-- purpose: Repair IM databases whose baseline created im_conversation_messages
--   without the search_vector column required by the PostgreSQL SearchProvider.
-- reversible: false
-- rollback: forward-fix (the additive nullable column and derived index preserve source data)
-- transactional: true
-- lock: access-exclusive for the metadata-only column add and bounded GIN index build
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: the nullable column add has no default and is metadata-only
-- backfill: idempotent null-only update; reruns skip already indexed rows
-- write_traffic: the trigger indexes inserts and updates while the backfill advances
-- replication_wal: bounded to rows whose search_vector is still null
-- observability: monitor null search_vector count and concurrent index validity
-- cancellation: PostgreSQL rolls back the complete migration transaction
-- recovery: apply a reviewed forward fix and retry this migration
-- idempotency: column, function, trigger, backfill predicate, and index are safe to repeat

SET lock_timeout = '5s';
SET statement_timeout = '2min';

ALTER TABLE im_conversation_messages
    ADD COLUMN IF NOT EXISTS search_vector TSVECTOR;

CREATE OR REPLACE FUNCTION im_messages_search_trigger() RETURNS trigger AS $$
DECLARE
    raw_text TEXT;
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

    NEW.search_vector := to_tsvector('simple', raw_text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS im_messages_search_update ON im_conversation_messages;
CREATE TRIGGER im_messages_search_update
    BEFORE INSERT OR UPDATE ON im_conversation_messages
    FOR EACH ROW EXECUTE FUNCTION im_messages_search_trigger();

UPDATE im_conversation_messages
SET search_vector = search_vector
WHERE search_vector IS NULL;

CREATE INDEX IF NOT EXISTS idx_im_messages_search_vector
    ON im_conversation_messages USING GIN (search_vector);

RESET statement_timeout;
RESET lock_timeout;
