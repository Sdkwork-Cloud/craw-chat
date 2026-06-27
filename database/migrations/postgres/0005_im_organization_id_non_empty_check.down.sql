-- Remove organization_id non-empty CHECK constraints.

DO $$
DECLARE
    tbl TEXT;
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
        IF EXISTS (
            SELECT 1 FROM information_schema.table_constraints
            WHERE constraint_name = 'chk_' || tbl || '_org_id_non_empty'
        ) THEN
            EXECUTE format(
                'ALTER TABLE %I DROP CONSTRAINT chk_%I_org_id_non_empty',
                tbl,
                tbl
            );
        END IF;
    END LOOP;
END $$;
