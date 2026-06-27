import type {
  ImSdkClient,
  MessageFavoriteType,
  MessageFavoriteView,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';

export interface FavoriteItem {
  id: string;
  type: 'link' | 'image' | 'file' | 'chat';
  title: string;
  content: string;
  source: string;
  timestamp: number;
  conversationId?: string;
  messageId?: string;
}

export interface FavoriteService {
  getFavorites(filter?: string): Promise<FavoriteItem[]>;
  addFavorite(item: Omit<FavoriteItem, 'id' | 'timestamp'>): Promise<FavoriteItem>;
  removeFavorite(id: string): Promise<void>;
}

const FAVORITES_PAGE_LIMIT = 100;

function mapFilterToFavoriteType(filter: string): MessageFavoriteType | undefined {
  const typeMap: Record<string, MessageFavoriteType> = {
    links: 'link',
    images: 'image',
    files: 'file',
    chat: 'chat',
  };
  return typeMap[filter];
}

function parseFavoriteTimestamp(value: string | undefined): number {
  if (!value) {
    return Date.now();
  }
  const timestamp = new Date(value).getTime();
  return Number.isFinite(timestamp) ? timestamp : Date.now();
}

function normalizeFavoriteText(value: string | undefined, fallback: string): string {
  const trimmed = value?.trim();
  return trimmed || fallback;
}

function mapFavoriteViewToFavoriteItem(view: MessageFavoriteView): FavoriteItem {
  return {
    id: view.favoriteId,
    type: view.favoriteType,
    title: normalizeFavoriteText(view.title, view.messageId),
    content: normalizeFavoriteText(view.contentPreview, ''),
    source: normalizeFavoriteText(view.sourceDisplayName, ''),
    timestamp: parseFavoriteTimestamp(view.favoritedAt),
    conversationId: view.conversationId,
    messageId: view.messageId,
  };
}

class SdkworkFavoriteService implements FavoriteService {
  constructor(private readonly getClient: () => ImSdkClient = getImSdkClientWithSession) {}

  private client(): ImSdkClient {
    return this.getClient();
  }

  async getFavorites(filter: string = 'all'): Promise<FavoriteItem[]> {
    const favoriteType = mapFilterToFavoriteType(filter);
    const response = await this.client().messages.favorites.list({
      ...(favoriteType ? { favoriteType } : {}),
      limit: FAVORITES_PAGE_LIMIT,
    });
    return response.items.map(mapFavoriteViewToFavoriteItem);
  }

  async addFavorite(item: Omit<FavoriteItem, 'id' | 'timestamp'>): Promise<FavoriteItem> {
    if (!item.messageId) {
      throw new Error('Favorite messageId is required');
    }
    if (!item.conversationId) {
      throw new Error('Favorite conversationId is required');
    }

    const view = await this.client().messages.favorites.create(item.messageId, {
      conversationId: item.conversationId,
      favoriteType: item.type,
      title: item.title,
      contentPreview: item.content,
      sourceDisplayName: item.source,
    });
    return mapFavoriteViewToFavoriteItem(view);
  }

  async removeFavorite(id: string): Promise<void> {
    await this.client().messages.favorites.delete(id);
  }
}

export function createSdkworkFavoriteService(getClient?: () => ImSdkClient): FavoriteService {
  return new SdkworkFavoriteService(getClient);
}

export const favoriteService = createSdkworkFavoriteService();
