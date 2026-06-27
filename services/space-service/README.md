# space-service

Domain: communication
Capability: space
Package type: rust-crate
Status: active

Gateway service `comms-space-service` for spaces, groups, channels, members, invitations, and bans on `/im/v3/api/spaces/*`. Machine-readable contract: `specs/component.spec.json`.

## Public API

- Space and group HTTP handlers mounted through the IM gateway.

## Verification

- `cargo check -p space-service`
- `cargo test -p space-service`
- Entity ids use `sdkwork-im-runtime-id` (`RuntimeSnowflakeIdGenerator`) via `src/id.rs`.
