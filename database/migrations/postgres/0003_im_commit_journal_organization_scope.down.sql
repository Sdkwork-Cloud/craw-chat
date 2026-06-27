DROP INDEX IF EXISTS idx_im_commit_journal_retention_until;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_retention_until
    ON im_commit_journal (retention_until)
    WHERE retention_until IS NOT NULL;

DROP INDEX IF EXISTS idx_im_commit_journal_tenant_occurred;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, occurred_at, event_id);

DROP INDEX IF EXISTS idx_im_commit_journal_tenant_aggregate_seq;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, aggregate_type, aggregate_id, aggregate_seq);

ALTER TABLE im_commit_journal
    DROP COLUMN IF EXISTS organization_id;
