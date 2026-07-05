import type { ConversationInboxEntry } from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';

const GROUP_INBOX_PAGE_LIMIT = 50;

export interface Group {
  id: string;
  name: string;
  type: 'public' | 'private';
  owner: string;
  members: number;
  messagesToDay: number;
  status: 'active' | 'muted' | 'dissolved';
}

export interface GroupListPage {
  data: Group[];
  hasMore: boolean;
  nextCursor?: string;
}

function mapInboxEntryToGroup(entry: ConversationInboxEntry): Group {
  return {
    id: entry.conversationId,
    name: entry.displayName ?? entry.conversationId,
    type: 'private',
    owner: entry.lastSenderId ?? 'unknown',
    members: entry.messageCount ?? 0,
    messagesToDay: entry.unreadCount ?? 0,
    status: 'active',
  };
}

function matchesGroupSearch(group: Group, search?: string): boolean {
  const normalizedSearch = search?.trim().toLowerCase();
  if (!normalizedSearch) {
    return true;
  }
  const haystack = `${group.name} ${group.owner} ${group.id}`.toLowerCase();
  return haystack.includes(normalizedSearch);
}

class GroupService {
  async listGroupsPage(params: {
    pageSize: number;
    cursor?: string;
    search?: string;
  }): Promise<GroupListPage> {
    const client = getImSdkClientWithSession();
    const response = await client.chat.inbox.retrieve({
      pageSize: Math.min(params.pageSize, GROUP_INBOX_PAGE_LIMIT),
      ...(params.cursor ? { cursor: params.cursor } : {}),
    });
    const data = response.items
      .filter((entry) => entry.conversationType.toLowerCase() === 'group')
      .map(mapInboxEntryToGroup)
      .filter((group) => matchesGroupSearch(group, params.search));

    return {
      data,
      hasMore: Boolean(response.hasMore),
      nextCursor: response.hasMore ? (response.nextCursor ?? undefined) : undefined,
    };
  }
}

export const groupService = new GroupService();
