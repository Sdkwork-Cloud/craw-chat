import type { PageInfo } from './page-info';
import type { SpaceMemberView } from './space-member-view';

export interface SpacesMembersListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
