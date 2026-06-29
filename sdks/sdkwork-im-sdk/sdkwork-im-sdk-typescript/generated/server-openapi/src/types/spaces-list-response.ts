import type { PageInfo } from './page-info';
import type { SpaceView } from './space-view';

export interface SpacesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
