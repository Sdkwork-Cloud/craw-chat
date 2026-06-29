import type { PageInfo } from './page-info';
import type { SpaceGroupMemberView } from './space-group-member-view';

export interface SpacesGroupsMembersListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
