import {
  getCmsAppSdkClient,
  type CmsAppSdkClient,
  type CmsFavoriteType,
  type CmsFavoriteView,
} from '@sdkwork/im-h5-core/sdk';
import type { FavoriteItem } from '../components/FavoriteCard';

export type FavoriteFilter =
  | 'all'
  | CmsFavoriteType;

export interface CreateFavoriteInput {
  targetType: string;
  targetId?: string;
  targetUuid?: string;
  targetUrl?: string;
  favoriteType: CmsFavoriteType;
  title: string;
  summary: string;
  sourceDisplayName: string;
  media?: Record<string, unknown>;
}

export interface FavoriteServicePort {
  getFavorites(filter?: FavoriteFilter): Promise<FavoriteItem[]>;
  removeFavorite(favoriteId: string): Promise<void>;
  createFavorite(input: CreateFavoriteInput): Promise<FavoriteItem>;
}

const FAVORITES_PAGE_LIMIT = 20;
/** Safety cap for the cursor loop; 20 pages x 20 items covers the list page. */
const FAVORITES_MAX_PAGES = 20;

/** Card meta aligned with the original favorites page mock (visual unchanged). */
const TYPE_META: Record<CmsFavoriteType, { typeLabel: string; icon: string; color: string }> = {
  link: { typeLabel: '链接', icon: 'Link', color: 'text-blue-400' },
  article: { typeLabel: '文章', icon: 'FileText', color: 'text-blue-500' },
  image: { typeLabel: '相册', icon: 'Image', color: 'text-green-500' },
  file: { typeLabel: '文件', icon: 'File', color: 'text-purple-500' },
  voice: { typeLabel: '语音', icon: 'Mic', color: 'text-orange-500' },
  chat: { typeLabel: '聊天记录', icon: 'MessageCircle', color: 'text-emerald-500' },
};

function formatFavoriteTime(value: string | undefined): string {
  if (!value) {
    return '';
  }
  const timestamp = new Date(value).getTime();
  if (!Number.isFinite(timestamp)) {
    return '';
  }
  const now = new Date();
  const target = new Date(timestamp);
  const startOfToday = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime();
  const startOfTarget = new Date(
    target.getFullYear(),
    target.getMonth(),
    target.getDate(),
  ).getTime();
  const dayDiff = Math.round((startOfToday - startOfTarget) / 86400000);
  if (dayDiff === 1) {
    return '昨天';
  }
  const pad = (value: number) => String(value).padStart(2, '0');
  return `${target.getFullYear()}-${pad(target.getMonth() + 1)}-${pad(target.getDate())}`;
}

function mapFavoriteViewToItem(view: CmsFavoriteView): FavoriteItem {
  const meta = TYPE_META[view.favoriteType] ?? TYPE_META.chat;
  return {
    id: view.favoriteId,
    title: view.title || view.targetId,
    type: view.favoriteType,
    typeLabel: meta.typeLabel,
    time: formatFavoriteTime(view.favoritedAt),
    source: view.sourceDisplayName,
    preview: view.summary,
    icon: meta.icon,
    color: meta.color,
  };
}

class CmsFavoriteService implements FavoriteServicePort {
  constructor(
    private readonly resolveClient: () => CmsAppSdkClient = getCmsAppSdkClient,
  ) {}

  private client(): CmsAppSdkClient {
    return this.resolveClient();
  }

  async getFavorites(filter: FavoriteFilter = 'all'): Promise<FavoriteItem[]> {
    const favoriteType = filter === 'all' ? undefined : (filter as CmsFavoriteType);
    const collected: CmsFavoriteView[] = [];
    let cursor: string | undefined;
    const visitedCursors = new Set<string>();

    for (let page = 0; page < FAVORITES_MAX_PAGES; page += 1) {
      const response = await this.client().favorites.list({
        favoriteType,
        cursor,
        pageSize: FAVORITES_PAGE_LIMIT,
      });
      collected.push(...response.items);
      const nextCursor = response.pageInfo.nextCursor;
      if (!response.pageInfo.hasMore || !nextCursor) {
        break;
      }
      if (visitedCursors.has(nextCursor)) {
        throw new Error('CMS favorites returned a repeated cursor.');
      }
      visitedCursors.add(nextCursor);
      cursor = nextCursor;
    }

    return collected.map(mapFavoriteViewToItem);
  }

  async removeFavorite(favoriteId: string): Promise<void> {
    await this.client().favorites.delete(favoriteId);
  }

  async createFavorite(input: CreateFavoriteInput): Promise<FavoriteItem> {
    const response = await this.client().favorites.create({
      targetType: input.targetType,
      targetId: input.targetId,
      targetUuid: input.targetUuid,
      targetUrl: input.targetUrl,
      favoriteType: input.favoriteType,
      title: input.title,
      summary: input.summary,
      sourceDisplayName: input.sourceDisplayName,
      media: input.media,
    });
    return mapFavoriteViewToItem(response.item);
  }
}

export function createFavoriteService(
  resolveClient: () => CmsAppSdkClient = getCmsAppSdkClient,
): FavoriteServicePort {
  return new CmsFavoriteService(resolveClient);
}

export const favoriteService = createFavoriteService();
