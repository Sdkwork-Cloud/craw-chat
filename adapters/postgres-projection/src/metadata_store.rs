use im_platform_contracts::{ContractError, MetadataSnapshotRecord, MetadataStore};
use r2d2_postgres::postgres::types::Json;
use sdkwork_utils_rust::sha256_hash;

use crate::{
    PostgresProjectionPool, now_rfc3339, postgres_pool_client, postgres_timestamptz,
    postgres_unavailable, run_postgres_io,
};

const UPSERT_METADATA_SNAPSHOT_SQL: &str = r#"
insert into im_projection_metadata_snapshots (
    snapshot_scope,
    snapshot_key,
    payload_json,
    payload_hash,
    created_at,
    updated_at
) values ($1, $2, $3::jsonb, $4, $5, $5)
on conflict (snapshot_scope, snapshot_key) do update set
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    updated_at = excluded.updated_at
"#;

const LOAD_METADATA_SNAPSHOT_SQL: &str = r#"
select payload_json::text
from im_projection_metadata_snapshots
where snapshot_scope = $1
  and snapshot_key = $2
"#;

#[derive(Clone)]
pub struct PostgresMetadataStore {
    pool: PostgresProjectionPool,
}

impl PostgresMetadataStore {
    pub fn from_pool(pool: PostgresProjectionPool) -> Self {
        Self { pool }
    }
}

impl MetadataStore for PostgresMetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let scope = scope.to_owned();
        let key = key.to_owned();
        let value = value.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "metadata put snapshot")?;
            let payload_hash = sha256_hash(value.as_bytes());
            let created_at = postgres_timestamptz(&now_rfc3339(), "created_at")?;
            client
                .execute(
                    UPSERT_METADATA_SNAPSHOT_SQL,
                    &[&scope, &key, &Json(value), &payload_hash, &created_at],
                )
                .map_err(|error| postgres_unavailable("metadata put snapshot", error))?;
            Ok(())
        })
    }

    fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError> {
        let pool = self.pool.clone();
        let scope = scope.to_owned();
        let key = key.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "metadata load snapshot")?;
            let row = client
                .query_opt(LOAD_METADATA_SNAPSHOT_SQL, &[&scope, &key])
                .map_err(|error| postgres_unavailable("metadata load snapshot", error))?;
            Ok(row.map(|row| row.get::<_, String>(0)))
        })
    }

    fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
        for snapshot in snapshots {
            self.put_snapshot(
                snapshot.scope.as_str(),
                snapshot.key.as_str(),
                snapshot.value.as_str(),
            )?;
        }
        Ok(())
    }
}
