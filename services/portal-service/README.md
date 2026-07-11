# portal-service

Domain: communication
Capability: portal
Package type: rust-service
Status: standardizing

This README is the SDKWork module entrypoint for `portal-service`. The service exposes IM portal snapshot HTTP routes for console/admin surfaces using `SdkWorkApiResponse` envelopes.

## Public API

- HTTP handlers under `/im/v3/api/portal/*` assembled via `sdkwork-routes-im-portal-app-api`.
- `build_app` / bootstrap helpers for embedded and cloud hosts.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Uses shared IM app context and ops/audit runtime wiring from host bootstrap; see `services/portal-service/src/bootstrap.rs`.

## SaaS/Private/Local Behavior

Follows topology env vars for ops and audit dependencies; production requires durable audit when governance snapshots are enabled.

## Security

Portal routes require authenticated app context; audit-backed sections propagate list failures as empty records with `dataAvailability: false`.

## Extension Points

Route manifest changes belong in `crates/sdkwork-routes-im-portal-app-api`; snapshot logic belongs in `crates/im-portal-snapshots`.

## Verification

- `cargo test -p portal-service`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
