# 2026-04-06 Audit Actor Kind Review Cycle

## 1. Findings

### 1.1 High: conversation governance audit events still downgraded actor identity to `user`

- Root cause:
  - `conversation.created` had already been hardened to preserve the real `actor_kind`.
  - But four mutation/governance event builders still accepted only `actor_id`.
  - Because the builder signatures had no `actor_kind` input, they fell back to `actor_kind = "user"`.
- Affected event families:
  - `conversation.member_joined`
  - `conversation.member_removed`
  - `conversation.member_left`
  - `conversation.read_cursor_updated`
  - `conversation.owner_transferred`
  - `conversation.member_role_changed`

### 1.2 High: create-path and mutation-path audit semantics had diverged

- The system could already create a group as `system` and record:
  - `conversation.created.actor.actor_kind = system`
- But the immediately following membership/governance events still recorded:
  - `actor.actor_kind = user`
- This meant the same authenticated actor produced contradictory audit identity inside one conversation lifecycle.

## 2. Reproduction

The defect was reproduced with four regression tests added before the fix:

- `test_create_group_member_joined_event_preserves_system_actor_kind`
- `test_read_cursor_event_preserves_agent_actor_kind`
- `test_owner_transfer_event_preserves_system_actor_kind`
- `test_member_role_changed_event_preserves_system_actor_kind`

Each test failed before the fix with the same mismatch pattern:

- actual: `user`
- expected: real actor kind such as `system` or `agent`

## 3. Fix Design

### 3.1 Standard

Audit actor metadata must follow the real authenticated or resolved runtime actor identity end to end.

- Builders must not invent `actor_kind`.
- Runtime mutation paths must carry both:
  - `actor_id`
  - `actor_kind`
- When the mutation is initiated by an active member, `actor_kind` must be derived from that member's `principal_kind`.

### 3.2 Mapping rules used in this repair

- `conversation.member_*`
  - actor kind comes from:
    - creator kind during conversation create
    - requester kind during `agent_dialog` create
    - inviter/remover/leaver active member kind during later membership mutations
- `conversation.read_cursor_updated`
  - actor kind comes from the active member who updates the cursor
- `conversation.owner_transferred`
  - actor kind comes from the transferring owner member
- `conversation.member_role_changed`
  - actor kind comes from the member performing the role change

## 4. Implementation Summary

- `services/conversation-runtime/src/lib.rs`
  - extended these builders to accept `actor_kind`:
    - `build_member_envelope(...)`
    - `build_read_cursor_envelope(...)`
    - `build_owner_transfer_envelope(...)`
    - `build_member_role_changed_envelope(...)`
  - propagated real actor kind from existing runtime context:
    - create paths use `creator_kind` or `requester_kind`
    - membership mutations use resolved active member `principal_kind`
    - read cursor mutation uses resolved active member `principal_kind`
- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - added four regression tests covering system and agent actor kinds

## 5. Verification

Targeted red-green verification was completed:

- red:
  - `cargo test -p conversation-runtime --offline test_create_group_member_joined_event_preserves_system_actor_kind`
  - `cargo test -p conversation-runtime --offline test_read_cursor_event_preserves_agent_actor_kind`
  - `cargo test -p conversation-runtime --offline test_owner_transfer_event_preserves_system_actor_kind`
  - `cargo test -p conversation-runtime --offline test_member_role_changed_event_preserves_system_actor_kind`
- green:
  - the same four commands passed after the fix

Full crate and workspace verification is executed in the close-out phase of this cycle.

## 6. Remaining Risks

- HTTP and local profile mutation tests still focus more on permission and behavior than audit payload introspection.
- Future dedicated special conversation contracts such as `system_channel` and `agent_handoff` must follow the same actor identity standard from day one.
- If new governance events are added later, the same regression can return unless every new envelope builder requires explicit actor identity input.

## 7. Next Wave

1. Add dedicated create standards for `system_channel`.
2. Add dedicated create standards for `agent_handoff`.
3. Extend audit hardening review to projection and downstream automation consumers that rely on event actor metadata.
