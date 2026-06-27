# 2026-04-06 System Channel Cross-Capability Write Gate Review Cycle

## 1. Finding

### 1.1 High: `system_channel` subscriber could still write conversation-bound `stream` and `RTC`

- Root cause:
  - the earlier `system_channel` hardening only froze publisher-only access for message post.
  - conversation-bound capability write gates in `sdkwork-im-server` only checked:
    - active membership
    - conversation lifecycle conflict rules already exposed by `conversation-runtime`
  - but `conversation-runtime.ensure_conversation_bound_write_allowed(...)` had no actor context, so it could not enforce type-specific publisher-only rules for `system_channel`.
- Impact:
  - a subscriber in `system_channel` could still:
    - open new conversation-bound streams
    - append frames to an existing publisher-created stream
    - create new conversation-bound RTC sessions
    - continue RTC signaling on an existing publisher-created session
  - this breaks the broadcast contract already frozen for `system_channel`:
    - publisher writes
    - subscriber reads/consumes
  - it also creates semantic drift:
    - message post is publisher-only
    - but stream/RTC side channels remained subscriber-writable

## 2. Scope Freeze

This wave fixes only conversation-bound write access for `system_channel`.

It does not:

- add scheduled or batch publish APIs
- define moderation or mute policies for `system_channel`
- widen the rule to non-conversation-bound private stream/RTC sessions

## 3. Design Decision

The write policy must stay centralized in `conversation-runtime`, because `system_channel` semantics belong to conversation durable truth, not to leaf services.

So the implementation is:

- `conversation-runtime`
  - make conversation-bound write gating actor-aware
  - resolve the active member inside the runtime gate
  - apply `system_channel` publisher-only rules there
- `sdkwork-im-server`
  - call the runtime gate with:
    - tenant
    - conversation
    - actor principal
    - capability name

This keeps one authoritative place for:

- membership validation
- special conversation write rules
- lifecycle conflict rules

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - changed `ensure_conversation_bound_write_allowed(...)` to accept `principal_id`
  - runtime now resolves the active member before approving conversation-bound writes
  - added shared helper:
    - `ensure_system_channel_publisher_write_allowed(...)`
  - reused that helper for:
    - `message.post`
    - conversation-bound capability writes such as `stream.*` and `rtc.*`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - `ensure_conversation_bound_write_access(...)` now delegates actor-aware enforcement directly to `conversation-runtime`
  - removed duplicate pre-check membership call from the local access layer
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - added stream and rtc regression tests for subscriber-side writes in `system_channel`

## 5. Tests Added

- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_system_channel_subscriber_cannot_write_conversation_bound_streams_in_local_profile`
  - `test_system_channel_subscriber_cannot_write_conversation_bound_rtc_in_local_profile`

These tests prove both:

- subscriber cannot mutate an existing publisher-created conversation-bound capability session
- subscriber cannot create a new conversation-bound capability session

## 6. Verification

### Red

- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_subscriber_cannot_write_conversation_bound_streams_in_local_profile -- --exact`
  - failed with `200 != 403`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_subscriber_cannot_write_conversation_bound_rtc_in_local_profile -- --exact`
  - failed with `200 != 403`

### Green

- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_subscriber_cannot_write_conversation_bound_streams_in_local_profile -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_subscriber_cannot_write_conversation_bound_rtc_in_local_profile -- --exact`

## 7. Remaining Risks

- the current rule freezes subscriber writes for conversation-bound `stream/RTC`, but `system_channel` still lacks a dedicated scheduled or batch publish orchestration contract.
- future edge profiles must call the same actor-aware runtime gate; otherwise the invariant can regress outside `sdkwork-im-server`.
- `agent_dialog` still has no equivalent cross-capability lifecycle/governance standard.

## 8. Next Wave

1. Freeze whether `system_channel` needs a dedicated publish command model for scheduled and bulk notification delivery.
2. Audit whether other conversation-bound capabilities beyond `stream/RTC` need the same publisher-only enforcement.
3. Continue post-create lifecycle hardening for `agent_dialog`.
