import type { AutomationExecutionRequestResponse } from './automation-execution-request-response';

export interface AutomationExecutionsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
