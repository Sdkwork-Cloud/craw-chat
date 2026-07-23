import type { ConversationAgentAssignments } from './conversation-agent-assignments';

export interface ConversationsAgentsRetrieveResponse {
  code: 0;
  data: unknown & { item: ConversationAgentAssignments; };
  /** Server-owned request correlation id. */
  traceId: string;
}
