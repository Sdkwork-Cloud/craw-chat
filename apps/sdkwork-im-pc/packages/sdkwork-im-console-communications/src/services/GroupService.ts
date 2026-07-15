import type { ConversationInboxEntry, ConversationProfileView } from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';

const GROUP_INBOX_PAGE_LIMIT = 200;

export interface Group {
  id: string;
  name: string;
  lastActivityAt?: string;
  unreadCount: number;
}

export interface GroupListPage {
  data: Group[];
  hasMore: boolean;
  nextCursor?: string;
}

function readSdkCursorPageInfo(
  pageInfo: { hasMore?: boolean; nextCursor?: string | null } | undefined,
): Pick<GroupListPage, 'hasMore' | 'nextCursor'> {
  const hasMore = pageInfo?.hasMore === true;
  return {
    hasMore,
    nextCursor: hasMore ? (pageInfo?.nextCursor ?? undefined) : undefined,
  };
}

function mapInboxEntryToGroup(entry: ConversationInboxEntry, profile?: ConversationProfileView): Group {
  return {
    id: entry.conversationId,
    name: profile?.displayName?.trim() || entry.displayName?.trim() || entry.conversationId,
    lastActivityAt: entry.lastActivityAt,
    unreadCount: entry.unreadCount ?? 0,
  };
}

class GroupService {
  async listGroupsPage(params: {
    pageSize: number;
    cursor?: string;
    q?: string;
  }): Promise<GroupListPage> {
    const client = getImSdkClientWithSession();
    const q = params.q?.trim();
    const response = await client.chat.inbox.list({
      pageSize: Math.min(params.pageSize, GROUP_INBOX_PAGE_LIMIT),
      conversationType: 'group',
      ...(params.cursor ? { cursor: params.cursor } : {}),
      ...(q ? { q } : {}),
    });
    const groupEntries = response.items.filter((entry) => entry.conversationType.toLowerCase() === 'group');
    const data = await Promise.all(groupEntries.map(async (entry) => {
      let profile: ConversationProfileView | undefined;
      try {
        profile = await client.conversations.getProfile(entry.conversationId);
      } catch {
        // The inbox projection remains the fallback when profile hydration is unavailable.
      }
      return mapInboxEntryToGroup(entry, profile);
    }));
    const page = readSdkCursorPageInfo(response.pageInfo);

    return {
      data,
      hasMore: page.hasMore,
      nextCursor: page.nextCursor,
    };
  }
}

export const groupService = new GroupService();
