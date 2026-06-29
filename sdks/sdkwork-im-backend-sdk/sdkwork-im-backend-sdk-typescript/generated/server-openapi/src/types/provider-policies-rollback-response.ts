import type { ProviderBindingCommitResponse } from './provider-binding-commit-response';

export interface ProviderPoliciesRollbackResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
