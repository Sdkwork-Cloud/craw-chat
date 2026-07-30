# notification-service

Domain: communication
Capability: chat
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `notification-service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Implemented Boundary

- Accepts tenant- and organization-scoped notification requests and persists `requested`/accepted.
- Reads notification history with store-level keyset pagination; it does not load a fixed full history and slice it in process memory.
- Does not implement authoritative device-token registration/routing or a push-provider delivery worker. A request is not `dispatched` until a real provider receipt is committed.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

- Production requires `SDKWORK_DATABASE_URL` for the notification store and PostgreSQL commit journal.
- `SDKWORK_IM_NOTIFICATION_TASK_STORE_FILE` is a bounded, single-node `dev`/`test` facility and is rejected in production.

## SaaS/Private/Local Behavior

The hot cache is bounded to 1,024 entries / 64 MiB. The local JSON store is capped at 32 MiB and 50,000 notification/index records. PostgreSQL updates serialize monotonic state merges with a transaction-scoped advisory lock so concurrent writers cannot regress a terminal state.

Device registration, provider routing, durable claim/lease, bounded retry/backoff, dead-letter handling, provider receipts, invalid-token retirement, readiness, and delivery metrics remain unimplemented release blockers. Configuration values that resemble FCM/APNs credentials do not constitute an end-to-end delivery capability.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test -p notification-service`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
