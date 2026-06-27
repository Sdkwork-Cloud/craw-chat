# interaction-service

Domain: communication
Capability: chat
Package type: rust-crate
Status: deprecated

Deprecated scaffold. Reactions, pins, threads, and settings use `/im/v3/api/chat/*` per `sdkwork-im-im.openapi.yaml`. See `specs/component.spec.json`.

## Public API

- None — canonical HTTP paths are owned by the IM open-api contract.

## Verification

- `cargo check -p interaction-service`
