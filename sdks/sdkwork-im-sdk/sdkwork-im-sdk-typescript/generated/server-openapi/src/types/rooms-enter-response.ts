import type { EnterRoomResponse } from './enter-room-response';

export interface RoomsEnterResponse {
  code: 0;
  data: unknown & { item: EnterRoomResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
