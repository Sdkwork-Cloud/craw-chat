# im-adapters-postgres-projection

Domain: communication  
Capability: im  
Package type: rust-crate  
Status: standardizing

PostgreSQL-backed durable stores for `projection-service` snapshot persistence:

- `PostgresTimelineProjectionStore` → `im_projection_timeline_entries`
- `PostgresMetadataStore` → `im_projection_metadata_snapshots`

## Configuration

Uses `SDKWORK_IM_DATABASE_*` through `sdkwork-database-config`, matching `im-adapters-postgres-journal`.

## Verification

- `cargo test -p im-adapters-postgres-projection`
