import type { SpaceInviteView } from './space-invite-view';

export interface SpacesInvitesCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
