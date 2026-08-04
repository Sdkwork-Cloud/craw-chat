# meta-cockroach

**Status: planned, not implemented.**

`meta-cockroach` is a reserved directory for a future CockroachDB-backed
`MetadataStore` adapter. It is **not** a production default and is not wired
into any build: the directory contains no Rust crate and `Cargo.toml` does not
reference it. The only currently implemented production metadata persistence is
PostgreSQL (`adapters/postgres-journal`, `adapters/social-postgres`).

Target capabilities once implemented:

- transaction
- unique constraint
- optimistic concurrency
- secondary index
- tenant scope

Implementation requirements (when the adapter is actually built):

- must not change tenant and entity primary-key semantics
- must pass the shared metadata-store conformance test
- must support SaaS and private-profile connection and migration strategies

Current state:

- no code exists in this directory
- nothing may claim CockroachDB metadata delivery until a real implementation
  and conformance evidence exist
