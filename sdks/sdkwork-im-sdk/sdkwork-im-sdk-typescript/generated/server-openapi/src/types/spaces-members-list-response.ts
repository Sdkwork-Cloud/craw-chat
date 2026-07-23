import type { PageInfo } from './page-info';
import type { SpaceMemberView } from './space-member-view';

export interface SpacesMembersListResponse {
  code: 0;
  data: unknown & { items: SpaceMemberView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
