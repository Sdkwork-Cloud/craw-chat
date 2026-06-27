import type { StreamFrameView } from './stream-frame-view';

export interface StreamFramesResponse {
  items: StreamFrameView[];
  nextCursor?: string | null;
  hasMore: boolean;
}
