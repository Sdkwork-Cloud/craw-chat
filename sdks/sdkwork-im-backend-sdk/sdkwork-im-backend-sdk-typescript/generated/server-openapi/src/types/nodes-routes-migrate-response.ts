import type { RouteMigrationResult } from './route-migration-result';

export interface NodesRoutesMigrateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
