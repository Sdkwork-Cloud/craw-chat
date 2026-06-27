export interface RequestAgentToolCallRequest {
  executionId: string;
  toolCallId: string;
  toolName: string;
  argumentsPayload: string;
}
