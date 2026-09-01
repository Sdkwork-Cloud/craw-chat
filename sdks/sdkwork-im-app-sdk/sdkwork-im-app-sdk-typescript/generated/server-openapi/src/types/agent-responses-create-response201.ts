import type { StreamSession } from './stream-session';

export interface AgentResponsesCreateResponse201 {
  code: 0;
  data: unknown & { item: StreamSession; };
  /** Server-owned request correlation id. */
  traceId: string;
}
