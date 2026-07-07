# sdkwork-routes-im-portal-app-api

Domain: communication
Capability: portal
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `sdkwork-routes-im-portal-app-api`. The crate owns the IM portal app-api route manifest and Axum router assembly for portal snapshot endpoints.

## Public API

- Route manifest and path constants for portal snapshot operations.
- Web bootstrap router merge helpers consumed by session-gateway and portal-service hosts.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

No direct environment access; hosts inject runtime state through bootstrap boundaries.

## SaaS/Private/Local Behavior

Routes follow standard IM app-api auth and SdkWorkApiResponse envelope rules across deployment profiles.

## Security

Operations inherit IAM app-api dual-token security from the host router; portal handlers must not bypass auth middleware.

## Extension Points

Add operations here first, then mirror OpenAPI authority and regenerate SDK artifacts when contracts change.

## Verification

- `cargo test -p sdkwork-routes-im-portal-app-api`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
