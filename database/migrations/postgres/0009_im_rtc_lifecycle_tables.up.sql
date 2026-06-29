-- ============================================================================
-- RTC Lifecycle Tables: outbox events, quality reports, participant credentials
-- ============================================================================
-- Adds three tables required for production-grade RTC:
--   1. im_rtc_outbox_events         - outbox pattern for cross-service fanout
--   2. im_rtc_quality_reports       - per-participant media quality telemetry
--   3. im_rtc_participant_credentials - credential TTL/rotation/revocation
--
-- Aligns with Discord/Teams/Zoom call telemetry and LiveKit/Agora credential
-- lifecycle management.
-- ============================================================================

-- ============================================================================
-- 1. RTC Outbox Events
-- ============================================================================
-- Decouples RTC state mutations from downstream consumers (notifications,
-- audit, analytics, recording). Dispatched by a relay worker via
-- FOR UPDATE SKIP LOCKED, mirroring the im_outbox_events pattern.

CREATE TABLE IF NOT EXISTS im_rtc_outbox_events (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    outbox_id           TEXT NOT NULL,
    rtc_session_id      TEXT NOT NULL,
    event_id            TEXT NOT NULL,
    event_type          TEXT NOT NULL,
    actor_principal_kind TEXT NOT NULL,
    actor_principal_id  TEXT NOT NULL,
    payload_json        JSONB NOT NULL,
    payload_hash        TEXT NOT NULL,
    publish_status      TEXT NOT NULL DEFAULT 'pending',
    attempt_count       INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until     TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_outbox_events PRIMARY KEY (tenant_id, organization_id, outbox_id),
    CONSTRAINT uk_im_rtc_outbox_events_event UNIQUE (tenant_id, organization_id, event_id),
    CONSTRAINT chk_im_rtc_outbox_events_status CHECK (publish_status IN ('pending', 'published', 'failed')),
    CONSTRAINT chk_im_rtc_outbox_events_type CHECK (event_type IN (
        'session.created', 'session.ringing', 'session.connected',
        'session.ended', 'session.canceled', 'session.rejected',
        'session.failed', 'session.timeout', 'session.hold', 'session.resumed',
        'participant.invited', 'participant.joined', 'participant.left',
        'participant.kicked', 'participant.credential_issued',
        'participant.credential_revoked',
        'recording.started', 'recording.stopped', 'recording.failed'
    ))
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_status_available
    ON im_rtc_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_session
    ON im_rtc_outbox_events (tenant_id, organization_id, rtc_session_id, created_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_retention_until
    ON im_rtc_outbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================================
-- 2. RTC Quality Reports
-- ============================================================================
-- Per-participant media quality telemetry for SLA dashboards, MOS scoring,
-- network diagnostics, and post-call analytics (Teams CQD equivalent).

CREATE TABLE IF NOT EXISTS im_rtc_quality_reports (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    report_id               TEXT NOT NULL,
    rtc_session_id          TEXT NOT NULL,
    participant_principal_kind TEXT NOT NULL,
    participant_principal_id   TEXT NOT NULL,
    participant_device_id     TEXT NOT NULL,
    reported_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- MOS score (ITU-T P.800, range 1.0-4.5)
    mos_score               DOUBLE PRECISION CHECK (mos_score IS NULL OR (mos_score >= 1.0 AND mos_score <= 4.5)),
    -- Network metrics (per reporting window)
    rtt_ms                  DOUBLE PRECISION CHECK (rtt_ms IS NULL OR rtt_ms >= 0),
    jitter_ms               DOUBLE PRECISION CHECK (jitter_ms IS NULL OR jitter_ms >= 0),
    packet_loss_rate        DOUBLE PRECISION CHECK (packet_loss_rate IS NULL OR (packet_loss_rate >= 0 AND packet_loss_rate <= 1.0)),
    packets_sent            BIGINT CHECK (packets_sent IS NULL OR packets_sent >= 0),
    packets_received        BIGINT CHECK (packets_received IS NULL OR packets_received >= 0),
    packets_lost            BIGINT CHECK (packets_lost IS NULL OR packets_lost >= 0),
    bytes_sent              BIGINT CHECK (bytes_sent IS NULL OR bytes_sent >= 0),
    bytes_received          BIGINT CHECK (bytes_received IS NULL OR bytes_received >= 0),
    -- Audio/Video quality
    audio_bitrate_kbps      INTEGER CHECK (audio_bitrate_kbps IS NULL OR audio_bitrate_kbps >= 0),
    video_bitrate_kbps      INTEGER CHECK (video_bitrate_kbps IS NULL OR video_bitrate_kbps >= 0),
    audio_codec             TEXT,
    video_codec             TEXT,
    resolution_width        INTEGER CHECK (resolution_width IS NULL OR resolution_width >= 0),
    resolution_height       INTEGER CHECK (resolution_height IS NULL OR resolution_height >= 0),
    frame_rate_fps          DOUBLE PRECISION CHECK (frame_rate_fps IS NULL OR frame_rate_fps >= 0),
    -- Quality classification
    quality_grade           TEXT CHECK (quality_grade IN ('excellent', 'good', 'fair', 'poor', 'bad')),
    -- Optional raw provider payload
    payload_json            JSONB,
    payload_hash            TEXT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until         TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_quality_reports PRIMARY KEY (tenant_id, organization_id, report_id),
    CONSTRAINT uk_im_rtc_quality_reports_session_report UNIQUE (tenant_id, organization_id, rtc_session_id, participant_principal_kind, participant_principal_id, participant_device_id, reported_at)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_session_time
    ON im_rtc_quality_reports (tenant_id, organization_id, rtc_session_id, reported_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_participant
    ON im_rtc_quality_reports (tenant_id, organization_id, participant_principal_kind, participant_principal_id, reported_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_grade
    ON im_rtc_quality_reports (tenant_id, organization_id, quality_grade, reported_at)
    WHERE quality_grade IN ('poor', 'bad');

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_retention_until
    ON im_rtc_quality_reports (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================================
-- 3. RTC Participant Credentials
-- ============================================================================
-- Tracks issued RTC credentials with TTL, rotation, and revocation state.
-- Replaces the "issue-and-forget" pattern with explicit lifecycle control.

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
    -- Credential state machine
    credential_state            TEXT NOT NULL DEFAULT 'active',
    -- TTL management
    issued_at                   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at                  TIMESTAMPTZ NOT NULL,
    rotated_from_credential_id  TEXT,
    rotated_at                  TIMESTAMPTZ,
    -- Revocation tracking
    revoked_at                  TIMESTAMPTZ,
    revoked_reason              TEXT,
    revoked_by_principal_kind   TEXT,
    revoked_by_principal_id     TEXT,
    -- Opaque credential payload (token/nonce, provider-specific)
    credential_payload_json     JSONB NOT NULL,
    credential_payload_hash     TEXT NOT NULL,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until             TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_participant_credentials PRIMARY KEY (tenant_id, organization_id, credential_id),
    CONSTRAINT uk_im_rtc_participant_credentials_session_part UNIQUE (
        tenant_id, organization_id, rtc_session_id,
        participant_principal_kind, participant_principal_id, participant_device_id,
        credential_state
    ),
    CONSTRAINT chk_im_rtc_participant_credentials_state CHECK (credential_state IN (
        'active', 'expired', 'revoked', 'rotated'
    )),
    CONSTRAINT chk_im_rtc_participant_credentials_revocation CHECK (
        (credential_state = 'revoked') = (revoked_at IS NOT NULL)
    ),
    CONSTRAINT chk_im_rtc_participant_credentials_rotation CHECK (
        (credential_state = 'rotated') = (rotated_at IS NOT NULL AND rotated_from_credential_id IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_session
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, participant_principal_kind, participant_principal_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_active
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, credential_state, expires_at)
    WHERE credential_state = 'active';

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_expiry
    ON im_rtc_participant_credentials (tenant_id, organization_id, expires_at)
    WHERE credential_state = 'active';

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_retention_until
    ON im_rtc_participant_credentials (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

COMMENT ON TABLE im_rtc_outbox_events IS
    'Outbox table for RTC lifecycle events. Relay worker dispatches to Kafka/Redis Streams via FOR UPDATE SKIP LOCKED.';
COMMENT ON TABLE im_rtc_quality_reports IS
    'Per-participant media quality telemetry. MOS score follows ITU-T P.800. Aligns with Teams CQD / Agora analytics.';
COMMENT ON TABLE im_rtc_participant_credentials IS
    'RTC credential lifecycle: issue -> active -> (rotated|expired|revoked). Mandatory TTL tracking and explicit revocation.';
