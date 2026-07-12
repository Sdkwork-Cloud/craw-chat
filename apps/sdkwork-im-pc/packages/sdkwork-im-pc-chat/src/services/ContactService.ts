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
  forEachCursorPage,
  SDKWORK_DEFAULT_PAGE_SIZE,
  SDKWORK_MAX_PAGE_SIZE,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';
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
  id: string;
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
  refreshedContacts: number;
}

export interface ContactListPage {
  items: User[];
  nextCursor?: string;
  hasMore: boolean;
}

export interface FriendRequestListPage {
  items: FriendRequest[];
  hasMore: boolean;
  nextCursor?: string;
}

export interface ContactTagListPage {
  items: ContactTag[];
  hasMore: boolean;
  nextCursor?: string;
}

export interface ContactService {
  getContacts(): Promise<User[]>;
  listContactsPage(params?: { cursor?: string; pageSize?: number }): Promise<ContactListPage>;
  listContactConversationIds(): Promise<string[]>;
  searchContacts(query: string): Promise<User[]>;
  addFriend(userId: string): Promise<void>;
  addFriendBySearchQuery(query: string): Promise<User>;
  getStarredContacts(): Promise<User[]>;
  getDepartments(): Promise<OrgDepartment[]>;
  getUsersByDepartment(departmentId: string): Promise<User[]>;
  getCurrentUser(): User;
  getUserById(id: string): Promise<User | null>;
  getFriendRequests(): Promise<FriendRequest[]>;
  listFriendRequestsPage(params?: {
    direction?: 'incoming' | 'outgoing' | 'all';
    cursor?: string;
    pageSize?: number;
  }): Promise<FriendRequestListPage>;
  getPendingFriendRequestCount(): Promise<number>;
  subscribePendingFriendRequestCount(handler: (count: number) => void): () => void;
  getTags(): Promise<ContactTag[]>;
  listTagsPage(params?: { cursor?: string; pageSize?: number }): Promise<ContactTagListPage>;
  addTag(tag: Omit<ContactTag, 'id'>): Promise<ContactTag>;
  updateTag(id: string, updates: Partial<ContactTag>): Promise<ContactTag>;
  removeTag(id: string): Promise<void>;
  updateProfile(update: Partial<User>): Promise<User>;
  deleteContact(userId: string): Promise<void>;
  handleFriendRequest(requestId: string, action: 'accept' | 'reject'): Promise<void>;
  cancelFriendRequest(requestId: string): Promise<void>;
  toggleStarContact(userId: string, isStarred: boolean): Promise<void>;
  setContactRemark(userId: string, remark: string): Promise<void>;
  addToBlacklist(userId: string): Promise<void>;
  removeFromBlacklist(userId: string): Promise<void>;
  getContactPreferences(userId: string): Promise<ContactPreferencesView>;
  recommendToFriend(userId: string): Promise<void>;
  syncContacts(): Promise<ContactSyncResult>;
}

const CONTACTS_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const CONTACT_TAGS_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const MAX_CONTACT_TAGS_SYNC = SDKWORK_MAX_PAGE_SIZE;
const FRIEND_REQUESTS_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const FRIEND_REQUEST_COUNT_TTL_MS = 5_000;
const SOCIAL_USER_SEARCH_LIMIT = 20;
const FRIEND_REQUEST_COUNT_REFRESH_MS = 12000;
const FRIEND_REQUEST_REALTIME_EVENT_TYPES = [
  'friend_request.submitted',
  'friend_request.accepted',
  'friend_request.declined',
  'friend_request.canceled',
];
export const SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT = 'sdkwork-im-pc:friend-requests-changed';

function normalizePageSize(value: number | undefined, fallback: number): number {
  if (value === undefined) {
    return fallback;
  }
  const normalized = Math.floor(value);
  if (!Number.isFinite(normalized) || normalized <= 0) {
    return fallback;
  }
  return Math.min(normalized, SDKWORK_MAX_PAGE_SIZE);
}

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

