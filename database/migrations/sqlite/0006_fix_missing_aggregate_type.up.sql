-- Fix for missing columns that should exist per baseline schema.
-- This migration ensures critical columns exist even if the database was bootstrapped
-- before the baseline rebuild or with incomplete migrations.

-- Add aggregate_type to im_commit_journal (already exists, but idempotent check)
ALTER TABLE im_commit_journal
    ADD COLUMN IF NOT EXISTS aggregate_type TEXT NOT NULL DEFAULT 'conversation';

-- Add aggregate_id to im_commit_journal (already exists, but idempotent check)
ALTER TABLE im_commit_journal
    ADD COLUMN IF NOT EXISTS aggregate_id TEXT NOT NULL DEFAULT '';

-- Add aggregate_type to im_audit_records (missing in current schema)
ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS aggregate_type TEXT NOT NULL DEFAULT 'unknown';

-- Add aggregate_id to im_audit_records (missing in current schema)
ALTER TABLE im_audit_records
    ADD COLUMN IF NOT EXISTS aggregate_id TEXT NOT NULL DEFAULT '';
