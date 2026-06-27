import type { AgentSubject } from './agent-subject';

export interface StartAgentResponseRequest {
  executionId: string;
  streamId: string;
  streamType: string;
  conversationId: string;
  schemaRef?: string;
  memberId?: string;
  agent: AgentSubject;
}
