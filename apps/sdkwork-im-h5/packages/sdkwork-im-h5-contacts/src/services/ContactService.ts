import { getImSdkClient } from "@sdkwork/im-h5-core/sdk";
import type { User } from "@sdkwork/im-h5-types";
import type {
  ContactPreferencesView,
  ContactsResponse,
  CreateConversationRequest,
  CreateConversationResult,
  FriendRequest,
  SocialFriendRequestAcceptanceResponse,
  SocialFriendRequestListResponse,
  SocialFriendRequestMutationResponse,
  SocialUserSearchResponse,
  UpdateContactPreferencesRequest,
} from "@sdkwork/im-h5-core/sdk";
import { MAX_LIST_PAGE_SIZE, uuid } from "@sdkwork/utils";

const CONTACT_PAGE_SIZE = Math.min(50, MAX_LIST_PAGE_SIZE);
const USER_SEARCH_PAGE_SIZE = Math.min(20, MAX_LIST_PAGE_SIZE);
/** Safety cap for the friendship lookup loop (pages of contacts). */
const FRIENDSHIP_LOOKUP_MAX_PAGES = 20;

/** Realtime user-scope event types that reflect friend request changes. */
export const FRIEND_REQUEST_REALTIME_EVENT_TYPES = [
  "friend_request.submitted",
  "friend_request.accepted",
  "friend_request.declined",
  "friend_request.canceled",
];

/** Broadcast after any local friend request mutation so mounted lists refresh. */
export const SDKWORK_IM_H5_FRIEND_REQUESTS_CHANGED_EVENT = "sdkwork-im-h5:friend-requests-changed";

function notifyFriendRequestsChanged(): void {
  if (typeof window !== "undefined") {
    window.dispatchEvent(new CustomEvent(SDKWORK_IM_H5_FRIEND_REQUESTS_CHANGED_EVENT));
  }
}

export type FriendRequestSubmitConflict = "already_friend" | "pending" | "blocked" | "unknown";

function toRecord(value: unknown): Record<string, unknown> {
  return typeof value === "object" && value !== null ? value as Record<string, unknown> : {};
}

function pickString(...values: unknown[]): string {
  for (const value of values) {
    if (typeof value === "string" && value.trim().length > 0) {
      return value.trim();
    }
  }
  return "";
}

/**
 * Classify friend request submission failures into user-facing conflicts
 * (already friends / request pending / blocked) from the ProblemDetail body.
 */
export function classifyFriendRequestSubmitError(error: unknown): FriendRequestSubmitConflict {
  const record = toRecord(error);
  const body = toRecord(record.body ?? record.detail ?? record.problem ?? record.data);
  const code = pickString(body.code, body.title, record.code).toLowerCase();
  const message = pickString(body.detail, body.title, body.message, record.message, record.error).toLowerCase();
  if (code.includes("friendship_pair") || message.includes("already a friend") || message.includes("already exists")) {
    return "already_friend";
  }
  if (code.includes("friend_request_pair") || code.includes("friend_request_conflict") || message.includes("already pending") || message.includes("open friend request")) {
    return "pending";
  }
  if (code.includes("blocked") || code.includes("friend_request_blocked")) {
    return "blocked";
  }
  return "unknown";
}

export interface Contact extends User {
  avatar: string;
  conversationId?: string;
  friendshipId?: string;
  relationshipState?: string;
  isBlocked?: boolean;
}

export interface ContactSearchResult extends User {
  chatId: string;
  email?: string;
  phone?: string;
  relationshipState: string;
}

export interface ContactPage {
  items: Contact[];
  hasMore: boolean;
  nextCursor?: string;
}

export interface FriendRequestPage {
  items: FriendRequest[];
  hasMore: boolean;
  nextCursor?: string;
}

export type FriendRequestDirection = "incoming" | "outgoing";

