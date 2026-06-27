# 78. Conversation-Bound Write Capability Actor Kind Standard (2026-04-06)

## 1. Goal

Any write that is authorized by conversation membership and then delegated into another runtime must reject ingress actor-kind spoofing before the delegated mutation executes.

This rule ensures:

- stream and RTC delegated writes use the same identity boundary as core conversation mutations
- capability gates cannot be abused as actor-kind bypass layers
- downstream sender/event metadata cannot be stamped with forged actor-kind claims
- cloud and local deployment profiles enforce the same identity invariant

## 2. Scope

This standard applies to any conversation-bound write capability gate, including:

- stream mutations bound to conversation scope
- RTC mutations bound to conversation scope
- any future notification, workflow, webhook, or event runtime that is authorized through conversation membership
- local profile and clustered adapters that authorize first and delegate second

## 3. Required Boundary Rule

Every untrusted conversation-bound delegated write must provide:

- tenant id
- conversation id
- actor id
- actor kind
- requested capability

At the runtime gate boundary, execution order is mandatory:

1. resolve active member by actor id
2. validate:
   - `actor_kind == member.principal_kind`
3. evaluate conversation-type capability rules
4. only after the gate succeeds, delegate into the downstream runtime mutation
5. emit downstream side effects only after delegated mutation succeeds

If actor kind mismatches member truth, the write must fail as:

- runtime: `PermissionDenied`
- API code: `conversation_permission_denied`
- HTTP status: `403`

## 4. Runtime API Standard

`conversation-runtime` must expose an actor-kind-aware capability gate:

- `ensure_conversation_bound_write_allowed_with_actor_kind(...)`

Legacy `ensure_conversation_bound_write_allowed(...)` may remain only as a compatibility wrapper for trusted/internal callers, and it must:

- derive actor kind from runtime member truth
- delegate into `ensure_conversation_bound_write_allowed_with_actor_kind(...)`

Untrusted adapters must not call the compatibility wrapper directly.

## 5. Adapter Standard

All public adapters that authorize conversation-bound delegated writes must pass raw ingress actor kind into the runtime capability gate.

Required current adapter:

- `services/sdkwork-im-cloud-gateway`

Current covered delegated write families:

- `stream.open`
- `stream.append`
- `stream.checkpoint`
- `stream.complete`
- `stream.abort`
- `rtc.create`
- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`
- `rtc.signal`

Adapters may continue passing raw `auth` into downstream runtimes only after the actor-kind-aware capability gate succeeds. Downstream runtimes are not the authority for conversation membership truth.

## 6. Verification Standard

This boundary family must always have:

1. runtime unit test proving the generic capability gate rejects actor-kind mismatch
2. local deployment e2e proving forged bearer actor kind is rejected for conversation-bound stream writes
3. local deployment e2e proving forged bearer actor kind is rejected for conversation-bound RTC writes
4. regression evidence that rejected writes do not leave downstream persisted side effects

## 7. Relationship To Other Standards

This standard extends the same actor identity invariant already enforced for:

- message write/edit/recall
- agent handoff lifecycle writes
- member governance writes
- read-cursor writes

The fact that a mutation is delegated into stream or RTC infrastructure does not weaken the identity boundary. The conversation capability gate is part of the security boundary and must be hardened accordingly.

## 8. Follow-Up Direction

Continue applying this contract to all conversation-authorized delegated runtimes until every public write path in the IM platform passes through a uniform actor-kind-aware authorization boundary before any downstream mutation or side effect occurs.
