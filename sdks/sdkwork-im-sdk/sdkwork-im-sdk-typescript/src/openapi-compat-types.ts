import type {
  ContactTagView,
  ContactView,
  ConversationInboxEntry,
  ConversationMember,
  FriendRequest as GeneratedFriendRequest,
  MessageFavoriteView,
  MessageInteractionSummaryView,
  SocialUserSearchResult,
  TimelineViewEntry,
} from '@sdkwork/im-sdk-generated';

/** OpenAPI-aligned friend request with stable request id. */
export interface FriendRequest extends Omit<GeneratedFriendRequest, 'requestId'> {
  friendRequestId: string;
  /** @deprecated Use friendRequestId. Retained for legacy wire compatibility. */
  requestId?: string;
}

/** Unwrapped inbox list page aligned with `sdkwork-im-im.openapi.yaml` `data.items` + `data.pageInfo`. */
export interface ConversationInboxPage {
  items: ConversationInboxEntry[];
  pageInfo: SdkWorkListPageInfo;
}

/** Unwrapped timeline list payload aligned with OpenAPI list `data`. */
export interface TimelineResponse {
  items: TimelineViewEntry[];
  pageInfo: SdkWorkListPageInfo;
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
  nextCursor?: string | null;
  hasMore: boolean;
}

export interface DeleteMessageFavoriteResponse {
  favoriteId: string;
  deleted: boolean;
}

export interface ContactsResponse {
  items: ContactView[];
  nextCursor?: string | null;
  hasMore: boolean;
}

/** Unwrapped cursor list page (`data.items` + `data.pageInfo`) for contact tags. */
export interface ContactTagsResponse {
  items: ContactTagView[];
  nextCursor?: string | null;
  hasMore: boolean;
}

export interface DeleteContactTagResponse {
  tagId: string;
  deleted: boolean;
}

export interface SocialUserSearchResponse {
  items: SocialUserSearchResult[];
  nextCursor?: string | null;
  hasMore: boolean;
}

/** Unwrapped cursor list page (`data.items` + `data.pageInfo`) for friend requests. */
export interface SocialFriendRequestListResponse {
  items: FriendRequest[];
  nextCursor?: string | null;
}
