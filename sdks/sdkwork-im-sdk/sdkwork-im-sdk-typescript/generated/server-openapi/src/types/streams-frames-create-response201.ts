import type { StreamFrameView } from './stream-frame-view';

export interface StreamsFramesCreateResponse201 {
  code: 0;
  data: unknown & { item: StreamFrameView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
