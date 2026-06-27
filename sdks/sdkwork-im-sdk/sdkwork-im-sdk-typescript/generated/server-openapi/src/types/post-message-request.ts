import type { ContentPart } from './content-part';
import type { MessageReplyReference } from './message-reply-reference';

export interface PostMessageRequest {
  text?: string | null;
  parts?: ContentPart[];
  replyTo?: MessageReplyReference | null;
  clientMsgId?: string | null;
  summary?: string | null;
  renderHints?: Record<string, unknown>;
}
