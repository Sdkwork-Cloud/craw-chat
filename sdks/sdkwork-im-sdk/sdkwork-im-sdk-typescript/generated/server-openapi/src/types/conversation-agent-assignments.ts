import type { ConversationAgentAssignment } from './conversation-agent-assignment';

export interface ConversationAgentAssignments {
  generation: string;
  source: 'default_policy' | 'conversation_override';
  agents: ConversationAgentAssignment[];
}
