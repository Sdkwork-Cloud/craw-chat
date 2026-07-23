import type { StreamView } from './stream-view';

export interface StreamsAbortResponse {
  code: 0;
  data: unknown & { item: StreamView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
