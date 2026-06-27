# 84. Current Session Route Fencing Standard (2026-04-06)

## 1. Goal

Once a device has resumed on a newer session, stale follow-up requests from an older session must not be able to reclaim route ownership or mutate the active device lifecycle.

## 2. Problem Boundary

This standard applies when all conditions hold:

- same logical device:
  - `tenantId`
  - `principalId`
  - `clientRouteId`
- route ownership already exists
- a newer `session_id` has taken over the route
- an older session later sends another client-route-bound request

The problem is specifically about stale post-resume traffic, not about whether a new resume is allowed to take over ownership.

## 3. Shared Truth Rule

The current client route session must be represented in the shared route directory, not only in node-local presence memory.

Required route metadata:

- `ownerNodeId`
- `sessionId`
- `connectionKind`
- `boundAt`

Reason:

- node-local presence state cannot protect against stale requests that land on another node
- the cluster-wide route directory is the only shared access-plane truth already consulted before delivery

## 4. Resume Takeover Rule

`session.resume` is the only access-plane request that may intentionally replace the current route session.

Required behavior:

1. a successful resume bind may update:
   - `ownerNodeId`
   - `sessionId`
   - `connectionKind`
2. the latest successful resume becomes the current route session
3. takeover may still trigger the existing route handoff rules:
   - runtime state handoff
   - stale runtime self-heal
   - drain lifecycle reconciliation

## 5. Non-Resume Fence Rule

Any other client-route-bound request that can implicitly register or rebind a device must first validate that its `session_id` still matches the current route session.

Covered examples:

- `session.disconnect`
- `presence.heartbeat`
- realtime subscription sync
- realtime event polling
- realtime ack
- websocket attach
- gateway-side command helpers that implicitly ensure client route registration before business writes

Required behavior:

1. read current route entry
2. if the route has no `sessionId`, allow current minimal behavior
3. if the request has no `session_id`, allow current minimal behavior
4. if both exist and differ:
   - reject the request before any bind or presence mutation
   - return `409 stale_session`

## 6. Error Contract

Stale session rejection must be explicit and fail-closed:

- HTTP status:
  - `409 Conflict`
- code:
  - `stale_session`

The rejection must happen before:

- client route registration write
- route ownership change
- presence snapshot change
- downstream realtime / command side effects

## 7. Local Profile Application

In the current minimal cluster profile:

- presence snapshot may remain node-local
- route directory is still the access-plane session fence
- `session.resume` remains the session takeover entrance

This allows the system to keep the minimal in-memory presence implementation while still preventing cross-node stale session takeover.

## 8. Verification Standard

Regression coverage must prove:

1. device resumes on node A with `s_old`
2. device resumes on node B with `s_new`
3. stale follow-up request from `s_old` is rejected with `409 stale_session`
4. route ownership remains on node B
5. current-session requests from `s_new` still succeed

At least one shared cluster-level test and one end-to-end node test must cover this rule.

## 9. Design Consequence

This standard freezes a critical commercial invariant:

- latest successful resume defines current session ownership
- stale sessions cannot silently take the route back
- access-plane rebinding is no longer equivalent to unconditional last-request-wins

That is the minimum safe boundary before introducing:

- durable route store
- lease / epoch / fencing
- explicit route release
- transport cutover orchestration
