import type { PageInfo } from './page-info';
import type { SpaceInviteView } from './space-invite-view';

export interface SpacesInvitesListResponse {
  code: 0;
  data: unknown & { items: SpaceInviteView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
