-- sdkwork:migration
-- id: 0002_im_notification_tasks_worker_columns
-- engine: postgres
-- module: sdkwork-im
-- purpose: Add delivery-worker metadata columns to im_notification_tasks so
--   the notification-service dispatch worker can claim tasks with a lease
--   (FOR UPDATE SKIP LOCKED), retry failed deliveries with exponential
--   backoff, and dead-letter tasks after the attempt cap. Additive columns
--   with defaults; existing rows remain immediately claimable.
-- reversible: false
-- rollback: forward-fix (columns are additive and worker-internal)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

ALTER TABLE im_notification_tasks ADD COLUMN IF NOT EXISTS attempt_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE im_notification_tasks ADD COLUMN IF NOT EXISTS available_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_claim
    ON im_notification_tasks (notification_status, available_at)
    WHERE notification_status = 'requested';

COMMIT;
