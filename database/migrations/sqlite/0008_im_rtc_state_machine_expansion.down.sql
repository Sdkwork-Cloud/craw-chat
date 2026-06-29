-- Rollback: SQLite does not support DROP COLUMN before 3.35.0.
-- This rollback is a no-op; columns remain. Use baseline rebuild for full reset.
SELECT 1;
