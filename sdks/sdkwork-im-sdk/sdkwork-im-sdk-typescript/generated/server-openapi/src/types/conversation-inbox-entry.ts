import type { ConversationInboxPeerView } from './conversation-inbox-peer-view';
import type { ConversationInboxPreferencesView } from './conversation-inbox-preferences-view';

export interface ConversationInboxEntry {
  tenantId: string;
  conversationId: string;
  agentHandoff?: boolean;
  conversationType: string;
  displayName?: string | null;
  avatarUrl?: string | null;
  displaySource?: string | null;
  peer?: ConversationInboxPeerView;
  preferences?: ConversationInboxPreferencesView;
  lastActivityAt: string;
  lastMessageId?: string | null;
  lastSenderId?: string | null;
  messageCount: number;
  lastMessageSeq: number;
  lastSummary?: string | null;
  lastMessageAt?: string;
  unreadCount: number;
}
