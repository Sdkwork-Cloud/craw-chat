import type { BlockScope } from './block-scope';

export interface BlockUserRequest {
  blockedUserId: string;
  scope: BlockScope;
  directChatId?: string | null;
  expiresAt?: string;
}
