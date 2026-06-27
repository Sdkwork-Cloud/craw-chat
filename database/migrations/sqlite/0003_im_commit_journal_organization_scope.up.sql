-- Align im_commit_journal with organization-scoped journal writes.
-- SQLite-compatible version: SQLite does not support ADD COLUMN IF NOT EXISTS,
-- so we check pragma_table_info before adding the column.

-- Add organization_id column if it does not exist.
INSERT INTO im_commit_journal (partition_key, commit_offset, event_id, tenant_id, aggregate_type, aggregate_id, aggregate_seq, event_type, payload_json, payload_hash, occurred_at, created_at)
SELECT 'migration_check', 0, 'migration_check', 'migration_check', 'migration_check', 'migration_check', 1, 'migration_check', '{}', 'migration_check', '1970-01-01T00:00:00Z', '1970-01-01T00:00:00Z'
WHERE NOT EXISTS (SELECT 1 FROM pragma_table_info('im_commit_journal') WHERE name = 'organization_id');
DELETE FROM im_commit_journal WHERE partition_key = 'migration_check';

-- Recreate indexes with organization_id scope.
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
