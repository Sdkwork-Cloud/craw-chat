import type { SpaceMemberView } from './space-member-view';

export interface SpacesMembersRetrieveResponse {
  code: 0;
  data: unknown & { item: SpaceMemberView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
