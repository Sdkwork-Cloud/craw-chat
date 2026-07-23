import type { PortalGovernanceSnapshot } from './portal-governance-snapshot';

export interface GovernanceRetrieveResponse {
  code: 0;
  data: unknown & { item: PortalGovernanceSnapshot; };
  /** Server-owned request correlation id. */
  traceId: string;
}
