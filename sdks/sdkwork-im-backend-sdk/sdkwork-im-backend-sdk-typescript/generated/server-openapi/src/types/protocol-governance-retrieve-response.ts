import type { ProtocolGovernanceResponse } from './protocol-governance-response';

export interface ProtocolGovernanceRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
