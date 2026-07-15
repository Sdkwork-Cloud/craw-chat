import type { StreamFrame } from './stream-frame';

export interface AutomationAgentResponsesFramesCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
