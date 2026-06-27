> Migrated from `docs/架构/77-read-cursor-runtime-actor-kind-guard-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 77. Read Cursor Runtime Actor Kind Guard Standard (2026-04-06)

## 1. Goal

Read cursor updates must be rejected when ingress actor kind does not match the resolved active conversation member kind for the same actor id.

This rule ensures:

- read-state mutations use the same identity boundary as message and governance writes
- unread counters, cursor events, and audit anchors cannot be advanced by spoofed actor-kind claims
- cloud and local deployment profiles enforce the same contract

## 2. Scope

This standard applies to all conversation read-cursor write paths:

- runtime API
- HTTP API
- local deployment profile
- any future gateway or clustered adapter that advances conversation read state

## 3. Required Boundary Rule

Every untrusted read-cursor write must provide:

- tenant id
- conversation id
- actor id
- actor kind
- requested read cursor payload

At the runtime boundary, execution order is mandatory:

1. resolve active member by actor id
2. validate:
   - `actor_kind == member.principal_kind`
3. validate read cursor semantics:
   - target sequence does not exceed high watermark
   - monotonic cursor rules
4. mutate cursor state
5. append events and trigger downstream projections

If actor kind mismatches member truth, the write must fail as:

- runtime: `PermissionDenied`
- API code: `conversation_permission_denied`
- HTTP status: `403`

## 4. Runtime API Standard

`conversation-runtime` must expose an actor-kind-aware read cursor mutation:

- `update_read_cursor_with_actor_kind(command, actor_kind)`

Legacy `update_read_cursor(command)` may remain only as a compatibility wrapper for trusted/internal callers, and it must:

- derive actor kind from runtime member truth
- delegate into `update_read_cursor_with_actor_kind(...)`

Untrusted adapters must not call the compatibility wrapper directly.

## 5. Adapter Standard

All public adapters must pass raw ingress actor kind into the runtime read-cursor write boundary.

Required current adapters:

- `services/conversation-runtime` HTTP handler
- `services/sdkwork-im-cloud-gateway` HTTP handler

Adapters must not normalize actor kind before runtime authorization. The runtime is the authority.

## 6. Verification Standard

This mutation family must always have:

1. runtime unit test proving mismatch is rejected
2. HTTP smoke test proving forged `x-actor-kind` is rejected
3. local deployment e2e test proving forged bearer `actor_kind` is rejected

## 7. Relationship To Other Standards

This standard extends the same actor identity invariant already enforced for:

- message post/edit/recall
- agent handoff lifecycle writes
- member governance writes

Read state is not exempt from identity hardening just because it is “user-local” state; it is still a durable conversation mutation.

## 8. Follow-Up Direction

Continue applying this contract to all remaining conversation-bound write surfaces until the kernel exposes a uniform actor-kind-aware mutation boundary across the entire IM runtime.

