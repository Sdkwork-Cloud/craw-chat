import type { LooseJsonValue } from './loose-json-value';

export interface StorageValidationCreateResponse201 {
  code: 0;
  data: unknown & { item: LooseJsonValue; };
  /** Server-owned request correlation id. */
  traceId: string;
}
