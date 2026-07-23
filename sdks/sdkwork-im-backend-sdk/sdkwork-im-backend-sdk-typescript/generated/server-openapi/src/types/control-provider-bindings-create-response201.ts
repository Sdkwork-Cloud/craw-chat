import type { ProviderBindingCommitResponse } from './provider-binding-commit-response';

export interface ControlProviderBindingsCreateResponse201 {
  code: 0;
  data: unknown & { item: ProviderBindingCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
