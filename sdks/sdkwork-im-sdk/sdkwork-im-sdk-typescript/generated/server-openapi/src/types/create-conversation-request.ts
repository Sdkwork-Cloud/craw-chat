import type { ConversationAgentAssignment } from './conversation-agent-assignment';

export interface CreateConversationRequest {
  conversationId?: string | null;
  conversationType: string;
  groupName?: string | null;
  clientRequestKey?: string | null;
  /** For group conversations only. When true, requests one Knowledgebase provisioning attempt after the group is durably created. Omitted or false never reserves, provisions, or validates a group Knowledgebase scope. */
  initializeKnowledgebase?: boolean;
  memberUserIds?: string[] | null;
  agentAssignments?: ConversationAgentAssignment[] | null;
  policyVersion?: string | null;
  capabilityFlags?: string[] | null;
  historyVisibility?: string | null;
  retentionPolicyRef?: string | null;
}
