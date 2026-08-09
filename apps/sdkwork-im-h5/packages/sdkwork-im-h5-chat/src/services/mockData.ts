import { useTranslation } from "react-i18next";
import type { Chat, Message, User } from "@sdkwork/im-h5-types";

const INITIAL_USERS: User[] = [
  {
    id: "u1",
    name: "Alex Chen",
    avatar: "https://picsum.photos/seed/alex/200/200",
    status: "online",
  },
  {
    id: "u2",
    name: "Sarah Jenkins",
    avatar: "https://picsum.photos/seed/sarah/200/200",
    status: "online",
  },
  {
    id: "u3",
    name: "David Lee",
    avatar: "https://picsum.photos/seed/david/200/200",
  },
  {
    id: "u4",
    name: "Emily Chen",
    avatar: "https://picsum.photos/seed/emily/200/200",
  },
  {
    id: "u5",
    name: "Michael Brown",
    avatar: "https://picsum.photos/seed/michael/200/200",
  },
];

const INITIAL_CHATS: Chat[] = [
  {
    id: "c1",
    type: "direct",
    participants: [INITIAL_USERS[1]],
    lastMessage: {
      id: "m1",
      chatId: "c1",
      senderId: "u2",
      content: "Perfect, see you then.",
      timestamp: Date.now() - 1000 * 60 * 1,
      type: "text",
    },
    unreadCount: 2,
  },
  {
    id: "c2",
    type: "group",
    name: "Design Team",
    avatar: "https://picsum.photos/seed/design/200/200",
    participants: [INITIAL_USERS[1], INITIAL_USERS[2]],
    lastMessage: {
      id: "m2_1",
      chatId: "c2",
      senderId: "u3",
      content: "I uploaded the new Figma files.",
      timestamp: Date.now() - 1000 * 60 * 60 * 2,
      type: "text",
    },
    unreadCount: 0,
  },
  {
    id: "c3",
    type: "direct",
    participants: [INITIAL_USERS[2]],
    lastMessage: {
      id: "m3_1",
      chatId: "c3",
      senderId: "u1",
      content: "Sounds good to me.",
      timestamp: Date.now() - 1000 * 60 * 60 * 24,
      type: "text",
    },
    unreadCount: 0,
  },
];

