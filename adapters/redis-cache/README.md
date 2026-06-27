# im-adapters-redis-cache

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

Redis cache adapter for IM runtime control and core contract caching.

## Public API

- Redis-backed cache implementations for platform contracts.

## Configuration

Redis connection is configured through IM runtime topology profiles under `configs/topology/`.

## Verification

- `cargo test -p im-adapters-redis-cache`
