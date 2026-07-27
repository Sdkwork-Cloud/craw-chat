import i18next from 'i18next';
const t = i18next.t.bind(i18next);
import type { Chat, Message } from "@sdkwork/im-h5-types";
import {
  MOCK_CHATS,
  MOCK_MESSAGES_STORE,
  MOCK_USERS_FOR_CHAT,
  setMockChats,
  setMockMessagesStore,
} from "./mockData";

export const ChatService = {
  async getChats(): Promise<Chat[]> {
    return [...MOCK_CHATS];
  },

  async getChatById(id: string): Promise<Chat | undefined> {
    return MOCK_CHATS.find((c) => c.id === id);
  },

  async getMessages(chatId: string): Promise<Message[]> {
    return MOCK_MESSAGES_STORE[chatId] || [];
  },

  async searchChatHistory(chatId: string, query: string): Promise<Message[]> {
    const messages = MOCK_MESSAGES_STORE[chatId] || [];
    if (!query.trim()) return [];
    const lowerQuery = query.toLowerCase();
    return messages.filter(
      (m) => m.type === "text" && m.content.toLowerCase().includes(lowerQuery),
    );
  },

  async sendMessage(
    chatId: string,
    senderId: string,
    content: string,
    type: Message["type"] = "text",
    metadata?: unknown,
  ): Promise<Message> {
    const newMessage: Message = {
      id: `m${Date.now()}`,
      chatId,
      senderId,
      content,
      timestamp: Date.now(),
      type,
      metadata,
    };

    const newStore = { ...MOCK_MESSAGES_STORE };
    if (!newStore[chatId]) {
      newStore[chatId] = [];
    }
    newStore[chatId] = [...newStore[chatId], newMessage];
    setMockMessagesStore(newStore);

    // Update last message in chat
    const newChats = [...MOCK_CHATS];
    const chatIndex = newChats.findIndex((c) => c.id === chatId);
    if (chatIndex !== -1) {
      newChats[chatIndex] = { ...newChats[chatIndex], lastMessage: newMessage };
      setMockChats(newChats);
    }

    return newMessage;
  },

  async searchChats(query: string): Promise<Chat[]> {
    if (!query.trim()) return [];
    const lowerQuery = query.toLowerCase();
    const chats = await this.getChats();
    return chats.filter((c) => {
      if (c.type === "group" && c.name?.toLowerCase().includes(lowerQuery))
        return true;
      if (
        c.type === "direct" &&
        c.participants[0]?.name.toLowerCase().includes(lowerQuery)
      )
        return true;
      return false;
    });
  },

  async createDirectChat(user: any, greeting?: string): Promise<Chat> {
    const newChat: Chat = {
      id: `c${Date.now()}`,
      type: "direct",
      participants: [user as import("@sdkwork/im-h5-types").User],
      unreadCount: 0,
      lastMessage: {
        id: `m${Date.now()}`,
        chatId: `c${Date.now()}`,
        senderId: user.id,
        content: greeting || t("chat:service.greeting", "我们已经是好友了，开始聊天吧！"),
        timestamp: Date.now(),
        type: "text",
      },
    };

    setMockChats([newChat, ...MOCK_CHATS]);

    const newStore = { ...MOCK_MESSAGES_STORE };
    newStore[newChat.id] = [newChat.lastMessage!];
    setMockMessagesStore(newStore);

    return newChat;
  },

  async createGroupChat(name: string, participantIds: string[]): Promise<Chat> {
    const participants = MOCK_USERS_FOR_CHAT.filter((u) =>
      participantIds.includes(u.id),
    );
    const newChat: Chat = {
      id: `c${Date.now()}`,
      type: "group",
      name: name || t("chat:service.new_group", "新群聊"),
      avatar: `https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/${Date.now()}/200.png`,
      participants,
      unreadCount: 0,
      settings: { showAvatar: true },
      lastMessage: {
        id: `m${Date.now()}`,
        chatId: `c${Date.now()}`,
        senderId: "u1",
        content: t("chat:service.started_group", "我发起了群聊"),
        timestamp: Date.now(),
        type: "text",
      },
    };

    setMockChats([newChat, ...MOCK_CHATS]);

    const newStore = { ...MOCK_MESSAGES_STORE };
    newStore[newChat.id] = [newChat.lastMessage!];
    setMockMessagesStore(newStore);

    return newChat;
  },

  async addParticipants(chatId: string, newParticipantIds: string[]): Promise<Chat | undefined> {
    const newChats = [...MOCK_CHATS];
    const chatIndex = newChats.findIndex(c => c.id === chatId);
    if (chatIndex !== -1 && newChats[chatIndex].type === "group") {
      const chat = newChats[chatIndex];
      const addedParticipants = MOCK_USERS_FOR_CHAT.filter(u => newParticipantIds.includes(u.id));
      
      const uniqueNewParticipants = addedParticipants.filter(
        newP => !chat.participants.some(existingP => existingP.id === newP.id)
      );

      if (uniqueNewParticipants.length > 0) {
        newChats[chatIndex] = {
          ...chat,
          participants: [...chat.participants, ...uniqueNewParticipants]
        };
        setMockChats(newChats);
      }
      return newChats[chatIndex];
    }
    return undefined;
  },

  async joinOrCreateGroupChat(name: string): Promise<Chat> {
    const existing = MOCK_CHATS.find((c) => c.type === "group" && c.name === name);
    if (existing) return existing;

    const newChat: Chat = {
      id: `c${Date.now()}_${Math.random()}`,
      type: "group",
      name: name,
      avatar: `https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/${encodeURIComponent(name)}/200.png`,
      participants: MOCK_USERS_FOR_CHAT.slice(0, 3),
      unreadCount: 0,
      settings: { showAvatar: true },
      lastMessage: {
        id: `m${Date.now()}`,
        chatId: `c${Date.now()}`,
        senderId: "system",
        content: t("chat:service.joined_group", `你已加入群聊 "${name}"`, { name }),
        timestamp: Date.now(),
        type: "text" as any,
      },
    };

    setMockChats([newChat, ...MOCK_CHATS]);

    const newStore = { ...MOCK_MESSAGES_STORE };
    newStore[newChat.id] = [newChat.lastMessage!];
    setMockMessagesStore(newStore);

    return newChat;
  },

  async updateChatSettings(
    chatId: string,
    settings: Partial<Chat["settings"]>,
  ): Promise<Chat | undefined> {
    const newChats = [...MOCK_CHATS];
    const chatIndex = newChats.findIndex((c) => c.id === chatId);
    if (chatIndex !== -1) {
      newChats[chatIndex] = {
        ...newChats[chatIndex],
        settings: { ...newChats[chatIndex].settings, ...settings },
      };
      setMockChats(newChats);
      return newChats[chatIndex];
    }
    return undefined;
  },

  async pinChat(chatId: string, isPinned: boolean): Promise<void> {
    const newChats = [...MOCK_CHATS];
    const index = newChats.findIndex((c) => c.id === chatId);
    if (index !== -1) {
      newChats[index] = { ...newChats[index], isPinned };
      setMockChats(newChats);
    }
  },

  async markAsUnread(chatId: string): Promise<void> {
    const newChats = [...MOCK_CHATS];
    const index = newChats.findIndex((c) => c.id === chatId);
    if (index !== -1) {
      newChats[index] = { ...newChats[index], unreadCount: 1 };
      setMockChats(newChats);
    }
  },

  async markAsRead(chatId: string): Promise<void> {
    const newChats = [...MOCK_CHATS];
    const index = newChats.findIndex((c) => c.id === chatId);
    if (index !== -1) {
      newChats[index] = { ...newChats[index], unreadCount: 0 };
      setMockChats(newChats);
    }
  },

  async deleteChat(chatId: string): Promise<void> {
    const newChats = MOCK_CHATS.filter((c) => c.id !== chatId);
    setMockChats(newChats);
    const newStore = { ...MOCK_MESSAGES_STORE };
    delete newStore[chatId];
    setMockMessagesStore(newStore);
  },

  async clearChatHistory(chatId: string): Promise<void> {
    const newStore = { ...MOCK_MESSAGES_STORE };
    newStore[chatId] = [];
    setMockMessagesStore(newStore);

    // Clear last message in chat list
    const newChats = [...MOCK_CHATS];
    const index = newChats.findIndex((c) => c.id === chatId);
    if (index !== -1) {
      newChats[index] = { ...newChats[index], lastMessage: undefined };
      setMockChats(newChats);
    }
  },

  async deleteMessage(chatId: string, messageId: string): Promise<void> {
    const newStore = { ...MOCK_MESSAGES_STORE };
    if (newStore[chatId]) {
      newStore[chatId] = newStore[chatId].filter((m) => m.id !== messageId);
      setMockMessagesStore(newStore);

      // Update last message if needed
      const newChats = [...MOCK_CHATS];
      const chatIndex = newChats.findIndex((c) => c.id === chatId);
      if (
        chatIndex !== -1 &&
        newChats[chatIndex].lastMessage?.id === messageId
      ) {
        const messages = newStore[chatId];
        newChats[chatIndex] = {
          ...newChats[chatIndex],
          lastMessage:
            messages.length > 0 ? messages[messages.length - 1] : undefined,
        };
        setMockChats(newChats);
      }
    }
  },

  async starMessage(
    chatId: string,
    messageId: string,
    isStarred: boolean,
  ): Promise<void> {
    const newStore = { ...MOCK_MESSAGES_STORE };
    if (newStore[chatId]) {
      newStore[chatId] = newStore[chatId].map((m) =>
        m.id === messageId ? { ...m, isStarred } : m,
      );
      setMockMessagesStore(newStore);
    }
  },

  async getEmojis(): Promise<string[]> {
    return [
      "😀",
      "😂",
      "🥺",
      "😍",
      "🙏",
      "👍",
      "😭",
      "✨",
      "🔥",
      "❤️",
      "🎉",
      "🤔",
      "😎",
      "😊",
      "🥰",
      "👏",
      "😁",
      "😆",
      "😅",
      "🤣",
      "😉",
      "😋",
      "😜",
      "🤪",
      "😝",
      "🤑",
      "🤗",
      "🤭",
      "🤫",
      "🤐",
      "🤨",
      "😐",
      "😑",
      "😶",
      "😏",
      "😒",
      "🙄",
      "😬",
      "🤥",
      "😌",
      "😔",
      "😪",
      "🤤",
      "😷",
      "🤒",
      "🤕",
      "🤢",
      "🤮",
      "🤧",
      "🥵",
      "🥶",
      "🥴",
      "😵",
      "🤯",
      "🤠",
      "🥳",
      "🤓",
      "🧐",
      "😕",
      "😟",
      "🙁",
      "☹️",
      "😮",
      "😯",
      "😲",
      "😳",
      "😦",
      "😧",
      "😨",
      "😰",
      "😥",
      "😢",
      "😱",
      "😖",
      "😣",
      "😞",
      "😓",
      "😩",
      "😫",
      "🥱",
      "😤",
      "😡",
      "😠",
      "🤬",
      "😈",
      "👿",
      "💀",
      "☠️",
      "💩",
      "🤡",
      "👹",
      "👺",
      "👻",
      "👽",
      "👾",
      "🤖",
    ];
  },
};
