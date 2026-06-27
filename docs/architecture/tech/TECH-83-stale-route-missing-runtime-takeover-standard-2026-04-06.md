> Migrated from `docs/架构/83-stale-route-missing-runtime-takeover-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 83. Stale Route Missing Runtime Takeover Standard (2026-04-06)

## 1. Goal

If route ownership still points to an old node but that node runtime is already gone, the platform must not let stale route metadata block client route recovery through another healthy active node.

## 2. Problem Boundary

This standard applies to:

- same logical device:
  - `tenantId`
  - `principalId`
  - `clientRouteId`
- route ownership exists
- previous owner runtime is missing
- new target node is active and can accept binds

This is a bind-path recovery problem, not a publish-path routing fallback problem.

## 3. Bind Recovery Rule

When `bind_client_route(...)` detects a cross-node takeover and the previous owner runtime is missing, it must treat the old route as stale takeover state.

Required behavior:

1. the new active owner node remains eligible for bind
2. missing previous runtime must not cause bind rejection
3. route ownership must move to the new owner
4. the target runtime should initialize any checkpoint truth it can restore locally

This prevents a dead runtime from pinning the device to a dead owner.

## 4. What Is Preserved

In stale takeover mode, the platform can still preserve:

- route availability
- resumed ownership on a healthy node
- durable checkpoint truth known to the target runtime's checkpoint store

## 5. What Is Not Magically Recoverable

If the previous runtime has already disappeared, the system must not pretend it can still recover volatile state that only lived in that dead runtime, such as:

- in-memory pending event window
- in-memory subscription set
- live transport socket state

Those require stronger later architecture:

- durable route ownership
- durable event buffering
- lease / epoch / fencing
- transport cutover orchestration

## 6. Relationship To Publish Semantics

This standard does not weaken the existing publish-path fail-closed rule.

If `publish_scope_event(...)` resolves a route but the resolved target runtime is missing, delivery still returns a missing-target outcome instead of falling back to origin runtime.

Reason:

- bind recovery is about allowing ownership repair
- publish fallback would hide stale route truth and risk phantom delivery

These two rules are intentionally different.

## 7. Lifecycle Reconciliation Rule

If the stale previous owner was already `draining`, route departure must still reconcile node lifecycle immediately:

- no remaining routes -> `drained + stable`
- remaining routes -> `draining + moving_routes`

Operator views must reflect actual residual ownership even in stale-takeover scenarios.

## 8. Verification Standard

Regression coverage must prove:

1. stale previous runtime does not block direct rebind to a healthy node
2. route ownership changes to the healthy node
3. the draining source lifecycle is reconciled after route departure
4. subsequent delivery can be written to the new owner runtime after the device resubscribes there
5. publish fail-closed semantics for missing resolved targets remain unchanged

## 9. Design Consequence

This standard gives the current in-memory cluster bridge a necessary high-availability baseline:

- dead runtime cannot permanently pin a device
- bind path self-heals stale route ownership
- publish path still exposes unresolved delivery truth

That is the minimum safe behavior before the system grows into full commercial-grade durable route coordination.

