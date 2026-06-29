import type { PageInfo } from './page-info';
import type { SpaceChannelAccessRuleView } from './space-channel-access-rule-view';

export interface SpacesChannelsAccessRulesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
