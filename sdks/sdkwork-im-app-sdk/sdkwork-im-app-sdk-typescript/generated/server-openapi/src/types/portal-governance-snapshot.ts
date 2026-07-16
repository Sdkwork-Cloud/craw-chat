import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalGovernanceRiskSample } from './portal-governance-risk-sample';
import type { PortalInt64Count } from './portal-int64-count';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalGovernanceSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
  sampledEventCount: PortalInt64Count;
  riskSample: PortalGovernanceRiskSample;
}
