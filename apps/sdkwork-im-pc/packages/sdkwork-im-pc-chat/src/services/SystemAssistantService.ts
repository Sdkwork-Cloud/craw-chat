import type { Chat } from '@sdkwork/im-pc-types';
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
  /** The server-delivered system-agent welcome conversation (canonical direct chat). */
  isSystemAssistantWelcomeChat(chat: Chat): boolean;
  selectInitialChat(chats: Chat[]): Chat | null;
}

function hasUnread(chat: Chat): boolean {
  return (chat.unreadCount ?? 0) > 0 || chat.isMarkedUnread === true;
}

class SdkworkSystemAssistantService implements SystemAssistantService {
  async ensureSystemAssistantChat(chats: Chat[]): Promise<SystemAssistantStartupResult> {
    const existingAssistantChat = chats.find((chat) => this.isSystemAssistantChat(chat));
    if (existingAssistantChat) {
      return {
        available: true,
        chat: existingAssistantChat,
        created: false,
      };
    }

    return {
      available: false,
      chat: null,
      created: false,
    };
  }

  isSystemAssistantChat(chat: Chat): boolean {
    const normalizedId = chat.id.trim().toLowerCase();
    if (normalizedId.includes(SYSTEM_ASSISTANT_AGENT.id)) {
      return true;
    }

    const isBackendAgentDialog = /^a_[a-f0-9]+$/u.test(normalizedId)
      || /^c_agent_[a-f0-9]+$/u.test(normalizedId);
    if (isBackendAgentDialog && chat.name === SYSTEM_ASSISTANT_AGENT.name) {
      return true;
    }

    return this.isSystemAssistantWelcomeChat(chat);
  }

  isSystemAssistantWelcomeChat(chat: Chat): boolean {
    const normalizedId = chat.id.trim().toLowerCase();
    // The system-agent welcome conversation is a canonical direct chat
    // (`c_<hash>` shape) named as the System Assistant by the inbox mapper.
    return /^c_[a-f0-9]+$/u.test(normalizedId) && chat.name === SYSTEM_ASSISTANT_AGENT.name;
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

export function createSdkworkSystemAssistantService(): SystemAssistantService {
  return new SdkworkSystemAssistantService();
}

export const systemAssistantService = createSdkworkSystemAssistantService();
