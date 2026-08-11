import type {
  SdkworkCommunityCategory,
  SdkworkCommunityEntry,
} from "@sdkwork/community-contracts";
import type { Moment, MomentCircle, MomentComment } from "../types";
import { getMomentsRuntimePort } from "./momentsRuntimePort";

/**
 * Moments feed facade backed by the injected Community App SDK port.
 *
 * Maps the community entry/comment/category records to the moments view
 * models. Viewer-scoped like state stays in session memory because the App
 * API exposes reaction counts but no per-viewer `isLiked` flag (same
 * convention as the canonical community mobile package).
 */

const MOMENT_ENTRY_KIND = "discussion" as const;
const DEFAULT_TITLE_FALLBACK = "分享";
const MAX_TITLE_LENGTH = 40;
const DEFAULT_PAGE_SIZE = 20;

const likedEntryIds = new Set<string>();

/**
 * Clears viewer-scoped like state (session memory). The host may call this on
 * logout so a new signed-in user starts with a clean like state; tests use it
 * to isolate cases.
 */
export function resetMomentsSessionState(): void {
  likedEntryIds.clear();
}

function toTimestamp(value: Date | number | string | null | undefined): number {
  if (!value) {
    return Date.now();
  }
  if (value instanceof Date) {
    return value.getTime();
  }
  if (typeof value === "number") {
    return value;
  }
  const parsed = Date.parse(value);
  return Number.isNaN(parsed) ? Date.now() : parsed;
}

function toContent(entry: SdkworkCommunityEntry): string {
  return (entry.body ?? entry.excerpt ?? entry.title ?? "").trim();
}

function toAvatarUrl(media: SdkworkCommunityEntry["author"]["avatar"]): string | undefined {
  return media?.publicUrl;
}

function mapEntryToMoment(entry: SdkworkCommunityEntry): Moment {
  return {
    id: entry.id,
    author: {
      id: entry.author.id,
      name: entry.author.name,
      avatar: toAvatarUrl(entry.author.avatar),
    },
    content: toContent(entry),
    categoryId: entry.categoryId,
    categoryLabel: entry.categoryLabel,
    timestamp: toTimestamp(entry.publishedAt ?? entry.lastActivityAt),
    isLiked: likedEntryIds.has(entry.id),
    likeCount: entry.stats?.reactionCount ?? 0,
    comments: [],
    commentCount: entry.stats?.commentCount ?? 0,
  };
}

function mapCategoryToCircle(category: SdkworkCommunityCategory): MomentCircle {
  return {
    id: category.id,
    name: category.title,
    avatar: category.avatar,
    description: category.description,
    memberCount: category.memberCount,
  };
}

export const MomentService = {
  /**
   * Global moments feed (no categoryId filter) with offset paging.
   * `hasMore` is a client heuristic: a full page means more pages may exist.
   */
  async getFeed(
    page = 1,
    pageSize = DEFAULT_PAGE_SIZE,
  ): Promise<{ moments: Moment[]; hasMore: boolean }> {
    const entries = await getMomentsRuntimePort().community.feed.list({ page, pageSize });
    return {
      moments: entries.map(mapEntryToMoment),
      hasMore: entries.length >= pageSize,
    };
  },

  /** Circles available for publishing (enabled categories only). */
  async getCircles(): Promise<MomentCircle[]> {
    const categories = await getMomentsRuntimePort().community.categories.list();
    return categories.filter((category) => category.enabled).map(mapCategoryToCircle);
  },

  /** Publish a text moment into the selected circle. */
  async publish(options: { categoryId: string; content: string }): Promise<Moment> {
    const content = options.content.trim();
    const title =
      content.split("\n").join(" ").slice(0, MAX_TITLE_LENGTH) || DEFAULT_TITLE_FALLBACK;
    const entry = await getMomentsRuntimePort().community.entries.create({
      categoryId: options.categoryId,
      kind: MOMENT_ENTRY_KIND,
      title,
      body: content,
    });
    return mapEntryToMoment(entry);
  },

  /** Toggle the viewer's like on a moment. */
  async toggleLike(
    momentId: string,
  ): Promise<{ isLiked: boolean; likeCount: number }> {
    const nextLiked = !likedEntryIds.has(momentId);
    const result = await getMomentsRuntimePort().community.reactions.set(momentId, {
      active: nextLiked,
      reactionType: "like",
    });
    if (nextLiked) {
      likedEntryIds.add(momentId);
    } else {
      likedEntryIds.delete(momentId);
    }
    return { isLiked: nextLiked, likeCount: result.reactionCount };
  },

  /** Fetch the comment list of one moment on demand. */
  async getComments(entryId: string): Promise<MomentComment[]> {
    const comments = await getMomentsRuntimePort().community.comments.list(entryId);
    return comments.map((comment) => ({
      id: comment.id,
      authorId: comment.author.id,
      authorName: comment.author.name,
      authorAvatar: toAvatarUrl(comment.author.avatar),
      content: comment.body,
      timestamp: toTimestamp(comment.createdAt),
    }));
  },

  /** Post a comment on a moment. */
  async addComment(entryId: string, content: string): Promise<MomentComment> {
    const comment = await getMomentsRuntimePort().community.comments.create(entryId, {
      body: content.trim(),
    });
    return {
      id: comment.id,
      authorId: comment.author.id,
      authorName: comment.author.name,
      authorAvatar: toAvatarUrl(comment.author.avatar),
      content: comment.body,
      timestamp: toTimestamp(comment.createdAt),
    };
  },

  /** Delete one of my moments. */
  async deleteMoment(momentId: string): Promise<void> {
    await getMomentsRuntimePort().community.entries.delete(momentId);
    likedEntryIds.delete(momentId);
  },
};
