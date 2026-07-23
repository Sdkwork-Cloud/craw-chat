import type { MessageFavoriteView } from './message-favorite-view';

export interface MessagesFavoritesListResponse {
  code: 0;
  data: unknown & { items: MessageFavoriteView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
