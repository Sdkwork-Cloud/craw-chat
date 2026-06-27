# 2026-04-06 Agent Handoff Dedicated Create Review Cycle

## 1. Findings

### 1.1 High: `agent_handoff` still had no dedicated create contract

- Root cause:
  - The generic `POST /im/v3/api/chat/conversations` path was already correctly frozen to `group / direct`.
  - `agent_handoff` remained a reserved model type without a dedicated create command or route.
- Impact:
  - The platform could not create a semantically correct handoff conversation carrying source actor, target actor, and handoff metadata.

### 1.2 High: even if created, `agent_handoff` would still be unusable for normal handoff dialogue

- Root cause:
  - `post_message(...)` only allowed `group / direct / agent_dialog`, while special write policy for `agent_handoff` did not exist.
- Impact:
  - A handoff conversation could exist in storage but still reject all message posts as an unsupported conversation type.

## 2. Design Decision

The current safest vertical slice is:

- Keep generic `POST /im/v3/api/chat/conversations` limited to:
  - `group`
  - `direct`
- Open a dedicated create route only for `agent_handoff`:
  - `POST /im/v3/api/chat/conversations/agent_handoffs`
- Request body accepts:
  - `conversationId`
  - `targetId`
  - `targetKind`
  - `handoffSessionId`
  - `handoffReason`
- The source actor identity comes only from auth context:
  - no `tenantId`
  - no `sourceId`
  - no `sourceKind`
- Dedicated create is restricted to:
  - `actor_kind=agent`
- Target kinds are currently restricted to:
  - `user`
  - `agent`
- The runtime creates exactly two active members:
  - source agent: `principalKind=agent`, `role=owner`, `attributes.handoffRole=source`
  - target principal: `principalKind=user|agent`, `role=member`, `attributes.handoffRole=target`
- Read cursors are initialized for both members.
- `agent_handoff` message posting follows the same active-member post rule as `agent_dialog`.
- Generic member governance for `agent_handoff` remains closed:
  - no generic `add/remove/leave/transfer-owner/change-role`

## 3. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - added `CreateAgentHandoffCommand`
  - added `create_agent_handoff_with_source_kind(...)`
  - enforced `source_kind == agent`
  - enforced `target_kind in {user, agent}`
  - enforced non-empty `handoffSessionId`
  - created source and target memberships together
  - initialized read cursors for both members
  - exposed `POST /im/v3/api/chat/conversations/agent_handoffs`
  - extended message post policy so `agent_handoff` can carry real handoff dialogue
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - exposed the same dedicated create route on the local profile
  - mapped auth context into the runtime command without exposing source identity in the body

## 4. Tests Added

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_create_agent_handoff_creates_source_agent_and_target_members`
  - `test_create_agent_handoff_rejects_non_agent_source_kind`
  - `test_agent_handoff_allows_source_and_target_posts`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_create_agent_handoff_over_http`
  - `test_create_agent_handoff_rejects_non_agent_actor_over_http`
  - `test_agent_handoff_target_can_post_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_agent_handoff_create_in_local_profile_creates_agent_and_target_members`
  - `test_agent_handoff_create_rejects_non_agent_creator_in_local_profile`
  - `test_agent_handoff_target_can_post_in_local_profile`

## 5. Verification

- `cargo test -p conversation-runtime --offline test_create_agent_handoff_creates_source_agent_and_target_members`
- `cargo test -p conversation-runtime --offline test_agent_handoff_allows_source_and_target_posts`
- `cargo test -p conversation-runtime --offline test_create_agent_handoff_over_http`
- `cargo test -p conversation-runtime --offline test_agent_handoff_target_can_post_over_http`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_agent_handoff_create_in_local_profile_creates_agent_and_target_members`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_agent_handoff_target_can_post_in_local_profile`

## 6. Remaining Risks

- `agent_handoff` still has no dedicated close/accept/resolve lifecycle command.
- `agent_handoff` currently models a minimal two-party handoff conversation; it does not yet carry richer operational state such as SLA, queue assignment, or takeover completion.
- Special conversation generic member governance remains intentionally closed outside dedicated commands.

## 7. Next Wave

1. Freeze the lifecycle matrix for `agent_dialog / agent_handoff / system_channel`.
2. Decide whether `agent_handoff` needs dedicated accept/resolve/end commands instead of reusing generic message flow only.
3. Review whether `system_channel` now needs specialized publish APIs for batch or scheduled delivery.
