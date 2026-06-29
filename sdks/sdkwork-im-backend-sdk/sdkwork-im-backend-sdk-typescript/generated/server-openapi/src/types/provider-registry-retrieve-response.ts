import type { ProviderRegistrySnapshotResponse } from './provider-registry-snapshot-response';

export interface ProviderRegistryRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
