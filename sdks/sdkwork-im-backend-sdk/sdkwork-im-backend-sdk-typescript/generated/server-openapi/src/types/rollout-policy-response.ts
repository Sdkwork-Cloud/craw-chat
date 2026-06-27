export interface RolloutPolicyResponse {
  cellSelector: string;
  operatorOverride: boolean;
  policyId: string;
  regionSelector: string;
  releaseChannel: string;
  tenantAllowlist: string[];
  trafficPercent: string;
}
