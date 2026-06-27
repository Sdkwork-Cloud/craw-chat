import type { ContentPart } from './content-part';
import type { MessageReplyReference } from './message-reply-reference';

export interface MessageBody {
  text?: string | null;
  parts: ContentPart[];
  replyTo?: MessageReplyReference | null;
  renderHints?: Record<string, unknown>;
  summary?: string | null;
  metadata?: Record<string, unknown>;
}
