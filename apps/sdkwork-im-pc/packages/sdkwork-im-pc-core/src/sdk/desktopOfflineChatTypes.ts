export type DesktopOfflineMessageType =
  | 'applet'
  | 'card'
  | 'file'
  | 'image'
  | 'link'
  | 'music'
  | 'system'
  | 'text'
  | 'video'
  | 'video_call'
  | 'voice';

export interface DesktopOfflineMessageReplyReference {
  id: string;
  senderName: string;
  content: string;
}

export interface DesktopOfflineMessageReaction {
  emoji: string;
  count: number;
  hasReacted: boolean;
}

export interface DesktopOfflineMessage {
  id: string;
  chatId: string;
  senderId: string;
  content: string;
  type: DesktopOfflineMessageType;
  timestamp: number;
  appIcon?: string;
  coverUrl?: string;
  desc?: string;
  duration?: number;
  fileName?: string;
  fileSize?: string;
  fileUrl?: string;
  isEdited?: boolean;
  isRecalled?: boolean;
  reactions?: DesktopOfflineMessageReaction[];
  replyTo?: DesktopOfflineMessageReplyReference;
  sendState?: 'failed' | 'pending';
  /** Structured message parts retained so offline/failed sends can retry exactly. */
  parts?: unknown[];
}

export interface DesktopOfflineChat {
  id: string;
  name: string;
  type: 'group' | 'single';
  unreadCount: number;
  updatedAt: number;
  activeCount?: number;
  avatar?: string;
  isMarkedUnread?: boolean;
  isMuted?: boolean;
  isPinned?: boolean;
  lastMessage?: DesktopOfflineMessage;
  memberCount?: number;
  members?: string[];
  notice?: string;
  welcomeMessage?: string;
}
