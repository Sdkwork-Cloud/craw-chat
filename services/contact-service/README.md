# contact-service

Domain: communication
Capability: social
Package type: rust-crate
Status: deprecated

Deprecated Postgres scaffold. Public HTTP owner is `social-service` (`comms-social-service`) on `/im/v3/api/social/*`. See `specs/component.spec.json` and ADR-20260617-comms-service-naming-boundaries.

## Public API

- None — consumers must use `social-service`.

## Verification

- `cargo check -p contact-service`
