import type { SocialExternalConnectionCommitResponse } from './social-external-connection-commit-response';

export interface SocialExternalConnectionsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
