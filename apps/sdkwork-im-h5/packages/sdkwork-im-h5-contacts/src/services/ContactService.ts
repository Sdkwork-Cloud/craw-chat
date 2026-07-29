import { getImSdkClient } from "@sdkwork/im-h5-core";
import type { User } from "@sdkwork/im-h5-types";
import type {
  ContactsResponse,
  CreateConversationRequest,
  CreateConversationResult,
  SocialFriendRequestMutationResponse,
  SocialUserSearchResponse,
} from "@sdkwork/im-sdk";
import { MAX_LIST_PAGE_SIZE, uuid } from "@sdkwork/utils";

const CONTACT_PAGE_SIZE = Math.min(50, MAX_LIST_PAGE_SIZE);
const USER_SEARCH_PAGE_SIZE = Math.min(20, MAX_LIST_PAGE_SIZE);

export interface Contact extends User {
  avatar: string;
  conversationId?: string;
  friendshipId?: string;
  relationshipState?: string;
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

export interface ContactsSdkPort {
  conversations: {
    create(body: CreateConversationRequest): Promise<CreateConversationResult>;
  };
  social: {
    contacts: {
      list(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<ContactsResponse>;
    };
    users: {
      list(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<SocialUserSearchResponse>;
    };
    friendRequests: {
      create(body: {
        requestMessage?: string;
        targetUserId: string;
      }): Promise<SocialFriendRequestMutationResponse>;
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

    searchFriends,

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
      });
    },

    async startDirectConversation(targetUserId: string): Promise<string> {
      const normalizedTargetUserId = targetUserId.trim();
      if (!normalizedTargetUserId) {
        throw new Error("A target user ID is required.");
      }
      const result = await resolveClient().conversations.create({
        clientRequestKey: uuid(),
        conversationType: "direct",
        memberUserIds: [normalizedTargetUserId],
      });
      return result.conversationId;
    },
  };
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
