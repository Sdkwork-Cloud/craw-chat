> Migrated from `docs/review/2026-04-06-disconnect-websocket-close-contract-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Disconnect WebSocket Close Contract Review Cycle

## 1. Finding

### 1.1 High: explicit disconnect closed websocket transports without a stable protocol contract

- Affected services:
  - `services/session-gateway`
  - `services/sdkwork-im-cloud-gateway`
- Previous behavior:
  - the server actively closed live realtime websocket transports after `session.disconnect`
  - but the close frame used `Message::Close(None)`
  - no stable close code or reason was exposed to SDKs, proxies, or operational tooling

This meant the platform had a transport action, but not a transport contract.

## 2. Impact

- SDKs could detect that the socket closed, but could not reliably distinguish:
  - explicit user/device disconnect
  - network drop
  - server shutdown
  - generic websocket closure
- private deployment integrations had no stable marker for telemetry, reconnect policy, or audit correlation
- cross-platform behavior risk increased because each client could invent its own interpretation of a close-without-reason event

For a commercial IM platform, "socket closed" is not precise enough. The server must expose a stable reason when the close is caused by `session.disconnect`.

## 3. Root Cause

The prior wave fixed lifecycle enforcement but intentionally left the public close contract unfrozen:

1. disconnect generation changes triggered websocket shutdown correctly
2. websocket handlers sent a close frame
3. but they sent `Close(None)` instead of a named, versioned application contract

That omission was acceptable only as an intermediate step. It is not sufficient as a stable SDK boundary.

## 4. Reproduction

Red coverage was added first by tightening the existing disconnect websocket regressions:

- `services/session-gateway/tests/websocket_smoke_test.rs`
  - `test_realtime_websocket_closes_when_session_disconnects`
- `services/sdkwork-im-cloud-gateway/tests/websocket_e2e_test.rs`
  - `test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects`

Red evidence:

- `cargo test -p session-gateway --offline test_realtime_websocket_closes_when_session_disconnects -- --exact --nocapture`
  - failed because `session_gateway::SESSION_DISCONNECT_CLOSE_CODE` and `session_gateway::SESSION_DISCONNECT_CLOSE_REASON` did not exist yet
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects -- --exact --nocapture`
  - failed for the same reason

That red state was intentional: the tests now required an explicit public close contract, which the implementation did not yet provide.

## 5. Fix Design

The minimum production-ready rule is:

1. explicit `session.disconnect` must close live websocket transports with a stable application-defined close code
2. the close frame must carry a stable reason string
3. both gateway and local profile must expose the same contract
4. SDKs must be able to treat this closure as a terminal session boundary, not a generic transient transport loss

## 6. Implementation

- `services/session-gateway/src/websocket.rs`
  - added public constants:
    - `SESSION_DISCONNECT_CLOSE_CODE = 4001`
    - `SESSION_DISCONNECT_CLOSE_REASON = "session.disconnect"`
  - introduced a shared close helper that emits:
    - websocket close code `4001`
    - reason `"session.disconnect"`
  - both disconnect-signal and late-frame protection paths now use the same close contract
- `services/session-gateway/src/lib.rs`
  - re-exported the disconnect close constants so tests and downstream consumers can bind to the same contract
- `services/session-gateway/tests/websocket_smoke_test.rs`
  - now asserts exact close code and reason
- `services/sdkwork-im-cloud-gateway/tests/websocket_e2e_test.rs`
  - now asserts exact close code and reason

## 7. Verification

### Red

- `cargo test -p session-gateway --offline test_realtime_websocket_closes_when_session_disconnects -- --exact --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects -- --exact --nocapture`

Observed red result:

- compilation failed because the public disconnect close constants were missing

### Green

- `cargo test -p session-gateway --offline test_realtime_websocket_closes_when_session_disconnects -- --exact --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_closes_realtime_websocket_when_session_disconnects -- --exact --nocapture`

Observed green result:

- explicit disconnect now closes the websocket with code `4001`
- close reason is now `"session.disconnect"`
- both gateway and local profile expose the same contract

## 8. Design Decision

`4001` was chosen deliberately:

- `1000` would only signal a generic normal close and would not carry product-specific meaning
- `4000-4999` is the private-use websocket close range intended for application-defined semantics
- `"session.disconnect"` is short, stable, and maps directly to the business event that caused the closure

## 9. Remaining Risks

- reconnect fence persistence is still in-memory only
- crash-recovery route/session epochs are still not durable across process loss
- reconnect policy above this close contract is still an SDK responsibility and should be standardized later

## 10. Next Wave

1. review durable reconnect-fence persistence across restart and active-passive failover
2. decide whether SDK behavior on close code `4001` should be frozen more explicitly at the public contract level
3. continue toward durable route/session epoch recovery once disconnect semantics are fully frozen

