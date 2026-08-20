export interface AuditChainVerification {
  tenantId: string;
  verifiedAt: string;
  total: string;
  chainHeadHash: string | null;
  chainValid: boolean;
}
