-- Rollback: audit records schema alignment

DROP INDEX IF EXISTS idx_audit_records_integrity_anchor_pending;
DROP INDEX IF EXISTS idx_audit_records_retention_class;
DROP INDEX IF EXISTS idx_audit_records_actor;
DROP INDEX IF EXISTS idx_audit_records_target;
DROP INDEX IF EXISTS idx_audit_records_tenant_occurred;

ALTER TABLE im_audit_records
    DROP CONSTRAINT IF EXISTS chk_im_audit_records_retention_class;

ALTER TABLE im_audit_records
    ALTER COLUMN occurred_at DROP NOT NULL;

ALTER TABLE im_audit_records
    DROP COLUMN IF EXISTS integrity_anchored_at;

ALTER TABLE im_audit_records
    DROP COLUMN IF EXISTS integrity_anchor;

ALTER TABLE im_audit_records
    DROP COLUMN IF EXISTS retention_class;

ALTER TABLE im_audit_records
    DROP COLUMN IF EXISTS target_id;

ALTER TABLE im_audit_records
    DROP COLUMN IF EXISTS target_type;

ALTER TABLE im_audit_records
    DROP COLUMN IF EXISTS occurred_at;
