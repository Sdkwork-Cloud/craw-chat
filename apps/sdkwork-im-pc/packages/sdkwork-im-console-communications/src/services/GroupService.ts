import { getImSdkClientWithSession } from '@sdkwork/im-pc-core';
import type { InboxResponse } from '@sdkwork/im-sdk';

export interface Group {
  id: string;
  name: string;
  type: 'public' | 'private';
  members: number;
  owner: string;
  status: 'active' | 'archived';
  messagesToDay: number;
  created: string;
}

export interface GetGroupsResponse {
  data: Group[];
  total: number;
}

const GROUP_INBOX_PAGE_LIMIT = 50;

type ConversationListEntry = InboxResponse['items'][number];

function resolveGroupOwner(entry: ConversationListEntry): string {
  return entry.lastSenderId?.trim()
    || entry.peer?.principalId?.trim()
    || '—';
}

function resolveGroupCreatedDate(entry: ConversationListEntry): string {
  const activityAt = entry.lastActivityAt?.trim();
  if (!activityAt) {
    return '';
  }
  return activityAt.slice(0, 10);
}

function resolveMessagesToday(entry: ConversationListEntry): number {
  const activityAt = entry.lastActivityAt?.trim();
  if (!activityAt) {
    return 0;
  }
  const activityDate = activityAt.slice(0, 10);
  const today = new Date().toISOString().slice(0, 10);
  return activityDate === today ? entry.unreadCount : 0;
}

function mapInboxEntryToGroup(entry: ConversationListEntry): Group {
  return {
    id: entry.conversationId,
    name: entry.displayName?.trim() || 'Group chat',
    type: 'private',
    members: 0,
    owner: resolveGroupOwner(entry),
    status: 'active',
    messagesToDay: resolveMessagesToday(entry),
    created: resolveGroupCreatedDate(entry),
  };
}

async function listAllInboxGroups(): Promise<ConversationListEntry[]> {
  const client = getImSdkClientWithSession();
  const items: ConversationListEntry[] = [];
  let cursor: string | undefined;

  do {
    const response = await client.chat.inbox.retrieve({
      limit: GROUP_INBOX_PAGE_LIMIT,
      ...(cursor ? { cursor } : {}),
    });
    items.push(
      ...response.items.filter((entry) => entry.conversationType.toLowerCase() === 'group'),
    );
    cursor = response.hasMore ? response.nextCursor : undefined;
  } while (cursor);

  return items;
}

class GroupService {
  async getGroups(params: { page: number; pageSize: number; search?: string }): Promise<GetGroupsResponse> {
    const inboxGroups = (await listAllInboxGroups()).map(mapInboxEntryToGroup);
    const normalizedSearch = params.search?.trim().toLowerCase();

    const filtered = normalizedSearch
      ? inboxGroups.filter((group) => (
        group.name.toLowerCase().includes(normalizedSearch)
        || group.owner.toLowerCase().includes(normalizedSearch)
        || group.id.toLowerCase().includes(normalizedSearch)
      ))
      : inboxGroups;

    const start = Math.max(0, (params.page - 1) * params.pageSize);
    const end = start + params.pageSize;

    return {
      data: filtered.slice(start, end),
      total: filtered.length,
    };
  }
}

export const groupService = new GroupService();
