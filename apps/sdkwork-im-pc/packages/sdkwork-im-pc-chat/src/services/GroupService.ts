import type {
  ConversationInboxEntry,
  ConversationMember,
  ImSdkClient,
  ImConversationAgentAssignment,
  ImConversationAgentAssignmentSet,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';
import { forEachCursorPage, SDKWORK_DEFAULT_PAGE_SIZE, SDKWORK_MAX_PAGE_SIZE } from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';
import {
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  readAppSdkSessionTokens,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core/sdk/session';
import type { Chat, Message, User } from '@sdkwork/im-pc-types';
import { uuid } from '@sdkwork/utils/id';
import { chatService, createSdkworkChatService, type ChatService } from './ChatService';
import { contactService } from './ContactService';
import { createDefaultAvatar } from './DefaultAvatarService';
import {
  isCurrentGroupOwnerMember,
  resolveCurrentGroupKnowledgebaseMemberAccess,
} from './GroupKnowledgebaseAccessPolicy';

export {
  isCurrentGroupOwnerMember,
  resolveCurrentGroupKnowledgebaseMemberAccess,
} from './GroupKnowledgebaseAccessPolicy';

export interface GroupListPage {
  items: Chat[];
  hasMore: boolean;
  nextCursor?: string;
}

export type GroupMemberRole = 'owner' | 'admin' | 'member' | 'guest' | 'unknown';

export interface GroupMemberListItem {
  id: string;
  memberId: string;
  role: GroupMemberRole;
}

export interface GroupMemberListPage {
  items: GroupMemberListItem[];
  hasMore: boolean;
  nextCursor?: string;
}

export interface GroupAgentAssignment {
  agentId: string;
  revisionId?: string;
  name?: string;
  avatar?: string;
  enabled?: boolean;
}

export interface GroupAgentAssignmentSet {
  generation: number;
  source: string;
  agents: GroupAgentAssignment[];
}

export interface GroupKnowledgebaseMemberAccess {
  canInitialize: boolean;
  canOpen: boolean;
}

export type GroupKnowledgebaseMemberAccessLookup =
  | { kind: 'resolved'; access: GroupKnowledgebaseMemberAccess }
  | { kind: 'failed' };

export interface CreateGroupOptions {
  clientRequestKey?: string;
  /**
   * Explicitly request one post-create Knowledgebase provisioning attempt.
   * Omission preserves the default lazy lifecycle and is the normal path.
   */
  initializeKnowledgebase?: boolean;
}

export const MAX_GROUP_INITIAL_MEMBERS = 200;

function readSdkCursorPageInfo(
  pageInfo: { hasMore?: boolean; nextCursor?: string | null } | undefined,
): Pick<GroupListPage, 'hasMore' | 'nextCursor'> {
  const hasMore = pageInfo?.hasMore === true;
  return {
    hasMore,
    nextCursor: hasMore ? (pageInfo?.nextCursor ?? undefined) : undefined,
  };
}

export interface GroupService {
  createGroup(
    name: string,
    members: string[],
    agentAssignments?: GroupAgentAssignment[],
    options?: CreateGroupOptions,
  ): Promise<Chat>;
  getGroupById(groupId: string): Promise<Chat | null>;
  getAgentAssignments(groupId: string): Promise<GroupAgentAssignmentSet>;
  canManageAgents(groupId: string): Promise<boolean>;
  isCurrentUserGroupOwner(groupId: string): Promise<boolean>;
  getCurrentUserGroupKnowledgebaseAccess(groupId: string): Promise<GroupKnowledgebaseMemberAccess>;
  retrieveCurrentUserGroupKnowledgebaseAccess(groupId: string): Promise<GroupKnowledgebaseMemberAccessLookup>;
  replaceAgentAssignments(groupId: string, expectedGeneration: number, assignments: GroupAgentAssignment[]): Promise<GroupAgentAssignmentSet>;
  getGroups(): Promise<Chat[]>;
  listGroupsPage(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<GroupListPage>;
  listGroupMembersPage(
    groupId: string,
    params?: { cursor?: string; pageSize?: number },
  ): Promise<GroupMemberListPage>;
  getCurrentUserGroupRole(groupId: string): Promise<GroupMemberRole | null>;
  updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat>;
  addMembers(groupId: string, memberIds: string[]): Promise<void>;
  inviteUserToGroup(group: Chat, targetUser: User): Promise<Message>;
  removeMember(groupId: string, memberId: string): Promise<void>;
  deleteGroup(groupId: string): Promise<void>;
}

type GroupViewState = Partial<Pick<Chat, 'activeCount' | 'avatar' | 'memberCount' | 'memberCountIsLowerBound' | 'members' | 'name' | 'notice' | 'agentAssignments' | 'agentAssignmentGeneration'>>;
type ConversationListEntry = ConversationInboxEntry;
const GROUP_INBOX_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const GROUP_MEMBERS_PAGE_LIMIT = SDKWORK_MAX_PAGE_SIZE;
const GROUP_MEMBER_LIST_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
// The API page cap is 200, while a group may contain up to 10,000 active
// members. Keep the bounded fallback lookup aligned with the domain limit.
const MAX_GROUP_MEMBER_LOOKUP_SCAN = 10_000;
const MAX_GROUP_MEMBER_VIEW_SYNC = SDKWORK_MAX_PAGE_SIZE;
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

export function createGroupClientRequestKey(): string {
  return `pc-group-${uuid()}`;
}

function createGroupAvatar(): string {
  return createDefaultAvatar('group');
}

function normalizeGroupName(name: string, memberCount: number): string {
  const trimmedName = name.trim();
  return trimmedName || `Group chat (${memberCount})`;
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

function normalizeGroupMemberRole(role: unknown): GroupMemberRole {
  switch (String(role ?? '').trim().toLowerCase()) {
    case 'owner':
      return 'owner';
    case 'admin':
      return 'admin';
    case 'member':
      return 'member';
    case 'guest':
      return 'guest';
    default:
      return 'unknown';
  }
}

function mapActiveGroupMembers(members: ConversationMember[]): GroupMemberListItem[] {
  const mapped = new Map<string, GroupMemberListItem>();
  for (const member of members) {
    if (member.state !== 'joined' && member.state !== 'invited') {
      continue;
    }
    const id = member.principalId.trim();
    if (!id || mapped.has(id)) {
      continue;
    }
    mapped.set(id, {
      id,
      memberId: member.memberId,
      role: normalizeGroupMemberRole(member.role),
    });
  }
  return Array.from(mapped.values());
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
    ...(group.memberCountIsLowerBound === undefined && state?.memberCountIsLowerBound !== undefined
      ? { memberCountIsLowerBound: state.memberCountIsLowerBound }
      : {}),
    ...(group.members === undefined && state?.members !== undefined ? { members: state.members } : {}),
    ...(isGeneratedGroupName(group) && state?.name !== undefined ? { name: state.name } : {}),
    ...(group.notice === undefined && state?.notice !== undefined ? { notice: state.notice } : {}),
    ...(group.agentAssignments === undefined && state?.agentAssignments !== undefined
      ? { agentAssignments: state.agentAssignments }
      : {}),
    ...(group.agentAssignmentGeneration === undefined && state?.agentAssignmentGeneration !== undefined
      ? { agentAssignmentGeneration: state.agentAssignmentGeneration }
      : {}),
  };
}

function mapConversationEntryToGroup(entry: ConversationListEntry): Chat {
  const updatedAt = new Date(entry.lastActivityAt).getTime();
  const entryRecord = toRecord(entry);
  const inboxName = pickString(entryRecord.displayName, entryRecord.display_name);
  const inboxAvatar = pickString(entryRecord.avatarUrl, entryRecord.avatar_url);
  const inboxAgentAssignments = entryRecord.agentAssignments ?? entryRecord.agent_assignments;
  const agentAssignments = inboxAgentAssignments === undefined
    ? undefined
    : normalizeAgentAssignments(inboxAgentAssignments);
  return {
    id: entry.conversationId,
    name: inboxName ?? 'Group chat',
    avatar: inboxAvatar ?? createGroupAvatar(),
    type: 'group',
    unreadCount: entry.unreadCount,
    updatedAt: Number.isFinite(updatedAt) ? updatedAt : Date.now(),
    ...(agentAssignments ? mapAssignmentSetToChatFields(agentAssignments) : {}),
  };
}

function readGroupPreferencesState(entry: ConversationListEntry): Record<string, unknown> {
  return toRecord(toRecord(entry).preferences);
}

function hasGroupPreferencesState(entry: ConversationListEntry): boolean {
  const preferences = readGroupPreferencesState(entry);
  return ['isPinned', 'isMuted', 'isMarkedUnread', 'isHidden']
    .every((field) => typeof preferences[field] === 'boolean');
}

function isGroupHiddenByPreferences(entry: ConversationListEntry): boolean {
  return readGroupPreferencesState(entry).isHidden === true;
}

function normalizeGroupPageSize(pageSize: number | undefined): number {
  if (pageSize === undefined) {
    return GROUP_INBOX_PAGE_LIMIT;
  }
  const normalizedPageSize = Math.floor(pageSize);
  if (!Number.isFinite(normalizedPageSize) || normalizedPageSize <= 0) {
    return GROUP_INBOX_PAGE_LIMIT;
  }
  return Math.min(normalizedPageSize, SDKWORK_MAX_PAGE_SIZE);
}

function normalizeAgentId(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

const STANDARD_AGENT_ID_PATTERN = /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;
const STANDARD_AGENT_REVISION_ID_PATTERN = /^revision\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;

function normalizeAgentAssignments(value: unknown): GroupAgentAssignmentSet | undefined {
  const record = toRecord(value);
  const rawAgents = Array.isArray(record.agents)
    ? record.agents
    : Array.isArray(record.agentAssignments)
      ? record.agentAssignments
      : undefined;
  if (!rawAgents || rawAgents.length < 1 || rawAgents.length > 10) {
    return undefined;
  }
  const seen = new Set<string>();
  const agents: GroupAgentAssignment[] = [];
  for (const rawAgent of rawAgents) {
    const item = toRecord(rawAgent);
    const agentId = normalizeAgentId(item.agentId ?? item.agent_id ?? item.id);
    if (!agentId || !STANDARD_AGENT_ID_PATTERN.test(agentId) || seen.has(agentId)) {
      return undefined;
    }
    seen.add(agentId);
    const revisionId = normalizeAgentId(item.revisionId ?? item.revision_id);
    if (revisionId && !STANDARD_AGENT_REVISION_ID_PATTERN.test(revisionId)) {
      return undefined;
    }
    agents.push({
      agentId,
      ...(revisionId ? { revisionId } : {}),
      ...(typeof item.name === 'string' && item.name.trim() ? { name: item.name.trim() } : {}),
      ...(typeof item.displayName === 'string' && item.displayName.trim() ? { name: item.displayName.trim() } : {}),
      ...(typeof item.avatar === 'string' && item.avatar.trim() ? { avatar: item.avatar.trim() } : {}),
      ...(typeof item.avatarUrl === 'string' && item.avatarUrl.trim() ? { avatar: item.avatarUrl.trim() } : {}),
      ...(typeof item.enabled === 'boolean' ? { enabled: item.enabled } : {}),
    });
  }
  const generationValue = Number(record.generation ?? record.assignmentGeneration ?? record.assignment_generation);
  if (!Number.isSafeInteger(generationValue) || generationValue < 1) {
    return undefined;
  }
  return {
    generation: generationValue,
    source: typeof record.source === 'string' ? record.source : 'conversation_override',
    agents,
  };
}

function assignmentsToSdk(
  assignments: GroupAgentAssignment[],
): ImConversationAgentAssignment[] {
  const seen = new Set<string>();
  return assignments.map((assignment) => {
    const agentId = assignment.agentId.trim();
    const revisionId = assignment.revisionId?.trim();
    if (!STANDARD_AGENT_ID_PATTERN.test(agentId)) {
      throw new Error(`Invalid group agent id: ${agentId || '(empty)'}`);
    }
    if (revisionId && !STANDARD_AGENT_REVISION_ID_PATTERN.test(revisionId)) {
      throw new Error(`Invalid group agent revision id: ${revisionId}`);
    }
    if (!seen.add(agentId)) {
      throw new Error(`Duplicate group agent id: ${agentId}`);
    }
    return {
      agentId,
      ...(revisionId ? { revisionId } : {}),
    };
  });
}

function mapAssignmentSetToChatFields(set: GroupAgentAssignmentSet): Pick<Chat, 'agentAssignments' | 'agentAssignmentGeneration'> {
  return {
    agentAssignments: set.agents,
    agentAssignmentGeneration: set.generation,
  };
}

function enrichAgentAssignmentSet(
  set: GroupAgentAssignmentSet,
  metadata: readonly GroupAgentAssignment[] | undefined,
): GroupAgentAssignmentSet {
  const metadataById = new Map(
    (metadata ?? []).map((assignment) => [assignment.agentId.trim(), assignment]),
  );
  return {
    ...set,
    agents: set.agents.map((assignment) => {
      const display = metadataById.get(assignment.agentId);
      return display
        ? { ...display, ...assignment, agentId: assignment.agentId }
        : assignment;
    }),
  };
}

function agentAssignmentSetsEqual(
  left: GroupAgentAssignmentSet,
  right: GroupAgentAssignmentSet,
): boolean {
  return left.generation === right.generation
    && left.source === right.source
    && left.agents.length === right.agents.length
    && left.agents.every((assignment, index) => {
      const other = right.agents[index];
      if (!other) {
        return false;
      }
      return assignment.agentId === other.agentId
        && (assignment.revisionId ?? '') === (other.revisionId ?? '');
    });
}

class SdkworkGroupService implements GroupService {
  private readonly groupViewState = new Map<string, GroupViewState>();
  private readonly agentAssignmentSnapshots = new Map<string, GroupAgentAssignmentSet>();
  private sessionGeneration = 0;
  private readonly chatClient: ChatService;

  constructor(
    private readonly getClient: () => ImSdkClient = getImSdkClientWithSession,
    chatClient?: ChatService,
    private readonly getSession: () => SdkworkChatSession | null = readAppSdkSessionTokens,
  ) {
    if (typeof window !== 'undefined') {
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
        this.sessionGeneration += 1;
        this.groupViewState.clear();
        this.agentAssignmentSnapshots.clear();
      });
    }
    this.chatClient = chatClient ?? (
      getClient === getImSdkClientWithSession && getSession === readAppSdkSessionTokens
        ? chatService
        : createSdkworkChatService({ getClient, getSession })
    );
  }

  private client(): ImSdkClient {
    return this.getClient();
  }

  private assertSessionGeneration(expected: number): void {
    if (this.sessionGeneration !== expected) {
      throw new Error('Chat session changed while loading group agent assignments.');
    }
  }

  private rememberAgentAssignmentSnapshot(
    groupId: string,
    snapshot: GroupAgentAssignmentSet,
    metadata?: readonly GroupAgentAssignment[],
  ): GroupAgentAssignmentSet {
    const existingSnapshot = this.agentAssignmentSnapshots.get(groupId);
    const existingState = this.groupViewState.get(groupId);
    const existingGeneration = Math.max(
      existingSnapshot?.generation ?? 0,
      existingState?.agentAssignmentGeneration ?? 0,
    );

    if (existingGeneration > snapshot.generation) {
      if (existingSnapshot?.generation === existingGeneration) {
        return existingSnapshot;
      }
      if (
        existingState?.agentAssignments
        && existingState.agentAssignmentGeneration === existingGeneration
      ) {
        const recoveredSnapshot: GroupAgentAssignmentSet = {
          generation: existingGeneration,
          source: 'conversation_override',
          agents: existingState.agentAssignments,
        };
        this.agentAssignmentSnapshots.set(groupId, recoveredSnapshot);
        return recoveredSnapshot;
      }
      throw new Error(
        `Stale group agent assignment response: current=${existingGeneration}, received=${snapshot.generation}`,
      );
    }

    const enriched = enrichAgentAssignmentSet(snapshot, metadata);
    if (
      existingSnapshot
      && existingSnapshot.generation === enriched.generation
      && !agentAssignmentSetsEqual(existingSnapshot, enriched)
    ) {
      // A generation identifies one immutable assignment snapshot. Preserve
      // the first accepted value if two concurrent reads disagree.
      return existingSnapshot;
    }

    this.agentAssignmentSnapshots.set(groupId, enriched);
    this.groupViewState.set(groupId, {
      ...existingState,
      ...mapAssignmentSetToChatFields(enriched),
    });
    return enriched;
  }

  async getAgentAssignments(groupId: string): Promise<GroupAgentAssignmentSet> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      throw new Error('Group id is required');
    }
    const response = await this.client().conversations.getAgentAssignments(normalizedId);
    this.assertSessionGeneration(sessionGeneration);
    const normalized = normalizeAgentAssignments(response);
    if (!normalized) {
      throw new Error('Conversation agent assignment snapshot is incomplete');
    }
    return this.rememberAgentAssignmentSnapshot(normalizedId, normalized);
  }

  async replaceAgentAssignments(
    groupId: string,
    expectedGeneration: number,
    assignments: GroupAgentAssignment[],
  ): Promise<GroupAgentAssignmentSet> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      throw new Error('Group id is required');
    }
    if (!Number.isSafeInteger(expectedGeneration) || expectedGeneration < 1) {
      throw new Error('A valid agent assignment generation is required');
    }
    const nextAssignments = assignmentsToSdk(assignments);
    if (nextAssignments.length < 1 || nextAssignments.length > 10) {
      throw new Error('A group must have between 1 and 10 agents');
    }
    const response = await this.client().conversations.replaceAgentAssignments(normalizedId, {
      expectedGeneration,
      agentAssignments: nextAssignments,
    });
    this.assertSessionGeneration(sessionGeneration);
    const normalized = normalizeAgentAssignments(response);
    if (!normalized) {
      throw new Error('Conversation agent assignment update did not return an authoritative snapshot');
    }
    return this.rememberAgentAssignmentSnapshot(normalizedId, normalized, assignments);
  }

  private resolveCurrentUserId(): string {
    return resolveAppSdkUserId(this.getSession())
      ?? contactService.getCurrentUser().id;
  }

  private async listConversationMembersPage(
    groupId: string,
    cursor?: string,
    pageSize = GROUP_MEMBERS_PAGE_LIMIT,
    expectedSessionGeneration = this.sessionGeneration,
  ): Promise<{ items: ConversationMember[]; hasMore: boolean; nextCursor?: string }> {
    const response = await this.client().conversations.listMembers(groupId, {
      pageSize: normalizeGroupPageSize(pageSize),
      ...(cursor ? { cursor } : {}),
    });
    this.assertSessionGeneration(expectedSessionGeneration);
    const page = readSdkCursorPageInfo(response.pageInfo);
    return {
      items: response.items,
      hasMore: page.hasMore,
      nextCursor: page.nextCursor,
    };
  }

  async canManageAgents(groupId: string): Promise<boolean> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      return false;
    }
    try {
      // The server resolves the authenticated actor (including principal kind)
      // and uses the aggregate store's exact member key. Do not infer
      // authorization by scanning a paginated roster in the browser.
      const current = await this.client().conversations.getCurrentMember(normalizedId);
      this.assertSessionGeneration(sessionGeneration);
      if (String(current.state).toLowerCase() !== 'joined') {
        return false;
      }
      const role = String(current.role ?? '').toLowerCase();
      return role === 'owner' || role === 'admin';
    } catch {
      return false;
    }
  }

  async getCurrentUserGroupRole(groupId: string): Promise<GroupMemberRole | null> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      return null;
    }
    try {
      const current = await this.client().conversations.getCurrentMember(normalizedId);
      this.assertSessionGeneration(sessionGeneration);
      if (String(current.state).toLowerCase() !== 'joined') {
        return null;
      }
      return normalizeGroupMemberRole(current.role);
    } catch {
      return null;
    }
  }

  async isCurrentUserGroupOwner(groupId: string): Promise<boolean> {
    const access = await this.getCurrentUserGroupKnowledgebaseAccess(groupId);
    return access.canInitialize;
  }

  async getCurrentUserGroupKnowledgebaseAccess(
    groupId: string,
  ): Promise<GroupKnowledgebaseMemberAccess> {
    const lookup = await this.retrieveCurrentUserGroupKnowledgebaseAccess(groupId);
    return lookup.kind === 'resolved'
      ? lookup.access
      : { canInitialize: false, canOpen: false };
  }

  async retrieveCurrentUserGroupKnowledgebaseAccess(
    groupId: string,
  ): Promise<GroupKnowledgebaseMemberAccessLookup> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      return { kind: 'failed' };
    }
    try {
      const current = await this.client().conversations.getCurrentMember(normalizedId);
      this.assertSessionGeneration(sessionGeneration);
      return {
        kind: 'resolved',
        access: resolveCurrentGroupKnowledgebaseMemberAccess(current),
      };
    } catch {
      return { kind: 'failed' };
    }
  }

  private async syncActiveMemberIds(
    groupId: string,
    expectedSessionGeneration = this.sessionGeneration,
  ): Promise<string[]> {
    const members: ConversationMember[] = [];
    await forEachCursorPage(
      (cursor) => this.listConversationMembersPage(
        groupId,
        cursor,
        GROUP_MEMBERS_PAGE_LIMIT,
        expectedSessionGeneration,
      ),
      (items) => {
        members.push(...items);
      },
      { maxItems: MAX_GROUP_MEMBER_VIEW_SYNC },
    );
    this.assertSessionGeneration(expectedSessionGeneration);
    return mapActiveMemberIds(members);
  }

  private async hydrateConversationEntryGroup(
    entry: ConversationListEntry,
    expectedSessionGeneration = this.sessionGeneration,
  ): Promise<Chat | null> {
    let group = mergeCachedGroupViewState(
      mapConversationEntryToGroup(entry),
      this.groupViewState.get(entry.conversationId),
    );
    if (hasGroupPreferencesState(entry)) {
      if (isGroupHiddenByPreferences(entry)) {
        return null;
      }
    }

    // Inbox state may omit the profile name. Hydrate the authoritative
    // profile before returning the page so the conversation list does not
    // expose a technical conversation id as the visible title.
    try {
      const profile = await this.client().conversations.getProfile(entry.conversationId);
      this.assertSessionGeneration(expectedSessionGeneration);
      if (profile.displayName?.trim()) {
        group = {
          ...group,
          name: profile.displayName.trim(),
          ...(profile.avatarUrl?.trim() ? { avatar: profile.avatarUrl.trim() } : {}),
          ...(profile.notice !== undefined ? { notice: profile.notice } : {}),
        };
        this.groupViewState.set(entry.conversationId, {
          ...this.groupViewState.get(entry.conversationId),
          name: group.name,
          avatar: group.avatar,
          notice: group.notice,
        });
      }
    } catch {
      // Keep the inbox-state fallback when profile hydration is unavailable.
    }

    if (group.agentAssignments !== undefined && group.agentAssignmentGeneration !== undefined) {
      this.assertSessionGeneration(expectedSessionGeneration);
      const snapshot = this.rememberAgentAssignmentSnapshot(entry.conversationId, {
        generation: group.agentAssignmentGeneration,
        source: 'conversation_override',
        agents: group.agentAssignments,
      });
      group = {
        ...group,
        ...mapAssignmentSetToChatFields(snapshot),
      };
    }

    return group;
  }

  async createGroup(
    name: string,
    memberIds: string[],
    agentAssignments?: GroupAgentAssignment[],
    options: CreateGroupOptions = {},
  ): Promise<Chat> {
    const sessionGeneration = this.sessionGeneration;
    const currentUserId = this.resolveCurrentUserId().trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }
    const invitedMemberIds = uniqueMemberIds(memberIds).filter((memberId) => memberId !== currentUserId);
    if (invitedMemberIds.length > MAX_GROUP_INITIAL_MEMBERS) {
      throw new Error(`A group may include at most ${MAX_GROUP_INITIAL_MEMBERS} invited members at creation time`);
    }
    const members = [currentUserId, ...invitedMemberIds];
    const groupName = normalizeGroupName(name, members.length);
    const clientRequestKey = options.clientRequestKey?.trim() || createGroupClientRequestKey();
    const initializeKnowledgebase = options.initializeKnowledgebase === true;

    // Group conversations use a server-derived canonical `g_` id seeded from
    // creator + group name + client request key. We send groupName +
    // clientRequestKey; the server returns the canonical conversationId.
    const requestedAgentAssignments = agentAssignments ?? [];
    if (agentAssignments && agentAssignments.length === 0) {
      throw new Error('A group must have between 1 and 10 agents when agents are explicitly selected');
    }
    const normalizedAssignments = assignmentsToSdk(requestedAgentAssignments);
    if (normalizedAssignments.length > 10) {
      throw new Error('A group may have at most 10 agents');
    }
    const result = await this.client().conversations.create({
      conversationType: 'group',
      groupName,
      clientRequestKey,
      memberUserIds: invitedMemberIds,
      ...(normalizedAssignments.length > 0
        ? { agentAssignments: normalizedAssignments }
        : {}),
      ...(initializeKnowledgebase ? { initializeKnowledgebase: true } : {}),
    });
    this.assertSessionGeneration(sessionGeneration);
    const boundGroupId = result.conversationId;

    const groupAvatar = createGroupAvatar();
    // The authoritative name, initial members and agents are committed by the
    // create command. Avatar/preferences are optional enrichments and must not
    // turn an already-created group into a false client-side failure.
    await Promise.allSettled([
      this.client().conversations.updateProfile(boundGroupId, {
        ...(groupAvatar ? { avatarUrl: groupAvatar } : {}),
        displayName: groupName,
      }),
      this.client().conversations.updatePreferences(boundGroupId, { isHidden: false }),
    ]);
    this.assertSessionGeneration(sessionGeneration);

    let initialAgentSet: GroupAgentAssignmentSet | undefined;
    try {
      initialAgentSet = await this.getAgentAssignments(boundGroupId);
      initialAgentSet = this.rememberAgentAssignmentSnapshot(
        boundGroupId,
        initialAgentSet,
        requestedAgentAssignments,
      );
    } catch {
      // Older SDK gateways may create the group before the assignment route is
      // deployed. Keep the group usable, but do not invent a generation: the
      // next hydration will obtain the authoritative snapshot.
      if (requestedAgentAssignments.length === 0) {
        initialAgentSet = undefined;
      }
    }
    const group: Chat = {
      id: boundGroupId,
      name: groupName,
      avatar: groupAvatar,
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.now(),
      memberCount: members.length,
      memberCountIsLowerBound: false,
      members,
      activeCount: members.length,
      ...(result.knowledgebaseInitialization
        ? { knowledgebaseInitialization: result.knowledgebaseInitialization }
        : {}),
      ...(initialAgentSet ? mapAssignmentSetToChatFields(initialAgentSet) : {}),
    };

    this.assertSessionGeneration(sessionGeneration);
    this.groupViewState.set(boundGroupId, {
      activeCount: group.activeCount,
      avatar: group.avatar,
      memberCount: group.memberCount,
      memberCountIsLowerBound: group.memberCountIsLowerBound,
      members: group.members,
      name: group.name,
      notice: group.notice,
      agentAssignments: group.agentAssignments,
      agentAssignmentGeneration: group.agentAssignmentGeneration,
    });
    return group;
  }

  async listGroupsPage(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<GroupListPage> {
    const sessionGeneration = this.sessionGeneration;
    const pageSize = normalizeGroupPageSize(params?.pageSize);
    const q = params?.q?.trim();
    const inboxPage = await this.client().chat?.inbox?.list({
      pageSize,
      conversationType: 'group',
      ...(params?.cursor ? { cursor: params.cursor } : {}),
      ...(q ? { q } : {}),
    });
    if (!inboxPage) {
      this.assertSessionGeneration(sessionGeneration);
      return { items: [], hasMore: false };
    }
    this.assertSessionGeneration(sessionGeneration);

    const groupEntries = inboxPage.items.filter(
      (entry) => entry.conversationType.toLowerCase() === 'group',
    );
    const hydratedGroups = await mapWithConcurrencyLimit(
      groupEntries,
      GROUP_LIST_HYDRATION_CONCURRENCY,
      async (entry) => this.hydrateConversationEntryGroup(entry, sessionGeneration),
    );
    this.assertSessionGeneration(sessionGeneration);
    const items = hydratedGroups
      .filter((group): group is Chat => group != null)
      .sort((left, right) => right.updatedAt - left.updatedAt);
    const page = readSdkCursorPageInfo(inboxPage.pageInfo);

    return {
      items,
      hasMore: page.hasMore,
      nextCursor: page.nextCursor,
    };
  }

  async getGroupById(groupId: string): Promise<Chat | null> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      return null;
    }

    const cachedState = this.groupViewState.get(normalizedId);
    let group = mergeCachedGroupViewState({
      id: normalizedId,
      name: cachedState?.name ?? normalizedId,
      avatar: cachedState?.avatar ?? createGroupAvatar(),
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.now(),
    }, cachedState);

    try {
      const preferences = await this.client().conversations.getPreferences(normalizedId);
      if (preferences.isHidden) {
        return null;
      }
    } catch {
      // Keep groups visible when preference hydration is temporarily unavailable.
    }

    try {
      const profile = await this.client().conversations.getProfile(normalizedId);
      group = {
        ...group,
        ...(profile.displayName ? { name: profile.displayName } : {}),
        ...(profile.avatarUrl ? { avatar: profile.avatarUrl } : {}),
        notice: profile.notice,
      };
    } catch {
      // Keep cached view state when profile hydration is temporarily unavailable.
    }

    try {
      const assignments = await this.getAgentAssignments(normalizedId);
      group = {
        ...group,
        ...mapAssignmentSetToChatFields(assignments),
      };
    } catch {
      // Keep group chat usable while an older gateway rolls out agent assignment routes.
    }

    const hydrated = await this.withMemberState(group, sessionGeneration);
    this.assertSessionGeneration(sessionGeneration);
    return hydrated;
  }

  async getGroups(): Promise<Chat[]> {
    const page = await this.listGroupsPage({ pageSize: GROUP_INBOX_PAGE_LIMIT });
    return page.items;
  }

  private async withMemberState(
    group: Chat,
    expectedSessionGeneration = this.sessionGeneration,
  ): Promise<Chat> {
    try {
      const memberState = await this.syncMemberViewState(group.id, false, expectedSessionGeneration);
      return {
        ...mergeCachedGroupViewState(group, this.groupViewState.get(group.id)),
        activeCount: memberState.activeCount,
        memberCount: memberState.memberCount,
        memberCountIsLowerBound: memberState.memberCountIsLowerBound,
        members: memberState.members,
      };
    } catch {
      return mergeCachedGroupViewState(group, this.groupViewState.get(group.id));
    }
  }

  async listGroupMembersPage(
    groupId: string,
    params?: { cursor?: string; pageSize?: number },
  ): Promise<GroupMemberListPage> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedId = groupId.trim();
    if (!normalizedId) {
      return { items: [], hasMore: false };
    }
    const page = await this.listConversationMembersPage(
      normalizedId,
      params?.cursor,
      normalizeGroupPageSize(params?.pageSize ?? GROUP_MEMBER_LIST_PAGE_LIMIT),
      sessionGeneration,
    );
    this.assertSessionGeneration(sessionGeneration);
    return {
      items: mapActiveGroupMembers(page.items),
      hasMore: page.hasMore,
      nextCursor: page.nextCursor,
    };
  }

  private async syncMemberViewState(
    groupId: string,
    _syncChatView = false,
    expectedSessionGeneration = this.sessionGeneration,
  ): Promise<Required<Pick<GroupViewState, 'activeCount' | 'memberCount' | 'memberCountIsLowerBound' | 'members'>>> {
    const page = await this.listGroupMembersPage(groupId, {
      pageSize: GROUP_MEMBER_LIST_PAGE_LIMIT,
    });
    this.assertSessionGeneration(expectedSessionGeneration);
    const members = page.items.map((member) => member.id);
    const existingState = this.groupViewState.get(groupId) ?? {};
    const nextState = {
      ...existingState,
      activeCount: members.length,
      memberCount: members.length,
      memberCountIsLowerBound: page.hasMore,
      members,
    };
    this.groupViewState.set(groupId, nextState);
    return {
      activeCount: nextState.activeCount,
      memberCount: nextState.memberCount,
      memberCountIsLowerBound: nextState.memberCountIsLowerBound,
      members: nextState.members,
    };
  }

  async updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat> {
    const sessionGeneration = this.sessionGeneration;
    const profileUpdate = {
      ...(updates.avatar !== undefined ? { avatarUrl: updates.avatar } : {}),
      ...(updates.name !== undefined ? { displayName: updates.name } : {}),
      ...(updates.notice !== undefined ? { notice: updates.notice } : {}),
    };
    const profile = Object.keys(profileUpdate).length > 0
      ? await this.client().conversations.updateProfile(groupId, profileUpdate)
      : undefined;
    this.assertSessionGeneration(sessionGeneration);
    const updatedGroup: Chat = {
      id: groupId,
      name: pickString(profile?.displayName, updates.name) ?? 'Group chat',
      avatar: pickString(profile?.avatarUrl, updates.avatar) ?? createGroupAvatar(),
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.now(),
      activeCount: updates.activeCount,
      memberCount: updates.memberCount,
      memberCountIsLowerBound: updates.memberCountIsLowerBound,
      members: updates.members,
      notice: profile?.notice ?? updates.notice,
    };
    const existingState = this.groupViewState.get(groupId) ?? {};
    this.groupViewState.set(groupId, {
      ...existingState,
      activeCount: updatedGroup.activeCount ?? updates.activeCount ?? existingState.activeCount,
      avatar: updatedGroup.avatar ?? updates.avatar ?? existingState.avatar,
      memberCount: updatedGroup.memberCount ?? updates.memberCount ?? existingState.memberCount,
      memberCountIsLowerBound: updatedGroup.memberCountIsLowerBound
        ?? updates.memberCountIsLowerBound
        ?? existingState.memberCountIsLowerBound,
      members: updatedGroup.members ?? updates.members ?? existingState.members,
      name: updatedGroup.name ?? updates.name ?? existingState.name,
      notice: updatedGroup.notice ?? updates.notice ?? existingState.notice,
      agentAssignments: updates.agentAssignments ?? existingState.agentAssignments,
      agentAssignmentGeneration: updates.agentAssignmentGeneration ?? existingState.agentAssignmentGeneration,
    });
    return updatedGroup;
  }

  async addMembers(groupId: string, memberIds: string[]): Promise<void> {
    const sessionGeneration = this.sessionGeneration;
    const existingMembers = await this.syncActiveMemberIds(groupId, sessionGeneration);
    this.assertSessionGeneration(sessionGeneration);
    const activeMemberIds = new Set(existingMembers);
    const membersToAdd = uniqueMemberIds(memberIds).filter((memberId) => !activeMemberIds.has(memberId));

    for (const memberId of membersToAdd) {
      this.assertSessionGeneration(sessionGeneration);
      await this.client().conversations.addMember(groupId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
      this.assertSessionGeneration(sessionGeneration);
      activeMemberIds.add(memberId);
    }

    await this.syncMemberViewState(groupId, false, sessionGeneration);
    this.assertSessionGeneration(sessionGeneration);
  }

  async inviteUserToGroup(group: Chat, targetUser: User): Promise<Message> {
    const sessionGeneration = this.sessionGeneration;
    const targetUserId = targetUser.id.trim();
    if (!targetUserId) {
      throw new Error('Group invite target user id is required');
    }

    await this.addMembers(group.id, [targetUserId]);
    this.assertSessionGeneration(sessionGeneration);
    const directChat = await this.chatClient.startDirectChat({
      avatar: targetUser.avatar,
      conversationId: targetUser.conversationId,
      directChatId: targetUser.directChatId,
      id: targetUserId,
      name: targetUser.name,
    });
    this.assertSessionGeneration(sessionGeneration);
    const currentUserId = this.resolveCurrentUserId().trim();
    const inviteMessage = await this.chatClient.sendMessage(
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
    this.assertSessionGeneration(sessionGeneration);
    return inviteMessage;
  }

  private async findConversationMember(
    groupId: string,
    memberId: string,
  ): Promise<ConversationMember | undefined> {
    const sessionGeneration = this.sessionGeneration;
    let cursor: string | undefined;
    let scanned = 0;
    const seenCursors = new Set<string>();

    while (scanned < MAX_GROUP_MEMBER_LOOKUP_SCAN) {
      const page = await this.listConversationMembersPage(
        groupId,
        cursor,
        GROUP_MEMBERS_PAGE_LIMIT,
        sessionGeneration,
      );
      const targetMember = page.items.find((member) => (
        member.memberId === memberId
        || (member.principalKind === 'user' && member.principalId === memberId)
      ));
      if (targetMember) {
        return targetMember;
      }
      scanned += page.items.length;
      if (
        page.items.length === 0
        || !page.hasMore
        || !page.nextCursor
        || seenCursors.has(page.nextCursor)
      ) {
        break;
      }
      seenCursors.add(page.nextCursor);
      cursor = page.nextCursor;
    }

    return undefined;
  }

  async removeMember(groupId: string, memberId: string): Promise<void> {
    const sessionGeneration = this.sessionGeneration;
    const normalizedMemberId = memberId.trim();
    if (!normalizedMemberId) {
      throw new Error('Group member id is required');
    }

    const targetMember = await this.findConversationMember(groupId, normalizedMemberId);
    this.assertSessionGeneration(sessionGeneration);
    if (!targetMember) {
      throw new Error('Group member is not available');
    }

    await this.client().conversations.removeMember(groupId, {
      memberId: targetMember.memberId,
    });
    this.assertSessionGeneration(sessionGeneration);
    await this.syncMemberViewState(groupId, false, sessionGeneration);
    this.assertSessionGeneration(sessionGeneration);
  }

  async deleteGroup(groupId: string): Promise<void> {
    const sessionGeneration = this.sessionGeneration;
    await this.client().conversations.leave(groupId);
    this.assertSessionGeneration(sessionGeneration);
    await this.chatClient.deleteChat(groupId).catch(() => undefined);
    this.assertSessionGeneration(sessionGeneration);
    this.groupViewState.delete(groupId);
    this.agentAssignmentSnapshots.delete(groupId);
  }

}

export function createSdkworkGroupService(
  getClient?: () => ImSdkClient,
  chatClient?: ChatService,
  getSession?: () => SdkworkChatSession | null,
): GroupService {
  return new SdkworkGroupService(getClient, chatClient, getSession);
}

export const groupService = createSdkworkGroupService();
