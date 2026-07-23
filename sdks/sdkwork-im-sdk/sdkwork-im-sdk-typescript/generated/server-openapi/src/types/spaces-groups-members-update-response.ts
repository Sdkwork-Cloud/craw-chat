import type { SpaceGroupMemberView } from './space-group-member-view';

export interface SpacesGroupsMembersUpdateResponse {
  code: 0;
  data: unknown & { item: SpaceGroupMemberView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
