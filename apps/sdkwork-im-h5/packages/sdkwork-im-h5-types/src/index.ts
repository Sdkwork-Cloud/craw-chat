export interface User {
  id: string;
  name: string;
  avatar?: string;
  status?: "online" | "offline" | "busy";
}

export interface Message {
  id: string;
  chatId: string;
  senderId: string;
  content: string; // Used generally, or as text content
  timestamp: number;
  type:
    | "text"
    | "image"
    | "video"
    | "voice"
    | "call"
    | "link"
    | "miniapp"
    | "card"
    | "file"
    | "music";
  isStarred?: boolean;
  metadata?: Record<string, any>; // To store additional info like file sizes, thumbnail urls, call duration etc.
}

export interface Chat {
  id: string;
  type: "direct" | "group";
  participants: User[];
  lastMessage?: Message;
  unreadCount: number;
  name?: string; // For groups
  avatar?: string; // For groups
  settings?: {
    showAvatar?: boolean;
    cleanMode?: boolean;
  };
  isPinned?: boolean;
}
