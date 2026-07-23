import type { MessageFavoriteView } from './message-favorite-view';

export interface MessagesFavoritesCreateResponse201 {
  code: 0;
  data: unknown & { item: MessageFavoriteView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
