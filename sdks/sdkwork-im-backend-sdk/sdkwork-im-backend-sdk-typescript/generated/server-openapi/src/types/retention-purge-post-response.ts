import type { RetentionPurgeResponse } from './retention-purge-response';

export interface RetentionPurgePostResponse {
  code: 0;
  data: unknown & { item: RetentionPurgeResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
