import type { StreamFrame } from './stream-frame';

export interface AgentResponsesFramesCreateResponse201 {
  code: 0;
  data: unknown & { item: StreamFrame; };
  /** Server-owned request correlation id. */
  traceId: string;
}
