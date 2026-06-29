import type { SpaceMemberView } from './space-member-view';

export interface SpacesMembersCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
