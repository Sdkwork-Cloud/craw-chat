import type { StreamFrameView } from './stream-frame-view';

export interface StreamsFramesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
