# 2026-04-06 Special Conversation Lifecycle Review Cycle

## 1. Findings

### 1.1 High: `agent_handoff` had create semantics but no durable lifecycle truth

- Root cause:
  - The runtime already supported dedicated create for `agent_handoff`, but `ConversationState` only modeled members, cursors, and messages.
  - There was no authoritative handoff status machine for `open / accepted / resolved / closed`.
- Impact:
  - Commercial handoff flows could not answer core operational questions such as:
    - has the target accepted takeover
    - has the handoff been resolved
    - is the handoff already closed
  - SLA, workflow, automation, and audit semantics would all be forced to infer state from free-form messages, which is not acceptable durable truth.

### 1.2 High: a closed handoff could not be expressed, so writes would remain open forever

- Root cause:
  - `post_message / edit_message / recall_message` had no special lifecycle gate for `agent_handoff`.
  - Even if the product wanted to close a handoff operationally, the server had no state to enforce write closure.
- Impact:
  - An already-completed handoff could still accept new chat writes, which breaks auditability and downstream automation.

### 1.3 Medium: special conversation lifecycle matrix was still implicit instead of frozen

- Root cause:
  - `agent_dialog / agent_handoff / system_channel` each had dedicated create semantics, but their post-create lifecycle boundaries were still only partially described across previous review waves.
- Impact:
  - Future iterations would risk re-opening generic governance or inventing inconsistent per-type mutations.

## 2. Design Decision

This wave freezes the lifecycle matrix first, then lands the smallest commercially useful state machine:

- `agent_dialog`
  - dedicated create only
  - no dedicated close in this wave
  - generic member governance remains closed
- `system_channel`
  - dedicated create only
  - publisher-only message post
  - no scheduled/bulk publish delegation in this wave
- `agent_handoff`
  - dedicated create
  - dedicated state read
  - dedicated lifecycle transitions:
    - `open -> accepted`
    - `accepted -> resolved`
    - `open|accepted|resolved -> closed`
  - actor rules:
    - `accept`: target only
    - `resolve`: target only
    - `close`: source or target
  - once `closed`, conversation message post/edit/recall are rejected

## 3. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - added durable `AgentHandoffStateView` state to runtime conversation memory
  - added:
    - `get_agent_handoff_state(...)`
    - `accept_agent_handoff_with_actor_kind(...)`
    - `resolve_agent_handoff_with_actor_kind(...)`
    - `close_agent_handoff_with_actor_kind(...)`
  - added `conversation.agent_handoff_status_changed` event
  - added `RuntimeError::Conflict` and mapped it to HTTP `409`
  - rejected `agent_handoff` post/edit/recall once status becomes `closed`
  - exposed routes:
    - `GET /im/v3/api/chat/conversations/{conversationId}/agent-handoff`
    - `POST /im/v3/api/chat/conversations/{conversationId}/agent-handoff/accept`
    - `POST /im/v3/api/chat/conversations/{conversationId}/agent-handoff/resolve`
    - `POST /im/v3/api/chat/conversations/{conversationId}/agent-handoff/close`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - exposed the same dedicated read/write lifecycle routes in the local profile
  - aligned `409 conversation_conflict` mapping with runtime

## 4. Tests Added

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_agent_handoff_accept_resolve_close_state_machine_and_closed_handoff_rejects_posts`
  - `test_agent_handoff_accept_requires_target_and_resolve_requires_accepted_state`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_agent_handoff_accept_resolve_close_over_http`
  - `test_agent_handoff_accept_rejects_non_target_actor_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_agent_handoff_accept_resolve_close_in_local_profile`
  - `test_agent_handoff_accept_rejects_source_actor_in_local_profile`

## 5. Verification

- Red phase:
  - `cargo test -p conversation-runtime --offline test_agent_handoff_accept_resolve_close_state_machine_and_closed_handoff_rejects_posts`
  - failed because the runtime had no handoff lifecycle commands, no state query, and no `409 conflict` support
- Green phase:
  - `cargo test -p conversation-runtime --offline test_agent_handoff_accept_resolve_close_state_machine_and_closed_handoff_rejects_posts`
  - `cargo test -p conversation-runtime --offline test_agent_handoff_accept_resolve_close_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline test_agent_handoff_accept_resolve_close_in_local_profile`

## 6. Remaining Risks

- `agent_handoff` status is now durable in runtime, but inbox/summary projections do not yet surface handoff status directly.
- `agent_dialog` still has no dedicated close/archive lifecycle command.
- `system_channel` still has no scheduled/bulk publish command, delegation policy, or mute/moderation lifecycle.
- Cross-service conversation-bound capabilities such as stream/RTC do not yet consume special-conversation lifecycle state.

## 7. Next Wave

1. Project `agent_handoff` status into read models so inbox/summary/admin views can query lifecycle state directly.
2. Freeze `agent_dialog` dedicated close/archive semantics instead of leaving it permanently open-only.
3. Review whether `system_channel` needs a dedicated publish API for scheduled and batch delivery rather than relying only on standard message post.
