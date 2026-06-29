import type { SpaceGroupView } from './space-group-view';

export interface SpacesGroupsUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
