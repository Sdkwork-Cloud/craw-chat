import type { ContentPart } from './content-part';
import type { MessageReplyReference } from './message-reply-reference';

export interface EditMessageRequest {
  text?: string | null;
  parts?: ContentPart[];
  replyTo?: MessageReplyReference | null;
  summary?: string | null;
  renderHints?: Record<string, unknown>;
  idempotencyKey?: string | null;
}
