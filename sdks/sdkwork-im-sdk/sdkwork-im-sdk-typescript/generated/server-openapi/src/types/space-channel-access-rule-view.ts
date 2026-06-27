export interface SpaceChannelAccessRuleView {
  ruleId: string;
  channelId: string;
  ruleType: string;
  principalKind?: string | null;
  principalId?: string | null;
  permission: string;
  createdAt: string;
}
