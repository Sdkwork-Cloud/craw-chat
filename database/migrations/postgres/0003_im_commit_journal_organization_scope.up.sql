-- Align im_commit_journal with organization-scoped journal writes.
-- Existing databases may have been bootstrapped before the baseline rebuild section
-- that re-created this table with organization_id.

ALTER TABLE im_commit_journal
    ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '0';

DROP INDEX IF EXISTS idx_im_commit_journal_tenant_aggregate_seq;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, organization_id, aggregate_type, aggregate_id, aggregate_seq);

DROP INDEX IF EXISTS idx_im_commit_journal_tenant_occurred;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, organization_id, occurred_at, event_id);

DROP INDEX IF EXISTS idx_im_commit_journal_retention_until;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_retention_until
    ON im_commit_journal (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;
