import type { RoomView } from './room-view';

export interface RoomsRetrieveResponse {
  code: 0;
  data: unknown & { item: RoomView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
