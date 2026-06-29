import type { RouteNodeLifecycle } from './route-node-lifecycle';

export interface NodesDrainResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
