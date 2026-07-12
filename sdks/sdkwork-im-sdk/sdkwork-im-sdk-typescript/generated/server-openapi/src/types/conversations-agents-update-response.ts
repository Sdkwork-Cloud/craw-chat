import type { ConversationAgentAssignments } from './conversation-agent-assignments';

export interface ConversationsAgentsUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
