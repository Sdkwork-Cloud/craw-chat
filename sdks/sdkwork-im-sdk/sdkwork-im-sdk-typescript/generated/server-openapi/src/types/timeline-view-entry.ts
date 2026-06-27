import type { MessageBody } from './message-body';
import type { MessageType } from './message-type';
import type { Sender } from './sender';

export interface TimelineViewEntry {
  tenantId: string;
  conversationId: string;
  messageId: string;
  messageSeq: number;
  summary?: string | null;
  sender: Sender;
  body: MessageBody;
  messageType: MessageType;
  deliveryMode: string;
  clientMsgId?: string | null;
  streamSessionId?: string | null;
  rtcSessionId?: string | null;
  occurredAt: string;
  committedAt?: string;
}
