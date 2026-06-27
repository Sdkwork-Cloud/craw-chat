# Rooms

## What This Module Is For

This module covers live, chat, and game room lifecycle: create a room bound to a group conversation,
self-serve enter/leave, and read active member capacity metadata.

## Public Entrypoints

| Language | Entry |
| --- | --- |
| TypeScript | `sdk.rooms.create`, `sdk.rooms.get`, `sdk.rooms.enter`, `sdk.rooms.leave` |
| Flutter | `client.chat.rooms.create`, `client.chat.rooms.get`, `client.chat.rooms.enter`, `client.chat.rooms.leave` |
| Generated transport | `rooms.create`, `rooms.get`, `rooms.enter`, `rooms.leave` operationIds |

## API Mapping

- [Rooms API reference](/api-reference/im/rooms)
- Architecture standard: `docs/架构/69-room-live-chat-game-capability-standard-2026-06-23.md`

## Common Workflows

1. Create a room with `roomKind` `live`, `chat`, or `game`.
2. Enter the room as the authenticated principal.
3. Post messages on the bound `conversationId` through `sdk.conversations`.
4. Subscribe to realtime `message.posted` events on the same conversation scope.
5. Leave the room when the session ends.

Game moves use `DataPart` payloads with schema `urn:sdkwork:sdkwork-im:message:custom:game.{gameKey}` on the bound conversation.

## Ownership and Status

`ImRoomsModule` is a composed facade over the generated `client.chat.rooms` transport. Do not call raw HTTP for room routes from application code.
