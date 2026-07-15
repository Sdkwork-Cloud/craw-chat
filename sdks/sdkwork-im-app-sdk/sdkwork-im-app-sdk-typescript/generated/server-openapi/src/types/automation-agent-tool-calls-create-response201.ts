import type { AgentToolCall } from './agent-tool-call';

export interface AutomationAgentToolCallsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
