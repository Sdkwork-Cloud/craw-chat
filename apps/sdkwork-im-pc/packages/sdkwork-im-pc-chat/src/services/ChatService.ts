import type {
  ContentPart,
  ConversationMessageListResponse,
  ConversationInboxEntry,
  ConversationMember,
  DriveReference,
  ImDecodedMessage,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
  ImSdkClient,
  MediaKind,
  MediaResource,
  MessageReplyReference,
  UpdateConversationProfileRequest,
} from '@sdkwork/im-sdk';
import type {
  DriveUploaderBlobLike,
  DriveUploaderProfile,
  DriveUploaderRequest,
  DriveUploaderUploadResult,
  SdkworkDriveUploader,
} from '@sdkwork/im-pc-core/sdk/driveAppSdkClient';
import {
  forEachCursorPage,
  SDKWORK_DEFAULT_PAGE_SIZE,
  SDKWORK_MAX_PAGE_SIZE,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';
import {
  configurePcRealtimeConnectionManager,
  onPcLiveAuthenticationFailure,
  onPcLiveConnectionOpen,
  recoverPcLiveConnection,
  subscribePcConversationMessages,
  subscribePcRealtimeScope,
} from '@sdkwork/im-pc-core/sdk/pcRealtimeConnectionManager';
import {
  ensureDesktopOfflineChatCache,
  loadDesktopOfflineChats,
  loadDesktopOfflineMessages,
  persistDesktopOfflineChats,
  persistDesktopOfflineMessages,
  type OfflinePersistableMessage,
} from '@sdkwork/im-pc-core/sdk/desktopOfflineChatCache';
import {
  enqueueDesktopPendingSend,
  isDesktopPendingSendClaimCurrent,
  isRetryableDesktopSendError,
  listDesktopPendingSends,
  removeDesktopPendingSend,
  releaseDesktopPendingSendClaim,
  runDesktopPendingSendFlush,
} from '@sdkwork/im-pc-core/sdk/desktopOfflineSendQueue';
import {
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  readAppSdkSessionTokens,
  resolveAppSdkOrganizationId,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core/sdk/session';
import type { Chat, Message } from '@sdkwork/im-pc-types';
import { resolveSdkworkChatPcClientId } from './ClientIdentityService';
import { contactService } from './ContactService';
import { createDefaultAvatar } from './DefaultAvatarService';
import i18n from '../i18n';

type ConversationMessageEntry = ConversationMessageListResponse['items'][number];
type ChatListHandler = (chats: Chat[]) => void;
type MessageHandler = (message: Message) => void;
type ImSdkClientProvider = () => Promise<ImSdkClient> | ImSdkClient;
type SendableMediaMessageType = Extract<Message['type'], 'file' | 'image' | 'video' | 'voice'>;
type SendableStructuredMessageType = Extract<Message['type'], 'applet' | 'card' | 'link' | 'music' | 'system' | 'video_call'>;

type ChatMessageExtraInfo = Partial<Message> & {
  file?: DriveUploaderBlobLike;
  mimeType?: string;
};

interface ChatMediaUploadResult {
  content: string;
  drive: DriveReference;
  resource: MediaResource;
}

interface ChatServiceDependencies {
  getClient?: ImSdkClientProvider;
  getDriveUploader?: () => Promise<SdkworkDriveUploader> | SdkworkDriveUploader;
  getSession?: () => SdkworkChatSession | null;
}

export interface ChatOfflineSyncResult {
  appliedMessages: number;
  refreshedChats: number;
}

export interface ChatListPage {
  items: Chat[];
  hasMore: boolean;
  nextCursor?: string;
}

interface ConversationLiveSubscription {
  chatId: string;
  handlers: Set<MessageHandler>;
  notifiedMessageVersions: Map<string, string>;
}

interface ConversationCacheToken {
  readonly serial: number;
}

export interface ChatService {
  getChats(): Promise<Chat[]>;
  listChatsPage(options?: { cursor?: string; pageSize?: number }): Promise<ChatListPage>;
  subscribeChats(handler: ChatListHandler): () => void;
  getMessages(chatId: string, options?: { pageSize?: number }): Promise<Message[]>;
  hasMoreMessages(chatId: string): boolean;
  loadMoreMessages(chatId: string, pageSize?: number): Promise<Message[]>;
  subscribeMessages(chatId: string, handler: MessageHandler): () => void;
  sendMessage(
    chatId: string,
    content: string,
    type?: Message['type'],
    replyTo?: Message['replyTo'],
    extraInfo?: ChatMessageExtraInfo
  ): Promise<Message>;
  forwardMessages(targetChatIds: string[], messages: Message[]): Promise<void>;
  markAsRead(chatId: string): Promise<void>;
  markAsUnread(chatId: string): Promise<void>;
  deleteMessage(chatId: string, messageId: string): Promise<void>;
  recallMessage(chatId: string, messageId: string): Promise<void>;
  editMessage(chatId: string, messageId: string, text: string): Promise<void>;
  deleteChat(chatId: string): Promise<void>;
  pinChat(chatId: string, isPinned: boolean): Promise<void>;
  muteChat(chatId: string, isMuted: boolean): Promise<void>;
  addReaction(chatId: string, messageId: string, emoji: string): Promise<void>;
  removeReaction(chatId: string, messageId: string, emoji: string): Promise<void>;
  updateChat(chatId: string, updates: Partial<Chat>): Promise<Chat>;
  createChat(chat: Chat): Promise<Chat>;
  startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { conversationId?: string; directChatId?: string; id: string }): Promise<Chat>;
  startAgentChat(agent: Pick<Chat, 'avatar' | 'name' | 'welcomeMessage'> & { id: string }): Promise<Chat>;
  startEnterpriseChat(enterprise: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat>;
  recoverRealtimeConnection(reason?: string): void;
  syncOfflineMessages(): Promise<ChatOfflineSyncResult>;
  retryFailedMessage(chatId: string, messageId: string): Promise<Message>;
  setReadFocusContext(context: { activeConversationId?: string; isWindowFocused?: boolean }): void;
}

type ConversationViewState = Partial<Pick<Chat, 'activeCount' | 'avatar' | 'isMarkedUnread' | 'isMuted' | 'isPinned' | 'memberCount' | 'members' | 'name' | 'notice' | 'type' | 'welcomeMessage'>> & {
  isHidden?: boolean;
};
const INBOX_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const MAX_INBOX_CONVERSATIONS = SDKWORK_MAX_PAGE_SIZE;
const LOCAL_MESSAGES_PER_CONVERSATION_CAP = SDKWORK_MAX_PAGE_SIZE;
const LOCAL_CONVERSATION_CACHE_CAP = SDKWORK_MAX_PAGE_SIZE * 10;
const MAX_CONCURRENT_MESSAGE_HISTORY_LOADS = SDKWORK_MAX_PAGE_SIZE;
const MAX_LIVE_CONVERSATION_SUBSCRIPTIONS = SDKWORK_MAX_PAGE_SIZE;
const MESSAGE_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;

function readSdkCursorPageInfo(
  pageInfo: { hasMore?: boolean; nextCursor?: string | null } | undefined,
): Pick<ChatListPage, 'hasMore' | 'nextCursor'> {
  const hasMore = pageInfo?.hasMore === true;
  return {
    hasMore,
    nextCursor: hasMore ? (pageInfo?.nextCursor ?? undefined) : undefined,
  };
}

const CONVERSATION_MEMBERS_PAGE_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const MAX_CONVERSATION_MEMBERS_SYNC = SDKWORK_MAX_PAGE_SIZE;
const CHAT_LIST_HYDRATION_CONCURRENCY = 4;
const REALTIME_READ_CURSOR_SYNC_CONCURRENCY = 4;
const DEFAULT_MESSAGE_INITIAL_LIMIT = SDKWORK_DEFAULT_PAGE_SIZE;
const CHAT_LIST_COALESCE_MS = 350;
// Short TTL for the inbox first page: covers the startup sequence where
// syncOfflineMessages and refreshChats call listChatsPage() back-to-back
// (serial await defeats the in-flight dedup). This window is short enough
// that realtime events still get a fresh load via emitChatList's coalesce.
const INBOX_FIRST_PAGE_TTL_MS = 800;
const CHAT_LIST_REALTIME_EVENT_TYPES = [
  'message.posted',
  'conversation.updated',
  'conversation.created',
  'conversation.member_joined',
  'conversation.member_role_changed',
  'conversation.member_removed',
  'conversation.member_left',
  'conversation.owner_transferred',
];

function normalizeInboxPageSize(pageSize: number | undefined): number {
  if (pageSize === undefined) {
    return INBOX_PAGE_LIMIT;
  }
  const normalizedPageSize = Math.floor(pageSize);
  if (!Number.isFinite(normalizedPageSize) || normalizedPageSize <= 0) {
    return INBOX_PAGE_LIMIT;
  }
  return Math.min(normalizedPageSize, SDKWORK_MAX_PAGE_SIZE);
}
const CHAT_DRIVE_SCENE = 'im';
const CHAT_DRIVE_SOURCE = 'chat_message';
const CHAT_DRIVE_APP_RESOURCE_TYPE = 'im_conversation';
const CHAT_MESSAGE_TYPES = new Set<Message['type']>([
  'applet',
  'card',
  'file',
  'image',
  'link',
  'music',
  'system',
  'text',
  'video',
  'video_call',
  'voice',
]);
const MEDIA_MESSAGE_TYPES = new Set<Message['type']>(['file', 'image', 'video', 'voice']);
const STRUCTURED_MESSAGE_SCHEMA_BY_TYPE: Record<SendableStructuredMessageType, string> = {
  applet: 'urn:sdkwork:sdkwork-im:message:applet',
  card: 'urn:sdkwork:sdkwork-im:message:card',
  link: 'urn:sdkwork:sdkwork-im:message:link',
  music: 'urn:sdkwork:sdkwork-im:message:music',
  system: 'urn:sdkwork:sdkwork-im:message:system',
  video_call: 'urn:sdkwork:sdkwork-im:message:video_call',
};

let driveUploaderClient: SdkworkDriveUploader | null = null;
let driveUploaderClientPromise: Promise<SdkworkDriveUploader> | null = null;
let driveUploaderClientGeneration = 0;

function invalidateDefaultDriveUploader(): void {
  driveUploaderClientGeneration += 1;
  driveUploaderClient = null;
  driveUploaderClientPromise = null;
}

function parseTimestamp(value: string | undefined): number {
  if (!value) {
    return Date.now();
  }
  const timestamp = new Date(value).getTime();
  return Number.isFinite(timestamp) ? timestamp : Date.now();
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

function normalizeConversationType(value: string | undefined): Chat['type'] {
  return value?.toLowerCase() === 'group' ? 'group' : 'single';
}

function createFallbackConversationAvatar(conversationType: Chat['type']): string | undefined {
  return createDefaultAvatar(conversationType === 'group' ? 'group' : 'direct');
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
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

function isLocalPreviewUrl(value: string | undefined): boolean {
  return Boolean(value && /^(?:blob:|data:)/iu.test(value.trim()));
}

function pickDurableDeliveryUrl(value: string | undefined): string | undefined {
  return value && !isLocalPreviewUrl(value) ? value : undefined;
}

function resolveChatMessageType(value: unknown): Message['type'] | undefined {
  return typeof value === 'string' && CHAT_MESSAGE_TYPES.has(value as Message['type'])
    ? value as Message['type']
    : undefined;
}

function isMediaMessageType(value: Message['type']): value is SendableMediaMessageType {
  return MEDIA_MESSAGE_TYPES.has(value);
}

function isStructuredMessageType(value: Message['type']): value is SendableStructuredMessageType {
  return Object.prototype.hasOwnProperty.call(STRUCTURED_MESSAGE_SCHEMA_BY_TYPE, value);
}

function resolveDecodedMessageType(message: ImDecodedMessage): Message['type'] {
  const hintedType = resolveChatMessageType(message.renderHints?.sdkworkChatPcType);
  if (hintedType) {
    return hintedType;
  }

  switch (message.type) {
    case 'image':
    case 'video':
    case 'file':
    case 'link':
    case 'card':
    case 'music':
    case 'voice':
      return message.type;
    case 'audio':
      return 'voice';
    case 'contact':
      return 'card';
    case 'data':
    case 'signal':
    case 'stream_ref':
      return 'system';
    default:
      return 'text';
  }
}

function resolveResourceUrl(resource: ImDecodedMessage['attachments'][number]['resource'] | undefined): string | undefined {
  return pickString(resource?.publicUrl, resource?.url, resource?.uri);
}

function resolveRenditionUrl(value: unknown): string | undefined {
  const rendition = toRecord(value);
  return pickString(rendition.publicUrl, rendition.url, rendition.uri);
}

function resolveAttachmentUrl(message: ImDecodedMessage): string | undefined {
  return resolveResourceUrl(message.attachments[0]?.resource);
}

function firstMessageEntryPart(entry: ConversationMessageEntry): Record<string, unknown> {
  const parts = Array.isArray(entry.body?.parts) ? entry.body.parts : [];
  return toRecord(parts[0]);
}

function resolvePartMessageType(part: Record<string, unknown>, renderHints: Record<string, unknown>): Message['type'] {
  const hintedType = resolveChatMessageType(renderHints.sdkworkChatPcType);
  if (hintedType) {
    return hintedType;
  }

  switch (part.kind) {
    case 'media': {
      const resource = toRecord(part.resource);
      const mediaKind = pickString(resource.kind, resource.mediaKind, resource.type);
      if (mediaKind === 'image' || mediaKind === 'video' || mediaKind === 'file') {
        return mediaKind;
      }
      if (mediaKind === 'audio' || mediaKind === 'voice') {
        return 'voice';
      }
      return 'file';
    }
    case 'data':
    case 'signal':
    case 'stream_ref':
      return 'system';
    default:
      return 'text';
  }
}

function resolveMessageEntryType(entry: ConversationMessageEntry): Message['type'] {
  const renderHints = toRecord(entry.body?.renderHints);
  return resolvePartMessageType(firstMessageEntryPart(entry), renderHints);
}

function resolveMessageEntryResource(entry: ConversationMessageEntry): Record<string, unknown> {
  const part = firstMessageEntryPart(entry);
  return part.kind === 'media' ? toRecord(part.resource) : {};
}

function resolveMessageEntryResourceUrl(entry: ConversationMessageEntry): string | undefined {
  const resource = resolveMessageEntryResource(entry);
  return pickString(resource.publicUrl, resource.url, resource.uri);
}

function resolveMessageEntryContent(entry: ConversationMessageEntry, type: Message['type']): string {
  const part = firstMessageEntryPart(entry);
  const resourceUrl = resolveMessageEntryResourceUrl(entry);
  switch (type) {
    case 'image':
    case 'video':
    case 'voice':
    case 'file':
      return pickString(resourceUrl, entry.body?.summary, entry.summary) ?? '';
    default:
      return pickString(part.text, entry.body?.summary, entry.summary) ?? '';
  }
}

type RtcCallDisplayState = 'accepted' | 'ended' | 'rejected' | 'started' | 'syncing';

interface ParsedRtcCallSignal {
  nestedPayload: Record<string, unknown>;
  payload: Record<string, unknown>;
  signalType: string;
}

interface RtcCallDescriptor {
  actorId?: string;
  initiatorId?: string;
  mode?: string;
  receiverId?: string;
  signalType: string;
  state: RtcCallDisplayState;
}

const RTC_CALL_MESSAGE_ID_PREFIX = 'call:';
const RTC_CALL_DESCRIPTOR_PREFIX = 'rtc-call:';

function bodyParts(body: unknown): Record<string, unknown>[] {
  const bodyRecord = toRecord(body);
  const parts = Array.isArray(bodyRecord.parts) ? bodyRecord.parts : [];
  return parts
    .map((part) => toRecord(part))
    .filter((part) => Object.keys(part).length > 0);
}

function isRtcSignalPart(
  part: Record<string, unknown>,
  payload: Record<string, unknown>,
  nestedPayload: Record<string, unknown>,
): boolean {
  const signalType = pickString(part.signalType, payload.signalType, nestedPayload.signalType);
  return pickString(part.kind) === 'signal'
    && Boolean(pickString(payload.rtcSessionId, nestedPayload.rtcSessionId))
    && (!signalType || signalType.startsWith('rtc.') || Boolean(signalType));
}

function parseRtcCallSignals(parts: Record<string, unknown>[]): ParsedRtcCallSignal[] {
  return parts
    .map((part) => {
      const payload = parseJsonRecord(part.payload) ?? {};
      const nestedPayload = parseJsonRecord(payload.signalPayload) ?? {};
      if (!isRtcSignalPart(part, payload, nestedPayload)) {
        return undefined;
      }
      const signalType = pickString(part.signalType, payload.signalType, nestedPayload.signalType) ?? 'rtc.signal';
      return {
        nestedPayload,
        payload,
        signalType,
      };
    })
    .filter((signal): signal is ParsedRtcCallSignal => Boolean(signal));
}

function normalizeRtcCallState(signalType: string, value: unknown): RtcCallDisplayState {
  const state = pickString(value)?.toLowerCase();
  if (state === 'accepted' || state === 'connected') {
    return 'accepted';
  }
  if (state === 'rejected' || state === 'declined') {
    return 'rejected';
  }
  if (state === 'ended' || state === 'closed') {
    return 'ended';
  }
  if (state === 'started' || state === 'ringing' || state === 'invited') {
    return 'started';
  }

  switch (signalType) {
    case 'rtc.invite':
      return 'started';
    case 'rtc.accept':
      return 'accepted';
    case 'rtc.reject':
      return 'rejected';
    case 'rtc.end':
      return 'ended';
    default:
      return 'syncing';
  }
}

function isVideoRtcMode(value: string | undefined): boolean {
  return Boolean(value && /video/iu.test(value));
}

function formatRtcCallMode(value: string | undefined): string {
  return isVideoRtcMode(value)
    ? i18n.t('chat.messageList.rtcCall.videoMode')
    : i18n.t('chat.messageList.rtcCall.voiceMode');
}

function formatRtcCallParticipant(value: string | undefined, fallback: string): string {
  return value && value.trim().length > 0 ? value.trim() : fallback;
}

function resolveRtcCallDurationSeconds(signal: ParsedRtcCallSignal): number | undefined {
  const duration = pickNumber(
    signal.payload.durationSeconds,
    signal.payload.duration,
    signal.nestedPayload.durationSeconds,
    signal.nestedPayload.duration,
  );
  if (duration !== undefined) {
    return Math.max(0, Math.round(duration));
  }

  const startedAt = pickString(signal.payload.startedAt, signal.nestedPayload.startedAt);
  const endedAt = pickString(signal.payload.endedAt, signal.nestedPayload.endedAt);
  if (!startedAt || !endedAt) {
    return undefined;
  }
  const startedAtMillis = new Date(startedAt).getTime();
  const endedAtMillis = new Date(endedAt).getTime();
  if (!Number.isFinite(startedAtMillis) || !Number.isFinite(endedAtMillis) || endedAtMillis < startedAtMillis) {
    return undefined;
  }
  return Math.round((endedAtMillis - startedAtMillis) / 1000);
}

function buildRtcCallDescriptor(descriptor: RtcCallDescriptor): string {
  return `${RTC_CALL_DESCRIPTOR_PREFIX}${encodeURIComponent(JSON.stringify(descriptor))}`;
}

function readRtcCallDescriptor(message: Message): RtcCallDescriptor | undefined {
  if (message.type !== 'video_call' || !message.id.startsWith(RTC_CALL_MESSAGE_ID_PREFIX)) {
    return undefined;
  }
  const descriptor = message.desc ?? '';
  if (!descriptor.startsWith(RTC_CALL_DESCRIPTOR_PREFIX)) {
    return undefined;
  }

  const encodedDescriptor = descriptor.slice(RTC_CALL_DESCRIPTOR_PREFIX.length);
  try {
    const parsed = parseJsonRecord(decodeURIComponent(encodedDescriptor));
    const state = pickString(parsed?.state);
    const signalType = pickString(parsed?.signalType) ?? 'rtc.signal';
    if (
      state === 'accepted'
      || state === 'ended'
      || state === 'rejected'
      || state === 'started'
      || state === 'syncing'
    ) {
      return {
        actorId: pickString(parsed?.actorId),
        initiatorId: pickString(parsed?.initiatorId),
        mode: pickString(parsed?.mode),
        receiverId: pickString(parsed?.receiverId),
        signalType,
        state,
      };
    }
  } catch {
    const [state, signalType = 'rtc.signal'] = encodedDescriptor.split(':');
    if (
      state === 'accepted'
      || state === 'ended'
      || state === 'rejected'
      || state === 'started'
      || state === 'syncing'
    ) {
      return {
        signalType,
        state,
      };
    }
  }

  return undefined;
}

function resolveRtcCallDisplayState(message: Message): RtcCallDisplayState | undefined {
  return readRtcCallDescriptor(message)?.state;
}

function buildMessageNotificationVersion(message: Message): string {
  const rtcDescriptor = readRtcCallDescriptor(message);
  if (!rtcDescriptor) {
    return 'posted';
  }
  return [
    'rtc',
    message.desc ?? '',
    message.duration ?? '',
  ].join(':');
}

function shouldPreferIncomingMessage(existing: Message, incoming: Message, defaultPreference: boolean): boolean {
  const existingRtcState = resolveRtcCallDisplayState(existing);
  const incomingRtcState = resolveRtcCallDisplayState(incoming);
  if (existingRtcState || incomingRtcState) {
    if (incomingRtcState === 'syncing' && existingRtcState && existingRtcState !== 'syncing') {
      return false;
    }
    if (existingRtcState === 'syncing' && incomingRtcState && incomingRtcState !== 'syncing') {
      return true;
    }
    return incoming.timestamp >= existing.timestamp;
  }
  return defaultPreference;
}

function mergeSameIdMessage(existing: Message, incoming: Message, preferIncoming: boolean): Message {
  const existingRtcDescriptor = readRtcCallDescriptor(existing);
  const incomingRtcDescriptor = readRtcCallDescriptor(incoming);
  if (existingRtcDescriptor && incomingRtcDescriptor) {
    const mergedDescriptor: RtcCallDescriptor = {
      ...existingRtcDescriptor,
      ...incomingRtcDescriptor,
      actorId: incomingRtcDescriptor.actorId ?? existingRtcDescriptor.actorId,
      initiatorId:
        incomingRtcDescriptor.initiatorId
        ?? existingRtcDescriptor.initiatorId
        ?? (existing.senderId !== 'system' ? existing.senderId : undefined)
        ?? (incoming.senderId !== 'system' ? incoming.senderId : undefined),
      mode: incomingRtcDescriptor.mode ?? existingRtcDescriptor.mode,
      receiverId: incomingRtcDescriptor.receiverId ?? existingRtcDescriptor.receiverId,
      signalType: incomingRtcDescriptor.signalType,
      state: incomingRtcDescriptor.state,
    };
    const merged = preferIncoming
      ? { ...existing, ...incoming }
      : { ...incoming, ...existing };
    return {
      ...merged,
      senderId: mergedDescriptor.initiatorId ?? existing.senderId,
      content: buildRtcCallMessageContent(mergedDescriptor),
      desc: buildRtcCallDescriptor(mergedDescriptor),
      ...(incoming.reactions ? { reactions: incoming.reactions } : existing.reactions ? { reactions: existing.reactions } : {}),
    };
  }

  const merged = preferIncoming
    ? { ...existing, ...incoming }
    : { ...incoming, ...existing };
  return {
    ...merged,
    ...(incoming.reactions ? { reactions: incoming.reactions } : existing.reactions ? { reactions: existing.reactions } : {}),
  };
}

function buildRtcCallMessageContent(descriptor: RtcCallDescriptor): string {
  const mode = formatRtcCallMode(descriptor.mode);
  const initiator = formatRtcCallParticipant(descriptor.initiatorId, i18n.t('chat.messageList.rtcCall.initiatorFallback'));
  const receiver = descriptor.receiverId;
  const actor = formatRtcCallParticipant(descriptor.actorId, i18n.t('chat.messageList.rtcCall.actorFallback'));
  const callSubject = receiver
    ? i18n.t('chat.messageList.rtcCall.subjectWithReceiver', { initiator, receiver, mode })
    : i18n.t('chat.messageList.rtcCall.subjectWithoutReceiver', { initiator, mode });

  switch (descriptor.state) {
    case 'accepted':
      return i18n.t('chat.messageList.rtcCall.accepted', { callSubject, actor });
    case 'rejected':
      return i18n.t('chat.messageList.rtcCall.rejected', { callSubject, actor });
    case 'ended':
      return i18n.t('chat.messageList.rtcCall.ended', { callSubject, actor });
    case 'started':
      return receiver
        ? i18n.t('chat.messageList.rtcCall.startedWithReceiver', { initiator, receiver, mode })
        : i18n.t('chat.messageList.rtcCall.startedWithoutReceiver', { initiator, mode });
    case 'syncing':
    default:
      return i18n.t('chat.messageList.rtcCall.syncing', { callSubject });
  }
}

function mapRtcSignalToCallMessage(options: {
  chatId: string;
  fallbackSenderId: string;
  parts: Record<string, unknown>[];
  timestamp: number;
}): Message | undefined {
  const signal = parseRtcCallSignals(options.parts)[0];
  if (!signal) {
    return undefined;
  }
  const rtcSessionId = pickString(signal.payload.rtcSessionId, signal.nestedPayload.rtcSessionId);
  if (!rtcSessionId) {
    return undefined;
  }
  const state = normalizeRtcCallState(
    signal.signalType,
    pickString(signal.payload.state, signal.nestedPayload.state),
  );
  const explicitInitiatorId = pickString(
    signal.payload.initiatorId,
    signal.nestedPayload.initiatorId,
  );
  const initiatorId = explicitInitiatorId
    ?? (signal.signalType === 'rtc.invite' ? options.fallbackSenderId : undefined);
  const descriptor: RtcCallDescriptor = {
    actorId: pickString(
      signal.payload.actorId,
      signal.payload.operatorId,
      signal.payload.senderId,
      signal.nestedPayload.actorId,
      signal.nestedPayload.operatorId,
      signal.nestedPayload.senderId,
      options.fallbackSenderId,
    ),
    initiatorId,
    mode: pickString(signal.payload.rtcMode, signal.nestedPayload.rtcMode),
    receiverId: pickString(
      signal.payload.receiverId,
      signal.payload.targetUserId,
      signal.payload.participantId,
      signal.nestedPayload.receiverId,
      signal.nestedPayload.targetUserId,
      signal.nestedPayload.participantId,
    ),
    signalType: signal.signalType,
    state,
  };
  const duration = resolveRtcCallDurationSeconds(signal);
  const senderId = initiatorId ?? options.fallbackSenderId ?? 'system';

  return {
    id: `${RTC_CALL_MESSAGE_ID_PREFIX}${rtcSessionId}`,
    chatId: pickString(signal.payload.conversationId, signal.nestedPayload.conversationId) ?? options.chatId,
    senderId,
    content: buildRtcCallMessageContent(descriptor),
    type: 'video_call',
    timestamp: options.timestamp,
    desc: buildRtcCallDescriptor(descriptor),
    ...(duration !== undefined ? { duration } : {}),
  };
}

function resolveDecodedMessageContent(message: ImDecodedMessage, type: Message['type']): string {
  const content = toRecord(message.content);
  const attachmentUrl = resolveAttachmentUrl(message);
  switch (type) {
    case 'image':
    case 'video':
    case 'voice':
    case 'file':
      return pickString(attachmentUrl, message.text, message.summary) ?? '';
    case 'link':
      return pickString(content.url, message.text, message.summary) ?? '';
    case 'music':
      return pickString(content.url, attachmentUrl, message.text, message.summary) ?? '';
    default:
      return pickString(
        message.text,
        message.summary,
        content.text,
        content.title,
        content.displayName,
        content.prompt,
      ) ?? '';
  }
}

function mapReplyReferenceToMessageReply(
  replyTo: MessageReplyReference | null | undefined,
): Message['replyTo'] | undefined {
  if (!replyTo) {
    return undefined;
  }

  return {
    id: replyTo.messageId,
    senderName: replyTo.senderDisplayName,
    content: replyTo.contentPreview,
  };
}

function buildReplyReference(replyTo: Message['replyTo'] | undefined): MessageReplyReference | undefined {
  if (!replyTo) {
    return undefined;
  }

  return {
    messageId: replyTo.id,
    senderDisplayName: replyTo.senderName,
    contentPreview: replyTo.content,
  };
}

function normalizeResourceNodeSegment(value: string): string {
  const normalized = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return normalized || 'resource';
}

const STANDARD_AGENT_ID_PATTERN = /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;

function requireStandardAgentChatId(value: string): string {
  const agentId = value.trim();
  if (!agentId) {
    throw new Error('Agent chat target id is required');
  }
  if (!STANDARD_AGENT_ID_PATTERN.test(agentId)) {
    throw new Error('Agent chat target id must use the standard agent. id format');
  }
  return agentId;
}

function isAgentDialogConversationId(conversationId: string): boolean {
  const value = conversationId.trim();
  return /^a_[a-f0-9]{24}$/u.test(value)
    || /^c_agent_[a-f0-9]{24}$/u.test(value)
    || /^pc-agent-[a-z0-9._-]+-agent[._-][a-z0-9._-]+$/iu.test(value);
}

function isAgentDialogInboxEntry(entry: ConversationInboxEntry): boolean {
  return !entry.agentHandoff
    && (
      String(entry.conversationType).trim().toLowerCase() === 'agent_dialog'
      || isAgentDialogConversationId(entry.conversationId)
    );
}

function resolveMediaKind(type: Message['type']): MediaKind {
  switch (type) {
    case 'image':
      return 'image';
    case 'video':
      return 'video';
    case 'voice':
      return 'voice';
    case 'file':
      return 'file';
    default:
      return 'document';
  }
}

async function createDefaultDriveUploaderClient(
  session: SdkworkChatSession | null = readAppSdkSessionTokens(),
): Promise<SdkworkDriveUploader> {
  const { getDriveAppSdkClientWithSession } = await import('@sdkwork/im-pc-core/sdk/driveAppSdkClient');
  const client = getDriveAppSdkClientWithSession(session ?? readAppSdkSessionTokens());
  return client.uploader;
}

function getDefaultDriveUploader(): Promise<SdkworkDriveUploader> {
  if (driveUploaderClient) {
    return Promise.resolve(driveUploaderClient);
  }
  if (!driveUploaderClientPromise) {
    const generation = driveUploaderClientGeneration;
    let uploaderPromise: Promise<SdkworkDriveUploader>;
    uploaderPromise = createDefaultDriveUploaderClient()
      .then((client) => {
        if (
          generation === driveUploaderClientGeneration
          && driveUploaderClientPromise === uploaderPromise
        ) {
          driveUploaderClient = client;
        }
        return client;
      })
      .catch((error: unknown) => {
        if (driveUploaderClientPromise === uploaderPromise) {
          driveUploaderClientPromise = null;
        }
        throw error;
      });
    driveUploaderClientPromise = uploaderPromise;
  }
  return driveUploaderClientPromise;
}

function resolveChatUploadUserId(session: SdkworkChatSession | null | undefined): string {
  const userId = resolveAppSdkUserId(session ?? null);
  if (!userId) {
    throw new Error('Chat media upload requires user_id in the authenticated session.');
  }
  return userId;
}

function parseFileSizeBytes(value: string | undefined): string | undefined {
  const normalized = value?.trim();
  if (!normalized) {
    return undefined;
  }

  const exactBytes = Number(normalized);
  if (Number.isFinite(exactBytes)) {
    return String(Math.max(0, Math.round(exactBytes)));
  }

  const match = normalized.match(/^(\d+(?:\.\d+)?)\s*(b|bytes|kb|mb|gb)$/i);
  if (!match) {
    return undefined;
  }

  const amount = Number(match[1]);
  if (!Number.isFinite(amount)) {
    return undefined;
  }

  const unit = match[2].toLowerCase();
  const multiplier = unit === 'gb'
    ? 1024 * 1024 * 1024
    : unit === 'mb'
      ? 1024 * 1024
      : unit === 'kb'
        ? 1024
        : 1;
  return String(Math.max(0, Math.round(amount * multiplier)));
}

function normalizeDriveUploadResult(result: DriveUploaderUploadResult): DriveReference {
  const spaceId = result.uploadItem.spaceId || result.uploadSession.spaceId;
  const nodeId = result.uploadItem.nodeId || result.uploadSession.nodeId;
  if (!spaceId || !nodeId) {
    throw new Error('Drive uploader result is missing spaceId or nodeId.');
  }
  return {
    driveUri: `drive://spaces/${spaceId}/nodes/${nodeId}`,
    spaceId,
    nodeId,
  };
}

function buildDriveMediaResource(
  drive: DriveReference,
  type: SendableMediaMessageType,
  extraInfo: ChatMessageExtraInfo | undefined,
  uploadResult?: DriveUploaderUploadResult,
): MediaResource {
  const mediaKind = resolveMediaKind(type);
  const uploadItem = uploadResult?.uploadItem;
  return {
    id: drive.nodeId,
    kind: mediaKind,
    source: 'drive',
    uri: drive.driveUri,
    fileName: uploadItem?.originalFileName ?? extraInfo?.fileName,
    mimeType: uploadItem?.contentType ?? extraInfo?.mimeType,
    sizeBytes: uploadItem?.contentLength ?? parseFileSizeBytes(extraInfo?.fileSize),
    durationSeconds: extraInfo?.duration,
  };
}

function buildMediaMessageParts(
  upload: ChatMediaUploadResult,
): ContentPart[] {
  return [{
    kind: 'media' as const,
    drive: upload.drive,
    resource: upload.resource,
    mediaRole: 'attachment',
  }];
}

function buildStructuredMessagePayload(
  content: string,
  type: SendableStructuredMessageType,
  extraInfo: ChatMessageExtraInfo | undefined,
): Record<string, unknown> {
  return {
    ...(extraInfo?.fileName ? { title: extraInfo.fileName } : {}),
    ...(type === 'video_call' ? { state: content } : { url: content }),
    ...(extraInfo?.desc ? { description: extraInfo.desc } : {}),
    ...(extraInfo?.appIcon ? { iconUrl: extraInfo.appIcon } : {}),
    ...(extraInfo?.coverUrl ? { coverUrl: extraInfo.coverUrl } : {}),
    ...(extraInfo?.duration ? { durationSeconds: extraInfo.duration } : {}),
  };
}

function buildStructuredMessageParts(
  content: string,
  type: SendableStructuredMessageType,
  extraInfo: ChatMessageExtraInfo | undefined,
): ContentPart[] {
  return [{
    kind: 'data' as const,
    schemaRef: STRUCTURED_MESSAGE_SCHEMA_BY_TYPE[type],
    encoding: 'application/json',
    payload: JSON.stringify(buildStructuredMessagePayload(content, type, extraInfo)),
  }];
}

function buildFallbackTextMessageParts(
  content: string,
): ContentPart[] {
  return [{
    kind: 'text' as const,
    text: content,
  }];
}

function buildMessageParts(
  content: string,
  type: Message['type'],
  extraInfo: ChatMessageExtraInfo | undefined,
  mediaUpload?: ChatMediaUploadResult,
): ContentPart[] | undefined {
  if (isMediaMessageType(type)) {
    if (!mediaUpload) {
      throw new Error('Chat media messages require Drive upload result before IM send.');
    }
    return buildMediaMessageParts(mediaUpload);
  }
  if (isStructuredMessageType(type)) {
    return buildStructuredMessageParts(content, type, extraInfo);
  }
  return buildFallbackTextMessageParts(content);
}

function buildMessageRenderHints(
  type: Message['type'],
  extraInfo: ChatMessageExtraInfo | undefined,
) {
  const coverUrl = type === 'link'
    ? pickDurableDeliveryUrl(extraInfo?.coverUrl)
    : undefined;
  return {
    sdkworkChatPcType: type,
    ...(coverUrl ? { coverUrl } : {}),
    ...(extraInfo?.fileName ? { fileName: extraInfo.fileName } : {}),
    ...(extraInfo?.fileSize ? { fileSize: extraInfo.fileSize } : {}),
    ...(extraInfo?.appIcon ? { appIcon: extraInfo.appIcon } : {}),
    ...(isStructuredMessageType(type) && extraInfo?.desc ? { desc: extraInfo.desc } : {}),
    ...(extraInfo?.duration ? { duration: String(extraInfo.duration) } : {}),
  };
}

function resolveMediaUploadProfile(type: SendableMediaMessageType): DriveUploaderProfile | undefined {
  switch (type) {
    case 'file':
      return 'attachment';
    default:
      return undefined;
  }
}

function resolveMediaUploadContentType(
  type: SendableMediaMessageType,
  file: DriveUploaderBlobLike,
  extraInfo: ChatMessageExtraInfo | undefined,
): string | undefined {
  return extraInfo?.mimeType
    ?? file.type
    ?? (type === 'voice' ? 'audio/webm' : undefined);
}

function resolveMediaUploadFileName(
  type: SendableMediaMessageType,
  file: DriveUploaderBlobLike,
  extraInfo: ChatMessageExtraInfo | undefined,
): string {
  const fallback = type === 'voice'
    ? `voice-${Date.now().toString(36)}.webm`
    : `chat-${type}-${Date.now().toString(36)}`;
  return pickString(extraInfo?.fileName, file.name, fallback) ?? fallback;
}

async function uploadChatMediaFile({
  chatId,
  content,
  extraInfo,
  getDriveUploader,
  getSession,
  type,
}: {
  chatId: string;
  content: string;
  extraInfo: ChatMessageExtraInfo | undefined;
  getDriveUploader: () => Promise<SdkworkDriveUploader> | SdkworkDriveUploader;
  getSession: () => SdkworkChatSession | null;
  type: SendableMediaMessageType;
}): Promise<ChatMediaUploadResult> {
  const file = extraInfo?.file;
  if (!file) {
    throw new Error('Chat media messages require a File or Blob before sending.');
  }

  const session = getSession();
  const organizationId = resolveAppSdkOrganizationId(session ?? null);
  resolveChatUploadUserId(session);
  const originalFileName = resolveMediaUploadFileName(type, file, extraInfo);
  const uploadRequest: DriveUploaderRequest = {
    file,
    ...(organizationId ? { organizationId } : {}),
    appResourceType: CHAT_DRIVE_APP_RESOURCE_TYPE,
    appResourceId: chatId,
    scene: CHAT_DRIVE_SCENE,
    source: CHAT_DRIVE_SOURCE,
    ...(resolveMediaUploadProfile(type) ? { uploadProfileCode: resolveMediaUploadProfile(type) } : {}),
    originalFileName,
    ...(resolveMediaUploadContentType(type, file, extraInfo) ? { contentType: resolveMediaUploadContentType(type, file, extraInfo) } : {}),
  };

  const uploader = await getDriveUploader();
  const uploadResult = type === 'image'
    ? await uploader.uploadImage(uploadRequest)
    : type === 'voice'
      ? await uploader.uploadAudio(uploadRequest)
      : type === 'video'
        ? await uploader.uploadVideo(uploadRequest)
        : await uploader.uploadAttachment(uploadRequest);
  const drive = normalizeDriveUploadResult(uploadResult);
  const resource = buildDriveMediaResource(drive, type, {
    ...extraInfo,
    fileName: originalFileName,
    mimeType: uploadRequest.contentType,
  }, uploadResult);
  return {
    content: pickString(content, drive.driveUri) ?? drive.driveUri,
    drive,
    resource,
  };
}

function mapLiveMessageToMessage(
  fallbackChatId: string,
  decodedMessage: ImDecodedMessage,
  context: ImMessageContext,
): Message {
  const payload = toRecord(context.payload);
  const rawEvent = toRecord(context.rawEvent);
  const content = toRecord(decodedMessage.content);
  const resource = decodedMessage.attachments[0]?.resource;
  const messageId = pickString(
    context.messageId,
    payload.messageId,
    rawEvent.eventId,
  ) ?? `${fallbackChatId}:${context.sequence}`;
  const conversationId = pickString(context.conversationId, payload.conversationId) ?? fallbackChatId;
  const senderId = pickString(
    context.sender?.principalId,
    context.sender?.id,
    decodedMessage.sender?.id,
  ) ?? 'system';
  const timestamp = parseTimestamp(context.receivedAt);
  const rtcCallMessage = mapRtcSignalToCallMessage({
    chatId: conversationId,
    fallbackSenderId: senderId,
    parts: [
      ...bodyParts(decodedMessage.body),
      ...bodyParts(payload.body),
    ],
    timestamp,
  });
  if (rtcCallMessage) {
    return rtcCallMessage;
  }

  const type = resolveDecodedMessageType(decodedMessage);
  const renderHints = decodedMessage.renderHints ?? {};
  const duration = pickNumber(renderHints.duration, renderHints.durationSeconds, content.durationSeconds, resource?.durationSeconds);
  const coverUrl = pickString(
    renderHints.coverUrl,
    content.coverUrl,
    content.imageUrl,
    resolveRenditionUrl(resource?.poster),
    resolveRenditionUrl(resource?.thumbnails?.[0]),
  );
  const fileName = pickString(
    renderHints.fileName,
    content.title,
    content.displayName,
    resource?.fileName,
    resource?.title,
  );
  const replyTo = mapReplyReferenceToMessageReply(decodedMessage.replyTo);

  return {
    id: messageId,
    chatId: conversationId,
    senderId,
    content: resolveDecodedMessageContent(decodedMessage, type),
    type,
    timestamp,
    ...(coverUrl ? { coverUrl } : {}),
    ...(duration ? { duration } : {}),
    ...(fileName ? { fileName } : {}),
    ...(pickString(renderHints.fileSize, resource?.sizeBytes) ? { fileSize: pickString(renderHints.fileSize, resource?.sizeBytes) } : {}),
    ...(pickString(renderHints.appIcon, content.avatarUrl, content.imageUrl) ? { appIcon: pickString(renderHints.appIcon, content.avatarUrl, content.imageUrl) } : {}),
    ...(pickString(renderHints.desc, content.description, content.subtitle, content.artist) ? { desc: pickString(renderHints.desc, content.description, content.subtitle, content.artist) } : {}),
    ...(replyTo ? { replyTo } : {}),
  };
}

function buildConversationName(entry: ConversationInboxEntry): string {
  const entryRecord = toRecord(entry);
  const displayName = pickString(entryRecord.displayName, entryRecord.display_name);
  if (displayName) {
    return displayName;
  }
  if (entry.agentHandoff) {
    return 'Support conversation';
  }
  if (isAgentDialogConversationId(entry.conversationId)) {
    return 'AI assistant chat';
  }
  return normalizeConversationType(entry.conversationType) === 'group'
    ? 'Group chat'
    : 'Direct chat';
}

function buildFallbackConversationName(conversationType: Chat['type']): string {
  return conversationType === 'group' ? 'Group chat' : 'Direct chat';
}

function createChatClientRequestKey(): string {
  const clientGeneratedId =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `pc-create-chat-${clientGeneratedId}`;
}

function buildLastMessage(entry: ConversationInboxEntry, timestamp: number): Message | undefined {
  if (!entry.lastMessageId && !entry.lastSummary) {
    return undefined;
  }

  return {
    id: entry.lastMessageId ?? `${entry.conversationId}:${entry.lastMessageSeq}`,
    chatId: entry.conversationId,
    senderId: entry.lastSenderId ?? 'system',
    content: entry.lastSummary ?? '',
    type: 'text',
    timestamp,
  };
}

function mapLiveEventToMessage(context: ImRealtimeEventContext): Message | undefined {
  const payload = toRecord(context.payload);
  const rawEvent = toRecord(context.rawEvent);
  const payloadBody = toRecord(payload.body);
  const bodyPartsValue = Array.isArray(payloadBody.parts) ? payloadBody.parts : [];
  const payloadSender = toRecord(payload.sender);
  const sender = pickString(payloadSender.id)
    ? {
        id: pickString(payloadSender.id) ?? '',
        kind: pickString(payloadSender.kind) ?? 'user',
        metadata: toRecord(payloadSender.metadata),
      }
    : undefined;
  const conversationId = pickString(
    payload.conversationId,
    rawEvent.conversationId,
    rawEvent.scopeType === 'conversation' ? rawEvent.scopeId : undefined,
  );
  if (!conversationId) {
    return undefined;
  }

  return mapLiveMessageToMessage(
    conversationId,
    {
      attachments: [],
      body: {
        parts: bodyPartsValue,
        renderHints: toRecord(payloadBody.renderHints),
        summary: pickString(payloadBody.summary),
      },
      conversationId,
      messageId: pickString(payload.messageId, context.eventId),
      messageSeq: pickNumber(payload.messageSeq, payload.sequence, context.sequence),
      messageType: pickString(payload.messageType, payload.type) as ImDecodedMessage['messageType'],
      occurredAt: pickString(payload.occurredAt, context.receivedAt),
      renderHints: toRecord(payloadBody.renderHints),
      sender,
      summary: pickString(payload.summary, payloadBody.summary),
      text: pickString(payload.text, payload.summary, payloadBody.summary),
      type: pickString(payload.type, payload.messageType),
    },
    {
      ack: context.ack,
      conversationId,
      eventId: context.eventId,
      eventType: context.eventType,
      messageId: pickString(payload.messageId, context.eventId),
      payload,
      rawEvent: context.rawEvent,
      receivedAt: context.receivedAt,
      sender,
      sequence: context.sequence,
    },
  );
}

function mapInboxEntryToChat(entry: ConversationInboxEntry, viewState: ConversationViewState | undefined): Chat {
  const updatedAt = parseTimestamp(entry.lastActivityAt);
  const conversationType = viewState?.type ?? normalizeConversationType(entry.conversationType);
  return {
    id: entry.conversationId,
    name: viewState?.name ?? buildConversationName(entry),
    avatar: viewState?.avatar ?? createFallbackConversationAvatar(conversationType),
    type: conversationType,
    unreadCount: entry.unreadCount,
    updatedAt,
    activeCount: viewState?.activeCount,
    memberCount: viewState?.memberCount,
    members: viewState?.members,
    isMarkedUnread: viewState?.isMarkedUnread,
    isMuted: viewState?.isMuted,
    isPinned: viewState?.isPinned,
    notice: viewState?.notice,
    welcomeMessage: viewState?.welcomeMessage,
    lastMessage: buildLastMessage(entry, updatedAt),
  };
}

function applyInboxProjectionToViewState(
  viewState: ConversationViewState | undefined,
  entry: ConversationInboxEntry,
): ConversationViewState | undefined {
  const entryRecord = toRecord(entry);
  const peerRecord = toRecord(entryRecord.peer);
  const projectedPreferences = toRecord(entryRecord.preferences);
  const projectedName = pickString(entryRecord.displayName, entryRecord.display_name)
    ?? (normalizeConversationType(entry.conversationType) === 'single'
      ? pickString(peerRecord.displayName, peerRecord.display_name)
      : undefined);
  const projectedAvatar = pickString(entryRecord.avatarUrl, entryRecord.avatar_url);
  const hasProjection = projectedName
    || projectedAvatar
    || Object.keys(projectedPreferences).length > 0;
  if (!hasProjection) {
    return viewState;
  }

  return {
    ...viewState,
    ...(projectedName ? { name: projectedName } : {}),
    ...(projectedAvatar ? { avatar: projectedAvatar } : {}),
    ...(typeof projectedPreferences.isPinned === 'boolean' ? { isPinned: projectedPreferences.isPinned } : {}),
    ...(typeof projectedPreferences.isMuted === 'boolean' ? { isMuted: projectedPreferences.isMuted } : {}),
    ...(typeof projectedPreferences.isMarkedUnread === 'boolean'
      ? { isMarkedUnread: projectedPreferences.isMarkedUnread }
      : {}),
    ...(viewState?.isHidden === true
      ? { isHidden: true }
      : typeof projectedPreferences.isHidden === 'boolean'
        ? { isHidden: projectedPreferences.isHidden }
        : {}),
    type: normalizeConversationType(entry.conversationType),
  };
}

function mapLocalMessageToChat(message: Message, viewState: ConversationViewState | undefined): Chat {
  const conversationType = viewState?.type ?? 'single';
  return {
    id: message.chatId,
    name: viewState?.name ?? buildFallbackConversationName(conversationType),
    avatar: viewState?.avatar ?? createFallbackConversationAvatar(conversationType),
    type: conversationType,
    unreadCount: viewState?.isMarkedUnread ? 1 : 0,
    updatedAt: message.timestamp,
    activeCount: viewState?.activeCount,
    memberCount: viewState?.memberCount,
    members: viewState?.members,
    isMarkedUnread: viewState?.isMarkedUnread,
    isMuted: viewState?.isMuted,
    isPinned: viewState?.isPinned,
    notice: viewState?.notice,
    welcomeMessage: viewState?.welcomeMessage,
    lastMessage: message,
  };
}

function applyLocalLastMessageToChat(chat: Chat, localLastMessage: Message | undefined): Chat {
  if (!localLastMessage) {
    return chat;
  }
  if (chat.lastMessage && chat.lastMessage.timestamp > localLastMessage.timestamp) {
    return chat;
  }
  return {
    ...chat,
    lastMessage: localLastMessage,
    updatedAt: Math.max(chat.updatedAt, localLastMessage.timestamp),
  };
}

function applyConversationProfile(
  viewState: ConversationViewState | undefined,
  profile: { avatarUrl?: string; displayName?: string; notice?: string },
): ConversationViewState {
  return {
    ...viewState,
    ...(pickString(profile.displayName) ? { name: pickString(profile.displayName) } : {}),
    ...(pickString(profile.avatarUrl) ? { avatar: pickString(profile.avatarUrl) } : {}),
    notice: profile.notice,
  };
}

function buildConversationProfileUpdate(updates: Partial<Chat>): UpdateConversationProfileRequest {
  return {
    ...(updates.avatar !== undefined ? { avatarUrl: updates.avatar } : {}),
    ...(updates.name !== undefined ? { displayName: updates.name } : {}),
    ...(updates.notice !== undefined ? { notice: updates.notice } : {}),
  };
}

function hasProfileUpdate(update: UpdateConversationProfileRequest): boolean {
  return update.avatarUrl !== undefined
    || update.displayName !== undefined
    || update.notice !== undefined;
}

function buildLocalConversationViewUpdate(updates: Partial<Chat>): ConversationViewState {
  return {
    ...(updates.activeCount !== undefined ? { activeCount: updates.activeCount } : {}),
    ...(updates.isMuted !== undefined ? { isMuted: updates.isMuted } : {}),
    ...(updates.isMarkedUnread !== undefined ? { isMarkedUnread: updates.isMarkedUnread } : {}),
    ...(updates.isPinned !== undefined ? { isPinned: updates.isPinned } : {}),
    ...(updates.memberCount !== undefined ? { memberCount: updates.memberCount } : {}),
    ...(updates.members !== undefined ? { members: updates.members } : {}),
    ...(updates.type !== undefined ? { type: updates.type } : {}),
  };
}

function mapConversationMessageEntryToMessage(
  entry: ConversationMessageEntry,
  index: number,
  total: number,
  cachedMessage?: Message,
): Message {
  const timestamp = parseTimestamp(entry.committedAt ?? entry.occurredAt) || Date.now() - Math.max(total - index, 0) * 1000;
  const senderId = pickString(entry.sender?.id) ?? 'system';
  const rtcCallMessage = mapRtcSignalToCallMessage({
    chatId: entry.conversationId,
    fallbackSenderId: senderId,
    parts: bodyParts(entry.body),
    timestamp,
  });
  if (rtcCallMessage) {
    return cachedMessage
      ? mergeSameIdMessage(cachedMessage, rtcCallMessage, shouldPreferIncomingMessage(cachedMessage, rtcCallMessage, true))
      : rtcCallMessage;
  }
  if (cachedMessage) {
    return cachedMessage;
  }

  const type = resolveMessageEntryType(entry);
  const renderHints = toRecord(entry.body?.renderHints);
  const resource = resolveMessageEntryResource(entry);
  const coverUrl = pickString(
    renderHints.coverUrl,
    resolveRenditionUrl(resource.poster),
    resolveRenditionUrl(Array.isArray(resource.thumbnails) ? resource.thumbnails[0] : undefined),
  );
  const fileName = pickString(
    renderHints.fileName,
    resource.fileName,
    resource.title,
  );
  const duration = pickNumber(renderHints.duration, renderHints.durationSeconds, resource.durationSeconds);
  const replyTo = mapReplyReferenceToMessageReply(entry.body?.replyTo);

  return {
    id: entry.messageId,
    chatId: entry.conversationId,
    senderId,
    content: resolveMessageEntryContent(entry, type),
    type,
    timestamp,
    ...(coverUrl ? { coverUrl } : {}),
    ...(duration ? { duration } : {}),
    ...(fileName ? { fileName } : {}),
    ...(pickString(renderHints.fileSize, resource.sizeBytes) ? { fileSize: pickString(renderHints.fileSize, resource.sizeBytes) } : {}),
    ...(replyTo ? { replyTo } : {}),
  };
}

function sortChats(left: Chat, right: Chat): number {
  if (left.isPinned !== right.isPinned) {
    return left.isPinned ? -1 : 1;
  }
  return right.updatedAt - left.updatedAt;
}

function mergeMessageLists(remoteMessages: Message[], localMessages: Message[]): Message[] {
  const byId = new Map<string, Message>();
  for (const message of remoteMessages) {
    const existing = byId.get(message.id);
    byId.set(
      message.id,
      existing
        ? mergeSameIdMessage(existing, message, shouldPreferIncomingMessage(existing, message, true))
        : message,
    );
  }
  for (const message of localMessages) {
    const existing = byId.get(message.id);
    if (!existing) {
      byId.set(message.id, message);
      continue;
    }
    byId.set(message.id, mergeSameIdMessage(existing, message, shouldPreferIncomingMessage(existing, message, false)));
  }
  return Array.from(byId.values()).sort((left, right) => left.timestamp - right.timestamp);
}

async function getDefaultImSdkClient(): Promise<ImSdkClient> {
  const { getImSdkClientWithSession } = await import('@sdkwork/im-pc-core/sdk/imSdkClient');
  return getImSdkClientWithSession();
}

configurePcRealtimeConnectionManager({
  getClient: getDefaultImSdkClient,
  getDeviceId: resolveSdkworkChatPcClientId,
  getSession: readAppSdkSessionTokens,
});

interface MessageHistoryPaginationState {
  hasMore: boolean;
  nextCursor?: string;
}

interface AuthSessionOperationContext {
  client: ImSdkClient;
  generation: number;
  operation: string;
}

function normalizeMessageHistoryPagination(
  pageInfo: { hasMore?: boolean; nextCursor?: string | null } | undefined,
  acceptedMessageCount: number,
): MessageHistoryPaginationState {
  const page = readSdkCursorPageInfo(pageInfo);
  const nextCursor = page.nextCursor?.trim();
  const hasNextPage = page.hasMore && acceptedMessageCount > 0 && Boolean(nextCursor);
  return {
    hasMore: hasNextPage,
    ...(hasNextPage && nextCursor ? { nextCursor } : {}),
  };
}

class SdkworkChatService implements ChatService {
  private activeMessageHistoryLoads = new Set<Promise<Message[]>>();
  private authSessionGeneration = 0;
  private readonly chatListHandlers = new Set<ChatListHandler>();
  private conversationCacheTokens = new Map<string, ConversationCacheToken>();
  private conversationViewState = new Map<string, ConversationViewState>();
  private conversationWireUnsubs = new Map<string, () => void>();
  private chatListRefreshPromise?: Promise<void>;
  private chatListRefreshPending = false;
  private chatListCoalesceTimer?: ReturnType<typeof setTimeout>;
  private chatListCoalescePromise?: Promise<void>;
  private chatListCoalesceResolve?: () => void;
  private inboxFirstPagePromise?: Promise<ChatListPage>;
  private inboxFirstPagePromiseGeneration?: number;
  private inboxFirstPagePromisePageSize?: number;
  private inboxFirstPagePromises = new Map<number, {
    generation: number;
    promise: Promise<ChatListPage>;
  }>();
  private inboxFirstPageCache?: {
    expiresAt: number;
    generation: number;
    pageSize: number;
    promise: Promise<ChatListPage>;
  };
  private inboxFirstPageCaches = new Map<number, {
    expiresAt: number;
    generation: number;
    pageSize: number;
    promise: Promise<ChatListPage>;
  }>();
  private getMessagesPromises = new Map<string, Promise<Message[]>>();
  private loadMoreMessagesPromises = new Map<string, Promise<Message[]>>();
  private liveInboxWireUnsub?: () => void;
  private liveSubscriptions = new Map<string, ConversationLiveSubscription>();
  private lastChatListSnapshot: Chat[] = [];
  private localConversationCacheRecency = new Map<string, undefined>();
  private localMessages = new Map<string, Message[]>();
  private latestReadSeq = new Map<string, number>();
  private messageHistoryPaginationState = new Map<string, MessageHistoryPaginationState>();
  private pendingRealtimeReadCursorSeqs = new Map<string, number>();
  private readCursorInFlightCounts = new Map<string, number>();
  private realtimeReadCursorSyncPromise?: Promise<void>;
  private focusedConversationId?: string;
  private nextConversationCacheTokenSerial = 0;
  private windowFocused = true;
  private readonly getClient: ImSdkClientProvider;
  private readonly getDriveUploader: () => Promise<SdkworkDriveUploader> | SdkworkDriveUploader;
  private readonly getSession: () => SdkworkChatSession | null;

  private readonly handleAuthSessionChanged = (): void => {
    this.authSessionGeneration += 1;
    invalidateDefaultDriveUploader();
    this.conversationCacheTokens.clear();
    this.localConversationCacheRecency.clear();
    this.localMessages.clear();
    this.latestReadSeq.clear();
    this.conversationViewState.clear();
    this.messageHistoryPaginationState.clear();
    this.getMessagesPromises.clear();
    this.loadMoreMessagesPromises.clear();
    this.pendingRealtimeReadCursorSeqs.clear();
    this.readCursorInFlightCounts.clear();
    this.realtimeReadCursorSyncPromise = undefined;
    this.focusedConversationId = undefined;
    this.lastChatListSnapshot = [];
    this.inboxFirstPagePromise = undefined;
    this.inboxFirstPagePromiseGeneration = undefined;
    this.inboxFirstPagePromisePageSize = undefined;
    this.inboxFirstPagePromises.clear();
    this.inboxFirstPageCache = undefined;
    this.inboxFirstPageCaches.clear();
    this.chatListRefreshPromise = undefined;
    this.chatListRefreshPending = false;
    if (this.chatListCoalesceTimer !== undefined) {
      clearTimeout(this.chatListCoalesceTimer);
      this.chatListCoalesceTimer = undefined;
    }
    const settleCoalescedRefresh = this.chatListCoalesceResolve;
    this.chatListCoalescePromise = undefined;
    this.chatListCoalesceResolve = undefined;
    settleCoalescedRefresh?.();
    this.closeAllLiveSubscriptions('auth session changed');
    for (const handler of this.chatListHandlers) {
      try {
        handler([]);
      } catch {
        // A failing view subscriber must not interrupt auth cache isolation.
      }
    }
    if (this.getSession()) {
      this.syncLiveSessionSubscriptions();
    }
  };

  private handleRealtimeAuthenticationFailure(reason: string): void {
    this.closeAllLiveSubscriptions(reason);
  }

  constructor(dependencies: ChatServiceDependencies | ImSdkClientProvider = {}) {
    if (typeof dependencies === 'function') {
      this.getClient = dependencies;
      this.getDriveUploader = getDefaultDriveUploader;
      this.getSession = readAppSdkSessionTokens;
    } else {
      this.getClient = dependencies.getClient ?? getDefaultImSdkClient;
      this.getDriveUploader = dependencies.getDriveUploader ?? getDefaultDriveUploader;
      this.getSession = dependencies.getSession ?? readAppSdkSessionTokens;
    }
    configurePcRealtimeConnectionManager({
      getClient: this.getClient,
      getDeviceId: resolveSdkworkChatPcClientId,
      getSession: this.getSession,
    });
    if (typeof window !== 'undefined') {
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, this.handleAuthSessionChanged);
    }
    onPcLiveConnectionOpen(() => {
      void this.handleConnectionOpen();
    });
    onPcLiveAuthenticationFailure((reason) => {
      this.handleRealtimeAuthenticationFailure(reason);
    });
  }

  private async client(): Promise<ImSdkClient> {
    return this.getClient();
  }

  private isAuthSessionGenerationCurrent(generation: number): boolean {
    return this.authSessionGeneration === generation;
  }

  private assertAuthSessionGenerationCurrent(generation: number, operation: string): void {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      throw new Error(`Chat session changed while ${operation}.`);
    }
  }

  private async beginAuthSessionOperation(
    operation: string,
  ): Promise<AuthSessionOperationContext> {
    const generation = this.authSessionGeneration;
    const client = await this.client();
    this.assertAuthSessionGenerationCurrent(generation, operation);
    return { client, generation, operation };
  }

  private configureRealtimeConnectionManager(): void {
    configurePcRealtimeConnectionManager({
      getClient: this.getClient,
      getDeviceId: resolveSdkworkChatPcClientId,
      getSession: this.getSession,
    });
  }

  private resolveChatListRealtimeUserId(): string | undefined {
    return resolveAppSdkUserId(this.getSession())
      ?? contactService.getCurrentUser().id;
  }

  private resolveCurrentUserId(): string {
    return resolveAppSdkUserId(this.getSession())
      ?? contactService.getCurrentUser().id;
  }

  private resolveCurrentUserIdentifiers(): Set<string> {
    const session = this.getSession();
    const sessionUserRecord = toRecord(session?.user);
    const sessionContextRecord = toRecord(session?.context);
    const currentUser = contactService.getCurrentUser();
    return new Set([
      resolveAppSdkUserId(session),
      pickString(sessionUserRecord.userId, sessionUserRecord.id),
      pickString(sessionUserRecord.chatId, sessionUserRecord.chat_id),
      pickString(sessionContextRecord.userId, sessionContextRecord.user_id),
      pickString(sessionContextRecord.chatId, sessionContextRecord.chat_id),
      currentUser.id,
      currentUser.chatId,
    ].filter((identifier): identifier is string => Boolean(identifier)));
  }

  private isLocalConversationCacheProtected(chatId: string): boolean {
    return this.liveSubscriptions.has(chatId)
      || this.getMessagesPromises.has(chatId)
      || this.loadMoreMessagesPromises.has(chatId)
      || this.pendingRealtimeReadCursorSeqs.has(chatId)
      || this.readCursorInFlightCounts.has(chatId)
      || this.isConversationHidden(chatId)
      || this.focusedConversationId === chatId;
  }

  private ensureConversationCacheToken(chatId: string): ConversationCacheToken {
    const existing = this.conversationCacheTokens.get(chatId);
    if (existing) {
      return existing;
    }
    const token = { serial: this.nextConversationCacheTokenSerial += 1 };
    this.conversationCacheTokens.set(chatId, token);
    return token;
  }

  private isConversationCacheTokenCurrent(
    chatId: string,
    token: ConversationCacheToken,
  ): boolean {
    return this.conversationCacheTokens.get(chatId) === token;
  }

  private isConversationHidden(chatId: string): boolean {
    return this.conversationViewState.get(chatId)?.isHidden === true;
  }

  private assertMessageHistoryLoadCapacity(): void {
    if (this.activeMessageHistoryLoads.size >= MAX_CONCURRENT_MESSAGE_HISTORY_LOADS) {
      throw new RangeError(
        `Concurrent message history load limit (${MAX_CONCURRENT_MESSAGE_HISTORY_LOADS}) reached.`,
      );
    }
  }

  private hasLocalConversationCacheState(chatId: string): boolean {
    return this.conversationCacheTokens.has(chatId)
      || this.localMessages.has(chatId)
      || this.latestReadSeq.has(chatId)
      || this.messageHistoryPaginationState.has(chatId)
      || this.conversationViewState.has(chatId)
      || this.pendingRealtimeReadCursorSeqs.has(chatId);
  }

  private touchLocalConversationCache(chatId: string): void {
    this.localConversationCacheRecency.delete(chatId);
    this.localConversationCacheRecency.set(chatId, undefined);
  }

  private touchLocalConversationCacheIfPresent(chatId: string): void {
    if (this.hasLocalConversationCacheState(chatId)) {
      this.touchLocalConversationCache(chatId);
    }
  }

  private recordLocalConversationCacheWrite(chatId: string): void {
    this.ensureConversationCacheToken(chatId);
    this.touchLocalConversationCache(chatId);
    this.trimLocalConversationCache();
  }

  private writeConversationViewState(chatId: string, state: ConversationViewState): void {
    this.conversationViewState.set(chatId, state);
    this.recordLocalConversationCacheWrite(chatId);
  }

  private writeLatestReadSeq(chatId: string, readSeq: number): void {
    this.latestReadSeq.set(chatId, readSeq);
    this.recordLocalConversationCacheWrite(chatId);
  }

  private writeMessageHistoryPaginationState(
    chatId: string,
    state: MessageHistoryPaginationState,
  ): void {
    this.messageHistoryPaginationState.set(chatId, state);
    this.recordLocalConversationCacheWrite(chatId);
  }

  private writePendingRealtimeReadCursorSeq(chatId: string, readSeq: number): void {
    this.pendingRealtimeReadCursorSeqs.set(chatId, readSeq);
    this.recordLocalConversationCacheWrite(chatId);
  }

  private beginReadCursorSync(chatId: string): void {
    this.readCursorInFlightCounts.set(
      chatId,
      (this.readCursorInFlightCounts.get(chatId) ?? 0) + 1,
    );
    this.touchLocalConversationCacheIfPresent(chatId);
    this.trimLocalConversationCache();
  }

  private endReadCursorSync(chatId: string): void {
    const remaining = (this.readCursorInFlightCounts.get(chatId) ?? 1) - 1;
    if (remaining > 0) {
      this.readCursorInFlightCounts.set(chatId, remaining);
    } else {
      this.readCursorInFlightCounts.delete(chatId);
    }
    this.trimLocalConversationCache();
  }

  private evictLocalConversationCache(chatId: string): void {
    this.conversationCacheTokens.delete(chatId);
    this.localConversationCacheRecency.delete(chatId);
    this.localMessages.delete(chatId);
    this.latestReadSeq.delete(chatId);
    this.messageHistoryPaginationState.delete(chatId);
    this.conversationViewState.delete(chatId);
    this.pendingRealtimeReadCursorSeqs.delete(chatId);
    this.pruneLiveSubscriptionNotificationVersions(chatId);
  }

  private trimLocalConversationCache(): void {
    while (this.localConversationCacheRecency.size > LOCAL_CONVERSATION_CACHE_CAP) {
      let evictionCandidate: string | undefined;
      for (const chatId of this.localConversationCacheRecency.keys()) {
        if (!this.isLocalConversationCacheProtected(chatId)) {
          evictionCandidate = chatId;
          break;
        }
      }
      if (!evictionCandidate) {
        for (const chatId of this.localConversationCacheRecency.keys()) {
          if (chatId !== this.focusedConversationId) {
            evictionCandidate = chatId;
            break;
          }
        }
      }
      evictionCandidate ??= this.localConversationCacheRecency.keys().next().value;
      if (!evictionCandidate) {
        return;
      }
      this.evictLocalConversationCache(evictionCandidate);
    }
  }

  private mergeLiveLocalChats(chats: Chat[]): Chat[] {
    const byId = new Map(chats.map((chat) => [chat.id, chat]));
    for (const [chatId, localMessages] of this.localMessages.entries()) {
      const viewState = this.conversationViewState.get(chatId);
      if (viewState?.isHidden) {
        continue;
      }
      const lastMessage = localMessages.at(-1);
      if (!lastMessage) {
        continue;
      }
      const existingChat = byId.get(chatId);
      byId.set(
        chatId,
        existingChat
          ? applyLocalLastMessageToChat(existingChat, lastMessage)
          : mapLocalMessageToChat(lastMessage, viewState),
      );
    }
    return Array.from(byId.values())
      .sort(sortChats)
      .slice(0, MAX_INBOX_CONVERSATIONS);
  }

  private notifyChatListHandlers(chats: Chat[]): void {
    this.lastChatListSnapshot = chats;
    for (const handler of this.chatListHandlers) {
      handler(chats);
    }
  }

  private emitLocalChatListSnapshot(): void {
    if (this.chatListHandlers.size === 0) {
      return;
    }
    this.notifyChatListHandlers(this.mergeLiveLocalChats(this.lastChatListSnapshot));
  }

  private buildCachedConversationChat(chatId: string): Chat | undefined {
    const viewState = this.conversationViewState.get(chatId);
    if (viewState?.isHidden) {
      return undefined;
    }
    const lastMessage = this.localMessages.get(chatId)?.at(-1);
    if (lastMessage) {
      return mapLocalMessageToChat(lastMessage, viewState);
    }
    if (!viewState) {
      return undefined;
    }
    const conversationType = viewState.type ?? 'single';
    return {
      id: chatId,
      name: viewState.name ?? buildFallbackConversationName(conversationType),
      avatar: viewState.avatar ?? createFallbackConversationAvatar(conversationType),
      type: conversationType,
      unreadCount: viewState.isMarkedUnread ? 1 : 0,
      updatedAt: Date.now(),
      activeCount: viewState.activeCount,
      memberCount: viewState.memberCount,
      members: viewState.members,
      isMarkedUnread: viewState.isMarkedUnread,
      isMuted: viewState.isMuted,
      isPinned: viewState.isPinned,
      notice: viewState.notice,
      welcomeMessage: viewState.welcomeMessage,
    };
  }

  private async syncConversationMembers(
    conversationId: string,
    auth: AuthSessionOperationContext,
  ): Promise<ConversationMember[]> {
    const members: ConversationMember[] = [];
    await forEachCursorPage(
      async (cursor) => {
        this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
        const response = await auth.client.conversations.listMembers(conversationId, {
          pageSize: CONVERSATION_MEMBERS_PAGE_LIMIT,
          ...(cursor ? { cursor } : {}),
        });
        this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
        const page = readSdkCursorPageInfo(response.pageInfo);
        return {
          items: response.items,
          hasMore: page.hasMore,
          nextCursor: page.nextCursor,
        };
      },
      (items) => {
        members.push(...items);
      },
      { maxItems: MAX_CONVERSATION_MEMBERS_SYNC },
    );
    return members;
  }

  private isJoinedAgentMember(member: ConversationMember, agentId: string): boolean {
    const state = String(member.state).trim().toLowerCase();
    return member.principalKind === 'agent'
      && member.principalId === agentId
      && state !== 'left'
      && state !== 'removed';
  }

  private async inboxEntryContainsAgent(
    entry: ConversationInboxEntry,
    agentId: string,
    auth: AuthSessionOperationContext,
  ): Promise<boolean> {
    try {
      const members = await this.syncConversationMembers(entry.conversationId, auth);
      return members.some((member) => this.isJoinedAgentMember(member, agentId));
    } catch {
      this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
      return false;
    }
  }

  private readInboxPageInfo(
    response: { hasMore?: boolean; nextCursor?: string | null; pageInfo?: { hasMore?: boolean; nextCursor?: string | null } },
  ): Pick<ChatListPage, 'hasMore' | 'nextCursor'> {
    if (response.pageInfo) {
      return readSdkCursorPageInfo(response.pageInfo);
    }
    const hasMore = response.hasMore === true;
    return {
      hasMore,
      nextCursor: hasMore ? (response.nextCursor ?? undefined) : undefined,
    };
  }

  private async findExistingAgentDialogEntry(
    agentId: string,
    auth: AuthSessionOperationContext,
  ): Promise<ConversationInboxEntry | undefined> {
    let cursor: string | undefined;
    let scanned = 0;

    do {
      this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
      const response = await auth.client.chat.inbox.list({
        pageSize: INBOX_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
      const entries = response.items ?? [];
      for (const entry of entries) {
        scanned += 1;
        if (
          isAgentDialogInboxEntry(entry)
          && await this.inboxEntryContainsAgent(entry, agentId, auth)
        ) {
          return entry;
        }
        if (scanned >= MAX_INBOX_CONVERSATIONS) {
          return undefined;
        }
      }
      const page = this.readInboxPageInfo(response);
      cursor = page.hasMore ? page.nextCursor : undefined;
    } while (cursor);

    return undefined;
  }

  private rememberAgentConversation(
    conversationId: string,
    agent: Pick<Chat, 'avatar' | 'name' | 'welcomeMessage'>,
    baseViewState?: ConversationViewState,
  ): ConversationViewState {
    const viewState = {
      ...baseViewState,
      avatar: agent.avatar,
      isHidden: false,
      name: agent.name,
      type: 'single' as const,
      welcomeMessage: agent.welcomeMessage,
    };
    this.writeConversationViewState(conversationId, viewState);
    return viewState;
  }

  private async syncAgentConversationPresentation(
    conversationId: string,
    agent: Pick<Chat, 'avatar' | 'name'>,
    auth: AuthSessionOperationContext,
  ): Promise<void> {
    const profileUpdate = buildConversationProfileUpdate({
      avatar: agent.avatar,
      name: agent.name,
    });
    try {
      this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
      if (hasProfileUpdate(profileUpdate)) {
        await auth.client.conversations.updateProfile(conversationId, profileUpdate);
        this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
      }
      await auth.client.conversations.updatePreferences(conversationId, { isHidden: false });
      this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
    } catch {
      this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
      // Keep local naming/avatar usable if profile sync is temporarily unavailable.
    }
  }

  private async restoreAgentChatFromInbox(
    entry: ConversationInboxEntry,
    agent: Pick<Chat, 'avatar' | 'name' | 'welcomeMessage'>,
    auth: AuthSessionOperationContext,
  ): Promise<Chat> {
    const projectedViewState = applyInboxProjectionToViewState(
      this.conversationViewState.get(entry.conversationId),
      entry,
    );
    await this.syncAgentConversationPresentation(entry.conversationId, agent, auth);
    this.assertAuthSessionGenerationCurrent(auth.generation, auth.operation);
    const viewState = this.rememberAgentConversation(entry.conversationId, agent, projectedViewState);
    return {
      ...mapInboxEntryToChat(entry, viewState),
      avatar: agent.avatar,
      name: agent.name,
      type: 'single',
      welcomeMessage: agent.welcomeMessage,
    };
  }

  private mapConversationMessageEntriesToMessages(
    chatId: string,
    entries: ConversationMessageEntry[],
    cachedMessages: Map<string, Message>,
  ): Message[] {
    return entries.map((entry, index): Message => {
      const message = mapConversationMessageEntryToMessage(
        entry,
        index,
        entries.length,
        cachedMessages.get(entry.messageId),
      );
      return message;
    });
  }

  private async hydrateInboxEntriesToChats(
    inboxEntries: ConversationInboxEntry[],
    generation: number,
  ): Promise<Chat[]> {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return [];
    }
    const chatResults = await mapWithConcurrencyLimit(
      inboxEntries,
      CHAT_LIST_HYDRATION_CONCURRENCY,
      async (entry): Promise<Chat | undefined> => {
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return undefined;
        }
        this.writeLatestReadSeq(entry.conversationId, Math.max(
          this.latestReadSeq.get(entry.conversationId) ?? 0,
          entry.lastMessageSeq,
        ));
        let viewState = applyInboxProjectionToViewState(
          this.conversationViewState.get(entry.conversationId),
          entry,
        );
        if (viewState) {
          this.writeConversationViewState(entry.conversationId, viewState);
        }
        if (viewState?.isHidden) {
          return undefined;
        }
        return applyLocalLastMessageToChat(
          mapInboxEntryToChat(entry, viewState),
          this.localMessages.get(entry.conversationId)?.at(-1),
        );
      },
    );
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return [];
    }
    return chatResults.filter((chat): chat is Chat => Boolean(chat));
  }

  async listChatsPage(options?: { cursor?: string; pageSize?: number }): Promise<ChatListPage> {
    const pageSize = normalizeInboxPageSize(options?.pageSize);
    const generation = this.authSessionGeneration;
    const emptyPage = (): ChatListPage => ({
      items: [],
      hasMore: false,
      nextCursor: undefined,
    });
    // First-page (no cursor) loads are deduplicated in-flight AND via a short
    // TTL cache. The TTL covers the startup sequence where syncOfflineMessages
    // and refreshChats call listChatsPage() back-to-back (serial await defeats
    // pure in-flight dedup). Concurrent callers, TTL-fresh calls, and in-flight
    // calls all share one network request.
    if (!options?.cursor) {
      const inFlight = this.inboxFirstPagePromises.get(pageSize);
      if (inFlight?.generation === generation) {
        return inFlight.promise;
      }
      const cached = this.inboxFirstPageCaches.get(pageSize);
      if (cached?.generation === generation && Date.now() < cached.expiresAt) {
        return cached.promise;
      }
      if (cached) {
        this.inboxFirstPageCaches.delete(pageSize);
        if (this.inboxFirstPageCache === cached) {
          this.inboxFirstPageCache = undefined;
        }
      }
    }
    const run = (async (): Promise<ChatListPage> => {
      try {
        const response = await (await this.client()).chat.inbox.list({
          pageSize,
          ...(options?.cursor ? { cursor: options.cursor } : {}),
        });
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return emptyPage();
        }
        const items = await this.hydrateInboxEntriesToChats(response.items, generation);
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return emptyPage();
        }
        void persistDesktopOfflineChats(items).catch(() => undefined);
        const pageInfo = (response as { pageInfo?: { hasMore?: boolean; nextCursor?: string | null } }).pageInfo;
        const page = readSdkCursorPageInfo(pageInfo);
        const sortedItems = items.sort(sortChats);
        if (!options?.cursor) {
          this.lastChatListSnapshot = sortedItems;
        }
        return {
          items: sortedItems,
          hasMore: page.hasMore,
          nextCursor: page.nextCursor,
        };
      } catch (error) {
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return emptyPage();
        }
        if (options?.cursor) {
          throw error;
        }
        const offlineChats = await loadDesktopOfflineChats(pageSize);
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return emptyPage();
        }
        if (offlineChats.length === 0) {
          throw error;
        }
        const sortedOfflineChats = offlineChats.sort(sortChats);
        this.lastChatListSnapshot = sortedOfflineChats;
        return {
          items: sortedOfflineChats,
          hasMore: false,
          nextCursor: undefined,
        };
      }
    })();
    if (!options?.cursor) {
      let firstPagePromise: Promise<ChatListPage>;
      firstPagePromise = run
        .then((page) => {
          const currentRequest = this.inboxFirstPagePromises.get(pageSize);
          if (
            this.isAuthSessionGenerationCurrent(generation)
            && currentRequest?.generation === generation
            && currentRequest.promise === firstPagePromise
          ) {
            const cacheEntry = {
              promise: firstPagePromise,
              expiresAt: Date.now() + INBOX_FIRST_PAGE_TTL_MS,
              generation,
              pageSize,
            };
            this.inboxFirstPageCache = cacheEntry;
            this.inboxFirstPageCaches.set(pageSize, cacheEntry);
          }
          return page;
        })
        .finally(() => {
          if (this.inboxFirstPagePromises.get(pageSize)?.promise === firstPagePromise) {
            this.inboxFirstPagePromises.delete(pageSize);
          }
          if (this.inboxFirstPagePromise === firstPagePromise) {
            this.inboxFirstPagePromise = undefined;
            this.inboxFirstPagePromiseGeneration = undefined;
            this.inboxFirstPagePromisePageSize = undefined;
          }
        });
      this.inboxFirstPagePromise = firstPagePromise;
      this.inboxFirstPagePromiseGeneration = generation;
      this.inboxFirstPagePromisePageSize = pageSize;
      this.inboxFirstPagePromises.set(pageSize, { generation, promise: firstPagePromise });
      return firstPagePromise;
    }
    return run;
  }

  async getChats(): Promise<Chat[]> {
    const page = await this.listChatsPage();
    const chats = [...page.items];

    for (const [chatId, localMessages] of this.localMessages.entries()) {
      if (this.conversationViewState.get(chatId)?.isHidden || chats.some((chat) => chat.id === chatId)) {
        continue;
      }
      const state = this.conversationViewState.get(chatId);
      const lastMessage = localMessages.at(-1);
      if (lastMessage) {
        chats.push(mapLocalMessageToChat(lastMessage, state));
      }
    }

    return chats.sort(sortChats);
  }

  subscribeChats(handler: ChatListHandler): () => void {
    this.chatListHandlers.add(handler);
    this.ensureLiveSession();
    this.syncLiveSessionSubscriptions();
    void this.emitChatList().catch(() => undefined);

    return () => {
      this.chatListHandlers.delete(handler);
      this.syncLiveSessionSubscriptions();
      if (this.chatListHandlers.size === 0 && this.liveSubscriptions.size === 0) {
        this.closeLiveSession('chat list subscription closed');
      }
    };
  }

  private setLocalMessages(chatId: string, messages: Message[]): void {
    this.localMessages.set(
      chatId,
      messages.length > LOCAL_MESSAGES_PER_CONVERSATION_CAP
        ? messages.slice(-LOCAL_MESSAGES_PER_CONVERSATION_CAP)
        : messages,
    );
    this.recordLocalConversationCacheWrite(chatId);
    this.pruneLiveSubscriptionNotificationVersions(chatId);
  }

  private pruneLiveSubscriptionNotificationVersions(
    chatId: string,
    subscription = this.liveSubscriptions.get(chatId),
  ): void {
    if (!subscription) {
      return;
    }
    const retainedMessageIds = new Set(
      (this.localMessages.get(chatId) ?? []).map((message) => message.id),
    );
    for (const messageId of subscription.notifiedMessageVersions.keys()) {
      if (!retainedMessageIds.has(messageId)) {
        subscription.notifiedMessageVersions.delete(messageId);
      }
    }
  }

  private queuePersistOfflineMessages(messages: OfflinePersistableMessage[]): void {
    if (messages.length === 0) {
      return;
    }
    void persistDesktopOfflineMessages(messages).catch(() => undefined);
  }

  private queuePersistOfflineMessage(message: OfflinePersistableMessage): void {
    this.queuePersistOfflineMessages([message]);
  }

  async getMessages(chatId: string, options?: { pageSize?: number }): Promise<Message[]> {
    if (this.isConversationHidden(chatId)) {
      return [];
    }
    // Deduplicate concurrent getMessages calls for the same chatId so that
    // MessageList's mount effect and subscribeMessages' catch-up share one request.
    const existing = this.getMessagesPromises.get(chatId);
    if (existing) {
      return existing;
    }
    this.assertMessageHistoryLoadCapacity();
    const cacheToken = this.ensureConversationCacheToken(chatId);
    const promise = this.doGetMessages(chatId, options, cacheToken).finally(() => {
      if (this.getMessagesPromises.get(chatId) === promise) {
        this.getMessagesPromises.delete(chatId);
      }
      this.activeMessageHistoryLoads.delete(promise);
      this.trimLocalConversationCache();
    });
    this.getMessagesPromises.set(chatId, promise);
    this.activeMessageHistoryLoads.add(promise);
    this.touchLocalConversationCacheIfPresent(chatId);
    this.trimLocalConversationCache();
    return promise;
  }

  private async doGetMessages(
    chatId: string,
    options: { pageSize?: number } | undefined,
    cacheToken: ConversationCacheToken,
  ): Promise<Message[]> {
    const pageSize = options?.pageSize ?? DEFAULT_MESSAGE_INITIAL_LIMIT;
    try {
      const cachedMessages = new Map(
        (this.localMessages.get(chatId) ?? []).map((message) => [message.id, message]),
      );
      const response = await (await this.client()).conversations.listMessages(chatId, {
        pageSize,
      });
      if (!this.isConversationCacheTokenCurrent(chatId, cacheToken)) {
        return [];
      }
      const pageMessages = this.mapConversationMessageEntriesToMessages(
        chatId,
        response.items,
        cachedMessages,
      );
      for (const message of pageMessages) {
        cachedMessages.set(message.id, message);
      }
      const latestMessageSeq = response.items.reduce(
        (latest, entry) => Math.max(latest, entry.messageSeq),
        this.latestReadSeq.get(chatId) ?? 0,
      );
      this.writeLatestReadSeq(chatId, latestMessageSeq);

      this.writeMessageHistoryPaginationState(
        chatId,
        normalizeMessageHistoryPagination(response.pageInfo, pageMessages.length),
      );

      const mergedMessages = mergeMessageLists(pageMessages, this.localMessages.get(chatId) ?? []);
      this.setLocalMessages(chatId, mergedMessages);
      this.queuePersistOfflineMessages(
        response.items.map((entry, index) => ({
          ...mapConversationMessageEntryToMessage(entry, index, response.items.length, cachedMessages.get(entry.messageId)),
          messageSeq: entry.messageSeq,
        })),
      );
      return mergedMessages;
    } catch (error) {
      if (!this.isConversationCacheTokenCurrent(chatId, cacheToken)) {
        return [];
      }
      const offlineMessages = await loadDesktopOfflineMessages(chatId, undefined, pageSize);
      if (!this.isConversationCacheTokenCurrent(chatId, cacheToken)) {
        return [];
      }
      if (offlineMessages.length === 0) {
        throw error;
      }
      const mergedMessages = mergeMessageLists(offlineMessages, this.localMessages.get(chatId) ?? []);
      this.setLocalMessages(chatId, mergedMessages);
      this.writeMessageHistoryPaginationState(chatId, {
        hasMore: false,
      });
      return mergedMessages;
    }
  }

  hasMoreMessages(chatId: string): boolean {
    return this.messageHistoryPaginationState.get(chatId)?.hasMore ?? false;
  }

  async loadMoreMessages(chatId: string, pageSize?: number): Promise<Message[]> {
    if (this.isConversationHidden(chatId)) {
      return [];
    }
    const existing = this.loadMoreMessagesPromises.get(chatId);
    if (existing) {
      return existing;
    }
    const state = this.messageHistoryPaginationState.get(chatId);
    if (!state?.hasMore || !state.nextCursor) {
      return [];
    }
    this.assertMessageHistoryLoadCapacity();
    const cacheToken = this.ensureConversationCacheToken(chatId);
    const promise = this.doLoadMoreMessages(
      chatId,
      pageSize,
      state,
      cacheToken,
    ).finally(() => {
      if (this.loadMoreMessagesPromises.get(chatId) === promise) {
        this.loadMoreMessagesPromises.delete(chatId);
      }
      this.activeMessageHistoryLoads.delete(promise);
      this.trimLocalConversationCache();
    });
    this.loadMoreMessagesPromises.set(chatId, promise);
    this.activeMessageHistoryLoads.add(promise);
    this.touchLocalConversationCacheIfPresent(chatId);
    this.trimLocalConversationCache();
    return promise;
  }

  private async doLoadMoreMessages(
    chatId: string,
    pageSize: number | undefined,
    state: MessageHistoryPaginationState,
    cacheToken: ConversationCacheToken,
  ): Promise<Message[]> {
    if (!state.nextCursor) {
      return [];
    }
    const response = await (await this.client()).conversations.listMessages(chatId, {
      cursor: state.nextCursor,
      pageSize: pageSize ?? MESSAGE_PAGE_LIMIT,
    });
    if (!this.isConversationCacheTokenCurrent(chatId, cacheToken)) {
      return [];
    }

    const cachedMessages = new Map(
      (this.localMessages.get(chatId) ?? []).map((message) => [message.id, message]),
    );
    const newMessages = this.mapConversationMessageEntriesToMessages(
      chatId,
      response.items,
      cachedMessages,
    );
    const latestMessageSeq = response.items.reduce(
      (latest, entry) => Math.max(latest, entry.messageSeq),
      this.latestReadSeq.get(chatId) ?? 0,
    );
    this.writeLatestReadSeq(chatId, latestMessageSeq);

    this.writeMessageHistoryPaginationState(
      chatId,
      normalizeMessageHistoryPagination(response.pageInfo, newMessages.length),
    );

    const mergedMessages = mergeMessageLists(newMessages, this.localMessages.get(chatId) ?? []);
    this.setLocalMessages(chatId, mergedMessages);
    this.queuePersistOfflineMessages(
      response.items.map((entry, index) => ({
        ...mapConversationMessageEntryToMessage(entry, index, response.items.length, cachedMessages.get(entry.messageId)),
        messageSeq: entry.messageSeq,
      })),
    );
    return newMessages;
  }

  subscribeMessages(chatId: string, handler: MessageHandler): () => void {
    if (this.isConversationHidden(chatId)) {
      return () => undefined;
    }
    const subscription = this.getOrCreateLiveSubscription(chatId);
    subscription.handlers.add(handler);

    return () => {
      subscription.handlers.delete(handler);
      if (subscription.handlers.size === 0) {
        this.closeLiveSubscription(chatId, subscription);
      }
    };
  }

  async sendMessage(
    chatId: string,
    content: string,
    type: Message['type'] = 'text',
    replyTo?: Message['replyTo'],
    extraInfo?: ChatMessageExtraInfo,
  ): Promise<Message> {
    const generation = this.authSessionGeneration;
    const assertCurrentGeneration = (): void => {
      if (!this.isAuthSessionGenerationCurrent(generation)) {
        throw new Error('Chat session changed while sending message.');
      }
    };
    const client = await this.client();
    assertCurrentGeneration();
    const currentUser = contactService.getCurrentUser();
    const clientMsgId = `pc-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
    const replyReference = buildReplyReference(replyTo);
    const {
      file: _file,
      mimeType: _mimeType,
      ...localExtraInfo
    } = extraInfo ?? {};

    let mediaUpload: ChatMediaUploadResult | undefined;
    let remoteSummary = content || extraInfo?.fileName || type;
    let parts: ContentPart[] | undefined;
    let renderHints: ReturnType<typeof buildMessageRenderHints> | undefined;

    try {
      mediaUpload = isMediaMessageType(type)
        ? await uploadChatMediaFile({
            chatId,
            content,
            extraInfo,
            getDriveUploader: async () => {
              const uploader = await this.getDriveUploader();
              assertCurrentGeneration();
              return uploader;
            },
            getSession: this.getSession,
            type,
          })
        : undefined;
      assertCurrentGeneration();
      remoteSummary = mediaUpload?.resource.fileName ?? remoteSummary;
      parts = type === 'text'
        ? undefined
        : buildMessageParts(mediaUpload?.content ?? content, type, extraInfo, mediaUpload);
      renderHints = type === 'text' ? undefined : buildMessageRenderHints(type, extraInfo);
      const postResult = type === 'text'
        ? await client.conversations.postText(chatId, content, {
            clientMsgId,
            summary: content,
            ...(replyReference ? { replyTo: replyReference } : {}),
          })
        : await client.conversations.postMessage(chatId, {
            clientMsgId,
            summary: remoteSummary,
            ...(replyReference ? { replyTo: replyReference } : {}),
            ...(parts ? { parts } : {}),
            renderHints,
          });
      assertCurrentGeneration();

      const message: Message = {
        id: postResult.messageId,
        chatId,
        senderId: extraInfo?.senderId ?? currentUser.id,
        content,
        type,
        timestamp: Date.now(),
        replyTo,
        ...localExtraInfo,
      };
      const storedMessage = this.upsertLocalMessage(chatId, message, true);
      this.writeLatestReadSeq(chatId, Math.max(
        this.latestReadSeq.get(chatId) ?? 0,
        postResult.messageSeq,
      ));
      const subscription = this.liveSubscriptions.get(chatId);
      if (subscription) {
        this.notifyLiveSubscription(subscription, storedMessage);
      }
      this.emitLocalChatListSnapshot();
      void this.emitChatList().catch(() => undefined);
      return storedMessage;
    } catch (error) {
      assertCurrentGeneration();
      const canQueueOffline = isRetryableDesktopSendError(error)
        && (await ensureDesktopOfflineChatCache())
        && (type === 'text' || Boolean(mediaUpload));
      assertCurrentGeneration();
      if (!canQueueOffline) {
        throw error;
      }

      await enqueueDesktopPendingSend({
        chatId,
        content: mediaUpload?.content ?? content,
        type,
        clientMsgId,
        replyTo,
        extraInfo: localExtraInfo,
        ...(type === 'text'
          ? {}
          : {
              summary: remoteSummary,
              parts,
              renderHints,
            }),
      });
      assertCurrentGeneration();
      const pendingMessage: Message = {
        id: clientMsgId,
        chatId,
        senderId: extraInfo?.senderId ?? currentUser.id,
        content,
        type,
        timestamp: Date.now(),
        replyTo,
        sendState: 'pending',
        ...localExtraInfo,
      };
      const storedMessage = this.upsertLocalMessage(chatId, pendingMessage, true);
      const subscription = this.liveSubscriptions.get(chatId);
      if (subscription) {
        this.notifyLiveSubscription(subscription, storedMessage);
      }
      this.emitLocalChatListSnapshot();
      void this.emitChatList().catch(() => undefined);
      return storedMessage;
    }
  }

  async forwardMessages(targetChatIds: string[], messages: Message[]): Promise<void> {
    const generation = this.authSessionGeneration;
    for (const targetChatId of targetChatIds) {
      for (const message of messages) {
        this.assertAuthSessionGenerationCurrent(generation, 'forwarding messages');
        if (isMediaMessageType(message.type)) {
          throw new Error('Forwarding media messages requires a reusable Drive reference before sending.');
        }
        await this.sendMessage(targetChatId, message.content, message.type, undefined, {
          fileName: message.fileName,
          fileSize: message.fileSize,
          coverUrl: message.coverUrl,
          duration: message.duration,
          appIcon: message.appIcon,
          desc: message.desc,
        });
        this.assertAuthSessionGenerationCurrent(generation, 'forwarding messages');
      }
    }
  }

  async retryFailedMessage(chatId: string, messageId: string): Promise<Message> {
    const messages = this.localMessages.get(chatId) ?? [];
    const failedMessage = messages.find(
      (message) => message.id === messageId && message.sendState === 'failed',
    );
    if (!failedMessage) {
      throw new Error('Failed outbound message not found.');
    }
    if (failedMessage.type !== 'text') {
      throw new Error('Only text messages can be retried from the local failed queue.');
    }

    this.setLocalMessages(
      chatId,
      messages.filter((message) => message.id !== messageId),
    );

    return this.sendMessage(
      chatId,
      failedMessage.content,
      failedMessage.type,
      failedMessage.replyTo,
    );
  }

  setReadFocusContext(context: { activeConversationId?: string; isWindowFocused?: boolean }): void {
    if ('activeConversationId' in context) {
      this.focusedConversationId = context.activeConversationId;
    }
    if (typeof context.isWindowFocused === 'boolean') {
      this.windowFocused = context.isWindowFocused;
    }
    if (context.activeConversationId) {
      this.touchLocalConversationCacheIfPresent(context.activeConversationId);
    }
    this.trimLocalConversationCache();
  }

  private resolveReadSeqForMarkAsRead(chatId: string): number {
    return this.latestReadSeq.get(chatId) ?? 0;
  }

  private isConversationActivelyViewed(chatId: string): boolean {
    return this.windowFocused && this.focusedConversationId === chatId;
  }

  private queueRealtimeReadCursorSync(
    chatId: string,
    readSeq: number,
    generation = this.authSessionGeneration,
  ): void {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    const normalizedReadSeq = Number.isFinite(readSeq) ? Math.max(0, Math.floor(readSeq)) : 0;
    if (normalizedReadSeq <= 0) {
      return;
    }

    this.writePendingRealtimeReadCursorSeq(
      chatId,
      Math.max(this.pendingRealtimeReadCursorSeqs.get(chatId) ?? 0, normalizedReadSeq),
    );
    this.writeConversationViewState(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: false,
    });
    this.scheduleRealtimeReadCursorSync(generation);
  }

  private scheduleRealtimeReadCursorSync(generation = this.authSessionGeneration): void {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    if (this.realtimeReadCursorSyncPromise) {
      return;
    }

    const syncPromise = Promise.resolve()
      .then(() => this.flushRealtimeReadCursorSync(generation))
      .catch(() => undefined)
      .finally(() => {
        if (this.realtimeReadCursorSyncPromise === syncPromise) {
          this.realtimeReadCursorSyncPromise = undefined;
        }
        if (
          this.isAuthSessionGenerationCurrent(generation)
          && this.pendingRealtimeReadCursorSeqs.size > 0
        ) {
          this.scheduleRealtimeReadCursorSync(generation);
        }
      });
    this.realtimeReadCursorSyncPromise = syncPromise;
  }

  private async flushRealtimeReadCursorSync(generation: number): Promise<void> {
    while (
      this.isAuthSessionGenerationCurrent(generation)
      && this.pendingRealtimeReadCursorSeqs.size > 0
    ) {
      const updates = Array.from(this.pendingRealtimeReadCursorSeqs.entries());
      for (const [conversationId] of updates) {
        this.beginReadCursorSync(conversationId);
      }
      this.pendingRealtimeReadCursorSeqs.clear();
      try {
        const client = await this.client();
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return;
        }
        await mapWithConcurrencyLimit(
          updates,
          REALTIME_READ_CURSOR_SYNC_CONCURRENCY,
          async ([conversationId, readSeq]) => {
            try {
              await client.conversations.updateReadCursor(conversationId, { readSeq });
              if (!this.isAuthSessionGenerationCurrent(generation)) {
                return;
              }
              this.writeLatestReadSeq(conversationId, Math.max(
                this.latestReadSeq.get(conversationId) ?? 0,
                readSeq,
              ));
            } catch {
              // Realtime read sync is opportunistic; explicit mark-as-read remains the durable retry path.
            }
          },
        );
      } finally {
        if (this.isAuthSessionGenerationCurrent(generation)) {
          for (const [conversationId] of updates) {
            this.endReadCursorSync(conversationId);
          }
        }
      }
    }
  }

  async markAsRead(chatId: string): Promise<void> {
    const generation = this.authSessionGeneration;
    const readSeq = this.resolveReadSeqForMarkAsRead(chatId);
    const client = await this.client();
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    if (readSeq > 0) {
      this.beginReadCursorSync(chatId);
      try {
        await client.conversations.updateReadCursor(chatId, { readSeq });
        if (!this.isAuthSessionGenerationCurrent(generation)) {
          return;
        }
        this.writeLatestReadSeq(chatId, readSeq);
      } finally {
        if (this.isAuthSessionGenerationCurrent(generation)) {
          this.endReadCursorSync(chatId);
        }
      }
    }
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    await client.conversations.updatePreferences(chatId, { isMarkedUnread: false });
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    this.writeConversationViewState(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: false,
    });
  }

  async markAsUnread(chatId: string): Promise<void> {
    const operation = 'marking a conversation unread';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.conversations.updatePreferences(chatId, { isMarkedUnread: true });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.writeConversationViewState(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: true,
    });
  }

  async deleteMessage(chatId: string, messageId: string): Promise<void> {
    const operation = 'deleting a message';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.messages.deleteForMe(messageId);
    this.assertAuthSessionGenerationCurrent(generation, operation);
    const messages = this.localMessages.get(chatId) ?? [];
    this.setLocalMessages(chatId, messages.filter((message) => message.id !== messageId));
  }

  async recallMessage(chatId: string, messageId: string): Promise<void> {
    const operation = 'recalling a message';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.recallMessage(messageId);
    this.assertAuthSessionGenerationCurrent(generation, operation);
    let recalledMessage: Message | undefined;
    this.updateLocalMessage(chatId, messageId, (message) => {
      recalledMessage = { ...message, isRecalled: true, content: '', reactions: [] };
      return recalledMessage;
    });
    const subscription = this.liveSubscriptions.get(chatId);
    if (subscription && recalledMessage) {
      this.notifyLiveSubscription(subscription, recalledMessage);
    }
  }

  async editMessage(chatId: string, messageId: string, text: string): Promise<void> {
    const trimmed = text.trim();
    if (!trimmed) {
      throw new Error('Edited message text must not be empty.');
    }
    const operation = 'editing a message';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.editMessage(messageId, { text: trimmed });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    let editedMessage: Message | undefined;
    this.updateLocalMessage(chatId, messageId, (message) => {
      editedMessage = { ...message, content: trimmed, isEdited: true };
      return editedMessage;
    });
    const subscription = this.liveSubscriptions.get(chatId);
    if (subscription && editedMessage) {
      this.notifyLiveSubscription(subscription, editedMessage);
    }
  }

  async deleteChat(chatId: string): Promise<void> {
    const operation = 'deleting a conversation';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.conversations.updatePreferences(chatId, { isHidden: true });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    const liveSubscription = this.liveSubscriptions.get(chatId);
    if (liveSubscription) {
      this.closeLiveSubscription(chatId, liveSubscription);
    }
    this.getMessagesPromises.delete(chatId);
    this.loadMoreMessagesPromises.delete(chatId);
    this.evictLocalConversationCache(chatId);
    this.writeConversationViewState(chatId, { isHidden: true });
    this.inboxFirstPageCache = undefined;
    this.emitLocalChatListSnapshot();
  }

  async pinChat(chatId: string, isPinned: boolean): Promise<void> {
    const operation = 'updating a conversation pin';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.conversations.updatePreferences(chatId, { isPinned });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.writeConversationViewState(chatId, {
      ...this.conversationViewState.get(chatId),
      isPinned,
    });
  }

  async muteChat(chatId: string, isMuted: boolean): Promise<void> {
    const operation = 'updating a conversation mute preference';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.conversations.updatePreferences(chatId, { isMuted });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.writeConversationViewState(chatId, {
      ...this.conversationViewState.get(chatId),
      isMuted,
    });
  }

  async addReaction(chatId: string, messageId: string, emoji: string): Promise<void> {
    const operation = 'adding a message reaction';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.addReaction(messageId, emoji);
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.updateLocalMessage(chatId, messageId, (message) => {
      const reactions = [...(message.reactions ?? [])];
      const existing = reactions.find((reaction) => reaction.emoji === emoji);
      if (existing) {
        existing.count += existing.hasReacted ? 0 : 1;
        existing.hasReacted = true;
      } else {
        reactions.push({ emoji, count: 1, hasReacted: true });
      }
      return { ...message, reactions };
    });
  }

  async removeReaction(chatId: string, messageId: string, emoji: string): Promise<void> {
    const operation = 'removing a message reaction';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    await client.removeReaction(messageId, emoji);
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.updateLocalMessage(chatId, messageId, (message) => {
      const reactions = (message.reactions ?? [])
        .map((reaction) => {
          if (reaction.emoji !== emoji || !reaction.hasReacted) {
            return reaction;
          }
          return {
            ...reaction,
            count: reaction.count - 1,
            hasReacted: false,
          };
        })
        .filter((reaction) => reaction.count > 0);
      return { ...message, reactions };
    });
  }

  async updateChat(chatId: string, updates: Partial<Chat>): Promise<Chat> {
    const operation = 'updating a conversation';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    const profileUpdate = buildConversationProfileUpdate(updates);
    if (hasProfileUpdate(profileUpdate)) {
      const profile = await client.conversations.updateProfile(chatId, profileUpdate);
      this.assertAuthSessionGenerationCurrent(generation, operation);
      this.writeConversationViewState(chatId, applyConversationProfile(
        this.conversationViewState.get(chatId),
        profile,
      ));
    }
    this.assertAuthSessionGenerationCurrent(generation, operation);
    const localViewUpdate = buildLocalConversationViewUpdate(updates);
    this.writeConversationViewState(chatId, {
      ...this.conversationViewState.get(chatId),
      ...localViewUpdate,
    });
    const updated = this.buildCachedConversationChat(chatId);
    if (!updated) {
      throw new Error('Chat not found');
    }
    return updated;
  }

  async createChat(chat: Chat): Promise<Chat> {
    if (chat.type !== 'group') {
      throw new Error('Single conversations must be created through startDirectChat.');
    }
    const operation = 'creating a conversation';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    const result = await client.conversations.create({
      conversationType: 'group',
      groupName: chat.name,
      clientRequestKey: createChatClientRequestKey(),
    });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    const conversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate(chat);
    if (hasProfileUpdate(profileUpdate)) {
      await client.conversations.updateProfile(conversationId, profileUpdate);
      this.assertAuthSessionGenerationCurrent(generation, operation);
    }
    await client.conversations.updatePreferences(conversationId, { isHidden: false });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.writeConversationViewState(conversationId, {
      avatar: chat.avatar,
      isHidden: false,
      memberCount: chat.memberCount,
      name: chat.name,
      notice: chat.notice,
      type: chat.type,
    });
    if (chat.lastMessage) {
      this.setLocalMessages(conversationId, [{ ...chat.lastMessage, chatId: conversationId }]);
    }
    return {
      ...chat,
      id: conversationId,
    };
  }

  async startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { conversationId?: string; directChatId?: string; id: string }): Promise<Chat> {
    const targetUserId = user.id.trim();
    if (!targetUserId) {
      throw new Error('Direct chat target user id is required');
    }
    const operation = 'starting a direct conversation';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    const projectedConversationId = user.conversationId?.trim();
    if (projectedConversationId) {
      await client.conversations.updatePreferences(projectedConversationId, { isHidden: false });
      this.assertAuthSessionGenerationCurrent(generation, operation);
      this.writeConversationViewState(projectedConversationId, {
        ...this.conversationViewState.get(projectedConversationId),
        avatar: user.avatar,
        isHidden: false,
        name: user.name,
        type: 'single',
      });
      return {
        id: projectedConversationId,
        name: user.name,
        avatar: user.avatar,
        type: 'single',
        unreadCount: 0,
        updatedAt: Date.now(),
      };
    }
    const currentUserId = this.resolveCurrentUserId().trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }
    const result = await client.conversations.bindDirectChat({
      leftActorId: currentUserId,
      leftActorKind: 'user',
      rightActorId: targetUserId,
      rightActorKind: 'user',
    });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: user.avatar,
      name: user.name,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await client.conversations.updateProfile(boundConversationId, profileUpdate);
      this.assertAuthSessionGenerationCurrent(generation, operation);
    }
    await client.conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.writeConversationViewState(boundConversationId, {
      ...this.conversationViewState.get(boundConversationId),
      avatar: user.avatar,
      isHidden: false,
      name: user.name,
      type: 'single',
    });
    return {
      id: boundConversationId,
      name: user.name,
      avatar: user.avatar,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
    };
  }

  async startAgentChat(agent: Pick<Chat, 'avatar' | 'name' | 'welcomeMessage'> & { id: string }): Promise<Chat> {
    const agentId = requireStandardAgentChatId(agent.id);
    const operation = 'starting an agent conversation';
    const auth = await this.beginAuthSessionOperation(operation);
    const currentUserId = this.resolveCurrentUserId().trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }

    const existingEntry = await this.findExistingAgentDialogEntry(agentId, auth);
    this.assertAuthSessionGenerationCurrent(auth.generation, operation);
    if (existingEntry) {
      return this.restoreAgentChatFromInbox(existingEntry, agent, auth);
    }

    const result = await auth.client.conversations.createAgentDialog({
      agentId,
    });
    this.assertAuthSessionGenerationCurrent(auth.generation, operation);
    const boundConversationId = result.conversationId;
    await this.syncAgentConversationPresentation(boundConversationId, agent, auth);
    this.assertAuthSessionGenerationCurrent(auth.generation, operation);
    this.rememberAgentConversation(
      boundConversationId,
      agent,
      this.conversationViewState.get(boundConversationId),
    );
    return {
      id: boundConversationId,
      name: agent.name,
      avatar: agent.avatar,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
      welcomeMessage: agent.welcomeMessage,
    };
  }

  async startEnterpriseChat(enterprise: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat> {
    const enterpriseId = enterprise.id.trim();
    if (!enterpriseId) {
      throw new Error('Enterprise chat target id is required');
    }
    const operation = 'starting an enterprise conversation';
    const { client, generation } = await this.beginAuthSessionOperation(operation);
    const currentUserId = this.resolveCurrentUserId().trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }

    const displayName = enterprise.name.endsWith(' (Official)')
      ? enterprise.name
      : `${enterprise.name} (Official)`;
    const result = await client.conversations.bindDirectChat({
      leftActorId: currentUserId,
      leftActorKind: 'user',
      rightActorId: enterpriseId,
      rightActorKind: 'enterprise',
    });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: enterprise.avatar,
      name: displayName,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await client.conversations.updateProfile(boundConversationId, profileUpdate);
      this.assertAuthSessionGenerationCurrent(generation, operation);
    }
    await client.conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.assertAuthSessionGenerationCurrent(generation, operation);
    this.writeConversationViewState(boundConversationId, {
      ...this.conversationViewState.get(boundConversationId),
      avatar: enterprise.avatar,
      isHidden: false,
      name: displayName,
      type: 'single',
    });
    return {
      id: boundConversationId,
      name: displayName,
      avatar: enterprise.avatar,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
    };
  }

  async syncOfflineMessages(): Promise<ChatOfflineSyncResult> {
    const page = await this.listChatsPage();

    return {
      appliedMessages: 0,
      refreshedChats: page.items.length,
    };
  }

  recoverRealtimeConnection(reason = 'realtime recovery requested'): void {
    if (!this.hasLiveSubscriptionDemand()) {
      return;
    }
    this.configureRealtimeConnectionManager();
    recoverPcLiveConnection(reason, { force: true });
  }

  private getOrCreateLiveSubscription(chatId: string): ConversationLiveSubscription {
    this.configureRealtimeConnectionManager();
    const existing = this.liveSubscriptions.get(chatId);
    if (existing) {
      this.liveSubscriptions.delete(chatId);
      this.liveSubscriptions.set(chatId, existing);
      return existing;
    }
    if (this.liveSubscriptions.size >= MAX_LIVE_CONVERSATION_SUBSCRIPTIONS) {
      throw new RangeError(
        `Live conversation subscription limit (${MAX_LIVE_CONVERSATION_SUBSCRIPTIONS}) reached.`,
      );
    }

    const subscription: ConversationLiveSubscription = {
      chatId,
      handlers: new Set<MessageHandler>(),
      notifiedMessageVersions: new Map(
        (this.localMessages.get(chatId) ?? []).map((message) => [
          message.id,
          buildMessageNotificationVersion(message),
        ]),
      ),
    };
    this.liveSubscriptions.set(chatId, subscription);
    this.ensureConversationWireSubscription(chatId);
    return subscription;
  }

  private ensureConversationWireSubscription(conversationId: string): void {
    this.configureRealtimeConnectionManager();
    if (this.conversationWireUnsubs.has(conversationId)) {
      return;
    }
    const generation = this.authSessionGeneration;
    const unsubscribe = subscribePcConversationMessages(
      conversationId,
      (message, context) => {
        this.handleLiveMessage(conversationId, message, context, generation);
      },
    );
    this.conversationWireUnsubs.set(conversationId, unsubscribe);
  }

  private releaseConversationWireSubscription(conversationId: string): void {
    this.conversationWireUnsubs.get(conversationId)?.();
    this.conversationWireUnsubs.delete(conversationId);
  }

  private ensureInboxWireSubscription(): void {
    this.configureRealtimeConnectionManager();
    if (this.liveInboxWireUnsub || this.chatListHandlers.size === 0) {
      return;
    }
    const scopes = this.getChatListRealtimeScopes();
    const inboxScope = scopes[0];
    if (!inboxScope) {
      return;
    }
    const generation = this.authSessionGeneration;
    this.liveInboxWireUnsub = subscribePcRealtimeScope(inboxScope, (context) => {
      this.handleLiveScopeEvent(context, generation);
    });
  }

  private releaseInboxWireSubscription(): void {
    this.liveInboxWireUnsub?.();
    this.liveInboxWireUnsub = undefined;
  }

  private ensureLiveSession(): void {
    void ensureDesktopOfflineChatCache().catch(() => undefined);
    this.ensureInboxWireSubscription();
  }

  private syncLiveSessionSubscriptions(): void {
    if (this.chatListHandlers.size > 0) {
      this.ensureInboxWireSubscription();
    } else {
      this.releaseInboxWireSubscription();
    }
  }

  private closeLiveSubscription(
    chatId: string,
    subscription: ConversationLiveSubscription,
  ): void {
    if (this.liveSubscriptions.get(chatId) !== subscription) {
      return;
    }
    this.liveSubscriptions.delete(chatId);
    this.releaseConversationWireSubscription(chatId);
    subscription.handlers.clear();
    subscription.notifiedMessageVersions.clear();
    this.trimLocalConversationCache();
    if (this.liveSubscriptions.size === 0 && this.chatListHandlers.size === 0) {
      this.releaseInboxWireSubscription();
    }
  }

  private closeAllLiveSubscriptions(_reason: string): void {
    for (const [conversationId, subscription] of this.liveSubscriptions.entries()) {
      this.releaseConversationWireSubscription(conversationId);
      subscription.handlers.clear();
      subscription.notifiedMessageVersions.clear();
    }
    this.liveSubscriptions.clear();
    this.releaseInboxWireSubscription();
    this.trimLocalConversationCache();
  }

  private closeLiveSession(_reason: string): void {
    this.releaseInboxWireSubscription();
  }

  private hasLiveSubscriptionDemand(): boolean {
    return this.liveSubscriptions.size > 0 || this.chatListHandlers.size > 0;
  }

  private async handleConnectionOpen(generation = this.authSessionGeneration): Promise<void> {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    await this.hydrateDesktopPendingSends(generation).catch(() => undefined);
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    await this.flushDesktopPendingSendQueue(generation).catch(() => undefined);
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }

    if (this.chatListHandlers.size > 0) {
      await this.emitChatList(generation).catch(() => undefined);
    }
  }

  private async hydrateDesktopPendingSends(generation = this.authSessionGeneration): Promise<void> {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    const pending = await listDesktopPendingSends();
    if (!this.isAuthSessionGenerationCurrent(generation) || pending.length === 0) {
      return;
    }

    const currentUser = contactService.getCurrentUser();
    for (const item of pending) {
      if (!this.isAuthSessionGenerationCurrent(generation)) {
        return;
      }
      const existing = this.localMessages.get(item.chatId) ?? [];
      if (existing.some((message) => message.id === item.clientMsgId)) {
        continue;
      }
      const extraInfo = item.extraInfo ?? {};
      const pendingMessage: Message = {
        id: item.clientMsgId,
        chatId: item.chatId,
        senderId: typeof extraInfo.senderId === 'string' ? extraInfo.senderId : currentUser.id,
        content: item.content,
        type: item.type,
        timestamp: Date.now(),
        replyTo: item.replyTo,
        sendState: 'pending',
        ...(extraInfo as Partial<Message>),
      };
      this.upsertLocalMessage(item.chatId, pendingMessage, true);
    }
  }

  private async flushDesktopPendingSendQueue(generation = this.authSessionGeneration): Promise<void> {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    let processedPending = false;
    await runDesktopPendingSendFlush(async (pending) => {
      const releasePendingClaims = async (startIndex: number): Promise<void> => {
        for (let index = startIndex; index < pending.length; index += 1) {
          const claim = pending[index];
          if (claim) {
            await releaseDesktopPendingSendClaim(claim).catch(() => undefined);
          }
        }
      };
      if (!this.isAuthSessionGenerationCurrent(generation)) {
        await releasePendingClaims(0);
        return { retryableFailure: true };
      }
      processedPending = pending.length > 0;
      let retryableFailure = false;
      for (const [index, item] of pending.entries()) {
        if (
          !this.isAuthSessionGenerationCurrent(generation)
          || !isDesktopPendingSendClaimCurrent(item)
        ) {
          await releasePendingClaims(index);
          retryableFailure = true;
          break;
        }
        try {
          const replyReference = buildReplyReference(item.replyTo);
          const client = await this.client();
          if (!this.isAuthSessionGenerationCurrent(generation)) {
            await releasePendingClaims(index);
            retryableFailure = true;
            break;
          }
          const postResult = item.type === 'text'
            ? await client.conversations.postText(item.chatId, item.content, {
                clientMsgId: item.clientMsgId,
                summary: item.content,
                ...(replyReference ? { replyTo: replyReference } : {}),
              })
            : await client.conversations.postMessage(item.chatId, {
                clientMsgId: item.clientMsgId,
                summary: item.summary ?? item.content,
                ...(replyReference ? { replyTo: replyReference } : {}),
                ...(item.parts ? { parts: item.parts as ContentPart[] } : {}),
                ...(item.renderHints ? { renderHints: item.renderHints } : {}),
              });
          await removeDesktopPendingSend(item);
          if (!this.isAuthSessionGenerationCurrent(generation)) {
            await releasePendingClaims(index + 1);
            retryableFailure = true;
            break;
          }

          const messages = this.localMessages.get(item.chatId) ?? [];
          let replacedMessage: Message | undefined;
          const updatedMessages = messages.map((message) => {
            if (message.id !== item.clientMsgId) {
              return message;
            }
            replacedMessage = {
              ...message,
              id: postResult.messageId,
              sendState: undefined,
              timestamp: Date.now(),
            };
            return replacedMessage;
          });
          this.setLocalMessages(item.chatId, updatedMessages);
          this.writeLatestReadSeq(
            item.chatId,
            Math.max(this.latestReadSeq.get(item.chatId) ?? 0, postResult.messageSeq),
          );
          const subscription = this.liveSubscriptions.get(item.chatId);
          if (subscription && replacedMessage) {
            this.notifyLiveSubscription(subscription, replacedMessage);
          }
        } catch (error) {
          if (!this.isAuthSessionGenerationCurrent(generation)) {
            await releasePendingClaims(index);
            retryableFailure = true;
            break;
          }
          if (isRetryableDesktopSendError(error)) {
            await releasePendingClaims(index);
            retryableFailure = true;
            break;
          }
          await removeDesktopPendingSend(item).catch(() => undefined);
          if (!this.isAuthSessionGenerationCurrent(generation)) {
            await releasePendingClaims(index + 1);
            retryableFailure = true;
            break;
          }
          const messages = this.localMessages.get(item.chatId) ?? [];
          const updatedMessages = messages.map((message) => (
            message.id === item.clientMsgId
              ? { ...message, sendState: 'failed' as const }
              : message
          ));
          this.setLocalMessages(item.chatId, updatedMessages);
          const subscription = this.liveSubscriptions.get(item.chatId);
          const failedMessage = updatedMessages.find((message) => message.id === item.clientMsgId);
          if (subscription && failedMessage) {
            this.notifyLiveSubscription(subscription, failedMessage);
          }
        }
      }
      return { retryableFailure };
    }, { generation });

    if (
      processedPending
      && this.isAuthSessionGenerationCurrent(generation)
      && this.chatListHandlers.size > 0
    ) {
      await this.emitChatList(generation).catch(() => undefined);
    }
  }

  private getChatListRealtimeScopes(): ImRealtimeScopeSubscription[] {
    if (this.chatListHandlers.size === 0) {
      return [];
    }
    const userId = this.resolveChatListRealtimeUserId();
    if (!userId) {
      return [];
    }
    return [
      {
        eventTypes: CHAT_LIST_REALTIME_EVENT_TYPES,
        scopeId: userId,
        scopeType: 'user',
      },
    ];
  }

  private handleLiveScopeEvent(
    context: ImRealtimeEventContext,
    generation = this.authSessionGeneration,
  ): void {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      void context.ack().catch(() => undefined);
      return;
    }
    if (
      context.eventType
      && !CHAT_LIST_REALTIME_EVENT_TYPES.includes(context.eventType)
    ) {
      return;
    }

    const message = context.eventType === 'message.posted'
      ? mapLiveEventToMessage(context)
      : undefined;
    if (message && !this.conversationViewState.get(message.chatId)?.isHidden) {
      const storedMessage = this.upsertLocalMessage(message.chatId, message, true);
      const messageSeq = pickNumber(context.payload?.messageSeq, context.sequence) ?? 0;
      if (storedMessage.senderId !== this.resolveCurrentUserId()) {
        if (this.isConversationActivelyViewed(message.chatId)) {
          this.queueRealtimeReadCursorSync(message.chatId, messageSeq);
        } else {
          this.writeConversationViewState(message.chatId, {
            ...this.conversationViewState.get(message.chatId),
            isMarkedUnread: true,
          });
        }
      }
      this.writeLatestReadSeq(
        message.chatId,
        Math.max(this.latestReadSeq.get(message.chatId) ?? 0, messageSeq),
      );
      this.queuePersistOfflineMessage({ ...storedMessage, messageSeq });
      const subscription = this.liveSubscriptions.get(message.chatId);
      if (subscription) {
        this.notifyLiveSubscription(subscription, storedMessage);
      }
    }

    void context.ack().catch(() => undefined);
    void this.emitChatList(generation).catch(() => undefined);
  }

  private handleLiveMessage(
    fallbackChatId: string,
    decodedMessage: ImDecodedMessage,
    context: ImMessageContext,
    generation = this.authSessionGeneration,
  ): void {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      void context.ack().catch(() => undefined);
      return;
    }
    const message = mapLiveMessageToMessage(fallbackChatId, decodedMessage, context);
    if (this.isConversationHidden(message.chatId) || this.isConversationHidden(fallbackChatId)) {
      void context.ack().catch(() => undefined);
      return;
    }
    const isRtcCallUpdate = Boolean(resolveRtcCallDisplayState(message));
    const storedMessage = this.upsertLocalMessage(message.chatId, message, isRtcCallUpdate);
    const messageSeq = pickNumber(decodedMessage.messageSeq, context.payload?.messageSeq, context.sequence) ?? 0;
    if (
      storedMessage.senderId !== this.resolveCurrentUserId()
      && this.isConversationActivelyViewed(message.chatId)
    ) {
      this.queueRealtimeReadCursorSync(message.chatId, messageSeq);
    }
    this.writeLatestReadSeq(
      message.chatId,
      Math.max(this.latestReadSeq.get(message.chatId) ?? 0, messageSeq),
    );
    this.queuePersistOfflineMessage({ ...storedMessage, messageSeq });
    const subscription = this.liveSubscriptions.get(message.chatId) ?? this.liveSubscriptions.get(fallbackChatId);
    if (subscription) {
      this.notifyLiveSubscription(subscription, storedMessage);
    }
    void this.emitChatList(generation).catch(() => undefined);
    void context.ack().catch(() => undefined);
  }

  private emitChatList(generation = this.authSessionGeneration): Promise<void> {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return Promise.resolve();
    }
    if (this.chatListHandlers.size === 0) {
      return Promise.resolve();
    }
    // If a refresh is already in-flight, mark that another is pending so the
    // completion handler will pick up the latest state once without extra requests.
    if (this.chatListRefreshPromise) {
      this.chatListRefreshPending = true;
      return this.chatListRefreshPromise;
    }
    // Coalesce bursts of realtime events into a single network request. If a
    // coalesce timer is already scheduled, just mark pending and return.
    if (this.chatListCoalesceTimer !== undefined) {
      this.chatListRefreshPending = true;
      return this.chatListCoalescePromise ?? Promise.resolve();
    }
    const coalescePromise = new Promise<void>((resolve) => {
      this.chatListCoalesceResolve = resolve;
      this.chatListCoalesceTimer = setTimeout(() => {
        this.chatListCoalesceTimer = undefined;
        if (this.chatListCoalescePromise === coalescePromise) {
          this.chatListCoalescePromise = undefined;
          this.chatListCoalesceResolve = undefined;
        }
        void this.runEmitChatList(generation).then(resolve).catch(() => resolve());
      }, CHAT_LIST_COALESCE_MS);
    });
    this.chatListCoalescePromise = coalescePromise;
    return coalescePromise;
  }

  private async runEmitChatList(generation = this.authSessionGeneration): Promise<void> {
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    if (this.chatListRefreshPromise) {
      this.chatListRefreshPending = true;
      return this.chatListRefreshPromise;
    }
    const refreshPromise = this.doEmitChatList(generation).finally(() => {
      if (this.chatListRefreshPromise === refreshPromise) {
        this.chatListRefreshPromise = undefined;
        if (
          this.chatListRefreshPending
          && this.isAuthSessionGenerationCurrent(generation)
        ) {
          this.chatListRefreshPending = false;
          void this.emitChatList(generation).catch(() => undefined);
        }
      }
    });
    this.chatListRefreshPromise = refreshPromise;
    return refreshPromise;
  }

  private async doEmitChatList(generation: number): Promise<void> {
    const page = await this.listChatsPage();
    if (!this.isAuthSessionGenerationCurrent(generation)) {
      return;
    }
    const chats = this.mergeLiveLocalChats(page.items);
    this.notifyChatListHandlers(chats);
  }

  private notifyLiveSubscription(subscription: ConversationLiveSubscription, message: Message): void {
    const nextVersion = buildMessageNotificationVersion(message);
    if (subscription.notifiedMessageVersions.get(message.id) === nextVersion) {
      subscription.notifiedMessageVersions.delete(message.id);
      subscription.notifiedMessageVersions.set(message.id, nextVersion);
      this.pruneLiveSubscriptionNotificationVersions(subscription.chatId, subscription);
      return;
    }
    subscription.notifiedMessageVersions.delete(message.id);
    subscription.notifiedMessageVersions.set(message.id, nextVersion);
    this.pruneLiveSubscriptionNotificationVersions(subscription.chatId, subscription);
    for (const handler of subscription.handlers) {
      handler(message);
    }
  }

  private upsertLocalMessage(chatId: string, message: Message, preferNew = false): Message {
    const messages = this.localMessages.get(chatId) ?? [];
    const byId = new Map(messages.map((item) => [item.id, item]));
    const existingMessage = byId.get(message.id);
    const nextMessage = existingMessage
      ? mergeSameIdMessage(
          existingMessage,
          message,
          shouldPreferIncomingMessage(existingMessage, message, preferNew),
        )
      : message;
    byId.set(message.id, nextMessage);
    this.setLocalMessages(
      chatId,
      Array.from(byId.values()).sort((left, right) => left.timestamp - right.timestamp),
    );
    return nextMessage;
  }

  private updateLocalMessage(
    chatId: string,
    messageId: string,
    updater: (message: Message) => Message,
  ): void {
    const messages = this.localMessages.get(chatId) ?? [];
    this.setLocalMessages(
      chatId,
      messages.map((message) => message.id === messageId ? updater(message) : message),
    );
  }
}

export function createSdkworkChatService(dependencies?: ChatServiceDependencies | ImSdkClientProvider): ChatService {
  return new SdkworkChatService(dependencies);
}

export function resolveIncomingCallWatchConversationIds(
  chats: Array<{ id: string; conversationId?: string }>,
  contacts: Array<{ conversationId?: string; id: string } | string>,
  _currentUserId?: string,
): string[] {
  const conversationIds = new Set<string>();
  for (const chat of chats) {
    const conversationId = chat.conversationId?.trim() || chat.id.trim();
    if (conversationId) {
      conversationIds.add(conversationId);
    }
  }
  for (const contact of contacts) {
    const conversationId = typeof contact === 'string'
      ? contact.trim()
      : contact.conversationId?.trim();
    if (conversationId) {
      conversationIds.add(conversationId);
    }
  }
  return [...conversationIds];
}

export const chatService = createSdkworkChatService();