const INITIAL_MESSAGES: Record<string, Message[]> = {
  c1: [
    {
      id: "m1_1",
      chatId: "c1",
      senderId: "u2",
      content: "Hey, are we still on for tomorrow?",
      timestamp: Date.now() - 1000 * 60 * 50,
      type: "text",
    },
    {
      id: "m1_2",
      chatId: "c1",
      senderId: "u1",
      content: "Yes! Same time and place.",
      timestamp: Date.now() - 1000 * 60 * 45,
      type: "text",
    },
    {
      id: "m1_3",
      chatId: "c1",
      senderId: "u2",
      content: "Here is the location.",
      timestamp: Date.now() - 1000 * 60 * 40,
      type: "text",
    },
    {
      id: "m1_4",
      chatId: "c1",
      senderId: "u2",
      content: "https://example.com/map",
      timestamp: Date.now() - 1000 * 60 * 39,
      type: "link",
      metadata: {
        title: "Google Maps Route",
        desc: "Central Park West, New York",
        icon: "MapPin",
      },
    },
    {
      id: "m1_5",
      chatId: "c1",
      senderId: "u1",
      content: "https://www.w3schools.com/html/horse.mp3",
      timestamp: Date.now() - 1000 * 60 * 38,
      type: "voice",
      metadata: { duration: "0:02" },
    },
    {
      id: "m1_6",
      chatId: "c1",
      senderId: "u1",
      content: "https://picsum.photos/seed/park/300/200",
      timestamp: Date.now() - 1000 * 60 * 30,
      type: "image",
    },
    {
      id: "m1_7",
      chatId: "c1",
      senderId: "u2",
      content: "Nice!",
      timestamp: Date.now() - 1000 * 60 * 20,
      type: "text",
    },
    {
      id: "m1_8",
      chatId: "c1",
      senderId: "u2",
      content: "Briefing_v2.pdf",
      timestamp: Date.now() - 1000 * 60 * 15,
      type: "file",
      metadata: { size: "2.4 MB", ext: "pdf" },
    },
    {
      id: "m1_9",
      chatId: "c1",
      senderId: "u1",
      content: "视频通话",
      timestamp: Date.now() - 1000 * 60 * 10,
      type: "call",
      metadata: { duration: "12:34", isVideo: true },
    },
    {
      id: "m1_10",
      chatId: "c1",
      senderId: "u2",
      content: "",
      timestamp: Date.now() - 1000 * 60 * 6,
      type: "miniapp",
      metadata: {
        title: "京东购物",
        desc: "Apple iPhone 15 Pro Max",
        icon: "ShoppingBag",
      },
    },
    {
      id: "m1_11",
      chatId: "c1",
      senderId: "u1",
      content: "https://www.w3schools.com/html/mov_bbb.mp4",
      timestamp: Date.now() - 1000 * 60 * 4,
      type: "video",
      metadata: {
        coverUrl: "https://picsum.photos/seed/video/300/400",
        duration: "0:10",
      },
    },
    {
      id: "m1_12",
      chatId: "c1",
      senderId: "u2",
      content:
        "https://assets.mixkit.co/active_storage/sfx/2869/2869-preview.mp3",
      timestamp: Date.now() - 1000 * 60 * 2,
      type: "music",
      metadata: {
        title: "Mixkit Tech House",
        artist: "Mixkit Author",
        coverUrl: "https://picsum.photos/seed/music_album/200/200",
      },
    },
    {
      id: "m1_13",
      chatId: "c1",
      senderId: "u2",
      content: "Perfect, see you then.",
      timestamp: Date.now() - 1000 * 60 * 1,
      type: "text",
    },
  ],
  c2: [
    {
      id: "m2_1",
      chatId: "c2",
      senderId: "u3",
      content: "I uploaded the new Figma files.",
      timestamp: Date.now() - 1000 * 60 * 60 * 2,
      type: "text",
    },
  ],
  c3: [
    {
      id: "m3_1",
      chatId: "c3",
      senderId: "u1",
      content: "Sounds good to me.",
      timestamp: Date.now() - 1000 * 60 * 60 * 24,
      type: "text",
    },
  ],
};

const STORAGE_KEY_CHATS = "clawchat_chats";
const STORAGE_KEY_MESSAGES = "clawchat_messages";

export let MOCK_USERS_FOR_CHAT: User[] = INITIAL_USERS;
export let MOCK_CHATS: Chat[] = [];
export let MOCK_MESSAGES_STORE: Record<string, Message[]> = {};

const loadData = () => {
  try {
    const chatsData = localStorage.getItem(STORAGE_KEY_CHATS);
    const messagesData = localStorage.getItem(STORAGE_KEY_MESSAGES);
    if (chatsData && messagesData) {
      MOCK_CHATS = JSON.parse(chatsData);
      MOCK_MESSAGES_STORE = JSON.parse(messagesData);
    } else {
      MOCK_CHATS = [...INITIAL_CHATS];
      MOCK_MESSAGES_STORE = { ...INITIAL_MESSAGES };
      saveData();
    }
  } catch (e) {
    MOCK_CHATS = [...INITIAL_CHATS];
    MOCK_MESSAGES_STORE = { ...INITIAL_MESSAGES };
  }
};

const saveData = () => {
  try {
    localStorage.setItem(STORAGE_KEY_CHATS, JSON.stringify(MOCK_CHATS));
    localStorage.setItem(
      STORAGE_KEY_MESSAGES,
      JSON.stringify(MOCK_MESSAGES_STORE),
    );
  } catch (e) {
    console.error("Failed to save chat data", e);
  }
};

// Initialize
loadData();

export const setMockChats = (chats: Chat[]) => {
  MOCK_CHATS = chats;
  saveData();
};
export const setMockMessagesStore = (store: Record<string, Message[]>) => {
  MOCK_MESSAGES_STORE = store;
  saveData();
};
export const setMockUsersForChat = (users: User[]) => {
  MOCK_USERS_FOR_CHAT = users;
};
