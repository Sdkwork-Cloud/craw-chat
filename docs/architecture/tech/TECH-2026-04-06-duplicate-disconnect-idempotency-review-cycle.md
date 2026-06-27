> Migrated from `docs/review/2026-04-06-duplicate-disconnect-idempotency-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Duplicate Disconnect Idempotency Review Cycle

## 1. Finding

### 1.1 High: reconnect fence accidentally broke `session.disconnect` retry idempotency

- Affected services:
  - `services/session-gateway`
  - `services/sdkwork-im-cloud-gateway`
- Root cause:
  - the previous wave correctly introduced a reconnect-required fence after disconnect
  - `session.disconnect` still reused the ordinary non-`resume` bind path
  - after the first successful disconnect wrote the fence, a duplicate disconnect retry from the same session hit that fence and returned `409 reconnect_required`

This created an incorrect retry contract:

1. client sent `session.disconnect`
2. server completed disconnect successfully
3. client retried the same request because of transport uncertainty or timeout
4. server returned `409 reconnect_required` instead of the same offline result

That is not acceptable for a control-style lifecycle endpoint. Explicit disconnect must remain safe to retry.

## 2. Impact

- clients could not safely retry disconnect on timeout or uncertain network delivery
- operationally successful disconnects could look like failures on retry
- SDKs would need custom retry suppression logic for one endpoint even though the request is logically idempotent

This is a robustness defect, not just a convenience issue.

## 3. Reproduction

Red coverage was added first in:

- `services/session-gateway/tests/http_smoke_test.rs`
  - `test_session_gateway_treats_duplicate_disconnect_as_idempotent_for_same_session`
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_treats_duplicate_disconnect_as_idempotent_for_same_session`

Red evidence:

- `cargo test -p session-gateway --offline test_session_gateway_treats_duplicate_disconnect_as_idempotent_for_same_session -- --exact --nocapture`
  - failed with:
    - expected status: `200`
    - actual status: `409`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_treats_duplicate_disconnect_as_idempotent_for_same_session -- --exact --nocapture`
  - failed with:
    - expected status: `200`
    - actual status: `409`

## 4. Fix Design

The minimum correct rule is:

1. reconnect fence must continue to block ordinary non-`resume` client-route-bound requests
2. the exact same session may retry `session.disconnect` idempotently
3. that retry path must not recreate route ownership or mutate live state again
4. it should simply return the current offline presence snapshot

This is a narrow carve-out, not a rollback of the reconnect-required standard.

## 5. Implementation

- `services/session-gateway/src/cluster.rs`
  - added `disconnect_fence_matches_session(...)`
  - extended cluster regression to prove fence/session matching is precise
- `services/session-gateway/src/lib.rs`
  - `disconnect_session(...)` now short-circuits duplicate same-session retries
  - duplicate retry returns the existing offline snapshot without re-binding any route
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - same idempotent duplicate-disconnect short-circuit for the local profile
- `services/session-gateway/tests/http_smoke_test.rs`
  - added duplicate disconnect regression
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
  - added duplicate disconnect regression

## 6. Verification

### Red

- `cargo test -p session-gateway --offline test_session_gateway_treats_duplicate_disconnect_as_idempotent_for_same_session -- --exact --nocapture`
  - failed:
    - left: `409`
    - right: `200`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_treats_duplicate_disconnect_as_idempotent_for_same_session -- --exact --nocapture`
  - failed:
    - left: `409`
    - right: `200`

### Green

- `cargo test -p session-gateway --offline test_session_gateway_treats_duplicate_disconnect_as_idempotent_for_same_session -- --exact --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_local_minimal_profile_treats_duplicate_disconnect_as_idempotent_for_same_session -- --exact --nocapture`
- `cargo test -p session-gateway --offline disconnect_fence_requires_resume_until_cleared -- --nocapture`

Observed green result:

- duplicate disconnect from the same session returns `200`
- returned snapshot stays `offline`
- no ordinary non-`resume` request gained a bypass around `reconnect_required`

## 7. Remaining Risks

- reconnect fence persistence is still in-memory
- duplicate disconnect idempotency currently keys on matching session identity only
- websocket transport-level close confirmation is still separate from HTTP disconnect retry semantics

## 8. Next Wave

1. review whether reconnect fence must survive process restart in a durable store
2. freeze websocket close/retry semantics against the same idempotent disconnect contract
3. continue evaluating whether route lifecycle needs durable epochs once access-plane retry semantics are fully stable

