import type { ProtocolRegistryResponse } from './protocol-registry-response';

export interface ProtocolRegistryRetrieveResponse {
  code: 0;
  data: unknown & { item: ProtocolRegistryResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
