import type { SpaceView } from './space-view';

export interface SpacesRetrieveResponse {
  code: 0;
  data: unknown & { item: SpaceView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
