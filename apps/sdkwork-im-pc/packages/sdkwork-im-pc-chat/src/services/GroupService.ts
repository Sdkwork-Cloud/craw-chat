import type {
  InboxResponse,
  ConversationMember,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';
import type { Chat, Message, User } from '@sdkwork/im-pc-types';
import { chatService, createSdkworkChatService, type ChatService } from './ChatService';
import { contactService } from './ContactService';
import { createDefaultAvatar } from './DefaultAvatarService';

export interface GroupService {
  createGroup(name: string, members: string[]): Promise<Chat>;
  getGroups(): Promise<Chat[]>;
  updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat>;
  addMembers(groupId: string, memberIds: string[]): Promise<void>;
  inviteUserToGroup(group: Chat, targetUser: User): Promise<Message>;
  removeMember(groupId: string, memberId: string): Promise<void>;
  deleteGroup(groupId: string): Promise<void>;
  syncGroupMembers(): Promise<GroupMemberSyncChange[]>;
}

type GroupViewState = Partial<Pick<Chat, 'activeCount' | 'avatar' | 'memberCount' | 'members' | 'name' | 'notice'>>;
export type GroupMemberSyncChange = Required<Pick<GroupViewState, 'activeCount' | 'memberCount' | 'members'>> & {
  groupId: string;
};
type ConversationListEntry = InboxResponse['items'][number];
const GROUP_INBOX_PAGE_LIMIT = 100;
const GROUP_MEMBERS_PAGE_LIMIT = 100;
const GROUPS_PAGE_LIMIT = 100;
const GROUP_LIST_HYDRATION_CONCURRENCY = 4;
export const GROUP_INVITE_DESCRIPTOR_PREFIX = 'group-invite:';

export interface GroupInviteDescriptor {
  groupAvatar?: string;
  groupId: string;
  groupName?: string;
  inviterId?: string;
  kind: 'group_invite';
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function parseJsonRecord(value: unknown): Record<string, unknown> | undefined {
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(value);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : undefined;
  } catch {
    return undefined;
  }
}

async function mapWithConcurrencyLimit<T, R>(
  items: T[],
  concurrency: number,
  mapper: (item: T, index: number) => Promise<R>,
): Promise<R[]> {
  const results = new Array<R>(items.length);
  const workerCount = Math.min(Math.max(1, Math.floor(concurrency)), items.length);
  let nextIndex = 0;

  await Promise.all(Array.from({ length: workerCount }, async () => {
    while (nextIndex < items.length) {
      const currentIndex = nextIndex;
      nextIndex += 1;
      results[currentIndex] = await mapper(items[currentIndex] as T, currentIndex);
    }
  }));

  return results;
}

function buildGroupInviteUrl(groupId: string): string {
  return `sdkwork-chat://groups/${encodeURIComponent(groupId)}`;
}

function readGroupIdFromInviteUrl(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }
  const match = /^sdkwork-chat:\/\/groups\/([^/?#]+)/u.exec(value.trim());
  if (!match?.[1]) {
    return undefined;
  }
  try {
    return decodeURIComponent(match[1]);
  } catch {
    return match[1];
  }
}

function buildGroupInviteDescriptor(group: Chat, inviterId: string): string {
  const descriptor: GroupInviteDescriptor = {
    groupId: group.id,
    kind: 'group_invite',
    ...(group.avatar ? { groupAvatar: group.avatar } : {}),
    ...(group.name ? { groupName: group.name } : {}),
    ...(inviterId ? { inviterId } : {}),
  };
  return `${GROUP_INVITE_DESCRIPTOR_PREFIX}${encodeURIComponent(JSON.stringify(descriptor))}`;
}

export function parseGroupInviteDescriptor(message: Message): GroupInviteDescriptor | undefined {
  if (message.type !== 'card') {
    return undefined;
  }

  if (message.desc?.startsWith(GROUP_INVITE_DESCRIPTOR_PREFIX)) {
    const payload = message.desc.slice(GROUP_INVITE_DESCRIPTOR_PREFIX.length);
    try {
      const parsed = parseJsonRecord(decodeURIComponent(payload));
      const groupId = pickString(parsed?.groupId);
      if (groupId) {
        return {
          groupId,
          kind: 'group_invite',
          ...(pickString(parsed?.groupAvatar) ? { groupAvatar: pickString(parsed?.groupAvatar) } : {}),
          ...(pickString(parsed?.groupName) ? { groupName: pickString(parsed?.groupName) } : {}),
          ...(pickString(parsed?.inviterId) ? { inviterId: pickString(parsed?.inviterId) } : {}),
        };
      }
    } catch {
      return undefined;
    }
  }

  const groupId = readGroupIdFromInviteUrl(message.content);
  return groupId
    ? {
        groupId,
        kind: 'group_invite',
      }
    : undefined;
}

function createGroupId(): string {
  const requestId =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `pc-group-${requestId}`;
}

function createGroupAvatar(): string {
  return createDefaultAvatar('group');
}

function normalizeGroupName(name: string, memberCount: number): string {
  const trimmedName = name.trim();
  return trimmedName || `群聊(${memberCount}人)`;
}

function uniqueMemberIds(memberIds: string[]): string[] {
  const result: string[] = [];
  const seen = new Set<string>();
  for (const memberId of memberIds) {
    const normalizedMemberId = memberId.trim();
    if (!normalizedMemberId || seen.has(normalizedMemberId)) {
      continue;
    }
    seen.add(normalizedMemberId);
    result.push(normalizedMemberId);
  }
  return result;
}

function mapActiveMemberIds(members: ConversationMember[]): string[] {
  return members
    .filter((member) => member.state === 'joined' || member.state === 'invited')
    .map((member) => member.principalId);
}

function isGeneratedGroupName(group: Chat): boolean {
  return group.name === 'Group chat'
    || group.name === `Group ${group.id}`
    || group.name === group.id
    || /^(?:Group\s+c_|c_group|pc-group-|conversation[-_:])/iu.test(group.name.trim());
}

function mergeCachedGroupViewState(group: Chat, state: GroupViewState | undefined): Chat {
  return {
    ...group,
    ...(group.activeCount === undefined && state?.activeCount !== undefined ? { activeCount: state.activeCount } : {}),
    ...(group.avatar === undefined && state?.avatar !== undefined ? { avatar: state.avatar } : {}),
    ...(group.memberCount === undefined && state?.memberCount !== undefined ? { memberCount: state.memberCount } : {}),
    ...(group.members === undefined && state?.members !== undefined ? { members: state.members } : {}),
    ...(isGeneratedGroupName(group) && state?.name !== undefined ? { name: state.name } : {}),
    ...(group.notice === undefined && state?.notice !== undefined ? { notice: state.notice } : {}),
  };
}

function mapConversationEntryToGroup(entry: ConversationListEntry): Chat {
  const updatedAt = new Date(entry.lastActivityAt).getTime();
  const entryRecord = toRecord(entry);
  const projectedName = pickString(entryRecord.displayName, entryRecord.display_name);
  const projectedAvatar = pickString(entryRecord.avatarUrl, entryRecord.avatar_url);
  return {
    id: entry.conversationId,
    name: projectedName ?? 'Group chat',
    avatar: projectedAvatar ?? createGroupAvatar(),
    type: 'group',
    unreadCount: entry.unreadCount,
    updatedAt: Number.isFinite(updatedAt) ? updatedAt : Date.now(),
  };
}

function hasGroupDisplayProjection(entry: ConversationListEntry): boolean {
  const entryRecord = toRecord(entry);
  return Boolean(pickString(entryRecord.displayName, entryRecord.display_name));
}

class SdkworkGroupService implements GroupService {
  private readonly groupViewState = new Map<string, GroupViewState>();
  private readonly chatClient: ChatService;

  constructor(
    private readonly getClient: () => ImSdkClient = getImSdkClientWithSession,
    chatClient?: ChatService,
  ) {
    this.chatClient = chatClient ?? (
      getClient === getImSdkClientWithSession
        ? chatService
        : createSdkworkChatService(getClient)
    );
  }

  private client(): ImSdkClient {
    return this.getClient();
  }

  private async listAllConversationMembers(groupId: string): Promise<ConversationMember[]> {
    const items: ConversationMember[] = [];
    let cursor: string | undefined;

    while (true) {
      const response = await this.client().conversations.listMembers(groupId, {
        limit: GROUP_MEMBERS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);

      if (!response.hasMore || !response.nextCursor || response.nextCursor === cursor) {
        break;
      }

      cursor = response.nextCursor;
    }

    return items;
  }

  private async listAllInboxGroups(): Promise<ConversationListEntry[]> {
    const items: ConversationListEntry[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().chat?.inbox?.retrieve({
        limit: GROUP_INBOX_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      if (!response) {
        break;
      }
      items.push(...response.items.filter((entry) => entry.conversationType.toLowerCase() === 'group'));
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  private async listAllConversationEntries(): Promise<ConversationListEntry[]> {
    const items: ConversationListEntry[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().conversations.list({
        limit: GROUPS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  private async hydrateConversationEntryGroup(entry: ConversationListEntry): Promise<Chat | null> {
    const group = mapConversationEntryToGroup(entry);
    try {
      const preferences = await this.client().conversations.getPreferences(entry.conversationId);
      if (preferences.isHidden) {
        return null;
      }
    } catch {
      // Keep newly available groups visible when preference hydration is temporarily unavailable.
    }

    try {
      const profile = await this.client().conversations.getProfile(entry.conversationId);
      return {
        ...group,
        ...(profile.displayName ? { name: profile.displayName } : {}),
        ...(profile.avatarUrl ? { avatar: profile.avatarUrl } : {}),
        notice: profile.notice,
      };
    } catch {
      return group;
    }
  }

  async createGroup(name: string, memberIds: string[]): Promise<Chat> {
    const currentUserId = contactService.getCurrentUser().id;
    const invitedMemberIds = uniqueMemberIds(memberIds).filter((memberId) => memberId !== currentUserId);
    const members = [currentUserId, ...invitedMemberIds];
    const groupId = createGroupId();

    const result = await this.client().conversations.create({
      conversationId: groupId,
      conversationType: 'group',
    });
    const boundGroupId = result.conversationId;

    const groupName = normalizeGroupName(name, members.length);
    const groupAvatar = createGroupAvatar();
    await this.client().conversations.updateProfile(boundGroupId, {
      ...(groupAvatar ? { avatarUrl: groupAvatar } : {}),
      displayName: groupName,
    });
    await this.client().conversations.updatePreferences(boundGroupId, { isHidden: false });

    for (const memberId of invitedMemberIds) {
      await this.client().conversations.addMember(boundGroupId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
    }

    const group: Chat = {
      id: boundGroupId,
      name: groupName,
      avatar: groupAvatar,
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.now(),
      memberCount: members.length,
      members,
      activeCount: members.length,
    };

    this.groupViewState.set(boundGroupId, {
      activeCount: group.activeCount,
      avatar: group.avatar,
      memberCount: group.memberCount,
      members: group.members,
      name: group.name,
      notice: group.notice,
    });
    return group;
  }

  async getGroups(): Promise<Chat[]> {
    const inboxGroups = await this.listAllInboxGroups();
    const groupsById = new Map<string, Chat>();
    const hydratedInboxGroups = await mapWithConcurrencyLimit(
      inboxGroups,
      GROUP_LIST_HYDRATION_CONCURRENCY,
      async (entry) => (hasGroupDisplayProjection(entry)
        ? mapConversationEntryToGroup(entry)
        : await this.hydrateConversationEntryGroup(entry)),
    );
    for (const group of hydratedInboxGroups) {
      if (group) {
        groupsById.set(group.id, group);
      }
    }
    const entries = await this.listAllConversationEntries().catch(() => []);
    const missingGroupEntries = entries.filter((entry) => (
      entry.conversationType.toLowerCase() === 'group'
      && !groupsById.has(entry.conversationId)
    ));
    const hydratedMissingGroups = await mapWithConcurrencyLimit(
      missingGroupEntries,
      GROUP_LIST_HYDRATION_CONCURRENCY,
      async (entry) => this.hydrateConversationEntryGroup(entry),
    );
    for (const group of hydratedMissingGroups) {
      if (group) {
        groupsById.set(group.id, group);
      }
    }
    const groups = await mapWithConcurrencyLimit(
      Array.from(groupsById.values()),
      GROUP_LIST_HYDRATION_CONCURRENCY,
      async (group) => this.withMemberState(group),
    );
    return groups.sort((left, right) => right.updatedAt - left.updatedAt);
  }

  async syncGroupMembers(): Promise<GroupMemberSyncChange[]> {
    const entries = await this.listAllConversationEntries();
    const groupIds = entries
      .filter((entry) => entry.conversationType.toLowerCase() === 'group')
      .map((entry) => entry.conversationId);
    const changes = await mapWithConcurrencyLimit(
      groupIds,
      GROUP_LIST_HYDRATION_CONCURRENCY,
      async (groupId): Promise<GroupMemberSyncChange> => {
        const state = await this.syncMemberViewState(groupId, false);
        return {
          ...state,
          groupId,
        };
      },
    );

    return changes.sort((left, right) => left.groupId.localeCompare(right.groupId));
  }

  private async withMemberState(group: Chat): Promise<Chat> {
    try {
      const memberState = await this.syncMemberViewState(group.id, false);
      return {
        ...mergeCachedGroupViewState(group, this.groupViewState.get(group.id)),
        activeCount: memberState.activeCount,
        memberCount: memberState.memberCount,
        members: memberState.members,
      };
    } catch {
      return mergeCachedGroupViewState(group, this.groupViewState.get(group.id));
    }
  }

  private async syncMemberViewState(
    groupId: string,
    _syncChatView = false,
  ): Promise<Required<Pick<GroupViewState, 'activeCount' | 'memberCount' | 'members'>>> {
    const members = mapActiveMemberIds(await this.listAllConversationMembers(groupId));
    const existingState = this.groupViewState.get(groupId) ?? {};
    const nextState = {
      ...existingState,
      activeCount: members.length,
      memberCount: members.length,
      members,
    };
    this.groupViewState.set(groupId, nextState);
    return {
      activeCount: nextState.activeCount,
      memberCount: nextState.memberCount,
      members: nextState.members,
    };
  }

  async updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat> {
    const profileUpdate = {
      ...(updates.avatar !== undefined ? { avatarUrl: updates.avatar } : {}),
      ...(updates.name !== undefined ? { displayName: updates.name } : {}),
      ...(updates.notice !== undefined ? { notice: updates.notice } : {}),
    };
    const profile = Object.keys(profileUpdate).length > 0
      ? await this.client().conversations.updateProfile(groupId, profileUpdate)
      : undefined;
    const updatedGroup: Chat = {
      id: groupId,
      name: pickString(profile?.displayName, updates.name) ?? 'Group chat',
      avatar: pickString(profile?.avatarUrl, updates.avatar) ?? createGroupAvatar(),
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.now(),
      activeCount: updates.activeCount,
      memberCount: updates.memberCount,
      members: updates.members,
      notice: profile?.notice ?? updates.notice,
    };
    const existingState = this.groupViewState.get(groupId) ?? {};
    this.groupViewState.set(groupId, {
      ...existingState,
      activeCount: updatedGroup.activeCount ?? updates.activeCount ?? existingState.activeCount,
      avatar: updatedGroup.avatar ?? updates.avatar ?? existingState.avatar,
      memberCount: updatedGroup.memberCount ?? updates.memberCount ?? existingState.memberCount,
      members: updatedGroup.members ?? updates.members ?? existingState.members,
      name: updatedGroup.name ?? updates.name ?? existingState.name,
      notice: updatedGroup.notice ?? updates.notice ?? existingState.notice,
    });
    return updatedGroup;
  }

  async addMembers(groupId: string, memberIds: string[]): Promise<void> {
    const existingMembers = await this.listAllConversationMembers(groupId);
    const activeMemberIds = new Set(mapActiveMemberIds(existingMembers));
    const membersToAdd = uniqueMemberIds(memberIds).filter((memberId) => !activeMemberIds.has(memberId));

    for (const memberId of membersToAdd) {
      await this.client().conversations.addMember(groupId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
      activeMemberIds.add(memberId);
    }

    await this.syncMemberViewState(groupId);
  }

  async inviteUserToGroup(group: Chat, targetUser: User): Promise<Message> {
    const targetUserId = targetUser.id.trim();
    if (!targetUserId) {
      throw new Error('Group invite target user id is required');
    }

    await this.addMembers(group.id, [targetUserId]);
    const directChat = await this.chatClient.startDirectChat({
      avatar: targetUser.avatar,
      conversationId: targetUser.conversationId,
      directChatId: targetUser.directChatId,
      id: targetUserId,
      name: targetUser.name,
    });
    const currentUserId = contactService.getCurrentUser().id;
    return this.chatClient.sendMessage(
      directChat.id,
      buildGroupInviteUrl(group.id),
      'card',
      undefined,
      {
        appIcon: group.avatar,
        desc: buildGroupInviteDescriptor(group, currentUserId),
        fileName: '邀请你加入群聊',
      },
    );
  }

  async removeMember(groupId: string, memberId: string): Promise<void> {
    const normalizedMemberId = memberId.trim();
    if (!normalizedMemberId) {
      throw new Error('Group member id is required');
    }

    const members = await this.listAllConversationMembers(groupId);
    const targetMember = members.find((member) => (
      member.memberId === normalizedMemberId
      || member.principalId === normalizedMemberId
    ));
    if (!targetMember) {
      throw new Error('Group member is not available');
    }

    await this.client().conversations.removeMember(groupId, {
      memberId: targetMember.memberId,
    });
    await this.syncMemberViewState(groupId);
  }

  async deleteGroup(groupId: string): Promise<void> {
    await this.client().conversations.leave(groupId);
    await this.chatClient.deleteChat(groupId).catch(() => undefined);
    this.groupViewState.delete(groupId);
  }

}

export function createSdkworkGroupService(getClient?: () => ImSdkClient, chatClient?: ChatService): GroupService {
  return new SdkworkGroupService(getClient, chatClient);
}

export const groupService = createSdkworkGroupService();
