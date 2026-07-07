import type { BlockScope } from './block-scope';
import type { UserBlockStatus } from './user-block-status';

export interface UserBlock {
  tenantId: string;
  blockId: string;
  blockerUserId: string;
  blockedUserId: string;
  scope: BlockScope;
  status: UserBlockStatus;
  directChatId?: string | null;
  expiresAt?: string;
  createdAt: string;
  updatedAt: string;
}
