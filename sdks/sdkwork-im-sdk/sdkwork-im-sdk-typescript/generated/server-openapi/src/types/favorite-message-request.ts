import type { MessageFavoriteType } from './message-favorite-type';

export interface FavoriteMessageRequest {
  conversationId: string;
  favoriteType: MessageFavoriteType;
  title: string;
  contentPreview: string;
  sourceDisplayName: string;
}
