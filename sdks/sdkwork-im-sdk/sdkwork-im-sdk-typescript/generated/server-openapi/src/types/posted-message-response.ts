import type { MessageBody } from './message-body';

export interface PostedMessageResponse {
  conversationId: string;
  messageId: string;
  messageSeq: number;
  body: MessageBody;
  occurredAt: string;
}
