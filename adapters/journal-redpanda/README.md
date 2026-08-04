# journal-redpanda

**Status: planned, not implemented.**

`journal-redpanda` is a reserved directory for a future Redpanda-backed
`CommitJournal` adapter. It is **not** a production default and is not wired
into any build: the directory contains no Rust crate and `Cargo.toml` does not
reference it. The only currently implemented production journal persistence is
`adapters/postgres-journal` (PostgreSQL).

Target capabilities once implemented:

- ordered append
- durable ack
- replay
- checkpoint
- retention
- partition routing

Implementation requirements (when the adapter is actually built):

- must not change `CommitEnvelope` semantics
- must pass the shared journal conformance test
- must support SaaS shared / dedicated and private-profile assembly

Current state:

- no code exists in this directory
- nothing may claim Redpanda journal delivery until a real implementation
  and conformance evidence exist
