-- Enforce organization_id cannot be empty string on all organization-scoped IM tables.
-- The schema baseline already declares NOT NULL DEFAULT '0'; this migration adds
-- a CHECK constraint to reject empty-string organization values that would bypass
-- multi-tenant isolation at the data layer.
--
-- Idempotent: checks pg_constraint before adding to support safe re-execution.

DO $$
DECLARE
    tbl TEXT;
    constraint_name TEXT;
    org_scoped_tables TEXT[] := ARRAY[
        'im_commit_journal',
        'im_outbox_events',
        'im_inbox_events',
        'im_conversation_messages',
        'im_conversation_seq_counters',
        'im_message_media_refs'
    ];
BEGIN
    FOREACH tbl IN ARRAY org_scoped_tables LOOP
        constraint_name := format('chk_%s_org_id_non_empty', tbl);
        IF EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_name = tbl
        ) AND NOT EXISTS (
            SELECT 1 FROM pg_constraint
            WHERE conname = constraint_name
        ) THEN
            EXECUTE format(
                'ALTER TABLE %I ADD CONSTRAINT %I CHECK (organization_id <> %L)',
                tbl,
                constraint_name,
                ''
            );
        END IF;
    END LOOP;
END $$;
