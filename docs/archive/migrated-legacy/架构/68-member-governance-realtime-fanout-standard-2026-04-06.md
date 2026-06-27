# 68. Member Governance Realtime Fanout Standard (2026-04-06)

## 1. Objective

Member governance is incomplete if it only updates durable truth and read APIs.

For a commercial IM system, the following member mutations must also reach subscribed realtime devices:

- member joined
- member role changed
- member removed
- member left

## 2. Canonical Event Types

The canonical business realtime event types are:

- `conversation.member_joined`
- `conversation.member_role_changed`
- `conversation.member_removed`
- `conversation.member_left`

These names must match the durable conversation event names so downstream consumers do not need a second mapping vocabulary.

## 3. Recipient Rule

### 3.1 Baseline recipients

The gateway/node layer must publish member governance realtime events to all current conversation members' registered client routes.

### 3.2 Affected principal rule

For `conversation.member_removed` and `conversation.member_left`, the affected principal must still receive the event on their other registered client routes even though they are no longer active after the mutation.

This is mandatory. A post-mutation active-member query alone is insufficient.

## 4. Scope Rule

Member governance realtime events use:

- `scopeType = conversation`
- `scopeId = conversationId`

Delivery still depends on normal realtime subscription matching:

- subscription `scopeType = conversation`
- subscription `scopeId = target conversation`
- subscription `eventTypes` contains the event type, or uses wildcard semantics already supported by runtime

## 5. Payload Rule

### 5.1 Join / Remove / Leave

Payload must include:

```json
{
  "conversationId": "c_demo",
  "member": {
    "memberId": "cm_c_demo_3",
    "principalId": "1",
    "principalKind": "user",
    "role": "member",
    "state": "joined"
  },
  "actor": {
    "id": "1",
    "kind": "user"
  }
}
```

### 5.2 Role Change

Payload must include:

```json
{
  "conversationId": "c_demo",
  "changedAt": "2026-04-06T12:00:00Z",
  "previousMember": {
    "memberId": "cm_c_demo_3",
    "role": "member"
  },
  "updatedMember": {
    "memberId": "cm_c_demo_3",
    "role": "admin"
  },
  "actor": {
    "id": "1",
    "kind": "user"
  }
}
```

## 6. Implementation Rule

The access node or gateway must publish these realtime events directly in the write path after the runtime mutation succeeds.

The durable runtime stays the source of truth. Realtime fanout is a derived delivery concern layered on top of the successful mutation result.

## 7. Test Standard

Every profile that claims realtime support must include:

1. an HTTP realtime event-window test proving member governance events are delivered
2. a websocket push test proving at least one member governance event is pushed live
3. a regression case proving `removed / left` fanout still reaches the affected principal's registered client routes or explicitly documenting the chosen delivery boundary

## 8. Non-Goals

This standard does not yet require:

- typed `client-route event window` roster delta entries
- admin roster analytics
- invite-state workflow beyond current joined/left/removed semantics

Those remain later waves. This standard only freezes the realtime fanout guarantee.
