import type { AutomationExecution } from './automation-execution';

export interface AutomationExecutionsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
