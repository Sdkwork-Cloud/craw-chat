# social-service

Domain: communication
Capability: social
Package type: rust-crate
Status: active

Gateway service `comms-social-service` owning `/im/v3/api/social/*`. Machine-readable contract: `specs/component.spec.json`.

## Public API

- `SocialRuntime`
- `build_app`
- `build_public_app`

## Verification

- `cargo check -p social-service`
- `cargo test -p social-service`
- Postgres supplemental routes allocate entity ids through `sdkwork-im-runtime-id` (`src/postgres/id.rs`).
- Postgres handlers cover friendships, blocks, direct chats, user profiles, and user settings under `/im/v3/api/social/*`.
