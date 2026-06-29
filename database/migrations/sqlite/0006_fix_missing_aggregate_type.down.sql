-- Down migration for aggregate_type fix.
-- This is a no-op since we don't want to remove the aggregate_type column
-- as it's required by the application.

-- No action: aggregate_type column should remain in the table
SELECT 1;