> Migrated from `docs/架构/82-direct-route-rebind-runtime-handoff-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 82. Direct Route Rebind Runtime Handoff Standard (2026-04-06)

## 1. Goal

The platform already uses `latest bind wins` for the same logical device.

That rule is only safe if a cross-node owner change also moves the client-route-scoped realtime truth to the new owner node. Otherwise the directory says one thing while the runtime state still lives somewhere else.

## 2. Device Owner Rule

For a single logical client route identified by:

- `tenantId`
- `principalId`
- `clientRouteId`

there must be exactly one logical owner node at a time.

If the same device later binds through another active node, the latest successful bind becomes the new owner.

## 3. Cross-Node Rebind Rule

When `bind_client_route(tenantId, principalId, clientRouteId, ownerNodeId, connectionKind)` changes the owner from `node_a` to `node_b`, the bridge must treat that operation as a state-bearing handoff, not as a directory-only overwrite.

The bridge must:

1. confirm the new owner node is still eligible for bind
2. export the client route runtime state from the previous owner
3. restore that state into the new owner runtime
4. overwrite the route directory entry only after the handoff inputs are available

This preserves the invariant:

- the node returned by route resolution is also the node that holds the authoritative realtime window for that device

## 4. Minimal Handoff Payload

The minimum direct-rebind handoff payload is the same as route migration:

- subscriptions
- realtime event window
- `latestRealtimeSeq`
- `ackedThroughSeq`
- `trimmedThroughSeq`

Without these fields, the new owner cannot safely continue:

- `GET /im/v3/api/realtime/events`
- `POST /im/v3/api/realtime/events/ack`
- subscription-filtered delivery after reconnect

## 5. Lifecycle Reconciliation Rule

If the previous owner node is already `draining`, direct route departure must immediately reconcile node lifecycle truth.

Required rule:

- `draining` node with `0` remaining owned routes -> `drained + stable`
- `draining` node with remaining routes -> `draining + moving_routes`

This prevents operator views and drain automation from reporting a stale moving-routes state after the last route leaves through a direct rebind.

## 6. Scope Boundary

This standard covers:

- route ownership truth
- runtime state truth
- node lifecycle truth after route departure

This standard does not yet require:

- WebSocket socket migration
- graceful old-transport shutdown
- lease/epoch fencing
- `awaiting_resume` orchestration

Those are later commercial-grade transport controls, but they must build on top of this correctness baseline.

## 7. Verification Standard

Regression coverage must prove all of the following:

1. a device bound to `node_a` can rebind directly to `node_b`
2. pending realtime events move with the device
3. `latestRealtimeSeq`, `ackedThroughSeq`, and `trimmedThroughSeq` remain continuous after rebind
4. subsequent delivery is written to the new owner runtime
5. the old owner no longer exposes the moved device window

## 8. Design Consequence

After this rule is enforced, the platform can safely keep `latest bind wins` without silently dropping client-route-scoped realtime truth during reconnects or cross-node ingress changes.

That gives the cluster a stable baseline for the next commercial waves:

- durable route ownership
- cutover orchestration
- resume fencing
- large-scale node elasticity

