-- Durable metadata snapshots for projection-service snapshot restore/persist.
-- Aligns MetadataStore persistence with split-service PostgreSQL production profile.

CREATE TABLE IF NOT EXISTS im_projection_metadata_snapshots (
    snapshot_scope TEXT NOT NULL,
    snapshot_key TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_projection_metadata_snapshots PRIMARY KEY (snapshot_scope, snapshot_key)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_metadata_snapshots_key
    ON im_projection_metadata_snapshots (snapshot_key);
