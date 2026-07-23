import type { SpaceChannelView } from './space-channel-view';

export interface SpacesChannelsUpdateResponse {
  code: 0;
  data: unknown & { item: SpaceChannelView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
