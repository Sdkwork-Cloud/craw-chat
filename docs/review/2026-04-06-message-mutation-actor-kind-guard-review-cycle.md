# 2026-04-06 Message Mutation Actor Kind Guard Review Cycle

## 1. Finding

### 1.1 High: `edit_message` and `recall_message` still trusted caller-supplied actor kind

- Root cause:
  - earlier runtime hardening already added:
    - `ensure_actor_kind_matches_member(...)` for `post_message(...)`
    - explicit actor-kind preservation for governance audit events
  - but the message mutation paths still only resolved the active member by actor id and then continued with the caller-supplied mutation actor:
    - `command.editor.kind`
    - `command.recalled_by.kind`
- Impact:
  - a direct runtime caller could edit or recall its own message while spoofing a different actor kind in the mutation command
  - the write itself succeeded because membership and mutation permission checks passed
  - downstream audit and event consumers could observe a false mutation actor identity even though the durable member truth said otherwise

## 2. Reproduction

Two regression tests were added before the fix:

- `test_edit_message_rejects_editor_kind_mismatch_against_member_principal_kind`
- `test_recall_message_rejects_actor_kind_mismatch_against_member_principal_kind`

Red verification proved the defect was real:

- `cargo test -p conversation-runtime --offline test_edit_message_rejects_editor_kind_mismatch_against_member_principal_kind`
  - failed because edit still succeeded with `editor.kind = agent` for a `user` member
- `cargo test -p conversation-runtime --offline test_recall_message_rejects_actor_kind_mismatch_against_member_principal_kind`
  - failed because recall still succeeded with `recalled_by.kind = agent` for a `user` member

## 3. Fix Design

The correct boundary is the same one already used by `post_message(...)`:

- resolve the active member from durable conversation membership truth
- verify the request actor kind matches `ConversationMember.principal_kind`
- only then continue with message mutation authorization and state change

This keeps mutation paths aligned with:

- message post
- governance mutations
- special conversation lifecycle transitions

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - added `ensure_actor_kind_matches_member(&editor_member, command.editor.kind.as_str())?` in `edit_message(...)`
  - added `ensure_actor_kind_matches_member(&recalled_member, command.recalled_by.kind.as_str())?` in `recall_message(...)`
- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - added the two regression tests above

The repair is intentionally minimal:

- no route changes
- no payload shape changes
- no new event types
- no mutation permission redesign

## 5. Verification

### Red

- `cargo test -p conversation-runtime --offline test_edit_message_rejects_editor_kind_mismatch_against_member_principal_kind`
- `cargo test -p conversation-runtime --offline test_recall_message_rejects_actor_kind_mismatch_against_member_principal_kind`

Both failed before the fix with `PermissionDenied` assertions unmet because the runtime still allowed the spoofed mutation.

### Green

- `cargo test -p conversation-runtime --offline test_edit_message_rejects_editor_kind_mismatch_against_member_principal_kind`
- `cargo test -p conversation-runtime --offline test_recall_message_rejects_actor_kind_mismatch_against_member_principal_kind`
- `cargo test -p conversation-runtime --offline test_edit_and_recall_message_emit_mutation_events_without_changing_sequence`

## 6. Remaining Risks

- HTTP and local profile tests still do not explicitly assert mutation actor-kind rejection at the API boundary.
- Other mutation or moderation entry points added later could reintroduce the same trust gap if they accept actor kind from the request but do not reconcile against membership truth.
- Projection or automation consumers that infer business semantics from mutation actor identity still rely on future broader event-payload review coverage.

## 7. Next Wave

1. Review HTTP and local profile mutation endpoints to decide whether dedicated API regression tests should pin the same actor-kind rejection behavior.
2. Audit other write-side commands for any remaining actor identity fields that are accepted from callers without reconciliation against durable member truth.
3. Extend review coverage from runtime mutation acceptance to downstream projection and notification consumers that read mutation actor metadata.
