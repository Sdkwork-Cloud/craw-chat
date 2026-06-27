# 2026-04-06 Opaque Payload ConversationId Audit Review Cycle

## 1. Finding

### 1.1 No current authorization bypass: `conversationId` inside automation/notification payload is opaque business data, not a conversation-binding control field

- Audit target:
  - `services/automation-service`
  - `services/notification-service`
  - downstream/local integration and projection consumers
- Initial concern:
  - both services accept payloads that may contain text such as `{"conversationId":"c_demo"}`
  - after hardening standalone stream/RTC gateways, these payload-shaped identifiers looked like the next possible indirect conversation bypass surface
- Root-cause conclusion:
  - no current code path in this repo treats `automation.input_payload`, `automation.output_payload`, or `notification.payload` as an authorization-bearing conversation binding field
  - these fields are currently:
    - stored
    - serialized into events
    - copied into audit/notification records
    - returned to the caller or recipient
  - they are not currently:
    - parsed into conversation membership checks
    - used to open conversation-bound stream/RTC state
    - used to mutate conversation membership, read cursors, or messages
    - consumed by `projection-service` as a conversation control-plane event

## 2. Investigation Evidence

### 2.1 `automation-service` runtime behavior

- `services/automation-service/src/lib.rs`
  - `RequestAutomationExecution.input_payload` is accepted as `Option<String>`
  - the runtime persists that value onto `AutomationExecution.input_payload`
  - append path serializes the whole execution object into event payload
  - there is no parsing branch that extracts `conversationId` from `input_payload`

### 2.2 `notification-service` runtime behavior

- `services/notification-service/src/lib.rs`
  - `RequestNotification.payload` is accepted as `Option<String>`
  - the runtime persists that value onto `NotificationTask.payload`
  - append path serializes the whole task object into event payload
  - access control is based on:
    - recipient self-access
    - or `notification.write`
  - there is no parsing branch that extracts `conversationId` from notification payload

### 2.3 Local orchestration does not reinterpret these payloads as conversation authority

- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - automation request audit writes `execution.input_payload` as opaque audit payload
  - automation completion emits an in-app notification using `execution.output_payload`
  - notification request audit records only summary metadata such as:
    - `sourceEventType`
    - `recipientId`
  - message fan-out notifications build their own explicit payload from already-authorized message context
  - no path reparses automation/notification payload blobs into conversation write authorization

### 2.4 Projection layer has no automation/notification consumer

- repo search confirmed `projection-service` does not handle:
  - `automation.execution_requested`
  - `automation.execution_completed`
  - `notification.requested`
  - `notification.dispatched`
- current projection parsing is limited to explicit conversation/message/member/read-cursor event schemas

## 3. Why This Is Not The Same Class As Stream/RTC

The previously fixed standalone stream/RTC issue was real because those requests carried explicit control fields that directly bound new state to a conversation:

- stream:
  - `scopeKind = conversation`
  - `scopeId = <conversationId>`
- RTC:
  - top-level `conversationId`
  - persisted RTC session `conversationId`

Those fields directly changed authorization scope and runtime routing.

By contrast, current automation/notification payloads are opaque strings whose internal JSON shape is not interpreted by these services as authority.

## 4. Standardization Outcome

This review wave does not add a new gateway rejection or runtime guard.

Instead it establishes the classification rule:

1. explicit contract fields that bind work to a conversation are authorization-bearing control inputs
2. opaque payload blobs that are merely stored or forwarded are not authorization-bearing by themselves
3. if a future feature wants to use data from an opaque payload to drive conversation mutations, that data must first be promoted into an explicit, authorized contract boundary

## 5. Verification

Static audit evidence:

- repo search for automation event consumers:
  - `rg -n "automation\\.execution_(requested|completed)" services -g "*.rs"`
- repo search for notification event consumers:
  - `rg -n "notification\\.(requested|dispatched)" services -g "*.rs"`
- repo search for payload parsing in affected runtimes:
  - `rg -n "input_payload|serde_json::from_str\\(&event\\.payload\\)" services/sdkwork-im-cloud-gateway/src/lib.rs services/automation-service/src/lib.rs services/notification-service/src/lib.rs -g "*.rs"`

Observed conclusions:

- no projection/runtime consumer reparses automation/notification payload blobs into conversation-bound state
- only explicit conversation/message/member/read-cursor contracts are used for conversation authorization and projection

## 6. Remaining Risks

- this is safe only while these payloads remain opaque
- a future worker could accidentally introduce a bypass by:
  - parsing `payload.conversationId`
  - treating it as authoritative conversation context
  - performing conversation writes without promoting it to an explicit authorized field
- that future change would require:
  - failing regression tests first
  - adapter-bound authorization hardening
  - standard updates in `/docs/架构/`

## 7. Next Wave

1. continue auditing remaining standalone or orchestration services for top-level conversation-binding inputs, not just payload strings containing `conversationId`
2. keep separating:
   - authorization-bearing contract fields
   - opaque business payload
3. if any downstream workflow begins interpreting opaque payload into conversation mutations, treat that as a new security review item immediately
