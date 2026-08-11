import type { User } from "@sdkwork/im-h5-types";

/** One comment on a moment (entry comment). */
export interface MomentComment {
  id: string;
  authorId: string;
  authorName: string;
  authorAvatar?: string;
  content: string;
  timestamp: number;
}

/**
 * Moments feed view model mapped from the community App SDK entry shape.
 *
 * `images` / `video` are reserved for the deferred media pipeline: the
 * community App API does not expose entry media yet, so they stay empty.
 */
export interface Moment {
  id: string;
  author: User;
  content: string;
  categoryId: string;
  categoryLabel?: string;
  /** Reserved for the future media upload capability; always empty today. */
  images?: string[];
  /** Reserved for the future media upload capability; always empty today. */
  video?: string;
  timestamp: number;
  /** Viewer-scoped like state kept in session memory (API has no isLiked field). */
  isLiked: boolean;
  likeCount: number;
  comments: MomentComment[];
  commentCount: number;
}

/** Circle (category) choice shown in the publish sheet. */
export interface MomentCircle {
  id: string;
  name: string;
  avatar?: string;
  description?: string;
  memberCount?: number;
}

/** Fullscreen media preview payload shared by the page and the cards. */
export type MomentPreviewState =
  | { type: "images"; images: string[]; index: number }
  | { type: "video"; url: string };
