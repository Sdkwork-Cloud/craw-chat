> Migrated from `docs/sites/sdk/module-map.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Module Map

Use this page to move from a product requirement to the correct semantic SDK module and then down
to the exact App API page when needed.

## Task To Module Routing

| I need to... | Primary SDK module page | Related App API page | Typical client entrypoints |
| --- | --- | --- | --- |
| resume or disconnect a user session | [/sdk/modules/session-and-presence](/sdk/modules/session-and-presence) | [/api-reference/im/session-and-realtime](/api-reference/im/session-and-realtime) | `session.resume`, `session.disconnectDevice`, `session().resume` |
| update or read current presence | [/sdk/modules/session-and-presence](/sdk/modules/session-and-presence) | [/api-reference/im/session-and-realtime](/api-reference/im/session-and-realtime) | `presence.heartbeat`, `presence.current` |
| replace subscriptions or run durable catch-up | [/sdk/modules/realtime](/sdk/modules/realtime) | [/api-reference/im/session-and-realtime](/api-reference/im/session-and-realtime) | `realtime.replaceSubscriptions`, `sync.catchUp`, `realtime().list_events` |
| read the app inbox | [/sdk/modules/conversations](/sdk/modules/conversations) | [/api-reference/im/conversations](/api-reference/im/conversations) | `inbox.list`, `inbox().list` |
| create or inspect conversations | [/sdk/modules/conversations](/sdk/modules/conversations) | [/api-reference/im/conversations](/api-reference/im/conversations) | `conversations.create`, `conversations.get`, `conversations().create` |
| create or join live/chat/game rooms | [/sdk/modules/rooms](/sdk/modules/rooms) | [/api-reference/im/rooms](/api-reference/im/rooms) | `rooms.create`, `rooms.enter`, `rooms.get`, `rooms.leave` |
| manage members or read cursors | [/sdk/modules/conversations](/sdk/modules/conversations) | [/api-reference/im/membership-and-read-state](/api-reference/im/membership-and-read-state) | `listMembers`, `addMember`, `updateReadCursor` |
| send, edit, or recall messages | [/sdk/modules/messages](/sdk/modules/messages) | [/api-reference/im/messages](/api-reference/im/messages) | `conversations.postText`, `messages.editText`, `messages.recall` |
| upload or attach media | [/sdk/modules/media](/sdk/modules/media) | [/api-reference/im/media](/api-reference/im/media) | `media.createUpload`, `media.completeUpload`, `media.attachText` |
| open or append stream frames | [/sdk/modules/streams](/sdk/modules/streams) | [/api-reference/im/streams](/api-reference/im/streams) | `streams.open`, `streams.appendTextFrame`, `streams.checkpoint` |
| create or coordinate calls | [/sdk/modules/calls](/sdk/modules/calls) | [/api-reference/im/calls](/api-reference/im/calls) | `calls.start`, `calls.sendSignal`, `calls.issueParticipantCredential` |

## API Alignment

The SDK pages answer "how do I integrate this capability?".

The App API pages answer "what exact operations, request payloads, and response schemas back this
capability?".

Use both views together:

1. start with the SDK module page
2. use the matching App API page for exact payload and status details
3. return to the scenario pages when you need an end-to-end integration pattern

## Scenario Routing

- session and realtime bootstrap: [/sdk/examples/session-bootstrap](/sdk/examples/session-bootstrap)
- conversations and membership flow: [/sdk/examples/conversation-workflow](/sdk/examples/conversation-workflow)
- messages plus media: [/sdk/examples/message-and-media](/sdk/examples/message-and-media)
- calls and realtime: [/api-reference/im/calls](/api-reference/im/calls)

