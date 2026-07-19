BEGIN;

UPDATE im_projection_metadata_snapshots
SET payload_json = (payload_json #>> '{}')::jsonb
WHERE jsonb_typeof(payload_json) = 'string';

UPDATE im_projection_timeline_entries
SET payload_json = (payload_json #>> '{}')::jsonb
WHERE jsonb_typeof(payload_json) = 'string';

COMMIT;
