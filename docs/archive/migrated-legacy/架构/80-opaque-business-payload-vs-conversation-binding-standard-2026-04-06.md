# 80. Opaque Business Payload vs Conversation-Binding Standard (2026-04-06)

## 1. Goal

The platform must distinguish between:

- authorization-bearing fields that bind a request or persisted session to a conversation
- opaque business payload that may incidentally contain a `conversationId` string but is not interpreted as authority

Without this distinction, the system either:

- leaves real authorization bypasses unprotected
- or incorrectly hardens harmless business payload and breaks extensibility

## 2. Definitions

### 2.1 Conversation-binding control field

A field is conversation-binding when it directly determines any of the following:

- which conversation a request operates on
- whether new state becomes attached to a conversation
- which conversation membership set is used for authorization
- which conversation receives side effects such as messages, RTC projections, stream projections, or governance mutations

Current examples:

- top-level route or request identifiers such as:
  - `/im/v3/api/chat/conversations/{conversationId}/...`
  - request `conversationId`
- stream binding fields:
  - `scopeKind = conversation`
  - `scopeId = <conversationId>`
- persisted RTC or stream session fields whose stored binding points to a conversation

### 2.2 Opaque business payload

A field is opaque business payload when the service:

- accepts it as a blob or string
- stores or forwards it
- may expose it back to callers or recipients
- does not interpret its internal structure as authorization or routing truth

Current examples:

- `automation-service`
  - `RequestAutomationExecution.input_payload`
  - `AutomationExecution.output_payload`
- `notification-service`
  - `RequestNotification.payload`
- audit and telemetry payload blobs that are stored for traceability only

## 3. Classification Rule

The presence of a JSON key named `conversationId` inside a payload blob does not, by itself, make that payload conversation-binding.

Only fields that the service actually interprets as conversation authority count as conversation-binding control inputs.

Therefore:

1. naming alone is not security semantics
2. interpretation and side effect are security semantics
3. authorization rules must follow actual control boundaries, not incidental JSON shape

## 4. Required Security Rule

If a field is conversation-binding, the adapter that owns conversation authorization truth must validate:

- tenant scope
- active membership or equivalent capability
- actor kind restrictions where applicable

If a field is opaque payload, the service must not:

- derive conversation authorization from it
- silently convert it into conversation mutation scope
- treat it as a substitute for explicit conversation-bound contract fields

## 5. Promotion Rule

If a future feature wants to use data currently stored inside opaque payload to drive conversation-bound side effects, that data must be promoted into an explicit contract field first.

Required steps:

1. define the field at the API or internal adapter boundary
2. classify it as conversation-binding
3. add authorization at that boundary
4. add regression tests for unauthorized and authorized paths
5. only then allow downstream runtime logic to act on it

This prevents hidden authority from leaking through arbitrary JSON blobs.

## 6. Service-Specific Application

### 6.1 `automation-service`

Current standard:

- `input_payload` and `output_payload` are opaque
- `automation.execute` / `automation.read` permissions govern access
- no conversation membership semantics are derived from payload internals

### 6.2 `notification-service`

Current standard:

- `payload` is opaque
- authorization is based on:
  - recipient self-scope
  - or `notification.write`
- no conversation membership semantics are derived from payload internals

### 6.3 `projection-service`

Current standard:

- only explicit conversation/message/member/read-cursor schemas are interpreted as conversation state
- automation/notification payload blobs are not projection inputs for conversation authorization or conversation state mutation

## 7. Relationship To Gateway Hardening Standards

This standard complements:

- `79-standalone-conversation-bound-service-gateway-standard-2026-04-06.md`
- earlier actor-kind-aware conversation write hardening standards

Those standards protect explicit conversation-bound control paths.

This standard explains why the same gateway rejection must not be applied blindly to every payload that merely contains a `conversationId` string.

## 8. Verification Standard

For any service carrying payload blobs that may include conversation-looking identifiers:

1. verify whether the payload is parsed into control decisions
2. verify whether any downstream consumer interprets that payload as conversation authority
3. if no such interpretation exists, document the payload as opaque
4. if such interpretation is introduced, reclassify the field as conversation-binding and harden the boundary

## 9. Design Consequence

This rule preserves both security and extensibility:

- security:
  - real conversation-binding inputs must be explicitly authorized
- extensibility:
  - workflows, bots, notifications, and application-defined data may carry conversation references as ordinary business data without forcing every generic service to become a conversation authorization gateway
