# 74. Message Mutation Actor Kind Guard Standard (2026-04-06)

## 1. Objective

Message mutation commands must not trust caller-supplied actor kind.

For edit and recall flows, actor identity must stay consistent with durable conversation membership truth, exactly as already required for message post and governance mutations.

## 2. Scope

This standard governs:

- `edit_message(...)`
- `recall_message(...)`
- any API or profile entry point that forwards those mutation commands

It does not redefine:

- who is allowed to edit
- who is allowed to recall
- message event schemas
- message storage layout

## 3. Runtime Trust Rule

Before applying any message mutation, the runtime must:

1. resolve the active member by actor id
2. verify request actor kind equals `ConversationMember.principal_kind`
3. only then evaluate edit/recall authorization and mutate state

The runtime must reject any mismatch with:

- `conversation_permission_denied`

This rule belongs in the runtime, not only at HTTP or gateway edges.

## 4. Why This Boundary Exists

Membership resolution proves who the actor really is inside the conversation.

Caller-supplied `editor.kind` or `recalled_by.kind` is only transport input. It must not override the durable member identity.

Without this guard:

- a valid member id can mutate its own message
- the mutation can succeed
- audit or realtime consumers can receive a false actor kind

That is an integrity failure even if the message content change itself is otherwise authorized.

## 5. Mutation Command Rule

The following fields are untrusted until reconciled:

- `EditMessageCommand.editor.kind`
- `RecallMessageCommand.recalled_by.kind`

The server must never use those fields to describe the mutation actor unless they match the resolved member principal kind.

## 6. Consistency Rule

Actor-kind validation for message mutation must stay consistent with existing runtime hardening for:

- `post_message(...)`
- conversation governance mutations
- special conversation lifecycle transitions such as `agent_handoff`

No write path may apply a weaker actor identity standard than another write path in the same runtime.

## 7. API/Profile Rule

Any HTTP gateway, local profile, or internal service facade that exposes message edit or recall must preserve this behavior:

- mismatched actor kind is rejected
- rejection must come from runtime truth, not route-local heuristics
- successful mutation behavior remains unchanged for valid actors

## 8. Test Standard

Implementations must include:

1. a runtime regression test proving edit rejects actor-kind mismatch
2. a runtime regression test proving recall rejects actor-kind mismatch
3. a regression test proving normal edit/recall success behavior still works after the guard

Additional HTTP or local profile coverage is recommended when those surfaces add their own identity translation logic.

## 9. Non-Goals

This standard does not yet require:

- mutation-specific HTTP error body assertions
- downstream projection schema redesign
- moderation workflow redesign
- actor session/device trust redesign

Those can be hardened later, but they must not weaken this runtime identity guard.