function pickBoolean(...values: unknown[]): boolean | undefined {
  for (const value of values) {
    if (typeof value === 'boolean') {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const normalized = value.trim().toLowerCase();
      if (normalized === 'true') {
        return true;
      }
      if (normalized === 'false') {
        return false;
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

function readProblemDetailText(error: unknown): { code: string; message: string } {
  const record = toRecord(error);
  const body = toRecord(record.body ?? record.detail ?? record.problem ?? record.data);
  const code = pickString(body.code, body.title, record.code)?.toLowerCase() ?? '';
  const message = pickString(body.detail, body.title, body.message, record.message, record.error)?.toLowerCase() ?? '';
  return { code, message };
}

type FriendRequestSubmitConflict = 'already_friend' | 'pending' | 'blocked' | 'unknown';

function classifyFriendRequestSubmitError(error: unknown): FriendRequestSubmitConflict {
  const { code, message } = readProblemDetailText(error);
  if (code.includes('friendship_pair') || message.includes('already a friend') || message.includes('already exists')) {
    return 'already_friend';
  }
  if (code.includes('friend_request_pair') || code.includes('friend_request_conflict') || message.includes('already pending') || message.includes('open friend request') || message.includes('already exists')) {
    return 'pending';
  }
  if (code.includes('blocked') || code.includes('friend_request_blocked')) {
    return 'blocked';
  }
  return 'unknown';
}

function isFriendshipRemovalIdempotentError(error: unknown): boolean {
  const { code, message } = readProblemDetailText(error);
  return code.includes('friendship_not_found')
    || code.includes('friendship_not_active')
    || message.includes('not active')
    || message.includes('was not found');
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

function wireFriendRequestId(request: ImFriendRequest): string {
  const requestRecord = toRecord(request);
  const requestId = pickString(
    requestRecord.friendRequestId,
    requestRecord.request_id,
    requestRecord.friend_request_id,
  );
  if (!requestId) {
    throw new Error('Friend request id is required');
  }
  return requestId;
}

function readFriendRequestListPage(
  response: {
    items?: ImFriendRequest[];
    nextCursor?: string | null;
    pageInfo?: {
      hasMore?: boolean;
      nextCursor?: string | null;
    };
  },
  cursor?: string,
): { items: ImFriendRequest[]; hasMore: boolean; nextCursor?: string } {
  const nextCursor = response.pageInfo?.nextCursor;
  const hasMore = response.pageInfo?.hasMore === true
    || Boolean(nextCursor && nextCursor !== cursor);
  return {
    items: response.items ?? [],
    hasMore,
    nextCursor,
  };
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
  private authSessionGeneration = 0;
  private readonly contactByUserId = new Map<string, ContactView>();
  private readonly preferenceByUserId = new Map<string, ContactPreferencesView>();
  private readonly processingFriendRequestIds = new Set<string>();
  private readonly processingAddFriendUserIds = new Set<string>();
  private readonly deletingContactUserIds = new Set<string>();
  private readonly blacklistingUserIds = new Set<string>();
  private readonly userCache = new Map<string, User>();
  private readonly userIdByChatId = new Map<string, string>();
  private readonly pendingFriendRequestCountHandlers = new Set<(count: number) => void>();
  private currentUserOverrides: Partial<User> = {};
  private pendingFriendRequestCount: number | undefined;
  private pendingFriendRequestCountRefresh?: Promise<number>;
  private pendingFriendRequestCountCacheAt = 0;
  private pendingFriendRequestRefreshTimer?: ReturnType<typeof setInterval>;
  private pendingFriendRequestRefreshListener?: () => void;
  private pendingFriendRequestRealtimeUnsub?: () => void;
  private pendingFriendRequestRealtimeUserId?: string;
  private lastKnownSessionUserId?: string;
  private friendRequestsListSync?: Promise<FriendRequest[]>;
  // In-flight dedup for getUserById: collapses concurrent calls for the same
  // id (e.g. Sidebar mount + session-changed event) onto one promise so only
  // one /social/users request is issued.
  private readonly getUserByIdInFlight = new Map<string, Promise<User | null>>();
  private readonly handleAuthSessionChanged = (event?: Event): void => {
    // Determine whether the session change is a real user switch (or logout)
    // vs an incremental profile refresh (e.g. chatId hydration). Only clear
    // user-scoped caches on a real switch; profile refreshes must not evict
    // the very cache that was just populated (which caused a feedback loop).
    type SessionDetail = {
      session: {
        context?: { userId?: string | number };
        user?: { id?: string | number; userId?: string | number };
      } | null;
    } | undefined;
    const detail = (event as CustomEvent<SessionDetail> | undefined)?.detail;
    const currentSession = readAppSdkSessionTokens();
    const nextUserId = pickString(
      detail?.session?.user?.userId,
      detail?.session?.user?.id,
      detail?.session?.context?.userId,
      currentSession?.user?.userId,
      currentSession?.user?.id,
      currentSession?.context?.userId,
    );
    const isUserSwitch = !nextUserId || !this.lastKnownSessionUserId || String(nextUserId) !== String(this.lastKnownSessionUserId);
    this.lastKnownSessionUserId = nextUserId ? String(nextUserId) : undefined;

    if (!isUserSwitch) {
      // Same user, profile-only update: keep caches, only stop the friend
      // request loop if the session is no longer authenticated.
      if (!nextUserId) {
        this.stopPendingFriendRequestRefreshLoop();
      }
      return;
    }

    const previousPendingFriendRequestCount = this.pendingFriendRequestCount;
    this.authSessionGeneration += 1;
    this.contactByUserId.clear();
    this.preferenceByUserId.clear();
    this.processingFriendRequestIds.clear();
    this.processingAddFriendUserIds.clear();
    this.deletingContactUserIds.clear();
    this.blacklistingUserIds.clear();
    this.userCache.clear();
    this.userIdByChatId.clear();
    this.getUserByIdInFlight.clear();
    this.pendingFriendRequestCount = undefined;
    this.pendingFriendRequestCountRefresh = undefined;
    this.pendingFriendRequestCountCacheAt = 0;
    this.friendRequestsListSync = undefined;
    this.stopPendingFriendRequestRefreshLoop();
    if (previousPendingFriendRequestCount !== undefined && previousPendingFriendRequestCount !== 0) {
      this.emitPendingFriendRequestCount(0);
    }
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

  private assertAuthSessionGenerationCurrent(generation: number, operation: string): void {
    if (generation !== this.authSessionGeneration) {
      throw new Error(`Contact session changed while ${operation}.`);
    }
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

  private async syncContactsFromServer(): Promise<number> {
    const page = await this.listContactsPage({ pageSize: CONTACTS_PAGE_LIMIT });
    return page.items.length;
  }

  private async syncFriendRequestsFromServer(
    direction: 'incoming' | 'outgoing' | 'all',
    status: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all' = 'all',
  ): Promise<ImFriendRequest[]> {
    const response = await this.client().social.friendRequests.list({
      direction,
      status,
      pageSize: FRIEND_REQUESTS_PAGE_LIMIT,
    });
    return readFriendRequestListPage(response).items;
  }

  private async refreshPendingFriendRequestCount(force = false): Promise<number> {
    const generation = this.authSessionGeneration;
    const client = this.client();
    if (!this.hasAuthenticatedSession()) {
      const previousCount = this.pendingFriendRequestCount;
      this.pendingFriendRequestCount = 0;
      if (previousCount !== 0) {
        this.emitPendingFriendRequestCount(0);
      }
      return 0;
    }
    // TTL cache: serve fresh results without re-hitting the network. This
    // coalesces bursts from setInterval, focus, visibilitychange, realtime
    // events, and multiple subscribers into one request per TTL window.
    if (this.pendingFriendRequestCountRefresh) {
      return this.pendingFriendRequestCountRefresh;
    }
    if (!force && Date.now() - this.pendingFriendRequestCountCacheAt < FRIEND_REQUEST_COUNT_TTL_MS) {
      return this.pendingFriendRequestCount ?? 0;
    }

    let refreshPromise: Promise<number>;
    refreshPromise = (async () => {
      let count: number;
      try {
        const response = await client.social.friendRequests.pendingCount();
        count = response.count;
      } catch (error) {
        if (isAuthenticationFailure(error)) {
          if (generation === this.authSessionGeneration) {
            this.handleAuthenticationFailure();
            const previousCount = this.pendingFriendRequestCount;
            this.pendingFriendRequestCount = 0;
            if (previousCount !== 0) {
              this.emitPendingFriendRequestCount(0);
            }
          }
          return 0;
        }
        if (generation !== this.authSessionGeneration) {
          return this.pendingFriendRequestCount ?? 0;
        }
        // Fallback for older gateways: keep the work bounded to one visible page.
        const incoming = await this.listFriendRequestsPage({ direction: 'incoming' });
        count = incoming.items.length;
      }
      if (generation !== this.authSessionGeneration) {
        return this.pendingFriendRequestCount ?? 0;
      }
      const previousCount = this.pendingFriendRequestCount;
      this.pendingFriendRequestCount = count;
      if (previousCount !== count) {
        this.emitPendingFriendRequestCount(count);
      }
      return count;
    })().finally(() => {
      if (this.pendingFriendRequestCountRefresh === refreshPromise) {
        this.pendingFriendRequestCountRefresh = undefined;
        if (generation === this.authSessionGeneration) {
          this.pendingFriendRequestCountCacheAt = Date.now();
        }
      }
    });
    this.pendingFriendRequestCountRefresh = refreshPromise;

    return this.pendingFriendRequestCountRefresh;
  }

  private emitPendingFriendRequestCount(count: number): void {
    for (const handler of this.pendingFriendRequestCountHandlers) {
      try {
        handler(count);
      } catch {
        // A stale UI observer must not abort notification delivery to others.
      }
    }
  }

  private dispatchFriendRequestChange(): void {
    this.friendRequestsListSync = undefined;
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

  async getContacts(): Promise<User[]> {
    const page = await this.listContactsPage();
    return page.items;
  }

  async listContactsPage(params?: { cursor?: string; pageSize?: number }): Promise<ContactListPage> {
    const generation = this.authSessionGeneration;
    const response = await this.client().chat.contacts.list({
      pageSize: normalizePageSize(params?.pageSize, CONTACTS_PAGE_LIMIT),
      ...(params?.cursor ? { cursor: params.cursor } : {}),
    });
    if (generation !== this.authSessionGeneration) {
      return { items: [], hasMore: false, nextCursor: undefined };
    }
    const contacts = this.hydrateContactUsers(response.items);
    return {
      items: contacts,
      nextCursor: (response.pageInfo?.hasMore ?? false) ? (response.pageInfo?.nextCursor ?? undefined) : undefined,
      hasMore: response.pageInfo?.hasMore ?? false,
    };
  }

  async listContactConversationIds(): Promise<string[]> {
    const conversationIds = new Set<string>();
    for (const contact of this.contactByUserId.values()) {
      const conversationId = normalizeString(contact.conversationId);
      if (conversationId) {
        conversationIds.add(conversationId);
      }
    }
    return [...conversationIds];
  }

  private hydrateContactUsers(contacts: ContactView[]): User[] {
    const users: User[] = [];
    for (const contact of contacts) {
      const preferences = this.readProjectedContactPreferences(contact);
      if (preferences?.isBlocked) {
        continue;
      }
      users.push(this.mapContactViewToUser(contact, preferences));
    }
    return users;
  }

  async searchContacts(query: string): Promise<User[]> {
    return this.searchSocialUsers(query, { includeCurrentUser: false });
  }

  async addFriend(userId: string): Promise<void> {
    const generation = this.authSessionGeneration;
    const client = this.client();
    const targetUserId = userId.trim();
    if (!targetUserId) {
      throw new Error('Friend user id is required');
    }
    if (this.isCurrentUserIdentifier(targetUserId)) {
      throw new Error('Cannot add yourself as a friend');
    }
    if (this.processingAddFriendUserIds.has(targetUserId)) {
      return;
    }
    this.processingAddFriendUserIds.add(targetUserId);
    try {
      await this.assertCanSendFriendRequest(targetUserId);
      this.assertAuthSessionGenerationCurrent(generation, 'adding a friend');
      await client.social.friendRequests.create({ targetUserId });
      this.assertAuthSessionGenerationCurrent(generation, 'adding a friend');
      this.dispatchFriendRequestChange();
      void this.refreshPendingFriendRequestCount(true).catch(() => undefined);
    } catch (error) {
      const conflict = classifyFriendRequestSubmitError(error);
      if (conflict === 'already_friend') {
        throw new Error('Contact is already a friend');
      }
      if (conflict === 'pending') {
        throw new Error('Friend request is already pending');
      }
      if (conflict === 'blocked') {
        throw new Error('Friend request is blocked');
      }
      throw error;
    } finally {
      if (generation === this.authSessionGeneration) {
        this.processingAddFriendUserIds.delete(targetUserId);
      }
    }
  }

  async addFriendBySearchQuery(query: string): Promise<User> {
    const generation = this.authSessionGeneration;
    const client = this.client();
    const normalizedQuery = query.trim();
    if (!normalizedQuery) {
      throw new Error('Friend search query is required');
    }

    const target = await this.findAddFriendTarget(normalizedQuery);
    this.assertAuthSessionGenerationCurrent(generation, 'adding a friend by search');
    if (!target) {
      throw new Error('Friend search target not found');
    }

    if (this.processingAddFriendUserIds.has(target.user.id)) {
      return target.user;
    }
    this.processingAddFriendUserIds.add(target.user.id);
    try {
      this.assertRelationshipAllowsFriendRequest(target.relationshipState);
      await client.social.friendRequests.create({ targetUserId: target.user.id });
      this.assertAuthSessionGenerationCurrent(generation, 'adding a friend by search');
      this.dispatchFriendRequestChange();
      void this.refreshPendingFriendRequestCount(true).catch(() => undefined);
      return target.user;
    } catch (error) {
      const conflict = classifyFriendRequestSubmitError(error);
      if (conflict === 'already_friend') {
        throw new Error('Contact is already a friend');
      }
      if (conflict === 'pending') {
        throw new Error('Friend request is already pending');
      }
      if (conflict === 'blocked') {
        throw new Error('Friend request is blocked');
      }
      throw error;
    } finally {
      if (generation === this.authSessionGeneration) {
        this.processingAddFriendUserIds.delete(target.user.id);
      }
    }
  }

  async getStarredContacts(): Promise<User[]> {
    const starred: User[] = [];
    for (const [userId, preferences] of this.preferenceByUserId) {
      if (!preferences.isStarred || preferences.isBlocked) {
        continue;
      }
      const cached = this.userCache.get(userId);
      const contact = this.contactByUserId.get(userId);
      if (cached) {
        starred.push({ ...cached });
      } else if (contact) {
        starred.push(this.mapContactViewToUser(contact, preferences));
      }
    }
    return starred;
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
    // In-flight dedup: if a lookup for this id is already running, share the
    // same promise. This prevents the login feedback loop (Sidebar mount +
    // session-changed event from appAuthService.getCurrentSession) from
    // issuing two concurrent /social/users requests for the same user.
    const inFlight = this.getUserByIdInFlight.get(normalizedId);
    if (inFlight) {
      return inFlight;
    }
    let promise: Promise<User | null>;
    promise = this.resolveUserById(normalizedId).finally(() => {
      if (this.getUserByIdInFlight.get(normalizedId) === promise) {
        this.getUserByIdInFlight.delete(normalizedId);
      }
    });
    this.getUserByIdInFlight.set(normalizedId, promise);
    return promise;
  }

  private async resolveUserById(normalizedId: string): Promise<User | null> {
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
    const cachedContact = this.contactByUserId.get(normalizedId);
    if (cachedContact) {
      return this.mapContactViewToUser(cachedContact, this.preferenceByUserId.get(normalizedId));
    }
    if (shouldReturnCurrentUserIfLookupFails) {
      return await this.findSocialUserByLookup(normalizedId) ?? currentUser;
    }
    return await this.findSocialUserByLookup(normalizedId);
  }

  async getFriendRequests(): Promise<FriendRequest[]> {
    if (this.friendRequestsListSync) {
      return this.friendRequestsListSync;
    }

    const generation = this.authSessionGeneration;
    let listPromise: Promise<FriendRequest[]>;
    listPromise = (async () => {
      const requests = await this.syncFriendRequestsFromServer('all', 'pending');
      if (generation !== this.authSessionGeneration) {
        return [];
      }
      return requests.map((request) => this.mapFriendRequest(request));
    })().finally(() => {
      if (this.friendRequestsListSync === listPromise) {
        this.friendRequestsListSync = undefined;
      }
    });
    this.friendRequestsListSync = listPromise;

    return this.friendRequestsListSync;
  }

  async listFriendRequestsPage(params: {
    direction?: 'incoming' | 'outgoing' | 'all';
    cursor?: string;
    pageSize?: number;
  } = {}): Promise<FriendRequestListPage> {
    const direction = params.direction ?? 'all';
    const generation = this.authSessionGeneration;
    const pageSize = normalizePageSize(params.pageSize, FRIEND_REQUESTS_PAGE_LIMIT);
    const response = await this.client().social.friendRequests.list({
      direction,
      status: 'pending',
      pageSize,
      ...(params.cursor ? { cursor: params.cursor } : {}),
    });
    const page = readFriendRequestListPage(response, params.cursor);
    if (generation !== this.authSessionGeneration) {
      return { items: [], hasMore: false, nextCursor: undefined };
    }
    return {
      items: page.items.map((request) => this.mapFriendRequest(request)),
      hasMore: page.hasMore,
      nextCursor: page.nextCursor,
    };
  }

  async getPendingFriendRequestCount(): Promise<number> {
    return this.refreshPendingFriendRequestCount();
  }

  subscribePendingFriendRequestCount(handler: (count: number) => void): () => void {
    this.pendingFriendRequestCountHandlers.add(handler);
    if (this.pendingFriendRequestCount !== undefined) {
      try {
        handler(this.pendingFriendRequestCount);
      } catch {
        // Subscription setup must still return a disposer for a failing view.
      }
    }
    if (this.hasAuthenticatedSession()) {
      // Only trigger a network refresh if we don't have a fresh cached value.
      // Multiple subscribers (ChatLayout + NewFriendsContainer) should not each
      // trigger a separate count request on subscribe.
      void this.refreshPendingFriendRequestCount().catch(() => undefined);
      void this.startPendingFriendRequestRealtime();
      this.startPendingFriendRequestRefreshLoop();
    } else {
      try {
        handler(0);
      } catch {
        // Subscription setup must still return a disposer for a failing view.
      }
    }

    return () => {
      this.pendingFriendRequestCountHandlers.delete(handler);
      if (this.pendingFriendRequestCountHandlers.size === 0) {
        this.stopPendingFriendRequestRefreshLoop();
      }
    };
  }

  async getTags(): Promise<ContactTag[]> {
    const tags: ContactTag[] = [];
    await forEachCursorPage(
      async (cursor) => {
        const page = await this.listTagsPage({ cursor });
        return {
          items: page.items,
          hasMore: page.hasMore,
          nextCursor: page.nextCursor,
        };
      },
      (items) => {
        tags.push(...items);
      },
      { maxItems: MAX_CONTACT_TAGS_SYNC },
    );
    return tags;
  }

  async listTagsPage(params: {
    cursor?: string;
    pageSize?: number;
  } = {}): Promise<ContactTagListPage> {
    const generation = this.authSessionGeneration;
    const pageSize = normalizePageSize(params.pageSize, CONTACT_TAGS_PAGE_LIMIT);
    const response = await this.client().social.contacts.tags.list({
      pageSize,
      ...(params.cursor ? { cursor: params.cursor } : {}),
    });
    if (generation !== this.authSessionGeneration) {
      return { items: [], hasMore: false, nextCursor: undefined };
    }
    return {
      items: response.items.map((tag) => this.mapContactTagViewToContactTag(tag)),
      hasMore: Boolean(response.pageInfo?.nextCursor),
      nextCursor: response.pageInfo?.nextCursor ?? undefined,
    };
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
    const generation = this.authSessionGeneration;
    const client = this.client();
    const normalizedUserId = userId.trim();
    if (!normalizedUserId) {
      throw new Error('Contact user id is required');
    }
    if (this.deletingContactUserIds.has(normalizedUserId)) {
      return;
    }
    this.deletingContactUserIds.add(normalizedUserId);
    try {
      const contact = this.contactByUserId.get(normalizedUserId);
      if (!contact?.friendshipId) {
        throw new Error('Contact friendship is not available');
      }

      try {
        await client.social.friendships.remove(contact.friendshipId);
      } catch (error) {
        if (!isFriendshipRemovalIdempotentError(error)) {
          throw error;
        }
      }
      this.assertAuthSessionGenerationCurrent(generation, 'deleting a contact');
      this.evictLocalContact(normalizedUserId);
      this.dispatchFriendRequestChange();
    } finally {
      if (generation === this.authSessionGeneration) {
        this.deletingContactUserIds.delete(normalizedUserId);
      }
    }
  }

  private evictLocalContact(normalizedUserId: string): void {
    this.contactByUserId.delete(normalizedUserId);
    const cached = this.userCache.get(normalizedUserId);
    if (cached?.chatId) {
      this.userIdByChatId.delete(cached.chatId);
    }
    this.userCache.delete(normalizedUserId);
    this.preferenceByUserId.delete(normalizedUserId);
  }

  async handleFriendRequest(requestId: string, action: 'accept' | 'reject'): Promise<void> {
    const generation = this.authSessionGeneration;
    const client = this.client();
    const normalizedRequestId = requestId.trim();
    if (!normalizedRequestId || this.processingFriendRequestIds.has(normalizedRequestId)) {
      return;
    }
    this.processingFriendRequestIds.add(normalizedRequestId);
    try {
      if (action === 'accept') {
        const result = await client.social.friendRequests.accept(normalizedRequestId);
        this.assertAuthSessionGenerationCurrent(generation, 'accepting a friend request');
        const userId = this.resolveFriendshipPeerId(result.friendship);
        if (userId) {
          await this.loadUserProfile(userId);
          this.assertAuthSessionGenerationCurrent(generation, 'accepting a friend request');
        }
        await this.refreshPendingFriendRequestCount(true);
        this.assertAuthSessionGenerationCurrent(generation, 'accepting a friend request');
        this.dispatchFriendRequestChange();
        return;
      }

      await client.social.friendRequests.decline(normalizedRequestId);
      this.assertAuthSessionGenerationCurrent(generation, 'rejecting a friend request');
      await this.refreshPendingFriendRequestCount(true);
      this.assertAuthSessionGenerationCurrent(generation, 'rejecting a friend request');
      this.dispatchFriendRequestChange();
    } finally {
      if (generation === this.authSessionGeneration) {
        this.processingFriendRequestIds.delete(normalizedRequestId);
      }
    }
  }

  async cancelFriendRequest(requestId: string): Promise<void> {
    const generation = this.authSessionGeneration;
    const client = this.client();
    const normalizedRequestId = requestId.trim();
    if (!normalizedRequestId || this.processingFriendRequestIds.has(normalizedRequestId)) {
      return;
    }
    this.processingFriendRequestIds.add(normalizedRequestId);
    try {
      await client.social.friendRequests.cancel(normalizedRequestId);
      this.assertAuthSessionGenerationCurrent(generation, 'canceling a friend request');
      await this.refreshPendingFriendRequestCount(true);
      this.assertAuthSessionGenerationCurrent(generation, 'canceling a friend request');
      this.dispatchFriendRequestChange();
    } finally {
      if (generation === this.authSessionGeneration) {
        this.processingFriendRequestIds.delete(normalizedRequestId);
      }
    }
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
    const generation = this.authSessionGeneration;
    const client = this.client();
    const normalizedUserId = this.normalizeContactUserId(userId);
    if (this.blacklistingUserIds.has(normalizedUserId)) {
      return;
    }
    this.blacklistingUserIds.add(normalizedUserId);
    try {
      const preferences = await client.social.contacts.preferences.update(normalizedUserId, {
        isBlocked: true,
        isStarred: false,
      });
      this.assertAuthSessionGenerationCurrent(generation, 'adding a contact to the blacklist');
      this.preferenceByUserId.set(normalizedUserId, preferences);
      this.evictLocalContact(normalizedUserId);
      this.dispatchFriendRequestChange();
    } finally {
      if (generation === this.authSessionGeneration) {
        this.blacklistingUserIds.delete(normalizedUserId);
      }
    }
  }

  async removeFromBlacklist(userId: string): Promise<void> {
    const generation = this.authSessionGeneration;
    const client = this.client();
    const normalizedUserId = this.normalizeContactUserId(userId);
    if (this.blacklistingUserIds.has(normalizedUserId)) {
      return;
    }
    this.blacklistingUserIds.add(normalizedUserId);
    try {
      const preferences = await client.social.contacts.preferences.update(normalizedUserId, {
        isBlocked: false,
      });
      this.assertAuthSessionGenerationCurrent(generation, 'removing a contact from the blacklist');
      this.preferenceByUserId.set(normalizedUserId, preferences);
      this.dispatchFriendRequestChange();
    } finally {
      if (generation === this.authSessionGeneration) {
        this.blacklistingUserIds.delete(normalizedUserId);
      }
    }
  }

  async getContactPreferences(userId: string): Promise<ContactPreferencesView> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const cached = this.preferenceByUserId.get(normalizedUserId);
    if (cached) {
      return cached;
    }
    const preferences = await this.client().social.contacts.preferences.retrieve(normalizedUserId);
    this.preferenceByUserId.set(normalizedUserId, preferences);
    return preferences;
  }

  async recommendToFriend(userId: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    await this.client().social.contacts.recommendations.create(normalizedUserId, {});
  }

  async syncContacts(): Promise<ContactSyncResult> {
    const refreshedContacts = await this.syncContactsFromServer();
    return { refreshedContacts };
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
      pageSize: SOCIAL_USER_SEARCH_LIMIT,
    });
    return response.items
      .filter((item: SocialUserSearchResult) => options.includeCurrentUser || !this.isCurrentUserSearchResult(item))
      .map((item: SocialUserSearchResult) => this.mapSocialUserSearchResultToUser(item));
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

  private readProjectedContactPreferences(contact: ContactView): ContactPreferencesView | undefined {
    const cached = this.preferenceByUserId.get(contact.targetUserId);
    const contactRecord = toRecord(contact);
    const preferencesRecord = toRecord(contactRecord.preferences ?? contactRecord.preference);
    const isBlocked = pickBoolean(
      contactRecord.isBlocked,
      contactRecord.is_blocked,
      preferencesRecord.isBlocked,
      preferencesRecord.is_blocked,
    );
    const isStarred = pickBoolean(
      contactRecord.isStarred,
      contactRecord.is_starred,
      preferencesRecord.isStarred,
      preferencesRecord.is_starred,
    );
    const remark = pickString(contactRecord.remark, preferencesRecord.remark);
    if (isBlocked === undefined && isStarred === undefined && remark === undefined) {
      return cached;
    }
    const projected: ContactPreferencesView = {
      isBlocked: isBlocked ?? cached?.isBlocked ?? false,
      isStarred: isStarred ?? cached?.isStarred ?? false,
      ownerUserId: contact.ownerUserId,
      remark: remark ?? cached?.remark ?? '',
      targetUserId: contact.targetUserId,
      tenantId: contact.tenantId,
      updatedAt: pickString(contactRecord.updatedAt, contactRecord.updated_at, preferencesRecord.updatedAt, preferencesRecord.updated_at)
        ?? cached?.updatedAt
        ?? contact.lastInteractionAt,
    };
    this.preferenceByUserId.set(contact.targetUserId, projected);
    return projected;
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
      email: result.email ?? undefined,
      phone: result.phone ?? undefined,
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

  private mapFriendRequest(request: ImFriendRequest): FriendRequest {
    const requestId = wireFriendRequestId(request);
    const currentUserId = this.getCurrentUser().id;
    const isOutgoing = request.requesterUserId === currentUserId;
    const peerUserId = isOutgoing ? request.targetUserId : request.requesterUserId;
    const peerUser = this.userCache.get(peerUserId);
    const name = this.preferenceByUserId.get(peerUserId)?.remark || peerUser?.name || peerUserId;
    return {
      avatar: peerUser?.avatar ?? createAvatar(peerUserId),
      direction: isOutgoing ? 'outgoing' : 'incoming',
      id: requestId,
      name,
      msg: request.requestMessage ?? '',
      status: normalizeRequestStatus(request.status),
    };
  }

  private async assertCanSendFriendRequest(targetUserId: string): Promise<void> {
    const response = await this.client().social.users.list({
      q: targetUserId,
      pageSize: SOCIAL_USER_SEARCH_LIMIT,
    });
    const match = response.items.find((item: SocialUserSearchResult) => item.userId === targetUserId);
    if (!match) {
      return;
    }
    this.assertRelationshipAllowsFriendRequest(match.relationshipState);
  }

  private assertRelationshipAllowsFriendRequest(relationshipState: string | undefined): void {
    if (relationshipState === 'active') {
      throw new Error('Contact is already a friend');
    }
    if (
      relationshipState === 'pending_incoming'
      || relationshipState === 'pending_outgoing'
      || relationshipState?.includes('pending')
    ) {
      throw new Error('Friend request is already pending');
    }
  }

  private async findAddFriendTarget(query: string): Promise<{ relationshipState: string; user: User } | null> {
    const response = await this.client().social.users.list({
      q: query,
      pageSize: SOCIAL_USER_SEARCH_LIMIT,
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
