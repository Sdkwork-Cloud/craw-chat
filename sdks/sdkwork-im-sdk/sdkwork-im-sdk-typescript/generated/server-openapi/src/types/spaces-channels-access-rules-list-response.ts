import type { PageInfo } from './page-info';
import type { SpaceChannelAccessRuleView } from './space-channel-access-rule-view';

export interface SpacesChannelsAccessRulesListResponse {
  code: 0;
  data: unknown & { items: SpaceChannelAccessRuleView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
