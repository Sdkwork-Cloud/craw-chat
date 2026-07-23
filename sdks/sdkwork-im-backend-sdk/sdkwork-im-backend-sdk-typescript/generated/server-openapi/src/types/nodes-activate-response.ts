import type { RouteNodeLifecycle } from './route-node-lifecycle';

export interface NodesActivateResponse {
  code: 0;
  data: unknown & { item: RouteNodeLifecycle; };
  /** Server-owned request correlation id. */
  traceId: string;
}
