# 2026-04-06 Special Conversation Creation Review Cycle

> Superseded on 2026-04-06 by `2026-04-06-special-conversation-generic-create-freeze-review-cycle.md`.
> The earlier decision to expose special conversation types through generic create was revised after the next review wave found that those types had no dedicated create payload or lifecycle contract.

## 1. Findings

### 1.1 High: unsupported `conversationType` values could be created successfully

- Root cause:
  - `conversation-runtime::create_conversation(...)` accepted raw strings without validating the supported conversation type set.
  - This allowed typos or undocumented custom types to enter the system and then fall into inconsistent downstream behavior.
- Risk:
  - Unsupported types could be stored, projected, and messaged before later governance APIs rejected them.
  - Error semantics were delayed from create-time to runtime mutation-time, which is the wrong boundary.

### 1.2 High: creator `actor_kind` was dropped during conversation creation

- Root cause:
  - Conversation creation always wrote the creator membership with `principal_kind = "user"`.
  - The emitted `conversation.created` event also hard-coded `actor_kind = "user"`.
- Risk:
  - `system_channel` and other special conversation types could not preserve service/system identities.
  - Projection, audit, and future policy logic would all read a polluted creator identity model.

## 2. Fix Strategy

- Freeze the supported `conversationType` whitelist at create time:
  - `group`
  - `direct`
  - `agent_dialog`
  - `agent_handoff`
  - `system_channel`
- Reject any other value at create time with `400 conversation_type_invalid`.
- Add a creator-kind-aware creation path used by app-facing HTTP entrypoints.
- Propagate `auth.actor_kind` into:
  - creator membership `principalKind`
  - `conversation.created` event actor metadata
- Keep current governance boundary unchanged:
  - `agent_dialog / agent_handoff / system_channel` are allowed as explicit types
  - generic `add/remove/leave/transfer-owner/change-role` remains rejected until dedicated lifecycle commands are standardized

## 3. Tests Added

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_create_conversation_rejects_unknown_conversation_type`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_create_conversation_rejects_unknown_type_over_http`
  - `test_create_system_channel_preserves_actor_kind_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_system_channel_create_preserves_actor_kind_in_local_profile`

## 4. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - Added create-time supported type validation
  - Added `RuntimeError::ConversationTypeInvalid`
  - Added `create_conversation_with_creator_kind(...)`
  - Ensured `conversation.created` event actor uses real creator kind
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - App-facing create endpoint now passes `auth.actor_kind`
  - Added `conversation_type_invalid` error mapping

## 5. Verification

- `cargo test -p conversation-runtime test_create_conversation_rejects_unknown_conversation_type --offline`
- `cargo test -p conversation-runtime test_create_conversation_rejects_unknown_type_over_http --offline`
- `cargo test -p conversation-runtime test_create_system_channel_preserves_actor_kind_over_http --offline`
- `cargo test -p sdkwork-im-cloud-gateway test_system_channel_create_preserves_actor_kind_in_local_profile --offline`

## 6. Remaining Risks

- `agent_dialog / agent_handoff / system_channel` still do not have dedicated lifecycle commands.
- Special conversation types are now validated and identity-correct at creation time, but their later governance remains intentionally closed.
- Membership episode read/audit query views are still a later wave.

## 7. Next Wave

1. Freeze the dedicated lifecycle/governance matrix for `agent_dialog / agent_handoff / system_channel`.
2. Decide whether special types need dedicated create commands or additional creation payload.
3. Review whether policy/moderation/audit should branch on `conversation_type` for special channels.
