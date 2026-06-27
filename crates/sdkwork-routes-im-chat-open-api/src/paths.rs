pub const PREFIX: &str = "/im/v3/api/chat";

pub const CONVERSATIONS: &str = "/im/v3/api/chat/conversations";
pub const CONVERSATION_THREADS: &str = "/im/v3/api/chat/conversations/threads";
pub const DIRECT_CHAT_BINDINGS: &str = "/im/v3/api/chat/conversations/direct_chats/bindings";
pub const SHARED_CHANNEL_LINKS_SYNC: &str =
    "/im/v3/api/chat/conversations/shared_channel_links/sync";
pub const AGENT_DIALOGS: &str = "/im/v3/api/chat/conversations/agent_dialogs";
pub const AGENT_HANDOFFS: &str = "/im/v3/api/chat/conversations/agent_handoffs";
pub const SYSTEM_CHANNELS: &str = "/im/v3/api/chat/conversations/system_channels";
pub const ROOMS: &str = "/im/v3/api/chat/rooms";
pub const ROOM: &str = "/im/v3/api/chat/rooms/{roomId}";
pub const ROOM_ENTER: &str = "/im/v3/api/chat/rooms/{roomId}/enter";
pub const ROOM_LEAVE: &str = "/im/v3/api/chat/rooms/{roomId}/leave";
pub const CONVERSATION_AGENT_HANDOFF: &str =
    "/im/v3/api/chat/conversations/{conversationId}/agent_handoff";
pub const CONVERSATION_AGENT_HANDOFF_ACCEPT: &str =
    "/im/v3/api/chat/conversations/{conversationId}/agent_handoff/accept";
pub const CONVERSATION_AGENT_HANDOFF_RESOLVE: &str =
    "/im/v3/api/chat/conversations/{conversationId}/agent_handoff/resolve";
pub const CONVERSATION_AGENT_HANDOFF_CLOSE: &str =
    "/im/v3/api/chat/conversations/{conversationId}/agent_handoff/close";
pub const CONVERSATION_MEMBERS: &str = "/im/v3/api/chat/conversations/{conversationId}/members";
pub const CONVERSATION_BINDING: &str =
    "/im/v3/api/chat/conversations/{conversationId}/binding";
pub const CONVERSATION_MEMBERS_ADD: &str =
    "/im/v3/api/chat/conversations/{conversationId}/members/add";
pub const CONVERSATION_MEMBERS_REMOVE: &str =
    "/im/v3/api/chat/conversations/{conversationId}/members/remove";
pub const CONVERSATION_MEMBERS_TRANSFER_OWNER: &str =
    "/im/v3/api/chat/conversations/{conversationId}/members/transfer_owner";
pub const CONVERSATION_MEMBERS_CHANGE_ROLE: &str =
    "/im/v3/api/chat/conversations/{conversationId}/members/change_role";
pub const CONVERSATION_MEMBERS_LEAVE: &str =
    "/im/v3/api/chat/conversations/{conversationId}/members/leave";
pub const CONVERSATION_READ_CURSOR: &str =
    "/im/v3/api/chat/conversations/{conversationId}/read_cursor";
pub const MESSAGE_EDIT: &str = "/im/v3/api/chat/messages/{messageId}/edit";
pub const MESSAGE_RECALL: &str = "/im/v3/api/chat/messages/{messageId}/recall";
pub const MESSAGE_REACTIONS: &str = "/im/v3/api/chat/messages/{messageId}/reactions";
pub const MESSAGE_REACTIONS_REMOVE: &str =
    "/im/v3/api/chat/messages/{messageId}/reactions/remove";
pub const MESSAGE_PIN: &str = "/im/v3/api/chat/messages/{messageId}/pin";
pub const MESSAGE_UNPIN: &str = "/im/v3/api/chat/messages/{messageId}/unpin";
pub const CONVERSATION_MESSAGES: &str =
    "/im/v3/api/chat/conversations/{conversationId}/messages";
pub const CONVERSATION_SYSTEM_CHANNEL_PUBLISH: &str =
    "/im/v3/api/chat/conversations/{conversationId}/system_channel/publish";
