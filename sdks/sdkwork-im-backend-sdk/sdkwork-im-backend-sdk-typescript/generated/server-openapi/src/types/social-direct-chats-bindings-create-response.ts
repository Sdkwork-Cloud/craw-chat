import type { SocialDirectChatCommitResponse } from './social-direct-chat-commit-response';

export interface SocialDirectChatsBindingsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
