# 2026-04-06 Agent Dialog Dedicated Create Review Cycle

## 1. Findings

### 1.1 High: `agent_dialog` had a data-model type but no safe public write path

- Root cause:
  - The previous review wave correctly froze generic `POST /im/v3/api/chat/conversations` to `group / direct`.
  - But `agent_dialog` still had no dedicated create command, so the platform could not create a commercially usable user-agent primary conversation through a safe public contract.
- Impact:
  - The model advertised an `agent_dialog` type, but the runtime and gateway still exposed no correct way to create the required two-party topology.

### 1.2 High: a non-user creator would produce the wrong `agent_dialog` semantics

- Root cause:
  - `agent_dialog` means a user-agent primary conversation.
  - If a `system` or other non-user principal could create it directly from auth context, the runtime would create a system-agent conversation instead of a user-agent conversation.
- Decision:
  - `agent_dialog` dedicated create is now restricted to authenticated `actor_kind=user`.

## 2. Design Decision

The current safest vertical slice is:

- Keep generic `POST /im/v3/api/chat/conversations` limited to:
  - `group`
  - `direct`
- Open a dedicated create route only for `agent_dialog`:
  - `POST /im/v3/api/chat/conversations/agent_dialogs`
- Request body accepts only:
  - `conversationId`
  - `agentId`
- The requester identity comes only from auth context:
  - no `tenantId`
  - no `requesterId`
  - no `requesterKind`
- The runtime creates exactly two active members:
  - requester: `principalKind=user`, `role=owner`
  - target agent: `principalKind=agent`, `role=member`
- `agent_handoff / system_channel` remain reserved for later dedicated contracts.

## 3. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - added `CreateAgentDialogCommand`
  - added `create_agent_dialog_with_requester_kind(...)`
  - enforced `requester_kind == user`
  - created requester and agent memberships together
  - initialized read cursors for both members
  - exposed `POST /im/v3/api/chat/conversations/agent_dialogs`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - exposed the same dedicated create route on the local profile
  - mapped auth context into the runtime command without exposing requester identity in the body

## 4. Tests Added

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_create_agent_dialog_creates_requester_and_agent_members`
  - `test_create_agent_dialog_rejects_non_user_requester_kind`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_create_agent_dialog_over_http`
  - `test_create_agent_dialog_rejects_non_user_actor_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_agent_dialog_create_in_local_profile_creates_user_and_agent_members`
  - `test_agent_dialog_create_rejects_non_user_creator_in_local_profile`

## 5. Verification

- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p conversation-runtime --test http_smoke_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`

## 6. Remaining Risks

- `agent_handoff` still has no dedicated source/target handoff contract.
- `system_channel` still has no dedicated subscriber/broadcast contract.
- Special conversation generic member governance is still intentionally closed outside the dedicated create path.
- Some non-create membership mutation envelopes still use simplified actor metadata; that audit-hardening wave is still open.

## 7. Next Wave

1. Define `system_channel` dedicated create with system-only creator and subscriber model.
2. Define `agent_handoff` dedicated create with explicit source/target/handoff metadata.
3. Harden member-governance audit envelopes so actor identity stays correct across all mutation events, not only conversation create.
