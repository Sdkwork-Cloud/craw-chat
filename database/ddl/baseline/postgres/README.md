# PostgreSQL baseline DDL

Immutable bootstrap base used with ordered PostgreSQL migrations under the
`baseline-plus-migrations` lifecycle strategy. The baseline is not the complete
active table inventory by itself.

`0001_im_baseline.sql` is idempotent for re-bootstrap on existing databases:
`CREATE TABLE IF NOT EXISTS`, `DROP TABLE IF EXISTS` for legacy rewrites, and
`CREATE INDEX IF NOT EXISTS` for all indexes.
