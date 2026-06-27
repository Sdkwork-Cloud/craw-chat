> Migrated from `docs/review/2026-04-06-agent-handoff-closed-capability-gate-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Agent Handoff Closed Capability Gate Review Cycle

## 1. Finding

### 1.1 High: closed `agent_handoff` still allowed conversation-bound `stream` and `RTC` writes

- Root cause:
  - previous lifecycle hardening closed `agent_handoff` for:
    - message post
    - message edit
    - message recall
  - but `sdkwork-im-server` access gates for conversation-bound sub-capabilities only validated active membership:
    - `ensure_stream_open_access(...)`
    - `ensure_stream_session_conversation_member(...)`
    - `ensure_rtc_create_access(...)`
    - `ensure_rtc_session_conversation_member(...)`
  - those gates never checked whether the bound conversation had already entered a special lifecycle terminal state.
- Impact:
  - a handoff already marked `closed` could still:
    - open new streams
    - append/checkpoint/complete/abort existing streams
    - create new rtc sessions
    - continue rtc signaling or state mutation
  - this breaks the earlier lifecycle standard that `closed` means the handoff is operationally finished.
  - for RTC there was an additional partial-side-effect risk:
    - mutate rtc runtime first
    - then try to emit conversation-bound signal message
    - causing inconsistent cross-service state if the message path conflicts later

## 2. Scope Freeze

This wave fixes only conversation-bound write access after `agent_handoff.status = closed`.

It does not:

- add new stream/rtc lifecycle states
- change read-only conversation views
- expand into `agent_dialog` close/archive or `system_channel` publish orchestration

## 3. Design Decision

The gate belongs at the access boundary, but the lifecycle truth belongs in `conversation-runtime`.

So the implementation is split as:

- `conversation-runtime`
  - expose a reusable capability check based on durable conversation lifecycle truth
- `sdkwork-im-server`
  - keep membership enforcement
  - additionally consult the runtime lifecycle gate before allowing conversation-bound write paths

Read-only history access remains intentionally narrower in this wave:

- `stream` write routes are blocked after close
- `stream` history listing is not widened into the same ban yet

This keeps the fix minimal while freezing the most dangerous mutation paths.

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - added `ensure_conversation_bound_write_allowed(...)`
  - introduced internal rule:
    - closed `agent_handoff` rejects further conversation-bound capability writes with `RuntimeError::Conflict`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - added `ensure_conversation_bound_write_access(...)`
  - added `ensure_stream_session_write_access(...)`
  - renamed the RTC mutation gate behavior to enforce lifecycle-aware access for:
    - `rtc.create`
    - `rtc.invite`
    - `rtc.accept`
    - `rtc.reject`
    - `rtc.end`
    - `rtc.signal`
  - stream write routes now enforce lifecycle-aware access for:
    - `stream.open`
    - `stream.append`
    - `stream.checkpoint`
    - `stream.complete`
    - `stream.abort`
  - read-only `list_stream_frames(...)` still uses membership-only access in this wave

## 5. Tests Added

- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_closed_agent_handoff_blocks_conversation_bound_stream_writes_in_local_profile`
  - `test_closed_agent_handoff_blocks_conversation_bound_rtc_writes_in_local_profile`

These tests prove both:

- existing bound session mutation is blocked after close
- new bound capability creation is blocked after close

## 6. Verification

### Red

- `cargo test -p sdkwork-im-cloud-gateway --offline test_closed_agent_handoff_blocks_conversation_bound_stream_writes_in_local_profile -- --exact`
  - failed with `200 != 409`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_closed_agent_handoff_blocks_conversation_bound_rtc_writes_in_local_profile -- --exact`
  - failed with `200 != 409`

### Green

- `cargo test -p sdkwork-im-cloud-gateway --offline test_closed_agent_handoff_blocks_conversation_bound_stream_writes_in_local_profile -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_closed_agent_handoff_blocks_conversation_bound_rtc_writes_in_local_profile -- --exact`

## 7. Remaining Risks

- read-only `stream` history access after close is still allowed for active members; whether that should remain readable as audit/history or also freeze with lifecycle needs an explicit decision.
- the new gate is currently enforced in `sdkwork-im-server`; any future gateway/edge profile must adopt the same invariant.
- `agent_dialog` and `system_channel` still do not have equivalent cross-service lifecycle/capability rules.

## 8. Next Wave

1. Decide whether closed `agent_handoff` should also freeze read-only stream history endpoints, or preserve them for audit/debug visibility.
2. Audit whether other conversation-bound capabilities beyond `stream/RTC` need the same lifecycle-aware write gate.
3. Continue special-conversation lifecycle hardening for `agent_dialog` and `system_channel`.

