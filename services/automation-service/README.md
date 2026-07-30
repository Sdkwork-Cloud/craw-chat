# automation-service

Domain: communication
Capability: chat
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `automation-service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Implemented Boundary

- Accepts tenant- and organization-scoped automation execution requests and persists `Requested`.
- Moves an execution to `Running` only when a real agent response starts and to `Succeeded` only when that response completes.
- Appends response-frame and tool-call events before committing their process-local projections. Journal failure does not return success or consume the next event order.
- Does not implement a general workflow target executor. Request acceptance alone is not execution success.

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

- Production requires `SDKWORK_DATABASE_URL` for the execution store and PostgreSQL commit journal.
- `SDKWORK_IM_AUTOMATION_EXECUTION_STORE_FILE` is a bounded, single-node `dev`/`test` facility and is rejected in production.
- Request concurrency and body limits are configured by `SDKWORK_IM_AUTOMATION_MAX_IN_FLIGHT_REQUESTS` and `SDKWORK_IM_AUTOMATION_MAX_REQUEST_BODY_BYTES`.

## SaaS/Private/Local Behavior

The process-local runtime is bounded to 1,024 executions / 64 MiB, 256 agent response streams / 64 MiB, 1,024 frames and 16 MiB per response, and 1,024 tool calls / 64 MiB. Terminal entries expire after 15 minutes and may be deterministically evicted; active entries are never evicted. New work fails closed with `automation_runtime_capacity_exhausted` when no safe capacity remains.

`/metrics` exposes capacity rejection and terminal eviction counters, current resident entries/estimated bytes, and journal append failures. Labels use only fixed resource and reason values; tenant, principal, execution, stream, and tool-call identifiers are never metric labels.

PostgreSQL execution updates serialize monotonic merges with a transaction-scoped advisory lock. Active response/tool-call projections, worker claim leases, restart recovery, and atomic journal/projection materialization are not yet durable. Those gaps block HA and commercial release; operators must not infer completion from an accepted request.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test -p automation-service`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
