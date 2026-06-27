import type { AgentToolCallState } from './agent-tool-call-state';

export interface AgentToolCall {
  tenantId: string;
  executionId: string;
  agentId: string;
  toolCallId: string;
  toolName: string;
  argumentsPayload: string;
  resultPayload?: string;
  state: AgentToolCallState;
  requestedAt: string;
  completedAt?: string;
}
