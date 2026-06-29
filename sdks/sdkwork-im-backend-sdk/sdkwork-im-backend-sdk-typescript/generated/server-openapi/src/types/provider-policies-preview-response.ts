import type { ProviderBindingCommitResponse } from './provider-binding-commit-response';

export interface ProviderPoliciesPreviewResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
