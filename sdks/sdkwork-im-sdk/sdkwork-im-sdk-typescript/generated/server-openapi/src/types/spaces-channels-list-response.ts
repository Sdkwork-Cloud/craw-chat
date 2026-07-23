import type { PageInfo } from './page-info';
import type { SpaceChannelView } from './space-channel-view';

export interface SpacesChannelsListResponse {
  code: 0;
  data: unknown & { items: SpaceChannelView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
