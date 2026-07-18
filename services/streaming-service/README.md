# streaming-service

Domain: communication
Capability: chat
Package type: rust-crate
Status: pre-GA production implementation

This README is the SDKWork module entrypoint for `streaming-service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Responsibility

`streaming-service` owns ordered application data streams at `/im/v3/api/streams/*`.
It does not own RTC media transport or Drive media objects.

The durable authority separates session metadata from frame rows. Every operation is scoped by
`tenant_id`, `organization_id`, and `stream_id`. Session mutations use optimistic versions, frame
append and session advancement commit in one PostgreSQL transaction, and duplicate frame sequence
numbers return the stored frame so the service can distinguish replay from payload conflict.

Frame reads use `frame_seq > cursor ORDER BY frame_seq LIMIT page_size + 1`. Neither reads nor
writes load or rewrite the complete stream, and the production runtime does not retain an
unbounded in-process session/frame mirror.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in `specs/component.spec.json`. Shared modules must receive configuration through typed bootstrap or service boundaries rather than reading host-local environment state directly.

## SaaS/Private/Local Behavior

This component follows the deployment and runtime rules referenced by its `canonicalSpecs` entries. SaaS, private, and local behavior must stay compatible with the relevant SDKWork specs before implementation changes are made.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test -p streaming-service`
- `cargo test -p im-adapters-postgres-journal --test stream_state_live_integration_test -- --ignored --nocapture`
- `pnpm run test:sqlite:smoke`
- `pnpm run check:pagination`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
