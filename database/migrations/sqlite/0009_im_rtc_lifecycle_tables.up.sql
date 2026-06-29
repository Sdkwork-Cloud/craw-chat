-- ============================================================================
-- RTC Lifecycle Tables (SQLite)
-- ============================================================================
-- SQLite adaptation of postgres migration 0009.
-- SQLite does not have DOUBLE PRECISION; use REAL.
-- SQLite does not enforce CHECK constraints on NULL unless explicitly written.
-- ============================================================================

-- RTC Outbox Events
CREATE TABLE IF NOT EXISTS im_rtc_outbox_events (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    outbox_id           TEXT NOT NULL,
    rtc_session_id      TEXT NOT NULL,
    event_id            TEXT NOT NULL,
    event_type          TEXT NOT NULL,
    actor_principal_kind TEXT NOT NULL,
    actor_principal_id  TEXT NOT NULL,
    payload_json        TEXT NOT NULL,
    payload_hash        TEXT NOT NULL,
    publish_status      TEXT NOT NULL DEFAULT 'pending',
    attempt_count       INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at        TEXT NOT NULL,
    published_at        TEXT,
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL,
    retention_until     TEXT,
    CONSTRAINT pk_im_rtc_outbox_events PRIMARY KEY (tenant_id, organization_id, outbox_id),
    CONSTRAINT uk_im_rtc_outbox_events_event UNIQUE (tenant_id, organization_id, event_id),
    CONSTRAINT chk_im_rtc_outbox_events_status CHECK (publish_status IN ('pending', 'published', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_status_available
    ON im_rtc_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_session
    ON im_rtc_outbox_events (tenant_id, organization_id, rtc_session_id, created_at);

-- RTC Quality Reports
CREATE TABLE IF NOT EXISTS im_rtc_quality_reports (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    report_id               TEXT NOT NULL,
    rtc_session_id          TEXT NOT NULL,
    participant_principal_kind TEXT NOT NULL,
    participant_principal_id   TEXT NOT NULL,
    participant_device_id     TEXT NOT NULL,
    reported_at             TEXT NOT NULL,
    mos_score               REAL,
    rtt_ms                  REAL,
    jitter_ms               REAL,
    packet_loss_rate        REAL,
    packets_sent            INTEGER,
    packets_received        INTEGER,
    packets_lost            INTEGER,
    bytes_sent              INTEGER,
    bytes_received          INTEGER,
    audio_bitrate_kbps      INTEGER,
    video_bitrate_kbps      INTEGER,
    audio_codec             TEXT,
    video_codec             TEXT,
    resolution_width        INTEGER,
    resolution_height       INTEGER,
    frame_rate_fps          REAL,
    quality_grade           TEXT,
    payload_json            TEXT,
    payload_hash            TEXT,
    created_at              TEXT NOT NULL,
    retention_until         TEXT,
    CONSTRAINT pk_im_rtc_quality_reports PRIMARY KEY (tenant_id, organization_id, report_id)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_session_time
    ON im_rtc_quality_reports (tenant_id, organization_id, rtc_session_id, reported_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_participant
    ON im_rtc_quality_reports (tenant_id, organization_id, participant_principal_kind, participant_principal_id, reported_at);

-- RTC Participant Credentials
CREATE TABLE IF NOT EXISTS im_rtc_participant_credentials (
    tenant_id                   TEXT NOT NULL,
    organization_id             TEXT NOT NULL DEFAULT '0',
    credential_id               TEXT NOT NULL,
    rtc_session_id              TEXT NOT NULL,
    participant_principal_kind  TEXT NOT NULL,
    participant_principal_id    TEXT NOT NULL,
    participant_device_id       TEXT,
    provider_plugin_id          TEXT NOT NULL,
    provider_token_id           TEXT,
    credential_state            TEXT NOT NULL DEFAULT 'active',
    issued_at                   TEXT NOT NULL,
    expires_at                  TEXT NOT NULL,
    rotated_from_credential_id  TEXT,
    rotated_at                  TEXT,
    revoked_at                  TEXT,
    revoked_reason              TEXT,
    revoked_by_principal_kind   TEXT,
    revoked_by_principal_id     TEXT,
    credential_payload_json     TEXT NOT NULL,
    credential_payload_hash     TEXT NOT NULL,
    created_at                  TEXT NOT NULL,
    updated_at                  TEXT NOT NULL,
    retention_until             TEXT,
    CONSTRAINT pk_im_rtc_participant_credentials PRIMARY KEY (tenant_id, organization_id, credential_id)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_session
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, participant_principal_kind, participant_principal_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_active
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, credential_state, expires_at)
    WHERE credential_state = 'active';

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_expiry
    ON im_rtc_participant_credentials (tenant_id, organization_id, expires_at)
    WHERE credential_state = 'active';
