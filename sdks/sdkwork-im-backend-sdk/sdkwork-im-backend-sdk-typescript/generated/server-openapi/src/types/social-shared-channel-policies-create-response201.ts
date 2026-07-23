import type { SocialSharedChannelPolicyCommitResponse } from './social-shared-channel-policy-commit-response';

export interface SocialSharedChannelPoliciesCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelPolicyCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
