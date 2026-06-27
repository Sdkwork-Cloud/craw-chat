import type { MessageFavoriteView } from './message-favorite-view';

export interface FavoriteMessagesResponse {
  items: MessageFavoriteView[];
  nextCursor?: string | null;
  hasMore: boolean;
}
