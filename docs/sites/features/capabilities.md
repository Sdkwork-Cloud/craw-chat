# Capabilities

Implementation-aligned capability map for the current Sdkwork IM repository.

## Conversation and messaging

| Area | Notes | Reference |
| --- | --- | --- |
| Conversations | Standard conversations, agent dialogs, handoffs, system channels | `services/sdkwork-im-cloud-gateway`, `services/comms-conversation-service` |
| Rooms | Live, chat, and game room binding with enter/leave orchestration | `services/comms-conversation-service` |
| Membership | List, add, remove, transfer owner, change role, leave | OpenAPI `/im/v3/api/chat/*` |
| Messages | Send, edit, recall, timeline reads | `services/comms-conversation-service` |
| Read models | Inbox, conversation summary, read cursor | `services/projection-service` |

## Realtime

| Area | Notes | Reference |
| --- | --- | --- |
| Presence | Heartbeat and current presence | `services/session-gateway` |
| Realtime delivery | Subscription sync, websocket upgrade | `services/session-gateway`, `services/sdkwork-im-cloud-gateway` |

## Media, streams, calls

| Area | Reference |
| --- | --- |
| Media | `services/media-service` |
| Streams | `services/streaming-service` |
| Calls | `services/im-calls-service` |

## Platform surfaces

| Area | Reference |
| --- | --- |
| Notifications | `services/notification-service` |
| Automation | `services/automation-service` |
| Audit / Ops | `services/audit-service`, `services/ops-service` |

## Entry point

Development stack: `pnpm dev` with application ingress at `http://127.0.0.1:18079`.
