import type { Friendship } from './friendship';

export interface SocialFriendshipsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
