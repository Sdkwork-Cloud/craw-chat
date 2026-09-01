import type { AutomationExecution } from './automation-execution';

export interface ExecutionsRetrieveResponse {
  code: 0;
  data: unknown & { item: AutomationExecution; };
  /** Server-owned request correlation id. */
  traceId: string;
}
