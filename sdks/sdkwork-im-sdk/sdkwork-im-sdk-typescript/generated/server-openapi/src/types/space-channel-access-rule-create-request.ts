export interface SpaceChannelAccessRuleCreateRequest {
  ruleType: string;
  principalKind?: string | null;
  principalId?: string | null;
  permission: string;
}
