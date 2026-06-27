export interface KillSwitchResponse {
  active: boolean;
  disabledBindings: string[];
  disabledCapabilities: string[];
  disabledCodecs: string[];
  reason: string;
  ruleId: string;
}
