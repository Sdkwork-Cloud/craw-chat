import type { StreamFrameView } from './stream-frame-view';

export interface StreamsFramesListResponse {
  code: 0;
  data: unknown & { items: StreamFrameView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
