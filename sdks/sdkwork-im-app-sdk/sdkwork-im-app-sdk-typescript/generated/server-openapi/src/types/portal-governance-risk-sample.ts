import type { PortalInt64Count } from './portal-int64-count';

export interface PortalGovernanceRiskSample {
  criticalCount: PortalInt64Count;
  highCount: PortalInt64Count;
  warningCount: PortalInt64Count;
  informationalCount: PortalInt64Count;
}
