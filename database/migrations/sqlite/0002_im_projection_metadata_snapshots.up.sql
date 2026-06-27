-- Durable metadata snapshots for projection-service snapshot restore/persist.
-- SQLite-compatible version of the PostgreSQL migration.
-- Uses TEXT instead of JSONB, TEXT instead of TIMESTAMPTZ.

CREATE TABLE IF NOT EXISTS im_projection_metadata_snapshots (
    snapshot_scope TEXT NOT NULL,
    snapshot_key TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    CONSTRAINT pk_im_projection_metadata_snapshots PRIMARY KEY (snapshot_scope, snapshot_key)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_metadata_snapshots_key
    ON im_projection_metadata_snapshots (snapshot_key);
