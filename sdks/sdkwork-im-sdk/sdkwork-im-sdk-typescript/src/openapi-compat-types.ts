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
export interface FriendRequest extends GeneratedFriendRequest {
  requestId: string;
}

/** Unwrapped inbox list payload aligned with `sdkwork-im-im.openapi.yaml#InboxResponse`. */
export interface InboxResponse {
  items: ConversationInboxEntry[];
  nextCursor?: string | null;
  hasMore: boolean;
}

/** Unwrapped timeline list payload aligned with `sdkwork-im-im.openapi.yaml#TimelineResponse`. */
export interface TimelineResponse {
  items: TimelineViewEntry[];
  nextAfterSeq?: number | null;
  hasMore: boolean;
}

export interface ListMembersResponse {
  items: ConversationMember[];
  nextCursor?: string | null;
  hasMore: boolean;
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

export interface MessageVisibilityMutationResult {
  tenantId: string;
  conversationId: string;
  messageId: string;
  messageSeq: number;
  principalKind: string;
  principalId: string;
  isDeleted: boolean;
  updatedAt: string;
}

export interface ContactsResponse {
  items: ContactView[];
  nextCursor?: string | null;
  hasMore: boolean;
}

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

export interface SocialFriendRequestListResponse {
  items: FriendRequest[];
  nextCursor?: string | null;
}
