import type { PageInfo } from './page-info';
import type { SpaceGroupView } from './space-group-view';

export interface SpacesGroupsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
