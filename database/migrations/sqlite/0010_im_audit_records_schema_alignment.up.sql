-- ============================================================================
-- Audit Records Schema Alignment (SQLite)
-- ============================================================================
-- SQLite adaptation of postgres migration 0010.
-- Adds occurred_at, target_type, target_id, retention_class, integrity_anchor,
-- integrity_anchored_at columns.
-- ============================================================================

ALTER TABLE im_audit_records ADD COLUMN occurred_at TEXT;
ALTER TABLE im_audit_records ADD COLUMN target_type TEXT;
ALTER TABLE im_audit_records ADD COLUMN target_id TEXT;
ALTER TABLE im_audit_records ADD COLUMN retention_class TEXT NOT NULL DEFAULT 'access';
ALTER TABLE im_audit_records ADD COLUMN integrity_anchor TEXT;
ALTER TABLE im_audit_records ADD COLUMN integrity_anchored_at TEXT;

-- Backfill occurred_at from recorded_at
UPDATE im_audit_records
    SET occurred_at = recorded_at
    WHERE occurred_at IS NULL AND recorded_at IS NOT NULL;

UPDATE im_audit_records
    SET occurred_at = created_at
    WHERE occurred_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_audit_records_tenant_occurred
    ON im_audit_records (tenant_id, organization_id, occurred_at);

CREATE INDEX IF NOT EXISTS idx_audit_records_target
    ON im_audit_records (tenant_id, organization_id, target_type, target_id, occurred_at)
    WHERE target_type IS NOT NULL AND target_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_audit_records_actor
    ON im_audit_records (tenant_id, organization_id, actor_id, actor_kind, occurred_at);

CREATE INDEX IF NOT EXISTS idx_audit_records_retention_class
    ON im_audit_records (tenant_id, organization_id, retention_class, occurred_at);

CREATE INDEX IF NOT EXISTS idx_audit_records_integrity_anchor_pending
    ON im_audit_records (tenant_id, organization_id, audit_seq)
    WHERE integrity_anchor IS NULL;
