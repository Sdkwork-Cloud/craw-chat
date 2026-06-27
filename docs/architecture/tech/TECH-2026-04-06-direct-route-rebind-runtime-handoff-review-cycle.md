> Migrated from `docs/review/2026-04-06-direct-route-rebind-runtime-handoff-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Direct Route Rebind Runtime Handoff Review Cycle

## 1. Finding

### 1.1 High: direct same-device cross-node rebind overwrote route ownership without moving realtime state

- Affected services:
  - `services/session-gateway`
  - `services/sdkwork-im-cloud-gateway`
- Root cause:
  - `RealtimeClusterBridge::bind_client_route(...)` implemented `latest bind wins` by overwriting the route directory entry only
  - unlike `migrate_node_routes(...)`, the direct rebind path did not move:
    - subscriptions
    - realtime event window
    - `latestRealtimeSeq`
    - `ackedThroughSeq`
    - `trimmedThroughSeq`
  - if the same `tenantId + principalId + clientRouteId` reconnected through another node, the new owner node became authoritative for delivery, but the old owner runtime still held the actual client route state

## 2. Impact

- pending realtime events could become invisible after a reconnect that landed on a different node
- the new owner node could resolve the route correctly but still deliver `0` events because the subscription set was left behind on the previous owner
- checkpoint continuity was broken for:
  - `latestRealtimeSeq`
  - `ackedThroughSeq`
  - `trimmedThroughSeq`
- when the previous owner was already `draining`, direct rebind could also leave node lifecycle truth stale until a later control action corrected it

This is a commercial-grade correctness issue because reconnect and cutover are normal cluster behaviors, not exceptional operator workflows.

## 3. Reproduction

Regression coverage was added first in:

- `services/session-gateway/tests/cluster_routing_test.rs`
  - `test_cluster_bridge_rebind_latest_owner_transfers_realtime_state`

Red evidence:

- `cargo test -p session-gateway --offline test_cluster_bridge_rebind_latest_owner_transfers_realtime_state`
  - failed at:
    - `source runtime must hand off pending window state on direct rebind`
  - actual behavior before the fix:
    - source runtime still retained the pending event window after route ownership moved to `node_b`

## 4. Fix Design

The correct fix boundary is the cluster bridge route bind operation itself.

Chosen rule:

1. `latest bind wins` remains the routing rule for the same device
2. when the new bind changes `ownerNodeId`, the bridge must execute the same minimal device-state handoff discipline already required by route migration
3. the bridge must update the previous owner lifecycle if that node was `draining` and the departing route changes its residual ownership count
4. this fix intentionally does not invent transport cutover orchestration:
   - socket migration
   - graceful disconnect
   - `awaiting_resume`

Those remain later commercial enhancements, but they must not block correctness of route ownership and runtime truth.

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - `bind_client_route(...)` now:
    - reads the previous route for the same device
    - detects cross-node owner changes
    - loads both source and target runtimes
    - performs `take_client_route_state(...)` from the old owner
    - performs `restore_client_route_state(...)` into the new owner
  - after the directory entry is overwritten, the bridge now reconciles the previous owner lifecycle:
    - `draining + no remaining routes -> drained + stable`
    - `draining + remaining routes -> draining + moving_routes`
- `services/session-gateway/tests/cluster_routing_test.rs`
  - added regression coverage proving that direct rebind preserves:
    - pending event window
    - checkpoint continuity
    - post-rebind event delivery on the new owner

## 6. Verification

### Red

- `cargo test -p session-gateway --offline test_cluster_bridge_rebind_latest_owner_transfers_realtime_state`
  - failed because the source runtime still contained the pending device window after route rebind

### Green

- `cargo test -p session-gateway --offline test_cluster_bridge_rebind_latest_owner_transfers_realtime_state`
- `cargo test -p session-gateway --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_routes_realtime_events_to_remote_owner_node`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_drain_migrates_routes_and_preserves_realtime_delivery`

Observed green result:

- direct cross-node rebind now preserves runtime state and continues delivery on the new owner node
- existing route migration and remote-owner realtime routing tests remain green

## 7. Remaining Risks

- this fix only covers in-process runtime handoff where both runtimes are available through the shared bridge
- explicit transport cutover is still not implemented for:
  - WebSocket socket migration
  - client resume choreography
  - `awaiting_resume`
- if a future commercial deployment allows route records to outlive runtime availability, stale-route recovery and fencing still need a separate design wave

## 8. Next Wave

1. review whether stale route ownership with missing source runtime should fail closed, self-heal, or consult durable checkpoint truth
2. audit `session.resume` and route ownership against future cutover expectations so resume semantics stay consistent once route fencing or lease epochs are introduced

