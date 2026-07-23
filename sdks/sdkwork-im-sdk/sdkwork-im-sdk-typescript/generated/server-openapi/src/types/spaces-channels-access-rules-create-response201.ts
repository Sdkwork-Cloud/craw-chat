import type { SpaceChannelAccessRuleView } from './space-channel-access-rule-view';

export interface SpacesChannelsAccessRulesCreateResponse201 {
  code: 0;
  data: unknown & { item: SpaceChannelAccessRuleView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
