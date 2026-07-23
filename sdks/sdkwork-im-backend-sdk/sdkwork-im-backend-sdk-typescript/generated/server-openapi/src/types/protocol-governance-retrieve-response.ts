import type { ProtocolGovernanceResponse } from './protocol-governance-response';

export interface ProtocolGovernanceRetrieveResponse {
  code: 0;
  data: unknown & { item: ProtocolGovernanceResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
