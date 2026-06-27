# 73. System Channel Dedicated Publish Contract Standard (2026-04-06)

## 1. Objective

`system_channel` is not a normal two-way conversation.

So its publish semantics must not remain coupled to the generic conversation message route.

This standard freezes a dedicated publish contract that later scheduled/bulk/delegated publish work must extend.

## 2. Scope

This standard governs immediate message publish for:

- `conversationType = system_channel`

It does not yet define:

- scheduled publish
- batch publish
- delegated publish orchestration
- moderation or mute lifecycle

## 3. Contract Rule

For `system_channel`, the server must expose a dedicated publish route:

- `POST /im/v3/api/chat/conversations/{conversationId}/system-channel/publish`

The generic conversation message route:

- `POST /im/v3/api/chat/conversations/{conversationId}/messages`

must not be used as the publish contract for `system_channel`.

## 4. Authorization Rule

Only the system publisher may publish through the dedicated route.

Publisher identity remains defined as:

1. resolved active member exists
2. `principalKind = system`
3. `attributes.channelRole = publisher`

Any subscriber or other non-publisher actor must be rejected with:

- HTTP status: `403 Forbidden`
- code: `conversation_permission_denied`

## 5. Generic Route Rule

If a caller attempts:

- `POST /im/v3/api/chat/conversations/{conversationId}/messages`

against a `system_channel`, the server must reject the request even if the actor is the real system publisher.

Reason:

- this preserves a stable specialized write surface
- future scheduled/bulk publish can extend a dedicated contract instead of mutating the generic message API

## 6. Durable Truth Rule

Dedicated publish does not introduce a separate storage model in this wave.

Successful dedicated publish still produces the unified durable message truth:

- `message.posted`
- stored in the same conversation message log
- returned as normal `PostMessageResult`

This keeps the slice minimal while freezing the API boundary.

## 7. Runtime Trust Boundary

The dedicated publish rule belongs in `conversation-runtime`, not only in edge handlers.

Therefore runtime must:

- resolve the active member from durable membership truth
- verify actor kind matches the resolved member principal kind
- reject generic `post_message(...)` for `system_channel`
- allow dedicated `publish_system_channel_message(...)` only for the publisher

This prevents route-only hardening that could be bypassed through lower-level runtime entry points.

## 8. Local/Profile Rule

Any application profile or gateway that exposes conversation publish must carry the same contract:

- dedicated system-channel publish route available
- generic `/messages` rejected for `system_channel`
- fanout/audit/realtime side effects preserved for successful dedicated publish

## 9. Test Standard

Implementations must include:

1. a test proving generic `/messages` is rejected for a `system_channel` publisher
2. a test proving dedicated publish succeeds for the system publisher
3. a test proving dedicated publish is rejected for the subscriber
4. at least one local-profile end-to-end test covering the same contract

## 10. Non-Goals

This standard does not yet decide:

- scheduled publish payload shape
- batch publish payload shape
- delegation semantics
- retry/orchestration policy

Those later waves must extend this dedicated contract and must not reopen generic `/messages` for `system_channel`.
