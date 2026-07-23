import type { SpaceInviteView } from './space-invite-view';

export interface SpacesInvitesCreateResponse201 {
  code: 0;
  data: unknown & { item: SpaceInviteView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
