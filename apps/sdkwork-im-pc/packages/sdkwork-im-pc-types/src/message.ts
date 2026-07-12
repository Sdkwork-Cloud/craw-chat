export interface Message {
  id: string;
  chatId: string;
  senderId: string;
  content: string;
  type: 'text' | 'image' | 'file' | 'system' | 'video' | 'voice' | 'video_call' | 'link' | 'applet' | 'card' | 'music';
  fileUrl?: string;
  fileName?: string;
  fileSize?: string;
  duration?: number;
  coverUrl?: string;
  appIcon?: string;
  desc?: string;
  timestamp: number;
  isRecalled?: boolean;
  isEdited?: boolean;
  /** Local-only delivery state for desktop offline send queue. */
  sendState?: 'pending' | 'failed';
  /** Structured wire parts retained for local retry (not rendered as raw JSON). */
  parts?: unknown[];
  replyTo?: {
    id: string;
    senderName: string;
    content: string;
  };
  reactions?: {
    emoji: string;
    count: number;
    hasReacted: boolean;
  }[];
}
