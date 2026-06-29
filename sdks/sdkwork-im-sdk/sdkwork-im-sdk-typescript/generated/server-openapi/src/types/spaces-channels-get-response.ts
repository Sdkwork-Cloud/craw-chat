import type { SpaceChannelView } from './space-channel-view';

export interface SpacesChannelsGetResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
