export interface ApplySharedChannelPolicyRequest {
  appliedAt: string;
  channelId: string;
  connectionId: string;
  conversationId?: string | null;
  eventId: string;
  historyVisibility: string;
  policyId: string;
  policyVersion: string;
}
