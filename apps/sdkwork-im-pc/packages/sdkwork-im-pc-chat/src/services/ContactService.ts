import type {
  ContactPreferencesView,
  ContactTagView,
  ContactView,
  CreateContactTagRequest,
  FriendRequest as ImFriendRequest,
  ImSdkClient,
  SocialUserSearchResult,
  UpdateContactTagRequest,
} from '@sdkwork/im-sdk';
import {
  getImSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/imSdkClient';
import {
  subscribePcRealtimeScope,
} from '@sdkwork/im-pc-core/sdk/pcRealtimeConnectionManager';
import {
  applyAppSdkSessionTokens,
  isAppSdkSessionAuthenticated,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  readAppSdkSessionTokens,
} from '@sdkwork/im-pc-core/sdk/session';
import type { User } from '@sdkwork/im-pc-types';
import {
  organizationDirectoryService,
  type OrganizationDirectoryService,
} from './OrganizationDirectoryService';
import { createDefaultAvatar } from './DefaultAvatarService';

export interface OrgDepartment {
  id: string;
  name: string;
  parentId: string | null;
  order: number;
}

export interface FriendRequest {
  avatar?: string;
  direction: 'incoming' | 'outgoing';
  id: number;
  name: string;
  msg: string;
  status: 'pending' | 'added' | 'rejected';
}

export interface ContactTag {
  id: string;
  name: string;
  color: string;
  count: number;
  bg: string;
  border: string;
}

export interface ContactSyncResult {
  contacts: User[];
  refreshedContacts: number;
}

export interface ContactService {
  getContacts(): Promise<User[]>;
  searchContacts(query: string): Promise<User[]>;
  addFriend(userId: string): Promise<void>;
  addFriendBySearchQuery(query: string): Promise<User>;
  getStarredContacts(): Promise<User[]>;
  getDepartments(): Promise<OrgDepartment[]>;
  getUsersByDepartment(departmentId: string): Promise<User[]>;
  getCurrentUser(): User;
  getUserById(id: string): Promise<User | null>;
  getFriendRequests(): Promise<FriendRequest[]>;
  getPendingFriendRequestCount(): Promise<number>;
  subscribePendingFriendRequestCount(handler: (count: number) => void): () => void;
  getTags(): Promise<ContactTag[]>;
  addTag(tag: Omit<ContactTag, 'id'>): Promise<ContactTag>;
  updateTag(id: string, updates: Partial<ContactTag>): Promise<ContactTag>;
  removeTag(id: string): Promise<void>;
  updateProfile(update: Partial<User>): Promise<User>;
  deleteContact(userId: string): Promise<void>;
  handleFriendRequest(requestId: number, action: 'accept' | 'reject'): Promise<void>;
  cancelFriendRequest(requestId: number): Promise<void>;
  toggleStarContact(userId: string, isStarred: boolean): Promise<void>;
  setContactRemark(userId: string, remark: string): Promise<void>;
  addToBlacklist(userId: string): Promise<void>;
  recommendToFriend(userId: string): Promise<void>;
  syncContacts(): Promise<ContactSyncResult>;
}

const CONTACTS_PAGE_LIMIT = 100;
const CONTACT_PREFERENCES_BATCH_SIZE = 20;
const CONTACT_PROFILE_BATCH_SIZE = 20;
const CONTACT_TAGS_PAGE_LIMIT = 100;
const FRIEND_REQUESTS_PAGE_LIMIT = 100;
const SOCIAL_USER_SEARCH_LIMIT = 20;
const FRIEND_REQUEST_COUNT_REFRESH_MS = 12000;
const FRIEND_REQUEST_REALTIME_EVENT_TYPES = [
  'friend_request.submitted',
  'friend_request.accepted',
  'friend_request.declined',
  'friend_request.canceled',
];
export const SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT = 'sdkwork-im-pc:friend-requests-changed';

function normalizeString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    const normalized = normalizeString(value);
    if (normalized) {
      return normalized;
    }
  }
  return undefined;
}

