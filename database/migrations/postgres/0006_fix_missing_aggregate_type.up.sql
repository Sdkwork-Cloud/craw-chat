-- Fix for missing aggregate_type column in im_commit_journal.
-- Databases bootstrapped before the baseline rebuild may lack this column.

ALTER TABLE im_commit_journal
    ADD COLUMN IF NOT EXISTS aggregate_type TEXT NOT NULL DEFAULT 'conversation';

COMMENT ON COLUMN im_commit_journal.aggregate_type IS 'Aggregate type (e.g., conversation, friendship)';
