# 2026-04-06 Special Conversation Generic Create Freeze Review Cycle

## 1. Findings

### 1.1 High: special conversation types were still creatable through the generic create API

- Root cause:
  - The generic `create_conversation` path still accepted `agent_dialog / agent_handoff / system_channel`.
  - Those types do not yet have dedicated create payloads or dedicated lifecycle commands.
- Evidence:
  - Generic member governance for those types is already intentionally closed.
  - Yet generic create still produced a persisted conversation with only the creator membership.

### 1.2 High: generic create produced semantically incomplete special conversations

- `agent_dialog`
  - should represent a user-agent dialogue, but generic create produced only one active member.
- `agent_handoff`
  - should represent a handoff topology, but generic create produced no source/target handoff structure.
- `system_channel`
  - should represent a broadcast/system channel, but generic create produced only a single creator member and no dedicated subscriber model.

## 2. Design Decision

The current safest commercial-grade standard is:

- Generic `POST /im/v3/api/chat/conversations` supports only:
  - `group`
  - `direct`
- The following remain valid data-model values but are reserved for future dedicated create commands:
  - `agent_dialog`
  - `agent_handoff`
  - `system_channel`
- Reserved types must be rejected by generic create with:
  - HTTP `400`
  - code `conversation_type_invalid`

This is stricter than the previous wave, but it eliminates half-implemented aggregates from the public write path.

## 3. Tests Added/Updated

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_generic_create_rejects_unknown_and_reserved_special_conversation_types`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_generic_create_rejects_reserved_special_types_over_http`
  - `test_group_create_preserves_actor_kind_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_generic_create_rejects_reserved_special_types_in_local_profile`

## 4. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - generic create now only allows `group / direct`
  - special types now return `conversation_type_invalid` with a dedicated-create message
- existing creator identity propagation remains preserved for supported generic creates

## 5. Verification

- `cargo test -p conversation-runtime test_generic_create_rejects_unknown_and_reserved_special_conversation_types --offline`
- `cargo test -p conversation-runtime test_generic_create_rejects_reserved_special_types_over_http --offline`
- `cargo test -p conversation-runtime test_group_create_preserves_actor_kind_over_http --offline`
- `cargo test -p sdkwork-im-cloud-gateway test_generic_create_rejects_reserved_special_types_in_local_profile --offline`

## 6. Remaining Risks

- Dedicated create commands for `agent_dialog / agent_handoff / system_channel` are still not implemented.
- The data model allows those types, but public write-path support is intentionally deferred.
- Membership episode audit/query views remain a later wave.

## 7. Next Wave

1. Define dedicated create commands for reserved special conversation types.
2. Freeze the participant topology for each special type before opening any write path.
3. Extend audit/policy/moderation standards after the dedicated create contracts are defined.
