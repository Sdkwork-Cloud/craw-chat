> Migrated from `docs/review/2026-04-06-stale-route-missing-runtime-takeover-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Stale Route Missing Runtime Takeover Review Cycle

## 1. Finding

### 1.1 High: direct device rebind failed closed when the previous owner runtime was already gone

- Affected services:
  - `services/session-gateway`
  - `services/sdkwork-im-cloud-gateway`
- Root cause:
  - the previous review wave made `bind_client_route(...)` perform runtime state handoff for cross-node `latest bind wins`
  - that implementation required the previous owner runtime to still exist
  - if the route directory still pointed to `node_a` but `node_a` runtime had already disappeared, `bind_client_route(...)` returned:
    - `409 node_runtime_missing`
  - both gateway stacks call route bind during client route registration / resume-related flows:
    - `services/session-gateway/src/lib.rs`
    - `services/sdkwork-im-cloud-gateway/src/lib.rs`

## 2. Impact

- after a node crash, abrupt restart, or stale in-memory route residue, the same device could fail to reconnect through another healthy node
- availability degraded from:
  - stale route present + old runtime gone + new active node available
  - into:
    - request rejected
    - route still pinned to dead owner
- the failure surfaced on normal access-plane operations that implicitly rebind devices, including:
  - session resume
  - heartbeat
  - realtime subscription sync
  - realtime event polling / ack
  - websocket attach

This is a high-availability defect. Once the old runtime is already gone, failing closed does not preserve recoverable state; it only blocks takeover.

## 3. Reproduction

Regression coverage was added first in:

- `services/session-gateway/src/cluster.rs`
  - `test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing`

Red evidence:

- `cargo test -p session-gateway --offline cluster::tests::test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing`
  - failed with:
    - `RealtimeClusterError { code: "node_runtime_missing", ... }`
  - actual behavior before the fix:
    - the device could not move ownership from a stale `node_a` route to healthy `node_b`

## 4. Fix Design

The correct boundary remains the route bind operation.

Chosen rule:

1. if the previous owner runtime still exists:
   - perform the normal minimal runtime state handoff
2. if the previous owner runtime is already missing:
   - do not block takeover
   - move route ownership to the new active node
   - let the target runtime restore any durable checkpoint truth it already knows about
3. keep publish semantics unchanged:
   - `publish_scope_event(...)` still stays fail-closed for a resolved route whose target runtime is missing
   - no origin fallback is introduced for resolved stale targets

This preserves availability on bind path without weakening delivery correctness on publish path.

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - `bind_client_route(...)` now:
    - loads the target runtime first for cross-node takeover
    - tries to load the previous owner runtime
    - if the previous owner runtime exists:
      - performs `take_client_route_state(...)` + `restore_client_route_state(...)`
    - if the previous owner runtime is missing:
      - treats the route as stale takeover
      - calls `target_runtime.ensure_client_route_state(...)`
      - continues ownership transfer instead of returning `node_runtime_missing`
  - existing lifecycle reconciliation remains in place, so a draining source node is updated after the stale route departs
- `services/session-gateway/src/cluster.rs`
  - added `test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing`

## 6. Verification

### Red

- `cargo test -p session-gateway --offline cluster::tests::test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing`
  - failed with `node_runtime_missing`

### Green

- `cargo test -p session-gateway --offline cluster::tests::test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing`
- `cargo test -p session-gateway --offline`

Observed green result:

- stale route takeover now succeeds
- route ownership moves to the healthy node
- source lifecycle is reconciled after route departure
- existing publish fail-closed behavior and rebind/migration tests remain green

## 7. Remaining Risks

- if the old runtime has already disappeared, in-memory pending events and in-memory subscriptions are already unrecoverable in the minimal profile
- the current self-heal path can only preserve:
  - availability
  - any checkpoint truth restorable from the target runtime's checkpoint store
- full commercial recovery still needs later work for:
  - durable route ownership
  - lease / epoch / fencing
  - transport cutover / resume coordination

## 8. Next Wave

1. review whether `session.resume` should expose an explicit stale-takeover hint for observability and ops audit
2. continue cluster hardening around durable route ownership and fencing so stale route self-heal can evolve from in-memory recovery to multi-node commercial behavior

