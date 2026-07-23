import type { SpaceView } from './space-view';

export interface SpacesUpdateResponse {
  code: 0;
  data: unknown & { item: SpaceView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
