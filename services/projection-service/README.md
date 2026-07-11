# projection-service

Domain: communication
Capability: chat
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `projection-service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `.`

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

## Runtime Guardrails

HTTP handlers MUST execute projection reads and writes through `run_blocking_projection`
in `src/http.rs`. Projection code acquires process-local mutexes and may call synchronous
PostgreSQL adapters; running that work on Tokio async workers can starve the standalone
gateway and wedge unrelated routes such as `/healthz` and `/app/v3/api/auth/sessions/current`.

In-flight HTTP capacity is bounded by `SDKWORK_IM_PROJECTION_MAX_IN_FLIGHT_REQUESTS`
(default `1000`). Health, readiness, metrics, and OpenAPI routes bypass the gate.

Inbox list reads MUST snapshot candidate conversation scopes under the `members` lock and
release that lock before building `ConversationInboxEntry` values or evaluating archive filters.
`build_inbox_entry_for_scope` also needs member data, so keeping the outer `members` lock while
building entries creates a same-thread mutex re-entry deadlock and leaves `/im/v3/api/chat/inbox`
requests pending whenever the caller has indexed inbox data.

Interactive list APIs use SDKWork cursor pagination with canonical HTTP query parameters
`page_size` and `cursor`. This pre-launch service rejects historical pagination aliases
(`pageSize`, `limit`, `page_no`, `pageNo`, `per_page`, and `size`) with
`40003 INVALID_PARAMETER`; `page` and `cursor` are also rejected when combined.

## Read-Model Consistency Model

The projection service maintains an in-memory read model backed by a Postgres durable
snapshot store. Multi-replica deployments and post-restart reads rely on a read-through
fallback to keep read models consistent without waiting for journal consumer replay.

### Durable Read-Through

The following read paths fall back to the durable metadata snapshot when the in-memory
store misses, hydrating the memory cache so subsequent reads hit memory directly:

| Read Path | Durable Snapshot Key |
| --- | --- |
| `conversation_summary` | `conversation-summary` |
| `history_visibility_for_conversation` | `conversation-catalog` |
| `member_snapshot_for_principal_kind` | `conversation-members` |
| `read_cursor_for_principal_kind_and_device` | `conversation-read_cursors` |
| `conversation_profile` | `conversation-profile` |
| `message_interaction_summary` | `message-interactions` |
| `message_visibility_for_principal` | `message-visibilities` |
| `timeline_window` | Timeline projection store (tiered) |

Read-through failures are warn-logged at `sdkwork.im.projection.read_through` and return
`None` / default values — they never fail the HTTP request.

### Journal Consumer Persist Retry

The journal consumer persists durable state after applying events. Persist failures are
retried up to 3 times with 50/100 ms backoff (`persist_durable_state_with_retry`). The
combined worst-case sleep (150 ms) stays below the default 250 ms poll interval so the
consumer never falls behind. When all retries fail, the consumer keeps advancing — the
next cycle re-attempts the full accumulated state.

### Embedded Apply (Unified vs Separated Deployment)

- **Unified-process (standalone):** the projection HTTP bootstrap initializes the shared
  runtime; journal append paths call `try_apply_commit_envelope` for immediate local
  projection feedback without waiting for replay polling.
- **Separated cloud deployment:** separated services (e.g. `conversation-service` without
  projection HTTP handlers) never initialize the shared runtime. `resolve_embedded_projection_service`
  uses `try_shared_projection_runtime` (no lazy init) in production, so embedded apply
  becomes a silent no-op. The journal consumer on `projection-service` replicas drives
  read-model consistency instead.

Regression coverage for pending-sensitive inbox reads:

- `cargo test -p projection-service inbox_window_from_auth_context_returns_without_reentrant_member_lock --lib -- --nocapture`
- `cargo test -p projection-service inbox_window_concurrent_reads_do_not_deadlock --lib -- --nocapture`
- `cargo test -p projection-service --test http_smoke_test test_inbox_query_returns_projected_entries -- --exact --nocapture`
- `cargo test -p projection-service --test http_smoke_test test_inbox_query_returns_bounded_cursor_window -- --exact --nocapture`
- `cargo test -p projection-service --test http_smoke_test test_inbox_query_rejects_forbidden_pagination_aliases -- --exact --nocapture`
- `cargo test -p projection-service --test http_smoke_test test_inbox_query_rejects_page_and_cursor_combination -- --exact --nocapture`

## Verification

- `cargo test --manifest-path apps/sdkwork-im/services/projection-service/Cargo.toml`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
