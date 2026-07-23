import type { ConversationMember } from './conversation-member';
import type { PageInfo } from './page-info';

export interface ConversationsMemberDirectoryListResponse {
  code: 0;
  data: unknown & { items: ConversationMember[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
