import type { SocialDirectChatCommitResponse } from './social-direct-chat-commit-response';

export interface SocialDirectChatsBindingsCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialDirectChatCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
