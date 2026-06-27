> Migrated from `docs/架构/76-governance-runtime-actor-kind-guard-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 76. Governance Runtime Actor Kind Guard Standard (2026-04-06)

## 1. Goal

Member-governance writes must be rejected when ingress actor kind does not match the resolved active member kind for the same actor id.

This rule exists to ensure:

- authorization truth is enforced in the runtime kernel, not repaired later by side effects
- durable mutation semantics, audit semantics, and realtime semantics stay aligned
- public HTTP, bearer-token, and local deployment profiles share the same hard boundary

## 2. Scope

This standard applies to conversation governance writes:

- add member
- remove member
- leave conversation
- transfer conversation owner
- change conversation member role

## 3. Required Boundary Rule

For every untrusted governance write ingress:

1. resolve auth context
2. pass:
   - tenant id
   - conversation id
   - actor id
   - actor kind
   - domain payload
   into `conversation-runtime`
3. inside runtime:
   - resolve the active member by actor id
   - compare supplied actor kind with resolved member `principal_kind`
   - reject on mismatch before permission checks, mutation, journaling, audit, or realtime side effects

If actor kind mismatches member truth, the write must fail as:

- runtime error: `PermissionDenied`
- API contract: `conversation_permission_denied`
- HTTP status: `403`

## 4. Runtime API Standard

`conversation-runtime` must expose actor-kind-aware governance entry points for all untrusted adapters.

Standard shape:

- `*_with_actor_kind(command, actor_kind)`

Current required methods:

- `add_member_with_actor_kind(...)`
- `remove_member_with_actor_kind(...)`
- `leave_conversation_with_actor_kind(...)`
- `transfer_conversation_owner_with_actor_kind(...)`
- `change_conversation_member_role_with_actor_kind(...)`

Inside each method, the order is mandatory:

1. `resolve_active_member(...)`
2. `ensure_actor_kind_matches_member(...)`
3. governance permission checks
4. mutation
5. event append / downstream side effects

## 5. Compatibility Wrapper Rule

Legacy id-only runtime methods may remain only as compatibility wrappers for trusted/internal call sites.

Those wrappers must:

- derive actor kind from runtime-resolved member truth
- delegate into the actor-kind-aware implementation

Untrusted ingresses must not call the compatibility wrappers directly.

## 6. Adapter Rule

All public or semi-public adapters must pass raw ingress actor kind into runtime governance writes.

This includes:

- `services/conversation-runtime` HTTP handlers
- `services/sdkwork-im-cloud-gateway` governance handlers
- future gateway or cluster-facing governance adapters

Adapters must not normalize or rewrite actor kind before runtime authorization.
Normalization is acceptable only for post-authorization side-effect shaping after the runtime has accepted the mutation.

## 7. Verification Standard

Every governance mutation family must have all three test layers:

1. runtime unit test:
   - mismatched actor kind is rejected directly by the kernel
2. runtime HTTP smoke test:
   - forged `x-actor-kind` or equivalent ingress metadata returns `403`
3. deployment-profile e2e test:
   - forged bearer or trusted-header actor kind returns `403`

## 8. Non-Goals

This standard does not redefine:

- membership role matrix
- owner transfer semantics
- side-effect payload formatting

It only fixes the actor identity consistency rule at the governance write boundary.

## 9. Follow-Up Direction

After governance writes, the same review standard should be applied to all remaining state-mutating conversation-bound APIs so the kernel exposes a uniformly constrained write surface.

