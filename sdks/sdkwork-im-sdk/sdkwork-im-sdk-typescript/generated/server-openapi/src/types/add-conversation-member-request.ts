export interface AddConversationMemberRequest {
  principalId: string;
  principalKind: string;
  role: string;
  attributes?: Record<string, unknown>;
}
