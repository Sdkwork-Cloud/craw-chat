import type {
  ContactPreferencesView,
  ContentPart,
  ConversationMember,
  DriveReference,
  InboxResponse,
  ConversationProfileView,
  ImDecodedMessage,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
  ImSdkClient,
  MediaKind,
  MediaResource,
  MessageInteractionSummaryView,
  MessageReplyReference,
  SocialUserSearchResult,
  TimelineResponse,
  UpdateConversationProfileRequest,
} from '@sdkwork/im-sdk';
import type {
  DriveUploaderBlobLike,
  DriveUploaderClient,
  DriveUploaderProfile,
  DriveUploaderRequest,
  DriveUploaderUploadResult,
} from '@sdkwork/drive-app-sdk';
import { getDriveAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/driveAppSdkClient';
import {
  getImSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/imSdkClient';
import {
  configurePcRealtimeConnectionManager,
  onPcLiveAuthenticationFailure,
  onPcLiveConnectionOpen,
  recoverPcLiveConnection,
  subscribePcConversationMessages,
  subscribePcRealtimeScope,
} from '@sdkwork/im-pc-core/sdk/pcRealtimeConnectionManager';
import {
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  readAppSdkSessionTokens,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core/sdk/session';
import type { Chat, Message } from '@sdkwork/im-pc-types';
import { resolveSdkworkChatPcClientId } from './ClientIdentityService';
import { contactService } from './ContactService';
import { createDefaultAvatar } from './DefaultAvatarService';
import i18n from '../i18n';

type ConversationInboxEntry = InboxResponse['items'][number];
type TimelineViewEntry = TimelineResponse['items'][number];
type ChatListHandler = (chats: Chat[]) => void;
type MessageHandler = (message: Message) => void;
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

interface DirectChatPeerProfile {
  avatar?: string;
  name: string;
  userId: string;
}

interface ChatServiceDependencies {
  getClient?: () => ImSdkClient;
  getDriveUploader?: () => Pick<
    DriveUploaderClient,
    'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
  >;
  getSession?: () => SdkworkChatSession | null;
}

export interface ChatOfflineSyncResult {
  appliedMessages: number;
  refreshedChats: number;
}

interface ConversationLiveSubscription {
  handlers: Set<MessageHandler>;
  notifiedMessageVersions: Map<string, string>;
}

export interface ChatService {
  getChats(): Promise<Chat[]>;
  subscribeChats(handler: ChatListHandler): () => void;
  getMessages(chatId: string, options?: { limit?: number }): Promise<Message[]>;
  hasMoreMessages(chatId: string): boolean;
  loadMoreMessages(chatId: string, limit?: number): Promise<Message[]>;
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
  createChat(chat: Chat): Promise<void>;
  startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { conversationId?: string; directChatId?: string; id: string }): Promise<Chat>;
  startAgentChat(agent: Pick<Chat, 'avatar' | 'name' | 'welcomeMessage'> & { id: string }): Promise<Chat>;
  startEnterpriseChat(enterprise: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat>;
  recoverRealtimeConnection(reason?: string): void;
  syncOfflineMessages(): Promise<ChatOfflineSyncResult>;
}

type ConversationViewState = Partial<Pick<Chat, 'activeCount' | 'avatar' | 'isMarkedUnread' | 'isMuted' | 'isPinned' | 'memberCount' | 'members' | 'name' | 'notice' | 'type' | 'welcomeMessage'>> & {
  isHidden?: boolean;
};
const INBOX_PAGE_LIMIT = 100;
const MESSAGE_PAGE_LIMIT = 50;
const CONVERSATION_MEMBERS_PAGE_LIMIT = 100;
const CHAT_LIST_HYDRATION_CONCURRENCY = 4;
const INTERACTION_SUMMARY_BATCH_CONCURRENCY = 8;
const DEFAULT_MESSAGE_INITIAL_LIMIT = 50;
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

let driveUploaderClient: Pick<
  DriveUploaderClient,
  'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
> | null = null;

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

function createFallbackAgentConversationAvatar(): string {
  return createDefaultAvatar('agent');
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

function firstTimelinePart(entry: TimelineViewEntry): Record<string, unknown> {
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

function resolveTimelineMessageType(entry: TimelineViewEntry): Message['type'] {
  const renderHints = toRecord(entry.body?.renderHints);
  return resolvePartMessageType(firstTimelinePart(entry), renderHints);
}

function resolveTimelineResource(entry: TimelineViewEntry): Record<string, unknown> {
  const part = firstTimelinePart(entry);
  return part.kind === 'media' ? toRecord(part.resource) : {};
}

function resolveTimelineResourceUrl(entry: TimelineViewEntry): string | undefined {
  const resource = resolveTimelineResource(entry);
  return pickString(resource.publicUrl, resource.url, resource.uri);
}

function resolveTimelineMessageContent(entry: TimelineViewEntry, type: Message['type']): string {
  const part = firstTimelinePart(entry);
  const resourceUrl = resolveTimelineResourceUrl(entry);
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
  return /^c_agent_[a-f0-9]{24}$/u.test(value)
    || /^pc-agent-[a-z0-9._-]+-agent[._-][a-z0-9._-]+$/iu.test(value);
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

function createDefaultDriveUploaderClient(
  session: SdkworkChatSession | null = readAppSdkSessionTokens(),
): Pick<DriveUploaderClient, 'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'> {
  const client = getDriveAppSdkClientWithSession(session ?? readAppSdkSessionTokens());
  return client.uploader;
}

function getDefaultDriveUploader(): Pick<
  DriveUploaderClient,
  'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
> {
  if (!driveUploaderClient) {
    driveUploaderClient = createDefaultDriveUploaderClient();
  }
  return driveUploaderClient;
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
  getDriveUploader: () => Pick<
    DriveUploaderClient,
    'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
  >;
  getSession: () => SdkworkChatSession | null;
  type: SendableMediaMessageType;
}): Promise<ChatMediaUploadResult> {
  const file = extraInfo?.file;
  if (!file) {
    throw new Error('Chat media messages require a File or Blob before sending.');
  }

  const session = getSession();
  const userId = resolveChatUploadUserId(session);
  const originalFileName = resolveMediaUploadFileName(type, file, extraInfo);
  const uploadRequest: DriveUploaderRequest = {
    file,
    userId,
    appResourceType: CHAT_DRIVE_APP_RESOURCE_TYPE,
    appResourceId: chatId,
    scene: CHAT_DRIVE_SCENE,
    source: CHAT_DRIVE_SOURCE,
    ...(resolveMediaUploadProfile(type) ? { uploadProfileCode: resolveMediaUploadProfile(type) } : {}),
    originalFileName,
    ...(resolveMediaUploadContentType(type, file, extraInfo) ? { contentType: resolveMediaUploadContentType(type, file, extraInfo) } : {}),
  };

  const uploader = getDriveUploader();
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

function buildLegacyDirectConversationName(conversationId: string): string {
  return `Chat ${conversationId}`;
}

function isInternalConversationIdentifier(value: string): boolean {
  return /^(?:c_direct|c_agent|pc-direct|pc-agent|direct[-_:]|conversation[-_:])[a-z0-9._:-]*/iu.test(value.trim());
}

function isGeneratedDirectConversationName(name: string | undefined, conversationId: string): boolean {
  const normalizedName = pickString(name);
  if (!normalizedName) {
    return true;
  }

  return normalizedName === buildLegacyDirectConversationName(conversationId)
    || normalizedName === 'Direct chat'
    || normalizedName === conversationId
    || isInternalConversationIdentifier(normalizedName);
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
    ...(typeof projectedPreferences.isHidden === 'boolean' ? { isHidden: projectedPreferences.isHidden } : {}),
    type: normalizeConversationType(entry.conversationType),
  };
}

function hasInboxPreferencesProjection(entry: ConversationInboxEntry): boolean {
  const preferences = toRecord(toRecord(entry).preferences);
  return ['isPinned', 'isMuted', 'isMarkedUnread', 'isHidden']
    .every((field) => typeof preferences[field] === 'boolean');
}

function hasInboxDisplayProjection(entry: ConversationInboxEntry): boolean {
  const entryRecord = toRecord(entry);
  const projectedName = pickString(entryRecord.displayName, entryRecord.display_name);
  if (projectedName) {
    return true;
  }

  if (normalizeConversationType(entry.conversationType) !== 'single') {
    return false;
  }

  const peerRecord = toRecord(entryRecord.peer);
  return Boolean(pickString(peerRecord.displayName, peerRecord.display_name));
}

function mapLocalMessageToChat(message: Message, viewState: ConversationViewState | undefined): Chat {
  const conversationType = viewState?.type ?? 'single';
  return {
    id: message.chatId,
    name: viewState?.name ?? 'Direct chat',
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
  profile: ConversationProfileView,
): ConversationViewState {
  return {
    ...viewState,
    ...(pickString(profile.displayName) ? { name: pickString(profile.displayName) } : {}),
    ...(pickString(profile.avatarUrl) ? { avatar: pickString(profile.avatarUrl) } : {}),
    notice: profile.notice,
  };
}

function mapSocialUserResultToDirectPeerProfile(
  result: SocialUserSearchResult,
  preferences?: ContactPreferencesView,
): DirectChatPeerProfile {
  const name = pickString(preferences?.remark, result.displayName, result.userId) ?? result.userId;
  return {
    userId: result.userId,
    name,
    ...(pickString(result.avatarUrl) ? { avatar: pickString(result.avatarUrl) } : {}),
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

function mapTimelineEntryToMessage(
  entry: TimelineViewEntry,
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

  const type = resolveTimelineMessageType(entry);
  const renderHints = toRecord(entry.body?.renderHints);
  const resource = resolveTimelineResource(entry);
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
    content: resolveTimelineMessageContent(entry, type),
    type,
    timestamp,
    ...(coverUrl ? { coverUrl } : {}),
    ...(duration ? { duration } : {}),
    ...(fileName ? { fileName } : {}),
    ...(pickString(renderHints.fileSize, resource.sizeBytes) ? { fileSize: pickString(renderHints.fileSize, resource.sizeBytes) } : {}),
    ...(replyTo ? { replyTo } : {}),
  };
}

function mapInteractionSummaryToReactions(
  summary: MessageInteractionSummaryView | undefined,
  cachedReactions: Message['reactions'],
): Message['reactions'] | undefined {
  if (!summary || summary.reactionCounts.length === 0) {
    return undefined;
  }

  return summary.reactionCounts
    .filter((reaction) => reaction.count > 0)
    .map((reaction) => {
      const cached = cachedReactions?.find((item) => item.emoji === reaction.reactionKey);
      return {
        emoji: reaction.reactionKey,
        count: reaction.count,
        hasReacted: cached?.hasReacted ?? false,
      };
    });
}

function withMessageInteractionSummary(
  message: Message,
  summary: MessageInteractionSummaryView | undefined,
): Message {
  const reactions = mapInteractionSummaryToReactions(summary, message.reactions);
  if (reactions && reactions.length > 0) {
    return { ...message, reactions };
  }

  const { reactions: _reactions, ...messageWithoutReactions } = message;
  return messageWithoutReactions;
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

configurePcRealtimeConnectionManager({
  getClient: getImSdkClientWithSession,
  getDeviceId: resolveSdkworkChatPcClientId,
  getSession: readAppSdkSessionTokens,
});

interface TimelinePaginationState {
  hasMore: boolean;
  nextAfterSeq: number;
}

class SdkworkChatService implements ChatService {
  private readonly chatListHandlers = new Set<ChatListHandler>();
  private conversationViewState = new Map<string, ConversationViewState>();
  private conversationWireUnsubs = new Map<string, () => void>();
  private liveCatchUpConversations = new Set<string>();
  private liveInboxWireUnsub?: () => void;
  private liveSubscriptions = new Map<string, ConversationLiveSubscription>();
  private localMessages = new Map<string, Message[]>();
  private latestReadSeq = new Map<string, number>();
  private timelinePaginationState = new Map<string, TimelinePaginationState>();
  private readonly getClient: () => ImSdkClient;
  private readonly getDriveUploader: () => Pick<
    DriveUploaderClient,
    'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
  >;
  private readonly getSession: () => SdkworkChatSession | null;

  private readonly handleAuthSessionChanged = (): void => {
    driveUploaderClient = null;
    this.localMessages.clear();
    this.latestReadSeq.clear();
    this.conversationViewState.clear();
    this.timelinePaginationState.clear();
    this.closeAllLiveSubscriptions('auth session changed');
  };

  private handleRealtimeAuthenticationFailure(reason: string): void {
    this.closeAllLiveSubscriptions(reason);
  }

  constructor(dependencies: ChatServiceDependencies | (() => ImSdkClient) = {}) {
    if (typeof dependencies === 'function') {
      this.getClient = dependencies;
      this.getDriveUploader = getDefaultDriveUploader;
      this.getSession = readAppSdkSessionTokens;
    } else {
      this.getClient = dependencies.getClient ?? getImSdkClientWithSession;
      this.getDriveUploader = dependencies.getDriveUploader ?? getDefaultDriveUploader;
      this.getSession = dependencies.getSession ?? readAppSdkSessionTokens;
    }
    if (typeof window !== 'undefined') {
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, this.handleAuthSessionChanged);
    }
    onPcLiveConnectionOpen(() => {
      void this.catchUpOnConnectionOpen();
    });
    onPcLiveAuthenticationFailure((reason) => {
      this.handleRealtimeAuthenticationFailure(reason);
    });
  }

  private client(): ImSdkClient {
    return this.getClient();
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

  private async listAllInboxEntries(): Promise<ConversationInboxEntry[]> {
    const items: ConversationInboxEntry[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().chat.inbox.retrieve({
        limit: INBOX_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      cursor = response.hasMore ? (response.nextCursor ?? undefined) : undefined;
    } while (cursor);

    return items;
  }

  private async listAllConversationMembers(conversationId: string): Promise<ConversationMember[]> {
    const items: ConversationMember[] = [];
    let cursor: string | undefined;

    while (true) {
      const response = await this.client().conversations.listMembers(conversationId, {
        limit: CONVERSATION_MEMBERS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);

      if (!response.hasMore || !response.nextCursor || response.nextCursor === cursor) {
        break;
      }

      cursor = response.nextCursor ?? undefined;
    }

    return items;
  }

  private isDisplayableDirectMember(member: ConversationMember): boolean {
    const state = String(member.state).trim().toLowerCase();
    return member.principalKind === 'user'
      && Boolean(pickString(member.principalId))
      && state !== 'left'
      && state !== 'removed';
  }

  private findDirectPeerMember(members: ConversationMember[]): ConversationMember | undefined {
    const currentUserIdentifiers = this.resolveCurrentUserIdentifiers();
    return members
      .filter((member) => this.isDisplayableDirectMember(member))
      .find((member) => !currentUserIdentifiers.has(member.principalId));
  }

  private async resolveDirectPeerSocialProfile(peerUserId: string): Promise<SocialUserSearchResult | undefined> {
    const response = await this.client().social.users.list({
      q: peerUserId,
      limit: 20,
    });
    return response.items.find((item: SocialUserSearchResult) => {
      const itemRecord = toRecord(item);
      const metadata = toRecord(itemRecord.metadata);
      const chatId = pickString(item.chatId, itemRecord.chat_id, metadata.chatId, metadata.chat_id);
      return item.userId === peerUserId || chatId === peerUserId;
    });
  }

  private async resolveDirectPeerContactPreferences(peerUserId: string): Promise<ContactPreferencesView | undefined> {
    try {
      return await this.client().social.contacts.preferences.retrieve(peerUserId);
    } catch {
      return undefined;
    }
  }

  private async resolveDirectChatPeerProfile(conversationId: string): Promise<DirectChatPeerProfile | undefined> {
    const members = await this.listAllConversationMembers(conversationId);
    const peerMember = this.findDirectPeerMember(members);
    const peerUserId = pickString(peerMember?.principalId);
    if (!peerUserId) {
      return undefined;
    }

    const [socialProfile, preferences] = await Promise.all([
      this.resolveDirectPeerSocialProfile(peerUserId).catch(() => undefined),
      this.resolveDirectPeerContactPreferences(peerUserId),
    ]);
    if (!socialProfile) {
      const remark = pickString(preferences?.remark);
      return remark ? { userId: peerUserId, name: remark } : undefined;
    }

    return mapSocialUserResultToDirectPeerProfile(socialProfile, preferences);
  }

  private async hydrateDirectConversationViewState(
    entry: ConversationInboxEntry,
    viewState: ConversationViewState | undefined,
  ): Promise<ConversationViewState | undefined> {
    if (entry.agentHandoff) {
      const handoffViewState = {
        ...viewState,
        name: viewState?.name ?? 'Support conversation',
        type: 'single' as const,
      };
      this.conversationViewState.set(entry.conversationId, handoffViewState);
      return handoffViewState;
    }

    if (isAgentDialogConversationId(entry.conversationId)) {
      const agentViewState = {
        ...viewState,
        avatar: viewState?.avatar ?? createFallbackAgentConversationAvatar(),
        name: viewState?.name ?? 'AI assistant chat',
        type: 'single' as const,
      };
      this.conversationViewState.set(entry.conversationId, agentViewState);
      return agentViewState;
    }

    if (
      normalizeConversationType(entry.conversationType) !== 'single'
      || !isGeneratedDirectConversationName(viewState?.name, entry.conversationId)
    ) {
      return viewState;
    }

    try {
      const peerProfile = await this.resolveDirectChatPeerProfile(entry.conversationId);
      const hydratedViewState = {
        ...viewState,
        name: peerProfile?.name ?? 'Direct chat',
        ...(peerProfile?.avatar ? { avatar: peerProfile.avatar } : {}),
        type: 'single' as const,
      };
      this.conversationViewState.set(entry.conversationId, hydratedViewState);
      return hydratedViewState;
    } catch {
      const fallbackViewState = {
        ...viewState,
        name: 'Direct chat',
        type: 'single' as const,
      };
      this.conversationViewState.set(entry.conversationId, fallbackViewState);
      return fallbackViewState;
    }
  }

  private async enrichMessagesWithInteractionSummaries(
    chatId: string,
    entries: TimelineViewEntry[],
    cachedMessages: Map<string, Message>,
  ): Promise<Message[]> {
    return mapWithConcurrencyLimit(
      entries,
      INTERACTION_SUMMARY_BATCH_CONCURRENCY,
      async (entry, index): Promise<Message> => {
        this.latestReadSeq.set(chatId, Math.max(
          this.latestReadSeq.get(chatId) ?? 0,
          entry.messageSeq,
        ));
        const message = mapTimelineEntryToMessage(
          entry,
          index,
          entries.length,
          cachedMessages.get(entry.messageId),
        );
        if (!entry.messageId?.trim()) {
          return message;
        }
        try {
          const summary = await this.client().conversations.getMessageInteractionSummary(
            chatId,
            entry.messageId,
          );
          return withMessageInteractionSummary(message, summary);
        } catch {
          return message;
        }
      },
    );
  }

  async getChats(): Promise<Chat[]> {
    const inboxEntries = await this.listAllInboxEntries();
    const chatResults = await mapWithConcurrencyLimit(
      inboxEntries,
      CHAT_LIST_HYDRATION_CONCURRENCY,
      async (entry): Promise<Chat | undefined> => {
        this.latestReadSeq.set(entry.conversationId, Math.max(
          this.latestReadSeq.get(entry.conversationId) ?? 0,
          entry.lastMessageSeq,
        ));
        let viewState = applyInboxProjectionToViewState(
          this.conversationViewState.get(entry.conversationId),
          entry,
        );
        if (viewState) {
          this.conversationViewState.set(entry.conversationId, viewState);
        }
        if (!hasInboxPreferencesProjection(entry)) {
          try {
            const preferences = await this.client().conversations.getPreferences(entry.conversationId);
            viewState = {
              ...viewState,
              isMuted: preferences.isMuted,
              isMarkedUnread: preferences.isMarkedUnread,
              isPinned: preferences.isPinned,
              isHidden: preferences.isHidden,
            };
            this.conversationViewState.set(entry.conversationId, viewState);
          } catch {
            // Keep the chat list usable if a preference sync request fails.
          }
        }
        if (viewState?.isHidden) {
          return undefined;
        }
        if (
          normalizeConversationType(entry.conversationType) !== 'single'
          && !hasInboxDisplayProjection(entry)
        ) {
          try {
            const profile = await this.client().conversations.getProfile(entry.conversationId);
            viewState = applyConversationProfile(viewState, profile);
            this.conversationViewState.set(entry.conversationId, viewState);
          } catch {
            // Keep local naming/avatar fallbacks usable if profile sync is temporarily unavailable.
          }
        }
        viewState = await this.hydrateDirectConversationViewState(entry, viewState);
        return applyLocalLastMessageToChat(
          mapInboxEntryToChat(entry, viewState),
          this.localMessages.get(entry.conversationId)?.at(-1),
        );
      },
    );
    const chats = chatResults.filter((chat): chat is Chat => Boolean(chat));

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

  async getMessages(chatId: string, options?: { limit?: number }): Promise<Message[]> {
    const limit = options?.limit ?? DEFAULT_MESSAGE_INITIAL_LIMIT;
    const response = await this.client().conversations.listMessages(chatId, {
      afterSeq: 0,
      limit,
    });

    const cachedMessages = new Map(
      (this.localMessages.get(chatId) ?? []).map((message) => [message.id, message]),
    );
    const firstPageMessages = await this.enrichMessagesWithInteractionSummaries(
      chatId,
      response.items,
      cachedMessages,
    );

    this.timelinePaginationState.set(chatId, {
      hasMore: Boolean(response.hasMore),
      nextAfterSeq: typeof response.nextAfterSeq === 'number' ? response.nextAfterSeq : 0,
    });

    const remoteMessages = firstPageMessages;

    const mergedMessages = mergeMessageLists(remoteMessages, this.localMessages.get(chatId) ?? []);
    this.localMessages.set(chatId, mergedMessages);
    return mergedMessages;
  }

  hasMoreMessages(chatId: string): boolean {
    return this.timelinePaginationState.get(chatId)?.hasMore ?? false;
  }

  async loadMoreMessages(chatId: string, limit?: number): Promise<Message[]> {
    const state = this.timelinePaginationState.get(chatId);
    if (!state || !state.hasMore) {
      return [];
    }

    const afterSeq = state.nextAfterSeq;
    const response = await this.client().conversations.listMessages(chatId, {
      afterSeq,
      limit: limit ?? MESSAGE_PAGE_LIMIT,
    });

    const cachedMessages = new Map(
      (this.localMessages.get(chatId) ?? []).map((message) => [message.id, message]),
    );
    const newMessages = await this.enrichMessagesWithInteractionSummaries(
      chatId,
      response.items,
      cachedMessages,
    );

    this.timelinePaginationState.set(chatId, {
      hasMore: Boolean(response.hasMore),
      nextAfterSeq: typeof response.nextAfterSeq === 'number' && response.nextAfterSeq > afterSeq
        ? response.nextAfterSeq
        : afterSeq,
    });

    const mergedMessages = mergeMessageLists(newMessages, this.localMessages.get(chatId) ?? []);
    this.localMessages.set(chatId, mergedMessages);
    return newMessages;
  }

  subscribeMessages(chatId: string, handler: MessageHandler): () => void {
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
    const client = this.client();
    const currentUser = contactService.getCurrentUser();
    const clientMsgId = `pc-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
    const replyReference = buildReplyReference(replyTo);
    const mediaUpload = isMediaMessageType(type)
      ? await uploadChatMediaFile({
          chatId,
          content,
          extraInfo,
          getDriveUploader: this.getDriveUploader,
          getSession: this.getSession,
          type,
        })
      : undefined;
    const remoteSummary = mediaUpload?.resource.fileName ?? (content || extraInfo?.fileName || type);
    const parts = type === 'text'
      ? undefined
      : buildMessageParts(mediaUpload?.content ?? content, type, extraInfo, mediaUpload);
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
          renderHints: buildMessageRenderHints(type, extraInfo),
        });

    const {
      file: _file,
      mimeType: _mimeType,
      ...localExtraInfo
    } = extraInfo ?? {};
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
    this.latestReadSeq.set(chatId, Math.max(this.latestReadSeq.get(chatId) ?? 0, postResult.messageSeq));
    const subscription = this.liveSubscriptions.get(chatId);
    if (subscription) {
      this.notifyLiveSubscription(subscription, storedMessage);
    }
    await this.emitChatList().catch(() => undefined);
    return storedMessage;
  }

  async forwardMessages(targetChatIds: string[], messages: Message[]): Promise<void> {
    for (const targetChatId of targetChatIds) {
      for (const message of messages) {
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
      }
    }
  }

  async markAsRead(chatId: string): Promise<void> {
    const readSeq = this.latestReadSeq.get(chatId) ?? 0;
    if (readSeq > 0) {
      await this.client().conversations.updateReadCursor(chatId, { readSeq });
    }
    await this.client().conversations.updatePreferences(chatId, { isMarkedUnread: false });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: false,
    });
  }

  async markAsUnread(chatId: string): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isMarkedUnread: true });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: true,
    });
  }

  async deleteMessage(chatId: string, messageId: string): Promise<void> {
    await this.client().messages.deleteForMe(messageId);
    const messages = this.localMessages.get(chatId) ?? [];
    this.localMessages.set(chatId, messages.filter((message) => message.id !== messageId));
  }

  async recallMessage(chatId: string, messageId: string): Promise<void> {
    await this.client().recallMessage(messageId);
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
    await this.client().editMessage(messageId, { text: trimmed });
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
    await this.client().conversations.updatePreferences(chatId, { isHidden: true });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isHidden: true,
    });
    this.localMessages.delete(chatId);
  }

  async pinChat(chatId: string, isPinned: boolean): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isPinned });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isPinned,
    });
  }

  async muteChat(chatId: string, isMuted: boolean): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isMuted });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isMuted,
    });
  }

  async addReaction(chatId: string, messageId: string, emoji: string): Promise<void> {
    await this.client().addReaction(messageId, emoji);
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
    await this.client().removeReaction(messageId, emoji);
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
    const profileUpdate = buildConversationProfileUpdate(updates);
    if (hasProfileUpdate(profileUpdate)) {
      const profile = await this.client().conversations.updateProfile(chatId, profileUpdate);
      this.conversationViewState.set(chatId, applyConversationProfile(
        this.conversationViewState.get(chatId),
        profile,
      ));
    }
    const localViewUpdate = buildLocalConversationViewUpdate(updates);
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      ...localViewUpdate,
    });
    const updated = (await this.getChats()).find((chat) => chat.id === chatId);
    if (!updated) {
      throw new Error('Chat not found');
    }
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      ...localViewUpdate,
    });
    return {
      ...updated,
      ...localViewUpdate,
    };
  }

  async createChat(chat: Chat): Promise<void> {
    await this.client().conversations.create({
      conversationId: chat.id,
      conversationType: chat.type,
    });
    const profileUpdate = buildConversationProfileUpdate(chat);
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(chat.id, profileUpdate);
    }
    await this.client().conversations.updatePreferences(chat.id, { isHidden: false });
    this.conversationViewState.set(chat.id, {
      avatar: chat.avatar,
      isHidden: false,
      memberCount: chat.memberCount,
      name: chat.name,
      notice: chat.notice,
      type: chat.type,
    });
    if (chat.lastMessage) {
      this.localMessages.set(chat.id, [chat.lastMessage]);
    }
  }

  async startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { conversationId?: string; directChatId?: string; id: string }): Promise<Chat> {
    const targetUserId = user.id.trim();
    if (!targetUserId) {
      throw new Error('Direct chat target user id is required');
    }
    const projectedConversationId = user.conversationId?.trim();
    if (projectedConversationId) {
      await this.client().conversations.updatePreferences(projectedConversationId, { isHidden: false });
      this.conversationViewState.set(projectedConversationId, {
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
    const currentUser = contactService.getCurrentUser();
    const currentUserId = currentUser.id.trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }
    const result = await this.client().conversations.bindDirectChat({
      leftActorId: currentUserId,
      leftActorKind: 'user',
      rightActorId: targetUserId,
      rightActorKind: 'user',
    });
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: user.avatar,
      name: user.name,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(boundConversationId, profileUpdate);
    }
    await this.client().conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.conversationViewState.set(boundConversationId, {
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
    const currentUser = contactService.getCurrentUser();
    const currentUserId = currentUser.id.trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }

    const result = await this.client().conversations.createAgentDialog({
      agentId,
    });
    const boundConversationId = result.conversationId;
    this.conversationViewState.set(boundConversationId, {
      ...this.conversationViewState.get(boundConversationId),
      avatar: agent.avatar,
      isHidden: false,
      name: agent.name,
      type: 'single',
      welcomeMessage: agent.welcomeMessage,
    });
    const profileUpdate = buildConversationProfileUpdate({
      avatar: agent.avatar,
      name: agent.name,
    });
    try {
      if (hasProfileUpdate(profileUpdate)) {
        await this.client().conversations.updateProfile(boundConversationId, profileUpdate);
      }
      await this.client().conversations.updatePreferences(boundConversationId, { isHidden: false });
    } catch {
      // Keep local naming/avatar usable if profile sync is temporarily unavailable.
    }
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
    const currentUser = contactService.getCurrentUser();
    const currentUserId = currentUser.id.trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }

    const displayName = enterprise.name.endsWith(' (Official)')
      ? enterprise.name
      : `${enterprise.name} (Official)`;
    const result = await this.client().conversations.bindDirectChat({
      leftActorId: currentUserId,
      leftActorKind: 'user',
      rightActorId: enterpriseId,
      rightActorKind: 'enterprise',
    });
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: enterprise.avatar,
      name: displayName,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(boundConversationId, profileUpdate);
    }
    await this.client().conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.conversationViewState.set(boundConversationId, {
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
    const chats = await this.getChats();
    let appliedMessages = 0;
    let refreshedChats = 0;

    for (const chat of chats) {
      const messages = await this.getMessages(chat.id);
      appliedMessages += messages.length;
      refreshedChats += 1;
    }

    return {
      appliedMessages,
      refreshedChats,
    };
  }

  recoverRealtimeConnection(reason = 'realtime recovery requested'): void {
    if (!this.hasLiveSubscriptionDemand()) {
      return;
    }
    recoverPcLiveConnection(reason);
  }

  private getOrCreateLiveSubscription(chatId: string): ConversationLiveSubscription {
    const existing = this.liveSubscriptions.get(chatId);
    if (existing) {
      return existing;
    }

    const subscription: ConversationLiveSubscription = {
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
    if (this.conversationWireUnsubs.has(conversationId)) {
      return;
    }
    const unsubscribe = subscribePcConversationMessages(
      conversationId,
      (message, context) => {
        this.handleLiveMessage(conversationId, message, context);
      },
    );
    this.conversationWireUnsubs.set(conversationId, unsubscribe);
  }

  private releaseConversationWireSubscription(conversationId: string): void {
    this.conversationWireUnsubs.get(conversationId)?.();
    this.conversationWireUnsubs.delete(conversationId);
  }

  private ensureInboxWireSubscription(): void {
    if (this.liveInboxWireUnsub || this.chatListHandlers.size === 0) {
      return;
    }
    const scopes = this.getChatListRealtimeScopes();
    const inboxScope = scopes[0];
    if (!inboxScope) {
      return;
    }
    this.liveInboxWireUnsub = subscribePcRealtimeScope(inboxScope, (context) => {
      this.handleLiveScopeEvent(context);
    });
  }

  private releaseInboxWireSubscription(): void {
    this.liveInboxWireUnsub?.();
    this.liveInboxWireUnsub = undefined;
  }

  private ensureLiveSession(): void {
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
    if (this.liveSubscriptions.size === 0 && this.chatListHandlers.size === 0) {
      this.releaseInboxWireSubscription();
    }
  }

  private closeAllLiveSubscriptions(_reason: string): void {
    for (const conversationId of [...this.liveSubscriptions.keys()]) {
      this.releaseConversationWireSubscription(conversationId);
    }
    this.liveSubscriptions.clear();
    this.releaseInboxWireSubscription();
  }

  private closeLiveSession(_reason: string): void {
    this.releaseInboxWireSubscription();
  }

  private hasLiveSubscriptionDemand(): boolean {
    return this.liveSubscriptions.size > 0 || this.chatListHandlers.size > 0;
  }

  private async catchUpOnConnectionOpen(): Promise<void> {
    if (!this.hasLiveSubscriptionDemand()) {
      return;
    }

    const conversationIds = Array.from(this.liveSubscriptions.keys());
    for (const conversationId of conversationIds) {
      if (!this.liveSubscriptions.has(conversationId)) {
        return;
      }
      await this.catchUpConversationMessages(conversationId).catch(() => undefined);
    }
    if (this.chatListHandlers.size > 0) {
      await this.emitChatList().catch(() => undefined);
    }
  }

  private async catchUpConversationMessages(conversationId: string): Promise<void> {
    if (this.liveCatchUpConversations.has(conversationId)) {
      return;
    }

    this.liveCatchUpConversations.add(conversationId);
    try {
      await this.doCatchUpConversationMessages(conversationId);
    } finally {
      this.liveCatchUpConversations.delete(conversationId);
    }
  }

  private async doCatchUpConversationMessages(conversationId: string): Promise<void> {
    const checkpointSeq = this.latestReadSeq.get(conversationId) ?? 0;
    if (checkpointSeq <= 0) {
      return;
    }

    const entries: TimelineViewEntry[] = [];
    let afterSeq = checkpointSeq;
    while (true) {
      const response = await this.client().conversations.listMessages(conversationId, {
        afterSeq,
        limit: MESSAGE_PAGE_LIMIT,
      });
      entries.push(...response.items.filter((entry) => entry.messageSeq > checkpointSeq));

      if (
        !response.hasMore
        || typeof response.nextAfterSeq !== 'number'
        || response.nextAfterSeq <= afterSeq
      ) {
        break;
      }

      afterSeq = response.nextAfterSeq;
    }

    if (entries.length === 0) {
      return;
    }

    const cachedMessages = new Map(
      (this.localMessages.get(conversationId) ?? []).map((message) => [message.id, message]),
    );
    const messages = await this.enrichMessagesWithInteractionSummaries(
      conversationId,
      entries,
      cachedMessages,
    );

    const mergedMessages = mergeMessageLists(messages, this.localMessages.get(conversationId) ?? []);
    this.localMessages.set(conversationId, mergedMessages);
    for (const entry of entries) {
      this.latestReadSeq.set(conversationId, Math.max(
        this.latestReadSeq.get(conversationId) ?? 0,
        entry.messageSeq,
      ));
    }

    const subscription = this.liveSubscriptions.get(conversationId);
    if (!subscription) {
      return;
    }
    for (const message of messages) {
      this.notifyLiveSubscription(subscription, message);
    }
    void this.emitChatList().catch(() => undefined);
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

  private handleLiveScopeEvent(context: ImRealtimeEventContext): void {
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
      if (storedMessage.senderId !== this.resolveCurrentUserId()) {
        this.conversationViewState.set(message.chatId, {
          ...this.conversationViewState.get(message.chatId),
          isMarkedUnread: true,
        });
      }
      const messageSeq = pickNumber(context.payload?.messageSeq, context.sequence) ?? 0;
      this.latestReadSeq.set(message.chatId, Math.max(this.latestReadSeq.get(message.chatId) ?? 0, messageSeq));
      const subscription = this.liveSubscriptions.get(message.chatId);
      if (subscription) {
        this.notifyLiveSubscription(subscription, storedMessage);
      }
    }

    void context.ack().catch(() => undefined);
    void this.emitChatList().catch(() => undefined);
  }

  private handleLiveMessage(
    fallbackChatId: string,
    decodedMessage: ImDecodedMessage,
    context: ImMessageContext,
  ): void {
    const message = mapLiveMessageToMessage(fallbackChatId, decodedMessage, context);
    const isRtcCallUpdate = Boolean(resolveRtcCallDisplayState(message));
    const storedMessage = this.upsertLocalMessage(message.chatId, message, isRtcCallUpdate);
    const messageSeq = pickNumber(decodedMessage.messageSeq, context.payload?.messageSeq, context.sequence) ?? 0;
    this.latestReadSeq.set(message.chatId, Math.max(this.latestReadSeq.get(message.chatId) ?? 0, messageSeq));
    const subscription = this.liveSubscriptions.get(message.chatId) ?? this.liveSubscriptions.get(fallbackChatId);
    if (subscription) {
      this.notifyLiveSubscription(subscription, storedMessage);
    }
    void this.emitChatList().catch(() => undefined);
    void context.ack().catch(() => undefined);
  }

  private async emitChatList(): Promise<void> {
    if (this.chatListHandlers.size === 0) {
      return;
    }
    const chats = await this.getChats();
    for (const handler of this.chatListHandlers) {
      handler(chats);
    }
  }

  private notifyLiveSubscription(subscription: ConversationLiveSubscription, message: Message): void {
    const nextVersion = buildMessageNotificationVersion(message);
    if (subscription.notifiedMessageVersions.get(message.id) === nextVersion) {
      return;
    }
    subscription.notifiedMessageVersions.set(message.id, nextVersion);
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
    this.localMessages.set(
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
    this.localMessages.set(
      chatId,
      messages.map((message) => message.id === messageId ? updater(message) : message),
    );
  }
}

export function createSdkworkChatService(dependencies?: ChatServiceDependencies | (() => ImSdkClient)): ChatService {
  return new SdkworkChatService(dependencies);
}

export function resolveIncomingCallWatchConversationIds(
  chats: Array<{ id: string; conversationId?: string }>,
  contacts: Array<{ conversationId?: string; id: string }>,
  _currentUserId: string,
): string[] {
  const conversationIds = new Set<string>();
  for (const chat of chats) {
    const conversationId = chat.conversationId?.trim() || chat.id.trim();
    if (conversationId) {
      conversationIds.add(conversationId);
    }
  }
  for (const contact of contacts) {
    const conversationId = contact.conversationId?.trim();
    if (conversationId) {
      conversationIds.add(conversationId);
    }
  }
  return [...conversationIds];
}

export const chatService = createSdkworkChatService();
