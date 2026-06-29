import type { SocialSharedChannelPolicyCommitResponse } from './social-shared-channel-policy-commit-response';

export interface SocialSharedChannelPoliciesCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
