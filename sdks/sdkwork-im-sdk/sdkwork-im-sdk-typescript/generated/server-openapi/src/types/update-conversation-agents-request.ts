import type { ConversationAgentAssignment } from './conversation-agent-assignment';

export interface UpdateConversationAgentsRequest {
  expectedGeneration: string;
  agentAssignments: ConversationAgentAssignment[];
}
