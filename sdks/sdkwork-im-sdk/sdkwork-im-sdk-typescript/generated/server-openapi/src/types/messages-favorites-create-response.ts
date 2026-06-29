import type { MessageFavoriteView } from './message-favorite-view';

export interface MessagesFavoritesCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
