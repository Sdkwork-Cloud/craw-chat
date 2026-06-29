-- ============================================================================
-- Audit Records Schema Alignment + WORM Protection
-- ============================================================================
-- Fixes schema drift between baseline (im_audit_records has recorded_at TEXT,
-- no occurred_at/target_type/target_id) and migration 0007 which references
-- those missing columns. Also upgrades audit to L3 compliance:
--   - Adds occurred_at TIMESTAMPTZ as the authoritative timestamp
--   - Adds target_type/target_id for BOLA-relevant audit scoping
--   - Adds integrity_anchor for external notary anchoring (WORM-like)
--   - Adds retention_class for differentiated retention (security/access/etc)
-- ============================================================================

-- 1. Add missing columns referenced by migration 0007 and audit-service
ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS occurred_at TIMESTAMPTZ;

ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS target_type TEXT;

ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS target_id TEXT;

-- 2. Add L3 compliance columns
ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS retention_class TEXT NOT NULL DEFAULT 'access';

ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS integrity_anchor TEXT;

ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS integrity_anchored_at TIMESTAMPTZ;

-- 3. Backfill occurred_at from recorded_at (which stores RFC3339 strings)
UPDATE im_audit_records
    SET occurred_at = recorded_at::TIMESTAMPTZ
    WHERE occurred_at IS NULL AND recorded_at IS NOT NULL AND recorded_at ~ '^\d{4}-\d{2}-\d{2}T';

-- 4. For rows where backfill failed (malformed recorded_at), use created_at
UPDATE im_audit_records
    SET occurred_at = created_at
    WHERE occurred_at IS NULL;

-- 5. Make occurred_at NOT NULL now that it is backfilled
ALTER TABLE im_audit_records
    ALTER COLUMN occurred_at SET NOT NULL;

-- 6. Add CHECK constraint for retention_class values
ALTER TABLE im_audit_records
    DROP CONSTRAINT IF EXISTS chk_im_audit_records_retention_class;

ALTER TABLE im_audit_records
    ADD CONSTRAINT chk_im_audit_records_retention_class CHECK (retention_class IN (
        'security',      -- security events: login, permission denied, cross-tenant attempts
        'access',        -- access events: data read, API calls
        'admin',         -- admin operations: config changes, user management
        'data_lifecycle' -- data events: export, delete, retention purge
    ));

-- 7. Index the new columns (migration 0007 already created some, use IF NOT EXISTS)
CREATE INDEX IF NOT EXISTS idx_audit_records_tenant_occurred
    ON im_audit_records (tenant_id, organization_id, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_records_target
    ON im_audit_records (tenant_id, organization_id, target_type, target_id, occurred_at DESC)
    WHERE target_type IS NOT NULL AND target_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_audit_records_actor
    ON im_audit_records (tenant_id, organization_id, actor_id, actor_kind, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_records_retention_class
    ON im_audit_records (tenant_id, organization_id, retention_class, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_records_integrity_anchor_pending
    ON im_audit_records (tenant_id, organization_id, audit_seq)
    WHERE integrity_anchor IS NULL;

-- 8. Comment
COMMENT ON TABLE im_audit_records IS
    'L3 compliance audit log. occurred_at is authoritative timestamp. integrity_anchor links to external notary for WORM-like tamper evidence. retention_class drives differentiated retention (security=2y, access=180d, admin=1y, data_lifecycle=3y).';
COMMENT ON COLUMN im_audit_records.occurred_at IS
    'Authoritative event timestamp (TIMESTAMPTZ). Backfilled from recorded_at; new rows MUST set this.';
COMMENT ON COLUMN im_audit_records.integrity_anchor IS
    'External notary anchor (e.g., hash written to object storage WORM bucket or blockchain). NULL until anchored.';
COMMENT ON COLUMN im_audit_records.retention_class IS
    'security|access|admin|data_lifecycle. Drives retention period and purge schedule.';

-- NOTE: WORM enforcement at the database role level is done via a separate
-- DDL script that creates a dedicated `im_audit_writer` role with INSERT/SELECT
-- only (no UPDATE/DELETE/TRUNCATE). See deployments/database/postgres/roles.sql.
-- This migration only adds the schema; role-based enforcement is deployment-time.
