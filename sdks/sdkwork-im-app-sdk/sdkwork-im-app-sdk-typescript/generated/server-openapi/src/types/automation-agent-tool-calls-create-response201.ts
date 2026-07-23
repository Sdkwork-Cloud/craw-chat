import type { AgentToolCall } from './agent-tool-call';

export interface AutomationAgentToolCallsCreateResponse201 {
  code: 0;
  data: unknown & { item: AgentToolCall; };
  /** Server-owned request correlation id. */
  traceId: string;
}
