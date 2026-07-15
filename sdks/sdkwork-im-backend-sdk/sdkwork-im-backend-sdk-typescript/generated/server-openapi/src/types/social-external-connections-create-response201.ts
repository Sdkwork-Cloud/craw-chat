import type { SocialExternalConnectionCommitResponse } from './social-external-connection-commit-response';

export interface SocialExternalConnectionsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
