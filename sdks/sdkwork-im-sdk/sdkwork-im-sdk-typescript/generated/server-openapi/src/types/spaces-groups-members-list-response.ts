import type { PageInfo } from './page-info';
import type { SpaceGroupMemberView } from './space-group-member-view';

export interface SpacesGroupsMembersListResponse {
  code: 0;
  data: unknown & { items: SpaceGroupMemberView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
