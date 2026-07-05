import type { BlockScope } from './block-scope';

export interface CreateUserBlockRequest {
  blockedUserId: string;
  scope: BlockScope;
  directChatId?: string | null;
  expiresAt?: string | null;
}
