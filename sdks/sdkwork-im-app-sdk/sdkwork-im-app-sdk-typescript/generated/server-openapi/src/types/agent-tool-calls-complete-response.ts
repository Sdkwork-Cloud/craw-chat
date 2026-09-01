import type { AgentToolCall } from './agent-tool-call';

export interface AgentToolCallsCompleteResponse {
  code: 0;
  data: unknown & { item: AgentToolCall; };
  /** Server-owned request correlation id. */
  traceId: string;
}
