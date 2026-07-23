# im-portal-snapshots

Domain: communication
Capability: portal
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `im-portal-snapshots`. Portal snapshot builders assemble console/admin dashboard payloads from ops and audit runtime views with fail-closed `dataAvailability` semantics.

## Public API

- `build_portal_*_snapshot` helpers for dashboard, conversations, governance, access, and realtime sections.
- `build_portal_snapshot_for_section` section router used by portal HTTP handlers.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Snapshot builders consume injected `OpsRuntime` and optional `AuditRuntime` handles; they do not read process environment directly.

## SaaS/Private/Local Behavior

`dataAvailability` remains false until ops health is `ok` and the relevant realtime, operational lag, or transactional outbox source is wired. Snapshot builders never synthesize missing metrics.

## Security

Audit-backed snapshots require authenticated `AppContext`; governance and access sections fail closed when audit records are unavailable.

## Extension Points

New portal sections must declare snapshot builders here and register routes in `sdkwork-routes-im-portal-app-api`.

## Verification

- `cargo test -p im-portal-snapshots`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
