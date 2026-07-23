import type { StreamView } from './stream-view';

export interface StreamsCreateResponse201 {
  code: 0;
  data: unknown & { item: StreamView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
