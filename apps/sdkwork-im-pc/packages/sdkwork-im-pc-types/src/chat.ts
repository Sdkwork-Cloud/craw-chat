import { Message } from './message';

export interface Chat {
  id: string;
  name: string;
  avatar?: string;
  type: 'single' | 'group';
  lastMessage?: Message;
  unreadCount: number;
  updatedAt: number;
  memberCount?: number;
  activeCount?: number;
  isPinned?: boolean;
  isMuted?: boolean;
  isMarkedUnread?: boolean;
  notice?: string;
  welcomeMessage?: string;
  members?: string[];
}
