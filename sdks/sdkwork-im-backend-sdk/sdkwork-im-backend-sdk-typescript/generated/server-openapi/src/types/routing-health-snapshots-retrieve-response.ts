import type { LooseJsonValue } from './loose-json-value';

export interface RoutingHealthSnapshotsRetrieveResponse {
  code: 0;
  data: unknown & { item: LooseJsonValue; };
  /** Server-owned request correlation id. */
  traceId: string;
}
