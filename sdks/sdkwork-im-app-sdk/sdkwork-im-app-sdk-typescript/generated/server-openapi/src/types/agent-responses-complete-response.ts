import type { StreamSession } from './stream-session';

export interface AgentResponsesCompleteResponse {
  code: 0;
  data: unknown & { item: StreamSession; };
  /** Server-owned request correlation id. */
  traceId: string;
}
