import type { SocialSharedChannelPolicySnapshotResponse } from './social-shared-channel-policy-snapshot-response';

export interface SocialSharedChannelPoliciesRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
