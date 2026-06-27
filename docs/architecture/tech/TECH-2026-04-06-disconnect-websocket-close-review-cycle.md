> Migrated from `docs/review/2026-04-06-disconnect-websocket-close-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Disconnect WebSocket Close Review Cycle

## 1. Finding

### 1.1 High: `session.disconnect` did not close already-open realtime websocket connections

- Affected services:
  - `services/session-gateway`
  - `services/sdkwork-im-cloud-gateway`
- Root cause:
  - explicit disconnect already cleared live subscriptions, released route ownership, and established reconnect-required fencing
  - but existing websocket connections had no shutdown signal tied to disconnect
  - websocket frame processing also lived entirely inside the runtime/socket loop, so an already-open connection could remain open after disconnect

This left a transport-level lifecycle mismatch:

1. HTTP/device semantics said the session was disconnected
2. route ownership was gone
3. reconnect fence existed
4. but the already-open websocket transport still stayed attached until the client closed it or the process ended

That is not a complete disconnect contract for a commercial IM access plane.

## 2. Impact

- clients could keep a stale websocket transport alive after explicit disconnect
- transport state diverged from presence and routing state
- websocket control frames had a window to outlive the disconnect boundary

Even if live subscription state had already been cleared, leaving the socket open weakens lifecycle clarity and makes SDK behavior harder to reason about.

## 3. Reproduction

Red coverage was added first in:

- `services/session-gateway/tests/websocket_smoke_test.rs`
  - `test_realtime_websocket_closes_when_session_disconnects`
- `services/sdkwork-im-cloud-gateway/tests/websocket_e2e_test.rs`
  - `test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects`

Red evidence:

- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects -- --exact --nocapture`
  - failed on timeout waiting for a websocket close frame after disconnect
- `cargo test -p session-gateway --offline test_realtime_websocket_closes_when_session_disconnects -- --exact --nocapture`
  - initially required test harness correction, then reproduced the same missing-close behavior

## 4. Fix Design

The minimum correct rule is:

1. disconnect must signal all currently attached realtime websocket connections for that device to close
2. sockets opened before disconnect must observe that signal and terminate promptly
3. sockets must not require a later event publish or extra client traffic to notice the disconnect

This is a transport-layer complement to Standards 86, 87, 88, and 89.

## 5. Implementation

- `services/session-gateway/src/realtime.rs`
  - added per-client-route websocket disconnect generations and watch notifiers
  - added:
    - `subscribe_disconnect_signal(...)`
    - `disconnect_generation(...)`
    - `signal_device_disconnect(...)`
- `services/session-gateway/src/websocket.rs`
  - websocket loop now subscribes to disconnect notifications
  - when disconnect generation changes, the server sends a websocket close frame and terminates the connection
  - inbound frame handling now also re-checks disconnect generation before processing a late client frame
- `services/session-gateway/src/lib.rs`
  - both normal disconnect and duplicate same-session disconnect retry paths now signal websocket closure
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - same websocket disconnect signal propagation for local profile disconnect paths
- `services/session-gateway/tests/websocket_smoke_test.rs`
  - added disconnect-close regression
- `services/sdkwork-im-cloud-gateway/tests/websocket_e2e_test.rs`
  - added integrated disconnect-close regression

## 6. Verification

### Red

- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects -- --exact --nocapture`
  - failed on websocket timeout waiting for close

### Green

- `cargo test -p session-gateway --offline test_realtime_websocket_closes_when_session_disconnects -- --exact --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects -- --exact --nocapture`

Observed green result:

- explicit disconnect now triggers a websocket close frame
- open realtime websocket transports no longer linger after disconnect
- transport lifecycle is aligned with route/presence disconnect semantics

## 7. Remaining Risks

- reconnect fence persistence is still in-memory
- websocket close currently uses the transport close frame without an application-specific close reason contract
- crash-recovery route epochs are still future work

## 8. Next Wave

1. review whether reconnect fence intent must survive process restart or active-passive failover
2. decide whether websocket close should expose a stable close code/reason contract for SDKs
3. continue toward durable route/session epochs after explicit lifecycle semantics are fully frozen

