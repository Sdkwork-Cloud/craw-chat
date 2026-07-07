import type { MessagePinView } from './message-pin-view';
import type { MessageReactionCountView } from './message-reaction-count-view';

export interface MessageInteractionSummaryView {
  tenantId: string;
  conversationId: string;
  messageId: string;
  messageSeq: number;
  totalReactionCount: number;
  reactionCounts: MessageReactionCountView[];
  pin?: MessagePinView | null;
}
