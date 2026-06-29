import type { LooseJsonValue } from './loose-json-value';

export interface StorageConfigRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
