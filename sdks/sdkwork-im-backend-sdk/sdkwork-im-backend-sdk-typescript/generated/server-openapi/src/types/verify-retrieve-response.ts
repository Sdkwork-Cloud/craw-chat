import type { AuditChainVerification } from './audit-chain-verification';

export interface VerifyRetrieveResponse {
  code: 0;
  data: unknown & { item: AuditChainVerification; };
  /** Server-owned request correlation id. */
  traceId: string;
}
