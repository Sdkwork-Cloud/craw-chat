import type {
  ContactTagView,
  ContactView,
  ConversationInboxEntry,
  ConversationMessageEntry,
  ConversationMember,
  FriendRequest as GeneratedFriendRequest,
  MessageFavoriteView,
  MessageInteractionSummaryView,
  SocialUserSearchResult,
} from '../generated/server-openapi/dist/index.js';

/** OpenAPI-aligned friend request. */
export type FriendRequest = GeneratedFriendRequest;

/** Unwrapped inbox list page aligned with `sdkwork-im-im.openapi.yaml` `data.items` + `data.pageInfo`. */
export interface ConversationInboxPage {
  items: ConversationInboxEntry[];
  pageInfo: SdkWorkListPageInfo;
}

/** Unwrapped conversation message list payload aligned with OpenAPI list `data`. */
export interface ConversationMessageListResponse {
  items: ConversationMessageEntry[];
  pageInfo: SdkWorkListPageInfo;
  highWatermark: number;
}

export interface ListMembersResponse {
  items: ConversationMember[];
  pageInfo: SdkWorkListPageInfo;
}

export interface SdkWorkListPageInfo {
  mode: 'cursor' | 'offset';
  hasMore?: boolean;
  nextCursor?: string | null;
  page?: number | null;
  pageSize?: number | null;
  totalItems?: string | null;
  totalPages?: number | null;
}

export interface PinnedMessagesResponse {
  items: MessageInteractionSummaryView[];
}

export interface FavoriteMessagesResponse {
  items: MessageFavoriteView[];
  pageInfo: SdkWorkListPageInfo;
}

export interface DeleteMessageFavoriteResponse {
  favoriteId: string;
  deleted: boolean;
}

export interface ContactsResponse {
  items: ContactView[];
  pageInfo: SdkWorkListPageInfo;
}

/** Unwrapped cursor list page (`data.items` + `data.pageInfo`) for contact tags. */
export interface ContactTagsResponse {
  items: ContactTagView[];
  pageInfo: SdkWorkListPageInfo;
}

export interface DeleteContactTagResponse {
  tagId: string;
  deleted: boolean;
}

export interface SocialUserSearchResponse {
  items: SocialUserSearchResult[];
  pageInfo: SdkWorkListPageInfo;
}

/** Unwrapped cursor list page (`data.items` + `data.pageInfo`) for friend requests. */
export interface SocialFriendRequestListResponse {
  items: FriendRequest[];
  pageInfo: SdkWorkListPageInfo;
}
