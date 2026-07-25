# @sdkwork/im-h5-chat

Domain: communication
Capability: im-h5-chat
Package type: node-package

IM H5 **chat capability** package for the SDKWork IM H5 mobile browser app. Wraps the realtime connection manager, conversation message APIs, and drive media uploads exposed by `@sdkwork/im-h5-core` into chat-specific pages and services.

Machine-readable contract: `specs/component.spec.json`. Canonical standards: `../../../../../sdkwork-specs/`.

## Modules

| Module | Role |
| --- | --- |
| `services/chatRealtimeService` | Re-exports and configures the shared chat live connection, plus chat-specific subscription helpers |
| `services/chatConversationService` | Conversation message list/send wrappers over `ImSdkClient.conversations` (`listMessages`, `postText`) |
| `services/chatMediaUploadService` | Drive app SDK media uploads via `getDriveAppSdkClientWithSession` |
| `pages/ChatInboxPage` | Inbox page subscribing to live inbox refresh events |
| `pages/ChatConversationPage` | Conversation page subscribing to live messages and sending text |
