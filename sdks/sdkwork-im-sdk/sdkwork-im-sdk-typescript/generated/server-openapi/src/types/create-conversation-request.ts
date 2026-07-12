import type { ConversationAgentAssignment } from './conversation-agent-assignment';

export interface CreateConversationRequest {
  conversationId?: string | null;
  conversationType: string;
  groupName?: string | null;
  clientRequestKey?: string | null;
  memberUserIds?: string[] | null;
  agentAssignments?: ConversationAgentAssignment[] | null;
  policyVersion?: string | null;
  capabilityFlags?: string[] | null;
  historyVisibility?: string | null;
  retentionPolicyRef?: string | null;
}
