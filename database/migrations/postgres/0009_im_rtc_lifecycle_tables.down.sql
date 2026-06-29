-- Rollback: RTC lifecycle tables

DROP TABLE IF EXISTS im_rtc_participant_credentials CASCADE;
DROP TABLE IF EXISTS im_rtc_quality_reports CASCADE;
DROP TABLE IF EXISTS im_rtc_outbox_events CASCADE;
