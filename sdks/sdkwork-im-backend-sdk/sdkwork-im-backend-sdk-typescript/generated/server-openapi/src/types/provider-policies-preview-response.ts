import type { ProviderBindingCommitResponse } from './provider-binding-commit-response';

export interface ProviderPoliciesPreviewResponse {
  code: 0;
  data: unknown & { item: ProviderBindingCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
