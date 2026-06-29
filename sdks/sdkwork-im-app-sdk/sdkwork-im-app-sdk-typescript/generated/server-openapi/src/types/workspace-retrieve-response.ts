import type { PortalWorkspaceView } from './portal-workspace-view';

export interface WorkspaceRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
