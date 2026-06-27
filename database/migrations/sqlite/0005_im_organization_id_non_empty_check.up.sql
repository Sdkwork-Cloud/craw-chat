-- Enforce organization_id cannot be empty string on all organization-scoped IM tables.
-- SQLite-compatible version: SQLite does not support adding CHECK constraints via
-- ALTER TABLE, so we use BEFORE INSERT/UPDATE triggers that reject empty organization_id.
--
-- Each trigger raises an ABORT if organization_id is empty string.

-- Note: SQLite triggers fire per-row, so we create one trigger pair per table.
-- Only tables that exist at migration time will have triggers created.

CREATE TRIGGER IF NOT EXISTS chk_im_commit_journal_org_id_non_empty_ins
    BEFORE INSERT ON im_commit_journal
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_commit_journal_org_id_non_empty_upd
    BEFORE UPDATE ON im_commit_journal
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_outbox_events_org_id_non_empty_ins
    BEFORE INSERT ON im_outbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_outbox_events_org_id_non_empty_upd
    BEFORE UPDATE ON im_outbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_inbox_events_org_id_non_empty_ins
    BEFORE INSERT ON im_inbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_inbox_events_org_id_non_empty_upd
    BEFORE UPDATE ON im_inbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_conversation_messages_org_id_non_empty_ins
    BEFORE INSERT ON im_conversation_messages
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_conversation_messages_org_id_non_empty_upd
    BEFORE UPDATE ON im_conversation_messages
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_idempotency_keys_org_id_non_empty_ins
    BEFORE INSERT ON im_idempotency_keys
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_idempotency_keys_org_id_non_empty_upd
    BEFORE UPDATE ON im_idempotency_keys
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_presence_states_org_id_non_empty_ins
    BEFORE INSERT ON im_presence_states
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_presence_states_org_id_non_empty_upd
    BEFORE UPDATE ON im_presence_states
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_sessions_org_id_non_empty_ins
    BEFORE INSERT ON im_stream_sessions
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_sessions_org_id_non_empty_upd
    BEFORE UPDATE ON im_stream_sessions
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_frames_org_id_non_empty_ins
    BEFORE INSERT ON im_stream_frames
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_frames_org_id_non_empty_upd
    BEFORE UPDATE ON im_stream_frames
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;