function pickNumber(...values: unknown[]): number | undefined {
  for (const value of values) {
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const parsed = Number(value);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return undefined;
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function isAuthenticationFailure(error: unknown): boolean {
  const record = toRecord(error);
  const code = pickString(record.code);
  const message = pickString(record.message, record.error, record.reason);
  const status = pickNumber(record.status, record.statusCode, record.httpStatus, record.http_status);
  return status === 401
    || Boolean(code && /(?:auth|session|token|unauthori(?:s|z)ed)/iu.test(code))
    || Boolean(message && /(?:auth|session|token).*(?:failed|expired|invalid|required)|unauthori(?:s|z)ed/iu.test(message));
}

function createAvatar(_seed: string): string {
  return createDefaultAvatar('user');
}

function createSearchKey(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .normalize('NFKD')
    .replace(/[^\da-z]+/gu, '');
}

function toRequestUiId(requestId: string): number {
  if (/^\d+$/u.test(requestId)) {
    const numericId = Number(requestId);
    if (Number.isSafeInteger(numericId) && numericId > 0) {
      return numericId;
    }
  }

  let hash = 2166136261;
  for (const char of requestId) {
    hash ^= char.codePointAt(0) ?? 0;
    hash = Math.imul(hash, 16777619);
  }
  return (hash >>> 0) || 1;
}

function normalizeRequestStatus(status: ImFriendRequest['status']): FriendRequest['status'] {
  if (status === 'pending') {
    return 'pending';
  }
  if (status === 'accepted') {
    return 'added';
  }
  return 'rejected';
}

class SdkworkContactService implements ContactService {
  private readonly contactByUserId = new Map<string, ContactView>();
  private readonly preferenceByUserId = new Map<string, ContactPreferencesView>();
  private readonly requestIdByUiId = new Map<number, string>();
  private readonly requestUiIdByBackendId = new Map<string, number>();
  private readonly userCache = new Map<string, User>();
  private readonly userIdByChatId = new Map<string, string>();
  private readonly pendingFriendRequestCountHandlers = new Set<(count: number) => void>();
  private currentUserOverrides: Partial<User> = {};
  private pendingFriendRequestCount: number | undefined;
  private pendingFriendRequestCountRefresh?: Promise<number>;
  private pendingFriendRequestRefreshTimer?: ReturnType<typeof setInterval>;
  private pendingFriendRequestRefreshListener?: () => void;
  private pendingFriendRequestRealtimeUnsub?: () => void;
  private pendingFriendRequestRealtimeUserId?: string;
  private readonly handleAuthSessionChanged = (): void => {
    this.contactByUserId.clear();
    this.preferenceByUserId.clear();
    this.requestIdByUiId.clear();
    this.requestUiIdByBackendId.clear();
    this.userCache.clear();
    this.userIdByChatId.clear();
    this.pendingFriendRequestCount = undefined;
    this.pendingFriendRequestCountRefresh = undefined;
    this.stopPendingFriendRequestRefreshLoop();
  };

  constructor(
    private readonly getClient: () => ImSdkClient = getImSdkClientWithSession,
    _getAppClient?: () => unknown,
    private readonly getOrganizationDirectoryService: () => OrganizationDirectoryService = () => organizationDirectoryService,
  ) {
    if (typeof window !== 'undefined') {
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, this.handleAuthSessionChanged);
    }
  }

  private client(): ImSdkClient {
    return this.getClient();
  }

  private organizationDirectory(): OrganizationDirectoryService {
    return this.getOrganizationDirectoryService();
  }

  private hasAuthenticatedSession(): boolean {
    return isAppSdkSessionAuthenticated(readAppSdkSessionTokens());
  }

  private handleAuthenticationFailure(): void {
    this.stopPendingFriendRequestRefreshLoop();
  }

  private async listAllContacts(): Promise<ContactView[]> {
    const items: ContactView[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().chat.contacts.list({
        limit: CONTACTS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  private async listAllFriendRequests(
    direction: 'incoming' | 'outgoing',
    status: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all' = 'all',
  ): Promise<ImFriendRequest[]> {
    const items: ImFriendRequest[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().social.friendRequests.list({
        direction,
        status,
        limit: FRIEND_REQUESTS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      if (!response.nextCursor || response.nextCursor === cursor) {
        break;
      }
      cursor = response.nextCursor;
    } while (cursor);

    return items;
  }

  private async refreshPendingFriendRequestCount(): Promise<number> {
    if (!this.hasAuthenticatedSession()) {
      this.pendingFriendRequestCount = 0;
      return 0;
    }
    if (this.pendingFriendRequestCountRefresh) {
      return this.pendingFriendRequestCountRefresh;
    }

    this.pendingFriendRequestCountRefresh = (async () => {
      let incoming: ImFriendRequest[];
      try {
        incoming = await this.listAllFriendRequests('incoming', 'pending');
      } catch (error) {
        if (isAuthenticationFailure(error)) {
          this.handleAuthenticationFailure();
          return 0;
        }
        throw error;
      }
      const count = incoming.length;
      const previousCount = this.pendingFriendRequestCount;
      this.pendingFriendRequestCount = count;
      if (previousCount !== count) {
        this.emitPendingFriendRequestCount(count);
      }
      return count;
    })().finally(() => {
      this.pendingFriendRequestCountRefresh = undefined;
    });

    return this.pendingFriendRequestCountRefresh;
  }

  private emitPendingFriendRequestCount(count: number): void {
    for (const handler of this.pendingFriendRequestCountHandlers) {
      handler(count);
    }
  }

  private dispatchFriendRequestChange(): void {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT));
    }
  }

  private startPendingFriendRequestRefreshLoop(): void {
    if (this.pendingFriendRequestRefreshTimer || typeof window === 'undefined') {
      return;
    }
    this.pendingFriendRequestRefreshTimer = setInterval(() => {
      void this.refreshPendingFriendRequestCount().catch(() => undefined);
    }, FRIEND_REQUEST_COUNT_REFRESH_MS);

    const refreshWhenVisible = () => {
      if (typeof document === 'undefined' || document.visibilityState === 'visible') {
        void this.refreshPendingFriendRequestCount().catch(() => undefined);
      }
    };
    this.pendingFriendRequestRefreshListener = refreshWhenVisible;
    window.addEventListener('focus', refreshWhenVisible);
    document.addEventListener('visibilitychange', refreshWhenVisible);
    window.addEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshWhenVisible);
  }

  private stopPendingFriendRequestRefreshLoop(): void {
    this.stopPendingFriendRequestRealtime();
    if (!this.pendingFriendRequestRefreshTimer || typeof window === 'undefined') {
      return;
    }
    clearInterval(this.pendingFriendRequestRefreshTimer);
    this.pendingFriendRequestRefreshTimer = undefined;
    if (this.pendingFriendRequestRefreshListener) {
      window.removeEventListener('focus', this.pendingFriendRequestRefreshListener);
      document.removeEventListener('visibilitychange', this.pendingFriendRequestRefreshListener);
      window.removeEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, this.pendingFriendRequestRefreshListener);
      this.pendingFriendRequestRefreshListener = undefined;
    }
  }

  private startPendingFriendRequestRealtime(): void {
    if (!this.hasAuthenticatedSession()) {
      this.stopPendingFriendRequestRealtime();
      return;
    }
    const currentUserId = this.getCurrentUser().id;
    if (!currentUserId) {
      return;
    }
    if (this.pendingFriendRequestRealtimeUnsub && this.pendingFriendRequestRealtimeUserId === currentUserId) {
      return;
    }

    this.stopPendingFriendRequestRealtime();
    this.pendingFriendRequestRealtimeUserId = currentUserId;
    this.pendingFriendRequestRealtimeUnsub = subscribePcRealtimeScope(
      {
        scopeType: 'user',
        scopeId: currentUserId,
        eventTypes: FRIEND_REQUEST_REALTIME_EVENT_TYPES,
      },
      (_context) => {
        if (!_context.eventType?.startsWith('friend_request.')) {
          return;
        }
        void _context.ack().catch(() => undefined);
        void this.refreshPendingFriendRequestCount().catch(() => undefined);
        this.dispatchFriendRequestChange();
      },
    );
  }

  private stopPendingFriendRequestRealtime(): void {
    this.pendingFriendRequestRealtimeUnsub?.();
    this.pendingFriendRequestRealtimeUnsub = undefined;
    this.pendingFriendRequestRealtimeUserId = undefined;
  }

  private async listAllContactTags(): Promise<ContactTagView[]> {
    const items: ContactTagView[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().social.contacts.tags.list({
        limit: CONTACT_TAGS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      if (!response.nextCursor || response.nextCursor === cursor) {
        break;
      }
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  async getContacts(): Promise<User[]> {
    const contacts = await this.listAllContacts();
    return this.hydrateContactUsers(contacts);
  }

  private async hydrateContactUsers(contacts: ContactView[]): Promise<User[]> {
    const preferences = await this.loadContactPreferences(contacts);
    await this.loadContactPeerProfiles(contacts);
    const users = contacts
      .map((contact) => this.mapContactViewToUser(contact, preferences.get(contact.targetUserId)))
      .filter((user) => !(preferences.get(user.id)?.isBlocked ?? false));
    return users.sort((left, right) => left.name.localeCompare(right.name));
  }

  async searchContacts(query: string): Promise<User[]> {
    return this.searchSocialUsers(query, { includeCurrentUser: false });
  }

  async addFriend(userId: string): Promise<void> {
    const targetUserId = userId.trim();
    if (!targetUserId) {
      throw new Error('Friend user id is required');
    }
    if (this.isCurrentUserIdentifier(targetUserId)) {
      throw new Error('Cannot add yourself as a friend');
    }
    await this.assertCanSendFriendRequest(targetUserId);
    await this.client().social.friendRequests.create({ targetUserId });
    this.dispatchFriendRequestChange();
  }

  async addFriendBySearchQuery(query: string): Promise<User> {
    const normalizedQuery = query.trim();
    if (!normalizedQuery) {
      throw new Error('Friend search query is required');
    }

    const target = await this.findAddFriendTarget(normalizedQuery);
    if (!target) {
      throw new Error('Friend search target not found');
    }

    this.assertRelationshipAllowsFriendRequest(target.relationshipState);
    await this.client().social.friendRequests.create({ targetUserId: target.user.id });
    this.dispatchFriendRequestChange();
    return target.user;
  }

  async getStarredContacts(): Promise<User[]> {
    const contacts = await this.getContacts();
    return contacts.filter((user) => this.preferenceByUserId.get(user.id)?.isStarred ?? false);
  }

  async getDepartments(): Promise<OrgDepartment[]> {
    return this.organizationDirectory().getDepartments();
  }

  async getUsersByDepartment(departmentId: string): Promise<User[]> {
    return this.organizationDirectory().getUsersByDepartment(departmentId);
  }

  getCurrentUser(): User {
    const session = readAppSdkSessionTokens();
    const sessionUser = session?.user;
    const id = pickString(
      sessionUser?.userId,
      sessionUser?.id,
      session?.context?.userId,
      this.currentUserOverrides.id,
      'current-user',
    ) ?? 'current-user';
    const name = pickString(
      this.currentUserOverrides.name,
      sessionUser?.displayName,
      sessionUser?.nickname,
      sessionUser?.name,
      sessionUser?.username,
      id,
    ) ?? id;
    const sessionUserRecord = toRecord(sessionUser);
    const sessionContextRecord = toRecord(session?.context);
    const cachedCurrentUser = this.userCache.get(id);
    const chatId = pickString(
      this.currentUserOverrides.chatId,
      sessionUserRecord.chatId,
      sessionUserRecord.chat_id,
      sessionUserRecord.imId,
      sessionUserRecord.sdkworkImId,
      sessionContextRecord.chatId,
      sessionContextRecord.chat_id,
      cachedCurrentUser?.chatId,
    );
    return {
      id,
      ...(chatId ? { chatId } : {}),
      name,
      avatar: pickString(this.currentUserOverrides.avatar, sessionUser?.avatar) ?? createAvatar(id),
      status: this.currentUserOverrides.status ?? 'online',
      email: pickString(this.currentUserOverrides.email, sessionUser?.email),
      phone: pickString(this.currentUserOverrides.phone, sessionUser?.phone),
      py: createSearchKey(name),
    };
  }

  async getUserById(id: string): Promise<User | null> {
    const normalizedId = id.trim();
    if (!normalizedId) {
      return null;
    }
    const currentUser = this.getCurrentUser();
    if (currentUser.chatId === normalizedId) {
      return currentUser;
    }
    const shouldReturnCurrentUserIfLookupFails = currentUser.id === normalizedId;
    if (shouldReturnCurrentUserIfLookupFails && currentUser.chatId) {
      return currentUser;
    }
    const cached = this.userCache.get(this.userIdByChatId.get(normalizedId) ?? normalizedId);
    if (cached) {
      return { ...cached };
    }
    if (shouldReturnCurrentUserIfLookupFails) {
      return await this.findSocialUserByLookup(normalizedId) ?? currentUser;
    }
    const contacts = await this.getContacts();
    const contact = contacts.find((user) => user.id === normalizedId || user.chatId === normalizedId);
    if (contact) {
      return contact;
    }
    return await this.findSocialUserByLookup(normalizedId);
  }

  async getFriendRequests(): Promise<FriendRequest[]> {
    const [incoming, outgoing] = await Promise.all([
      this.listAllFriendRequests('incoming', 'pending'),
      this.listAllFriendRequests('outgoing', 'pending'),
    ]);
    const requests = [...incoming, ...outgoing];
    await this.loadFriendRequestPeerProfiles(requests);
    return requests.map((request) => this.mapFriendRequest(request));
  }

  async getPendingFriendRequestCount(): Promise<number> {
    return this.refreshPendingFriendRequestCount();
  }

  subscribePendingFriendRequestCount(handler: (count: number) => void): () => void {
    this.pendingFriendRequestCountHandlers.add(handler);
    if (this.pendingFriendRequestCount !== undefined) {
      handler(this.pendingFriendRequestCount);
    }
    if (this.hasAuthenticatedSession()) {
      void this.refreshPendingFriendRequestCount().catch(() => undefined);
      void this.startPendingFriendRequestRealtime();
      this.startPendingFriendRequestRefreshLoop();
    } else {
      handler(0);
    }

    return () => {
      this.pendingFriendRequestCountHandlers.delete(handler);
      if (this.pendingFriendRequestCountHandlers.size === 0) {
        this.stopPendingFriendRequestRefreshLoop();
      }
    };
  }

  async getTags(): Promise<ContactTag[]> {
    const tags = await this.listAllContactTags();
    return tags.map((tag) => this.mapContactTagViewToContactTag(tag));
  }

  async addTag(tag: Omit<ContactTag, 'id'>): Promise<ContactTag> {
    const created = await this.client().social.contacts.tags.create(
      this.mapContactTagInputToCreateRequest(tag),
    );
    return this.mapContactTagViewToContactTag(created);
  }

  async updateTag(id: string, updates: Partial<ContactTag>): Promise<ContactTag> {
    const tagId = this.normalizeContactTagId(id);
    const updated = await this.client().social.contacts.tags.update(
      tagId,
      this.mapContactTagUpdateToRequest(updates),
    );
    return this.mapContactTagViewToContactTag(updated);
  }

  async removeTag(id: string): Promise<void> {
    await this.client().social.contacts.tags.delete(this.normalizeContactTagId(id));
  }

  async updateProfile(update: Partial<User>): Promise<User> {
    this.currentUserOverrides = {
      ...this.currentUserOverrides,
      ...update,
    };
    return this.getCurrentUser();
  }

  async deleteContact(userId: string): Promise<void> {
    const normalizedUserId = userId.trim();
    if (!normalizedUserId) {
      throw new Error('Contact user id is required');
    }

    let contact = this.contactByUserId.get(normalizedUserId);
    if (!contact) {
      await this.getContacts();
      contact = this.contactByUserId.get(normalizedUserId);
    }
    if (!contact?.friendshipId) {
      throw new Error('Contact friendship is not available');
    }

    await this.client().social.friendships.remove(contact.friendshipId);
    this.contactByUserId.delete(normalizedUserId);
    const cached = this.userCache.get(normalizedUserId);
    if (cached?.chatId) {
      this.userIdByChatId.delete(cached.chatId);
    }
    this.userCache.delete(normalizedUserId);
    this.preferenceByUserId.delete(normalizedUserId);
    this.dispatchFriendRequestChange();
  }

  async handleFriendRequest(requestId: number, action: 'accept' | 'reject'): Promise<void> {
    const backendRequestId = this.requestIdByUiId.get(requestId) ?? String(requestId);
    if (action === 'accept') {
      const result = await this.client().social.friendRequests.accept(backendRequestId);
      this.requestIdByUiId.delete(requestId);
      this.requestUiIdByBackendId.delete(backendRequestId);
      const userId = this.resolveFriendshipPeerId(result.friendship);
      if (userId) {
        await this.loadUserProfile(userId);
      }
      await this.refreshPendingFriendRequestCount();
      this.dispatchFriendRequestChange();
      return;
    }

    await this.client().social.friendRequests.decline(backendRequestId);
    this.requestIdByUiId.delete(requestId);
    this.requestUiIdByBackendId.delete(backendRequestId);
    await this.refreshPendingFriendRequestCount();
    this.dispatchFriendRequestChange();
  }

  async cancelFriendRequest(requestId: number): Promise<void> {
    const backendRequestId = this.requestIdByUiId.get(requestId) ?? String(requestId);
    await this.client().social.friendRequests.cancel(backendRequestId);
    this.requestIdByUiId.delete(requestId);
    this.requestUiIdByBackendId.delete(backendRequestId);
    await this.refreshPendingFriendRequestCount();
    this.dispatchFriendRequestChange();
  }

  async toggleStarContact(userId: string, isStarred: boolean): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const preferences = await this.client().social.contacts.preferences.update(normalizedUserId, {
      isStarred,
    });
    this.preferenceByUserId.set(normalizedUserId, preferences);
  }

  async setContactRemark(userId: string, remark: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const normalizedRemark = remark.trim();
    const preferences = await this.client().social.contacts.preferences.update(normalizedUserId, {
      remark: normalizedRemark,
    });
    this.preferenceByUserId.set(normalizedUserId, preferences);
    const cached = this.userCache.get(normalizedUserId);
    if (cached) {
      this.cacheUser({
        ...cached,
        name: preferences.remark || normalizedUserId,
        py: createSearchKey(preferences.remark || normalizedUserId),
      });
    }
  }

  async addToBlacklist(userId: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const preferences = await this.client().social.contacts.preferences.update(normalizedUserId, {
      isBlocked: true,
      isStarred: false,
    });
    this.preferenceByUserId.set(normalizedUserId, preferences);
    this.dispatchFriendRequestChange();
  }

  async recommendToFriend(userId: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    await this.client().social.contacts.recommendations.create(normalizedUserId, {});
  }

  async syncContacts(): Promise<ContactSyncResult> {
    const contacts = await this.hydrateContactUsers(await this.listAllContacts());
    return {
      contacts,
      refreshedContacts: contacts.length,
    };
  }

  private async loadContactPreferences(contacts: ContactView[]): Promise<Map<string, ContactPreferencesView>> {
    const entries: Array<readonly [string, ContactPreferencesView]> = [];
    for (let offset = 0; offset < contacts.length; offset += CONTACT_PREFERENCES_BATCH_SIZE) {
      const batch = contacts.slice(offset, offset + CONTACT_PREFERENCES_BATCH_SIZE);
      entries.push(...await Promise.all(batch.map(async (contact) => {
        const preferences = await this.client().social.contacts.preferences.retrieve(contact.targetUserId);
        return [contact.targetUserId, preferences] as const;
      })));
    }
    const preferencesByUserId = new Map(entries);
    for (const [userId, preferences] of preferencesByUserId) {
      this.preferenceByUserId.set(userId, preferences);
    }
    return preferencesByUserId;
  }

  private async loadContactPeerProfiles(contacts: ContactView[]): Promise<void> {
    const userIds = [...new Set(contacts.map((contact) => contact.targetUserId))]
      .filter((userId) => !this.userCache.has(userId));

    for (let offset = 0; offset < userIds.length; offset += CONTACT_PROFILE_BATCH_SIZE) {
      const batch = userIds.slice(offset, offset + CONTACT_PROFILE_BATCH_SIZE);
      await Promise.all(batch.map((userId) => this.loadUserProfile(userId)));
    }
  }

  private async loadUserProfile(userId: string): Promise<User | null> {
    const [profile] = await this.searchSocialUsers(userId, { includeCurrentUser: true });
    return profile?.id === userId ? profile : null;
  }

  private async searchSocialUsers(
    query: string,
    options: { includeCurrentUser: boolean },
  ): Promise<User[]> {
    const normalizedQuery = query.trim();
    if (!normalizedQuery) {
      return [];
    }

    const response = await this.client().social.users.list({
      q: normalizedQuery,
      limit: SOCIAL_USER_SEARCH_LIMIT,
    });
    return response.items
      .filter((item) => options.includeCurrentUser || !this.isCurrentUserSearchResult(item))
      .map((item) => this.mapSocialUserSearchResultToUser(item));
  }

  private async findSocialUserByLookup(lookup: string): Promise<User | null> {
    const users = await this.searchSocialUsers(lookup, { includeCurrentUser: true });
    return users.find((user) => user.id === lookup || user.chatId === lookup) ?? null;
  }

  private normalizeContactUserId(userId: string): string {
    const normalizedUserId = userId.trim();
    if (!normalizedUserId) {
      throw new Error('Contact user id is required');
    }
    return normalizedUserId;
  }

  private normalizeContactTagId(tagId: string): string {
    const normalizedTagId = tagId.trim();
    if (!normalizedTagId) {
      throw new Error('Contact tag id is required');
    }
    return normalizedTagId;
  }

  private mapContactTagViewToContactTag(tag: ContactTagView): ContactTag {
    return {
      id: tag.tagId,
      name: tag.name,
      color: tag.color,
      count: tag.count,
      bg: tag.bg,
      border: tag.border,
    };
  }

  private mapContactTagInputToCreateRequest(tag: Omit<ContactTag, 'id'>): CreateContactTagRequest {
    return {
      name: tag.name,
      color: tag.color,
      count: tag.count,
      bg: tag.bg,
      border: tag.border,
    };
  }

  private mapContactTagUpdateToRequest(updates: Partial<ContactTag>): UpdateContactTagRequest {
    const request: UpdateContactTagRequest = {};
    if (updates.name !== undefined) {
      request.name = updates.name;
    }
    if (updates.color !== undefined) {
      request.color = updates.color;
    }
    if (updates.count !== undefined) {
      request.count = updates.count;
    }
    if (updates.bg !== undefined) {
      request.bg = updates.bg;
    }
    if (updates.border !== undefined) {
      request.border = updates.border;
    }
    return request;
  }

  private mapContactViewToUser(contact: ContactView, preferences?: ContactPreferencesView): User {
    this.contactByUserId.set(contact.targetUserId, contact);
    const user = {
      ...this.createUserFromId(contact.targetUserId, preferences),
      ...(contact.conversationId ? { conversationId: contact.conversationId } : {}),
      ...(contact.directChatId ? { directChatId: contact.directChatId } : {}),
    };
    this.cacheUser(user);
    return user;
  }

  private mapSocialUserSearchResultToUser(result: SocialUserSearchResult): User {
    const resultRecord = toRecord(result);
    const metadata = toRecord(resultRecord.metadata);
    const isCurrentProfile = this.isCurrentUserSearchResult(result);
    const chatId = pickString(
      resultRecord.chatId,
      resultRecord.chat_id,
      metadata.chatId,
      metadata.chat_id,
    );
    const name = result.displayName || result.userId;
    const user: User = {
      id: result.userId,
      ...(chatId ? { chatId } : {}),
      name,
      avatar: result.avatarUrl ?? createAvatar(result.userId),
      status: result.relationshipState === 'active' || result.relationshipState === 'self' ? 'online' : 'offline',
      email: result.email,
      phone: result.phone,
      departmentId: pickString(
        toRecord(result).departmentId,
        toRecord(result).department_id,
        toRecord(result).orgUnitId,
        toRecord(result).org_unit_id,
      ),
      py: createSearchKey(name),
    };
    this.cacheUser(user);
    if (isCurrentProfile) {
      this.syncCurrentUserProfile(user);
    }
    return user;
  }

  private isCurrentUserIdentifier(userId: unknown): boolean {
    const normalizedUserId = normalizeString(userId);
    if (!normalizedUserId) {
      return false;
    }
    const currentUser = this.getCurrentUser();
    return normalizedUserId === currentUser.id || (Boolean(currentUser.chatId) && normalizedUserId === currentUser.chatId);
  }

  private isCurrentUserSearchResult(result: SocialUserSearchResult): boolean {
    const resultRecord = toRecord(result);
    const metadata = toRecord(resultRecord.metadata);
    const chatId = pickString(
      resultRecord.chatId,
      resultRecord.chat_id,
      metadata.chatId,
      metadata.chat_id,
    );
    return result.relationshipState === 'self'
      || this.isCurrentUserIdentifier(result.userId)
      || this.isCurrentUserIdentifier(chatId);
  }

  private createUserFromId(userId: string, preferences = this.preferenceByUserId.get(userId)): User {
    const cached = this.userCache.get(userId);
    const name = preferences?.remark || cached?.name || userId;
    return {
      id: userId,
      ...(cached?.chatId ? { chatId: cached.chatId } : {}),
      ...(cached?.conversationId ? { conversationId: cached.conversationId } : {}),
      ...(cached?.directChatId ? { directChatId: cached.directChatId } : {}),
      name,
      avatar: cached?.avatar ?? createAvatar(userId),
      status: cached?.status ?? 'offline',
      ...(cached?.departmentId ? { departmentId: cached.departmentId } : {}),
      py: createSearchKey(name),
    };
  }

  private async loadFriendRequestPeerProfiles(requests: ImFriendRequest[]): Promise<void> {
    const currentUserId = this.getCurrentUser().id;
    const peerUserIds = [...new Set(requests.map((request) => (
      request.requesterUserId === currentUserId
        ? request.targetUserId
        : request.requesterUserId
    )))];

    await Promise.all(peerUserIds.map(async (peerUserId) => {
      if (this.userCache.has(peerUserId)) {
        return;
      }
      try {
        const [profile] = await this.searchContacts(peerUserId);
        if (profile?.id === peerUserId) {
          this.cacheUser(profile);
        }
      } catch {
        // Keep the friend-request list usable when profile lookup is temporarily unavailable.
      }
    }));
  }

  private mapFriendRequest(request: ImFriendRequest): FriendRequest {
    const uiId = this.getOrCreateRequestUiId(request.requestId);
    const currentUserId = this.getCurrentUser().id;
    const isOutgoing = request.requesterUserId === currentUserId;
    const peerUserId = isOutgoing ? request.targetUserId : request.requesterUserId;
    const peerUser = this.userCache.get(peerUserId);
    const name = this.preferenceByUserId.get(peerUserId)?.remark || peerUser?.name || peerUserId;
    return {
      avatar: peerUser?.avatar,
      direction: isOutgoing ? 'outgoing' : 'incoming',
      id: uiId,
      name,
      msg: request.requestMessage ?? '',
      status: normalizeRequestStatus(request.status),
    };
  }

  private async assertCanSendFriendRequest(targetUserId: string): Promise<void> {
    const response = await this.client().social.users.list({
      q: targetUserId,
      limit: SOCIAL_USER_SEARCH_LIMIT,
    });
    const match = response.items.find((item) => item.userId === targetUserId);
    if (!match) {
      return;
    }
    this.assertRelationshipAllowsFriendRequest(match.relationshipState);
  }

  private assertRelationshipAllowsFriendRequest(relationshipState: string | undefined): void {
    if (relationshipState === 'active') {
      throw new Error('Contact is already a friend');
    }
    if (relationshipState?.includes('pending')) {
      throw new Error('Friend request is already pending');
    }
  }

  private async findAddFriendTarget(query: string): Promise<{ relationshipState: string; user: User } | null> {
    const response = await this.client().social.users.list({
      q: query,
      limit: SOCIAL_USER_SEARCH_LIMIT,
    });
    for (const item of response.items) {
      if (this.isCurrentUserSearchResult(item)) {
        continue;
      }
      const user = this.mapSocialUserSearchResultToUser(item);
      return {
        relationshipState: item.relationshipState,
        user,
      };
    }
    return null;
  }

  private getOrCreateRequestUiId(requestId: string): number {
    const existing = this.requestUiIdByBackendId.get(requestId);
    if (existing) {
      return existing;
    }

    let uiId = toRequestUiId(requestId);
    while (this.requestIdByUiId.has(uiId) && this.requestIdByUiId.get(uiId) !== requestId) {
      uiId += 1;
    }
    this.requestIdByUiId.set(uiId, requestId);
    this.requestUiIdByBackendId.set(requestId, uiId);
    return uiId;
  }

  private resolveFriendshipPeerId(friendship: {
    initiatorUserId: string;
    userHighId: string;
    userLowId: string;
  }): string | undefined {
    const currentUserId = this.getCurrentUser().id;
    if (friendship.userLowId === currentUserId) {
      return friendship.userHighId;
    }
    if (friendship.userHighId === currentUserId) {
      return friendship.userLowId;
    }
    return friendship.initiatorUserId === currentUserId
      ? undefined
      : friendship.initiatorUserId;
  }

  private cacheUser(user: User): void {
    this.userCache.set(user.id, user);
    if (user.chatId) {
      this.userIdByChatId.set(user.chatId, user.id);
    }
  }

  private syncCurrentUserProfile(user: User): void {
    const currentUser = this.getCurrentUser();
    const currentUserProfile: User = {
      ...currentUser,
      ...user,
      id: currentUser.id,
      name: user.name || currentUser.name,
      avatar: user.avatar ?? currentUser.avatar,
      status: currentUser.status ?? user.status,
      py: createSearchKey(user.name || currentUser.name),
    };

    this.currentUserOverrides = {
      ...this.currentUserOverrides,
      ...(currentUserProfile.chatId ? { chatId: currentUserProfile.chatId } : {}),
      name: currentUserProfile.name,
      avatar: currentUserProfile.avatar,
      status: currentUserProfile.status,
      email: currentUserProfile.email,
      phone: currentUserProfile.phone,
    };
    this.cacheUser(currentUserProfile);
    this.persistCurrentUserProfile(currentUserProfile);
  }

  private persistCurrentUserProfile(user: User): void {
    if (!user.chatId) {
      return;
    }

    const session = readAppSdkSessionTokens();
    if (!session || session.user?.chatId === user.chatId) {
      return;
    }

    applyAppSdkSessionTokens({
      ...session,
      user: {
        ...(session.user ?? {}),
        id: pickString(session.user?.id, session.context?.userId, user.id) ?? user.id,
        userId: pickString(session.user?.userId, session.context?.userId, user.id) ?? user.id,
        chatId: user.chatId,
        ...(user.name ? { displayName: user.name, name: user.name } : {}),
        ...(user.avatar ? { avatar: user.avatar } : {}),
        ...(user.email ? { email: user.email } : {}),
        ...(user.phone ? { phone: user.phone } : {}),
      },
    });
  }

}

export function createSdkworkContactService(
  getClient?: () => ImSdkClient,
  getAppClient?: () => unknown,
  getOrganizationDirectoryService?: () => OrganizationDirectoryService,
): ContactService {
  return new SdkworkContactService(getClient, getAppClient, getOrganizationDirectoryService);
}

export const contactService = createSdkworkContactService();
