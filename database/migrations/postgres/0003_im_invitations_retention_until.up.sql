-- sdkwork:migration
-- id: 0003_im_invitations_retention_until
-- engine: postgres
-- module: sdkwork-im
-- purpose: Add the retention_until expiry column (and its partial index) to
--   im_invitations so the retention purge scheduler can delete terminal
--   invitations (accepted/declined/expired/canceled) once their PII retention
--   window expires. The baseline DDL already declares the column for fresh
--   databases; this migration evolves databases provisioned before the
--   baseline update. Without it, the retention purge DELETE fails with
--   "column retention_until does not exist" on every tick.
-- reversible: false
-- rollback: forward-fix (additive column with no default; terminal-row purge only)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

ALTER TABLE im_invitations ADD COLUMN IF NOT EXISTS retention_until TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_im_invitations_retention_until
    ON im_invitations (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

COMMIT;
