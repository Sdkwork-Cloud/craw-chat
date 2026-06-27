import type { Chat } from '@sdkwork/im-pc-types';
import { chatService, type ChatService } from './ChatService';
import { createDefaultAvatar } from './DefaultAvatarService';

export const SYSTEM_ASSISTANT_AGENT = {
  avatar: createDefaultAvatar('agent'),
  id: 'agent.sdkwork_assistant',
  name: 'System Assistant',
} as const;

export interface SystemAssistantStartupResult {
  available: boolean;
  chat: Chat | null;
  created: boolean;
  error?: unknown;
}

export interface SystemAssistantService {
  ensureSystemAssistantChat(chats: Chat[]): Promise<SystemAssistantStartupResult>;
  isSystemAssistantChat(chat: Chat): boolean;
  selectInitialChat(chats: Chat[]): Chat | null;
}

type SystemAssistantChatClient = Pick<ChatService, 'startAgentChat'>;

function hasUnread(chat: Chat): boolean {
  return (chat.unreadCount ?? 0) > 0 || chat.isMarkedUnread === true;
}

class SdkworkSystemAssistantService implements SystemAssistantService {
  constructor(private readonly chatClient: SystemAssistantChatClient = chatService) {}

  async ensureSystemAssistantChat(chats: Chat[]): Promise<SystemAssistantStartupResult> {
    const existingAssistantChat = chats.find((chat) => this.isSystemAssistantChat(chat));
    if (existingAssistantChat) {
      return {
        available: true,
        chat: existingAssistantChat,
        created: false,
      };
    }

    try {
      const assistantChat = await this.chatClient.startAgentChat(SYSTEM_ASSISTANT_AGENT);
      return {
        available: true,
        chat: assistantChat,
        created: true,
      };
    } catch (error) {
      return {
        available: false,
        chat: null,
        created: false,
        error,
      };
    }
  }

  isSystemAssistantChat(chat: Chat): boolean {
    return chat.id.toLowerCase().includes(SYSTEM_ASSISTANT_AGENT.id);
  }

  selectInitialChat(chats: Chat[]): Chat | null {
    const realChats = chats.filter((chat) => !this.isSystemAssistantChat(chat));
    if (realChats.length === 0) {
      return chats.find((chat) => this.isSystemAssistantChat(chat)) ?? null;
    }

    return [...realChats].sort((left, right) => {
      const leftUnread = hasUnread(left);
      const rightUnread = hasUnread(right);
      if (leftUnread !== rightUnread) {
        return leftUnread ? -1 : 1;
      }

      const leftPinned = left.isPinned === true;
      const rightPinned = right.isPinned === true;
      if (leftPinned !== rightPinned) {
        return leftPinned ? -1 : 1;
      }

      return right.updatedAt - left.updatedAt;
    })[0] ?? null;
  }
}

export function createSdkworkSystemAssistantService(
  chatClient?: SystemAssistantChatClient,
): SystemAssistantService {
  return new SdkworkSystemAssistantService(chatClient);
}

export const systemAssistantService = createSdkworkSystemAssistantService();
