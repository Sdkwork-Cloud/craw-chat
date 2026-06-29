import type { StreamView } from './stream-view';

export interface StreamsCheckpointCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
