-- sdkwork:deployment
-- id: im_audit_writer_role
-- engine: postgres
-- module: sdkwork-im
-- purpose: WORM-like tamper enforcement for the L3 compliance audit log.
--   Creates a dedicated `im_audit_writer` role with INSERT/SELECT only so a
--   compromised writer credential can never UPDATE, DELETE, or TRUNCATE
--   `im_audit_records`. Apply at deployment time after the IM baseline
--   (see database/ddl/baseline/postgres/0001_im_baseline.sql).
-- reversible: true (DROP OWNED BY + DROP ROLE)
-- transactional: true

BEGIN;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'im_audit_writer') THEN
        CREATE ROLE im_audit_writer NOLOGIN;
    END IF;
END
$$;

-- Grant schema usage and the minimal table privileges. The writer can append
-- and read audit evidence but can never mutate or destroy it.
GRANT USAGE ON SCHEMA public TO im_audit_writer;
GRANT SELECT, INSERT ON TABLE im_audit_records TO im_audit_writer;
GRANT SELECT, INSERT ON TABLE im_commit_journal TO im_audit_writer;

-- Revoke any broader default privileges the role might inherit.
REVOKE UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER ON TABLE im_audit_records FROM im_audit_writer;
REVOKE UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER ON TABLE im_commit_journal FROM im_audit_writer;

COMMIT;
