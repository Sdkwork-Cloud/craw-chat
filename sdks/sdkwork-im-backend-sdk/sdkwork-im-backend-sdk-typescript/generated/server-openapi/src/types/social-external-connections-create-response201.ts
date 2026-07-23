import type { SocialExternalConnectionCommitResponse } from './social-external-connection-commit-response';

export interface SocialExternalConnectionsCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialExternalConnectionCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
