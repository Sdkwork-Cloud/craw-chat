# PostgreSQL baseline DDL

Optional full baseline snapshots when `baselineStrategy` is not `migrations-only`.

`0001_im_legacy_baseline.sql` is idempotent for re-bootstrap on existing databases:
`CREATE TABLE IF NOT EXISTS`, `DROP TABLE IF EXISTS` for legacy rewrites, and
`CREATE INDEX IF NOT EXISTS` for all indexes.