export interface ContactsSdkPort {
  conversations: {
    create(body: CreateConversationRequest): Promise<CreateConversationResult>;
    addMember(
      conversationId: string,
      body: {
        principalId: string;
        principalKind: string;
        role: string;
        attributes?: Record<string, unknown>;
      },
    ): Promise<unknown>;
  };
  social: {
    contacts: {
      list(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<ContactsResponse>;
      preferences: {
        retrieve(targetUserId: string): Promise<ContactPreferencesView>;
        update(targetUserId: string, body: UpdateContactPreferencesRequest): Promise<ContactPreferencesView>;
      };
    };
    friendships: {
      remove(friendshipId: string): Promise<unknown>;
    };
    users: {
      list(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<SocialUserSearchResponse>;
    };
    friendRequests: {
      list(params?: {
        cursor?: string;
        direction?: string;
        pageSize?: number;
        status?: string;
      }): Promise<SocialFriendRequestListResponse>;
      create(body: {
        requestMessage?: string;
        targetUserId: string;
      }): Promise<SocialFriendRequestMutationResponse>;
      accept(requestId: string): Promise<SocialFriendRequestAcceptanceResponse>;
      decline(requestId: string): Promise<SocialFriendRequestMutationResponse>;
      cancel(requestId: string): Promise<SocialFriendRequestMutationResponse>;
      pendingCount(): Promise<{ count: number }>;
    };
  };
}

export function createContactService(
  resolveClient: () => ContactsSdkPort = getImSdkClient,
) {
  const listContactPage = async (
    cursor?: string,
    q?: string,
  ): Promise<ContactPage> => {
    const response = await resolveClient().social.contacts.list({
      pageSize: CONTACT_PAGE_SIZE,
      ...(cursor ? { cursor } : {}),
      ...(q ? { q } : {}),
    });
    assertCursorPage(response.pageInfo, "IM contacts");
    return {
      items: response.items.map((item) => ({
        id: item.targetUserId,
        name: item.remark?.trim() || item.displayName?.trim() || item.targetUserId,
        avatar: item.avatarUrl ?? "",
        conversationId: item.conversationId ?? item.directChatId ?? item.chatId ?? undefined,
        friendshipId: item.friendshipId,
        relationshipState: item.relationshipState,
        isBlocked: item.isBlocked ?? undefined,
      })),
      hasMore: response.pageInfo.hasMore === true,
      ...(response.pageInfo.nextCursor ? { nextCursor: response.pageInfo.nextCursor } : {}),
    };
  };

  const searchFriends = async (query: string): Promise<ContactSearchResult[]> => {
    const normalizedQuery = query.trim();
    if (!normalizedQuery) {
      return [];
    }
    const response = await resolveClient().social.users.list({
      pageSize: USER_SEARCH_PAGE_SIZE,
      q: normalizedQuery,
    });
    assertCursorPage(response.pageInfo, "IM social user search");
    return response.items.map((item) => ({
      id: item.userId,
      name: item.displayName,
      avatar: item.avatarUrl ?? undefined,
      chatId: item.chatId,
      email: item.email ?? undefined,
      phone: item.phone ?? undefined,
      relationshipState: item.relationshipState,
    }));
  };

  return {
    listContactPage,

    async getContactsDict(): Promise<Record<string, Contact[]>> {
      const page = await listContactPage();
      return groupContacts(page.items);
    },

    async getContacts(): Promise<User[]> {
      const page = await listContactPage();
      return page.items;
    },

    async searchContacts(query: string): Promise<User[]> {
      const normalizedQuery = query.trim();
      if (!normalizedQuery) {
        return [];
      }
      const page = await listContactPage(undefined, normalizedQuery);
      return page.items;
    },

    async getContactPreferences(targetUserId: string): Promise<ContactPreferencesView> {
      return resolveClient().social.contacts.preferences.retrieve(
        requireIdentifier(targetUserId, "target user ID"),
      );
    },

    async updateContactPreferences(
      targetUserId: string,
      body: UpdateContactPreferencesRequest,
    ): Promise<ContactPreferencesView> {
      return resolveClient().social.contacts.preferences.update(
        requireIdentifier(targetUserId, "target user ID"),
        body,
      );
    },

    async removeFriend(targetUserId: string): Promise<void> {
      const normalizedTargetUserId = requireIdentifier(targetUserId, "target user ID");
      // The contact may sit beyond the first page: walk cursor pages until the
      // friendship id is found (or the list is exhausted).
      let cursor: string | undefined;
      for (let depth = 0; depth < FRIENDSHIP_LOOKUP_MAX_PAGES; depth += 1) {
        const page = await listContactPage(cursor);
        const contact = page.items.find((item) => item.id === normalizedTargetUserId);
        if (contact?.friendshipId) {
          await resolveClient().social.friendships.remove(contact.friendshipId);
          notifyFriendRequestsChanged();
          return;
        }
        if (!page.hasMore || !page.nextCursor) {
          break;
        }
        cursor = page.nextCursor;
      }
      throw new Error(`Friendship not found for user: ${normalizedTargetUserId}`);
    },

    async blockContact(targetUserId: string): Promise<ContactPreferencesView> {
      const preferences = await resolveClient().social.contacts.preferences.update(
        requireIdentifier(targetUserId, "target user ID"),
        { isBlocked: true },
      );
      notifyFriendRequestsChanged();
      return preferences;
    },

    searchFriends,

    async listFriendRequests(
      direction: FriendRequestDirection,
      cursor?: string,
      status = "pending",
    ): Promise<FriendRequestPage> {
      const response = await resolveClient().social.friendRequests.list({
        direction,
        pageSize: CONTACT_PAGE_SIZE,
        status,
        ...(cursor ? { cursor } : {}),
      });
      assertCursorPage(response.pageInfo, "IM friend requests");
      return {
        items: response.items,
        hasMore: response.pageInfo.hasMore === true,
        ...(response.pageInfo.nextCursor ? { nextCursor: response.pageInfo.nextCursor } : {}),
      };
    },

    getPendingFriendRequestCount(): Promise<number> {
      return resolveClient().social.friendRequests.pendingCount().then(({ count }) => count);
    },

    acceptFriendRequest(requestId: string): Promise<SocialFriendRequestAcceptanceResponse> {
      return resolveClient().social.friendRequests.accept(requireIdentifier(requestId, "request ID"))
        .then((response) => {
          notifyFriendRequestsChanged();
          return response;
        });
    },

    declineFriendRequest(requestId: string): Promise<SocialFriendRequestMutationResponse> {
      return resolveClient().social.friendRequests.decline(requireIdentifier(requestId, "request ID"))
        .then((response) => {
          notifyFriendRequestsChanged();
          return response;
        });
    },

    cancelFriendRequest(requestId: string): Promise<SocialFriendRequestMutationResponse> {
      return resolveClient().social.friendRequests.cancel(requireIdentifier(requestId, "request ID"))
        .then((response) => {
          notifyFriendRequestsChanged();
          return response;
        });
    },

    async searchFriend(query: string): Promise<ContactSearchResult | null> {
      const results = await searchFriends(query);
      return results.length === 1 ? results[0] : null;
    },

    async addFriend(
      targetUserId: string,
      requestMessage?: string,
    ): Promise<SocialFriendRequestMutationResponse> {
      const normalizedTargetUserId = targetUserId.trim();
      if (!normalizedTargetUserId) {
        throw new Error("A target user ID is required.");
      }
      return resolveClient().social.friendRequests.create({
        targetUserId: normalizedTargetUserId,
        ...(requestMessage?.trim() ? { requestMessage: requestMessage.trim() } : {}),
      }).then((response) => {
        notifyFriendRequestsChanged();
        return response;
      });
    },

    async startDirectConversation(targetUserId: string): Promise<string> {
      const normalizedTargetUserId = targetUserId.trim();
      if (!normalizedTargetUserId) {
        throw new Error("A target user ID is required.");
      }
      // Direct conversations accept a client-supplied id and attach members
      // through the member endpoint; memberUserIds is a group-only field.
      const conversationId = `direct-${uuid()}`;
      await resolveClient().conversations.create({
        conversationId,
        conversationType: "direct",
      });
      await resolveClient().conversations.addMember(conversationId, {
        principalId: normalizedTargetUserId,
        principalKind: "user",
        role: "member",
      });
      return conversationId;
    },
  };
}

function requireIdentifier(value: string, label: string): string {
  const normalized = value.trim();
  if (!normalized) {
    throw new Error(`A ${label} is required.`);
  }
  return normalized;
}

function assertCursorPage(
  pageInfo: { mode: string; hasMore?: boolean; nextCursor?: string | null },
  resource: string,
): void {
  if (pageInfo.mode !== "cursor") {
    throw new Error(`${resource} must use cursor pagination.`);
  }
  if (pageInfo.hasMore && !pageInfo.nextCursor) {
    throw new Error(`${resource} returned hasMore without nextCursor.`);
  }
}

function groupContacts(contacts: Contact[]): Record<string, Contact[]> {
  const grouped = new Map<string, Contact[]>();
  for (const contact of contacts) {
    const firstCharacter = contact.name.charAt(0).toUpperCase();
    const group = /^[A-Z]$/u.test(firstCharacter) ? firstCharacter : "#";
    const items = grouped.get(group) ?? [];
    items.push(contact);
    grouped.set(group, items);
  }

  const result: Record<string, Contact[]> = {};
  for (const group of Array.from(grouped.keys()).sort()) {
    result[group] = (grouped.get(group) ?? []).sort((left, right) =>
      left.name.localeCompare(right.name),
    );
  }
  return result;
}

export const ContactService = createContactService();
