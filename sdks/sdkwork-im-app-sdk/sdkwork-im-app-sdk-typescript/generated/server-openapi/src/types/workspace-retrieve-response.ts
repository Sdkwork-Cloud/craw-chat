import type { PortalWorkspaceView } from './portal-workspace-view';

export interface WorkspaceRetrieveResponse {
  code: 0;
  data: unknown & { item: PortalWorkspaceView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
