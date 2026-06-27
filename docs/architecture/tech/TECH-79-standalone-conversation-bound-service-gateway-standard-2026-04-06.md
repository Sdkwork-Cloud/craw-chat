> Migrated from `docs/架构/79-standalone-conversation-bound-service-gateway-standard-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 79. Standalone Conversation-Bound Service Gateway Standard (2026-04-06)

## 1. Goal

Standalone infrastructure services must not accept conversation-bound requests unless they are fronted by an authorizing IM gateway that owns conversation membership truth.

This rule ensures:

- standalone stream/RTC services cannot become authorization bypass points
- conversation-bound semantics stay attached to the IM gateway or integrated profile that can verify membership
- shared runtimes remain reusable while security stays at the correct adapter boundary

## 2. Scope

This standard applies to standalone service HTTP apps that expose generic infrastructure runtimes, including:

- `streaming-service`
- `im-call-runtime`
- any future standalone service whose request can directly or indirectly bind work to a conversation

## 3. Required Boundary Rule

If a standalone HTTP request is conversation-bound, the standalone service must reject it unless an upstream gateway has already transformed it into an authorized internal/runtime call.

Current conversation-bound request markers:

- stream:
  - request `scopeKind = conversation`
  - persisted stream session `scopeKind = conversation`
- RTC:
  - request carries `conversationId`
  - persisted RTC session carries `conversationId`

Rejection contract:

- HTTP status: `403`
- error code: `conversation_gateway_required`

## 4. Layering Rule

Shared runtimes may remain generic and conversation-agnostic.

The rejection must live at the standalone adapter boundary because:

- integrated profiles such as `sdkwork-im-server` legitimately reuse the same runtimes after successful conversation authorization
- moving this rule into the shared runtime would break authorized integrated flows

Therefore:

1. standalone HTTP handlers must detect conversation-bound requests/sessions
2. standalone HTTP handlers must reject them
3. authorized gateways may still call the underlying runtimes after enforcing conversation membership and capability rules

## 5. Positive Path Rule

Standalone services must continue supporting generic non-conversation use cases.

Current examples:

- generic stream scopes such as request-scoped or task-scoped transport flows
- RTC sessions that are not bound to an IM conversation

Security hardening must not collapse these generic capabilities.

## 6. Verification Standard

Each standalone service covered by this rule must have:

1. a regression test proving conversation-bound standalone create/open is rejected
2. positive standalone tests proving generic non-conversation use cases still succeed
3. bearer-auth tests proving public auth still works for standalone generic flows

## 7. Relationship To Other Standards

This standard complements the already implemented rules for:

- conversation-bound writes in `sdkwork-im-server`
- actor-kind-aware conversation authorization
- message/governance/read-cursor mutation hardening

Those rules secure the authorized integrated path.

This standard secures the inverse path:

- a generic standalone service must not pretend to be the conversation authorization boundary

## 8. Follow-Up Direction

Continue auditing all standalone services that accept payloads containing `conversationId`, conversation scope identifiers, or equivalent binding metadata, and apply the same gateway-only rule wherever the service does not own conversation membership authorization itself.

