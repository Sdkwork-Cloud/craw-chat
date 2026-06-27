# 2026-04-06 System Channel Dedicated Create Review Cycle

## 1. Findings

### 1.1 High: `system_channel` had no dedicated create contract

- Root cause:
  - The generic `POST /im/v3/api/chat/conversations` path was already correctly frozen to `group / direct`.
  - `system_channel` remained a reserved data-model type without a dedicated create route, so the platform could not build a correct system-to-subscriber broadcast conversation through a public contract.
- Impact:
  - The model exposed `system_channel`, but runtime and gateway had no safe way to create the required publisher/subscriber topology.

### 1.2 High: a route-only fix would still leave the wrong message semantics

- Root cause:
  - `post_message(...)` only checked active membership.
  - If a `system_channel` were created without a type-specific write policy, any active subscriber could still post into the channel.
- Impact:
  - The platform would create something named `system_channel` but behave like a regular two-way conversation, which violates the intended broadcast contract.

## 2. Design Decision

The safest vertical slice is:

- Keep generic `POST /im/v3/api/chat/conversations` limited to:
  - `group`
  - `direct`
- Open a dedicated create route only for `system_channel`:
  - `POST /im/v3/api/chat/conversations/system_channels`
- Request body accepts only:
  - `conversationId`
  - `subscriberId`
- The requester identity comes only from auth context:
  - no `tenantId`
  - no `requesterId`
  - no `requesterKind`
- Dedicated create is restricted to:
  - `actor_kind=system`
- The runtime creates exactly two active members:
  - publisher: `principalKind=system`, `role=owner`, `attributes.channelRole=publisher`
  - subscriber: `principalKind=user`, `role=member`, `attributes.channelRole=subscriber`
- Read cursors are initialized for both members.
- `POST /im/v3/api/chat/conversations/{id}/messages` is constrained by conversation type:
  - `group / direct / agent_dialog`: active member may post
  - `system_channel`: only the system publisher may post
- Generic member governance for `system_channel` remains closed:
  - no generic `add/remove/leave/transfer-owner/change-role`

## 3. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - added `CreateSystemChannelCommand`
  - added `create_system_channel_with_requester_kind(...)`
  - enforced `requester_kind == system`
  - created publisher and subscriber memberships together
  - initialized read cursors for both members
  - exposed `POST /im/v3/api/chat/conversations/system_channels`
  - added `ensure_message_post_allowed(...)` and enforced system-channel publisher-only posting
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - exposed the same dedicated create route on the local profile
  - mapped auth context into the runtime command without exposing requester identity in the body

## 4. Tests Added

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_create_system_channel_creates_system_and_subscriber_members`
  - `test_create_system_channel_rejects_non_system_requester_kind`
  - `test_system_channel_rejects_subscriber_post_but_allows_system_publisher_post`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_create_system_channel_over_http`
  - `test_create_system_channel_rejects_non_system_actor_over_http`
  - `test_system_channel_subscriber_cannot_post_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_system_channel_create_in_local_profile_creates_system_and_subscriber_members`
  - `test_system_channel_create_rejects_non_system_creator_in_local_profile`
  - `test_system_channel_subscriber_cannot_post_in_local_profile`

## 5. Verification

- `cargo test -p conversation-runtime --offline test_create_system_channel_creates_system_and_subscriber_members`
- `cargo test -p conversation-runtime --offline test_system_channel_rejects_subscriber_post_but_allows_system_publisher_post`
- `cargo test -p conversation-runtime --offline test_create_system_channel_over_http`
- `cargo test -p conversation-runtime --offline test_system_channel_subscriber_cannot_post_over_http`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_create_in_local_profile_creates_system_and_subscriber_members`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_subscriber_cannot_post_in_local_profile`

## 6. Remaining Risks

- `system_channel` still has no specialized moderation, mute, or publish delegation contract.
- `agent_handoff` still has no dedicated create contract.
- Special conversation generic member governance remains intentionally closed outside dedicated commands.
- Message edit/recall rules are still shared across conversation types and may need a future special-type governance review.

## 7. Next Wave

1. Define `agent_handoff` dedicated create with explicit source actor, target actor, and handoff metadata.
2. Review whether `system_channel` needs dedicated publish APIs for scheduled/bulk notification delivery instead of reusing standard post only.
3. Continue special-conversation governance hardening so `agent_dialog / agent_handoff / system_channel` each have an explicit lifecycle matrix.
