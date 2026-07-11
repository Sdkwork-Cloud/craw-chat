export interface CreateConversationRequest {
  conversationId?: string | null;
  conversationType: string;
  groupName?: string | null;
  clientRequestKey?: string | null;
  policyVersion?: string | null;
  capabilityFlags?: string[] | null;
  historyVisibility?: string | null;
  retentionPolicyRef?: string | null;
}
