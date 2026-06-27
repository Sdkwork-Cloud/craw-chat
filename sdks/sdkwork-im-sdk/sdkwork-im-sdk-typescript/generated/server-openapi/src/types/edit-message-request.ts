import type { ContentPart } from './content-part';
import type { MessageReplyReference } from './message-reply-reference';

export interface EditMessageRequest {
  text?: string | null;
  parts?: ContentPart[];
  replyTo?: MessageReplyReference | null;
}
