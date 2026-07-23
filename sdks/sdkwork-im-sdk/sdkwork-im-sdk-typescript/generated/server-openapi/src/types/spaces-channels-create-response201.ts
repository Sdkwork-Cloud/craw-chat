import type { SpaceChannelView } from './space-channel-view';

export interface SpacesChannelsCreateResponse201 {
  code: 0;
  data: unknown & { item: SpaceChannelView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
