import type { PortalInt64Count } from './portal-int64-count';

export interface PortalWorkspaceView {
  name: string;
  slug: string;
  environment: string;
  tier?: string;
  region?: string;
  supportPlan?: string;
  seats?: PortalInt64Count;
  activeBrands?: PortalInt64Count;
}
