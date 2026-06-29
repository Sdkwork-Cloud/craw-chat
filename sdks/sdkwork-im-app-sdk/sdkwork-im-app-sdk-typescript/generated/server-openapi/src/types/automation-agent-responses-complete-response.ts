import type { StreamSession } from './stream-session';

export interface AutomationAgentResponsesCompleteResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
