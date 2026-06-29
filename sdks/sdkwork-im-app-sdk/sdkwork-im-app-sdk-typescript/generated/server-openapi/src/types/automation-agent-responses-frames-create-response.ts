import type { StreamFrame } from './stream-frame';

export interface AutomationAgentResponsesFramesCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
