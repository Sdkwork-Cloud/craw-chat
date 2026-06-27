# 72. System Channel Cross-Capability Write Gate Standard (2026-04-06)

## 1. Objective

For a commercial IM platform, `system_channel` cannot mean:

- messages are publisher-only
- but conversation-bound side channels are still writable by subscribers

The broadcast contract must stay consistent across all conversation-bound write capabilities.

## 2. Scope

This standard applies to conversation-bound capabilities attached to a `system_channel`, currently:

- `stream`
- `RTC`

It governs write paths only in this wave.

## 3. Rule

When all of the following are true:

- `conversationType = system_channel`
- the actor is an active member of the conversation
- the actor is not the system publisher member

then the server must reject conversation-bound write capabilities with:

- HTTP status: `403 Forbidden`
- code: `conversation_permission_denied`

## 4. Publisher Definition

An actor is considered the `system_channel` publisher only when both are true:

1. `principalKind = system`
2. `attributes.channelRole = publisher`

Any other active member, including the subscriber member, is read/consume-only for conversation-bound capability writes.

## 5. Covered Capability Writes

### 5.1 Stream

The following operations must be publisher-only when bound to a `system_channel` conversation:

- `stream.open`
- `stream.append`
- `stream.checkpoint`
- `stream.complete`
- `stream.abort`

### 5.2 RTC

The following operations must be publisher-only when bound to a `system_channel` conversation:

- `rtc.create`
- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`
- `rtc.signal`

## 6. Architecture Rule

The authorization truth belongs in `conversation-runtime`, not in leaf services.

Therefore:

- runtime capability gates must receive actor identity
- runtime must resolve the active member and apply special-conversation write policy
- edge/profile services must call the runtime gate before mutating conversation-bound `stream/RTC`

This prevents:

- message-only hardening while side channels remain open
- duplicated `system_channel` policy logic in stream and rtc services
- inconsistent policy drift across profiles and gateways

## 7. Consistency Rule

`system_channel` write semantics must be uniform across:

- message post
- stream write paths
- rtc write paths

The platform must not allow one conversation-bound write surface to bypass a publisher-only rule already enforced on another surface.

## 8. Test Standard

Implementations must include:

1. a test proving a subscriber cannot mutate an existing conversation-bound stream in `system_channel`
2. a test proving a subscriber cannot open a new conversation-bound stream in `system_channel`
3. a test proving a subscriber cannot mutate an existing conversation-bound RTC session in `system_channel`
4. a test proving a subscriber cannot create a new conversation-bound RTC session in `system_channel`

At least one local-profile end-to-end implementation must carry these tests.

## 9. Non-Goals

This standard does not yet define:

- scheduled publish
- batch publish
- moderation or mute policies for `system_channel`
- non-conversation-bound private `stream/RTC` sessions

Those remain later waves and must not weaken the publisher-only rule defined here.
