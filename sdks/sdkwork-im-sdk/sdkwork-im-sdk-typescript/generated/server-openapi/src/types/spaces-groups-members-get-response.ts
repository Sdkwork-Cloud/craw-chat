import type { SpaceGroupMemberView } from './space-group-member-view';

export interface SpacesGroupsMembersGetResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
