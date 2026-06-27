# 75. Local Profile Governance Side-Effect Actor Kind Standard (2026-04-06)

## 1. Objective

When local profile code emits audit or realtime side effects for conversation governance mutations, actor kind must match durable conversation member truth.

Local side effects must not trust `auth.actor_kind` when the runtime can resolve the actor member.

## 2. Scope

This standard applies to `sdkwork-im-server` side effects for:

- `conversation.member_joined`
- `conversation.member_removed`
- `conversation.member_role_changed`
- `conversation.member_left`
- `conversation.owner_transferred`

It governs:

- audit anchors
- realtime governance payloads

It does not change:

- runtime mutation authorization rules
- public auth policy
- conversation event schemas

## 3. Truth Source Rule

For governance side effects, actor kind must come from:

- `conversation_runtime.require_active_member(...).principal_kind`

not from:

- `auth.actor_kind`

if both are available.

The runtime-resolved member is the durable conversation truth.

The auth context is only transport identity input.

## 4. Why This Boundary Exists

Auth context can be correct for authentication and still be unsuitable as side-effect truth:

- trusted headers may intentionally inject actor kind in tests or internal chains
- runtime governance methods often authorize by resolved conversation member
- local profile side effects can drift if they keep using raw auth metadata

This creates a split where:

- durable event truth says actor kind = real member kind
- local realtime or audit truth says actor kind = request auth kind

That split is not allowed.

## 5. Local Profile Rule

Before emitting governance side effects, the local profile must normalize actor auth context to the resolved member kind.

An acceptable implementation is:

1. clone `AuthContext`
2. resolve active member from `conversation_runtime`
3. overwrite cloned `actor_kind` with `member.principal_kind`
4. use the normalized auth context for audit and realtime emission

## 6. Leave Special Rule

`leave_conversation(...)` is special because the actor is no longer active after the mutation succeeds.

Therefore:

- the local profile must resolve the actor member kind before executing the leave mutation

Waiting until after the mutation is too late.

## 7. Coverage Rule

Implementations must include:

1. a local-profile regression test proving governance realtime and audit side effects preserve runtime actor kind when auth kind is spoofed
2. a local-profile regression test proving owner-transfer audit preserves runtime actor kind when auth kind is spoofed
3. a regression test proving normal governance realtime fanout still works after the repair

## 8. Non-Goals

This standard does not yet require:

- runtime governance APIs to reject actor-kind mismatch
- public bearer claim redesign
- downstream projection schema changes

Those are later hardening waves.

This wave only freezes the rule that local-profile side effects must follow runtime member truth.
