import type { SpaceGroupView } from './space-group-view';

export interface SpacesGroupsUpdateResponse {
  code: 0;
  data: unknown & { item: SpaceGroupView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
