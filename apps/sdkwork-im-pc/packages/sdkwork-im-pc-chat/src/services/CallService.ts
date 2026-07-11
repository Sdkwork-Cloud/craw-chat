import type {
  ImCallParticipantCredential,
  ImCallSession,
  ImCallSessionMutationResponse,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/imSdkClient';
import {
  readAppSdkSessionTokens,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core/sdk/session';
import {
  acquirePcLiveConnectionLease,
  configurePcRealtimeConnectionManager,
  ensurePcLiveConnection,
  subscribePcRealtimeScope,
} from '@sdkwork/im-pc-core/sdk/pcRealtimeConnectionManager';
import { resolveSdkworkChatPcClientId } from './ClientIdentityService';
import {
  resolveRtcMediaPublishKinds,
  rtcMediaService,
  type SdkworkRtcMediaService,
} from './RtcMediaService';

export type SdkworkCallType = 'voice' | 'video';

export type SdkworkCallState =
  | 'idle'
  | 'ringing'
  | 'connecting'
  | 'connected'
  | 'ended'
  | 'rejected'
  | 'errored';

export type SdkworkCallControllerState =
  | 'idle'
  | 'watching'
  | 'incoming_ringing'
  | 'outgoing_ringing'
  | 'connecting'
  | 'connected'
  | 'ended'
  | 'rejected'
  | 'errored';

export interface SdkworkCallSnapshot {
  accessEndpoint?: string;
  state: SdkworkCallState;
  controllerState?: SdkworkCallControllerState;
  conversationId?: string;
  direction?: 'incoming' | 'outgoing';
  errorMessage?: string;
  initiatorId?: string;
  isParticipantCredentialReady?: boolean;
  isAudioMuted: boolean;
  isVideoMuted: boolean;
  participantCredentialExpiresAt?: string;
  participantId?: string;
  peerUserId?: string;
  providerKey?: string;
  providerRegion?: string;
  roomId?: string;
  rtcMode?: string;
  rtcSessionId?: string;
  targetName?: string;
  type?: SdkworkCallType;
}

export interface StartOutgoingCallOptions {
  conversationId: string;
  targetName: string;
  targetUserId?: string;
  type: SdkworkCallType;
}

export interface RecoverRtcSessionOptions {
  targetName?: string;
  type?: SdkworkCallType;
}

export interface EndCallOptions {
  reason?: string;
}

export interface WatchIncomingCallsOptions {
  conversationIds: string[];
}

export interface RecoverActiveCallResult {
  recovered: boolean;
  snapshot: SdkworkCallSnapshot;
}

export interface CallService {
  acceptIncomingCall(): Promise<SdkworkCallSnapshot>;
  bindLocalVideoElement(element: HTMLElement | null): Promise<void>;
  bindRemoteVideoElement(remoteUserId: string | null | undefined, element: HTMLElement | null): Promise<void>;
  endCall(options?: EndCallOptions): Promise<void>;
  getSnapshot(): SdkworkCallSnapshot;
  recoverActiveCall(): Promise<RecoverActiveCallResult>;
  recoverRtcSession(rtcSessionId: string, options?: RecoverRtcSessionOptions): Promise<SdkworkCallSnapshot>;
  rejectIncomingCall(options?: EndCallOptions): Promise<SdkworkCallSnapshot>;
  setAudioMuted(muted: boolean): Promise<SdkworkCallSnapshot>;
  setVideoMuted(muted: boolean): Promise<SdkworkCallSnapshot>;
  startOutgoingCall(options: StartOutgoingCallOptions): Promise<SdkworkCallSnapshot>;
  subscribe(handler: (snapshot: SdkworkCallSnapshot) => void): () => void;
  watchIncomingCalls(conversationIds: string[]): Promise<SdkworkCallSnapshot>;
}

export interface SdkworkCallServiceDependencies {
  getClient?: (session: SdkworkChatSession | null) => ImSdkClient;
  readSession?: () => SdkworkChatSession | null;
  rtcMediaService?: SdkworkRtcMediaService;
}

function createIdleSnapshot(): SdkworkCallSnapshot {
  return {
    state: 'idle',
    controllerState: 'idle',
    isAudioMuted: false,
    isVideoMuted: false,
  };
}

function normalizeIdSegment(value: string): string {
  return value
    .trim()
    .replace(/[^a-zA-Z0-9_-]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .slice(0, 48);
}

function createRuntimeId(prefix: string, stablePart: string): string {
  const randomPart =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `${prefix}-${normalizeIdSegment(stablePart) || 'conversation'}-${randomPart}`;
}

function resolveParticipantId(session: SdkworkChatSession | null): string {
  const candidate =
    session?.context?.userId
    ?? session?.user?.userId
    ?? session?.user?.id
    ?? session?.sessionId
    ?? session?.context?.sessionId;
  if (candidate) {
    return String(candidate);
  }
  throw new Error('Sdkwork IM login session does not include a user id for calls.');
}

function toRtcMode(type: SdkworkCallType): string {
  return type === 'video' ? 'video' : 'voice';
}

function toCallType(rtcMode: string | undefined): SdkworkCallType {
  return rtcMode === 'video' || rtcMode === 'video_call' ? 'video' : 'voice';
}

function toRecoveredServiceState(state: ImCallSession['state']): SdkworkCallState {
  switch (state) {
    case 'accepted':
    case 'connecting':
    case 'connected':
      return 'connected';
    case 'rejected':
      return 'rejected';
    case 'ended':
    case 'canceled':
    case 'failed':
    case 'timeout':
      return 'ended';
    case 'on_hold':
    case 'reconnecting':
      return 'connected';
    case 'started':
    case 'initiating':
    case 'ringing':
    default:
      return 'ringing';
  }
}

function toControllerState(state: SdkworkCallState, direction?: 'incoming' | 'outgoing'): SdkworkCallControllerState {
  switch (state) {
    case 'ringing':
      return direction === 'incoming' ? 'incoming_ringing' : 'outgoing_ringing';
    case 'connecting':
    case 'connected':
    case 'ended':
    case 'rejected':
    case 'errored':
      return state;
    case 'idle':
    default:
      return 'idle';
  }
}

function resolvePeerUserId(session: ImCallSession, participantId: string | undefined): string | undefined {
  if (!session.initiatorId) {
    return undefined;
  }
  if (!participantId || session.initiatorId !== participantId) {
    return session.initiatorId;
  }
  return undefined;
}

function resolveCallAccessEndpoint(session: ImCallSession): string | undefined {
  return session.accessEndpoint ?? undefined;
}

function sessionFromMutation(response: ImCallSessionMutationResponse): ImCallSession {
  return response;
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return typeof error === 'string' && error.trim().length > 0 ? error : 'Call signaling failed.';
}

// 检测 SDK 抛出的 404 / 资源不存在错误。SDK 底层抛出 SdkError(NotFoundError)，
// 其 code === 'NOT_FOUND' 且 httpStatus === 404。由于本包不直接依赖 @sdkwork/sdk-common，
// 这里使用鸭子类型检测以避免引入额外耦合。
function isNotFoundHttpError(error: unknown): boolean {
  if (!error || typeof error !== 'object') {
    return false;
  }
  const record = error as Record<string, unknown>;
  if (record.httpStatus === 404 || record.code === 'NOT_FOUND') {
    return true;
  }
  if (error instanceof Error) {
    return /not\s+found/iu.test(error.message) || error.name === 'NotFoundError';
  }
  return false;
}

// --- 活跃通话持久化（用于页面刷新后恢复） ---
const ACTIVE_CALL_STORAGE_KEY = 'sdkwork-im-pc:active-call:v1';

interface PersistedActiveCall {
  rtcSessionId: string;
  conversationId?: string;
  direction?: 'incoming' | 'outgoing';
  targetName?: string;
  type?: SdkworkCallType;
}

function readPersistedActiveCall(): PersistedActiveCall | null {
  if (typeof window === 'undefined') {
    return null;
  }
  try {
    const raw = window.localStorage.getItem(ACTIVE_CALL_STORAGE_KEY);
    if (!raw) {
      return null;
    }
    const parsed = JSON.parse(raw) as Partial<PersistedActiveCall>;
    if (typeof parsed.rtcSessionId !== 'string' || !parsed.rtcSessionId.trim()) {
      return null;
    }
    return parsed as PersistedActiveCall;
  } catch {
    return null;
  }
}

function writePersistedActiveCall(call: PersistedActiveCall | null): void {
  if (typeof window === 'undefined') {
    return;
  }
  try {
    if (call) {
      window.localStorage.setItem(ACTIVE_CALL_STORAGE_KEY, JSON.stringify(call));
    } else {
      window.localStorage.removeItem(ACTIVE_CALL_STORAGE_KEY);
    }
  } catch {
    // localStorage 不可用时静默降级，不影响通话功能。
  }
}

class SdkworkImCallService implements CallService {
  private readonly listeners = new Set<(snapshot: SdkworkCallSnapshot) => void>();
  private readonly getClient: (session: SdkworkChatSession | null) => ImSdkClient;
  private readonly readSession: () => SdkworkChatSession | null;
  private readonly rtcMediaService: SdkworkRtcMediaService;
  private activeMediaRtcSessionId?: string;
  private incomingConnectionLease?: () => void;
  private incomingScopeSubscription?: () => void;
  private incomingSubscription?: () => void;
  private participantCredential?: ImCallParticipantCredential;
  private snapshot: SdkworkCallSnapshot = createIdleSnapshot();
  private sequence = 0;

  constructor(dependencies: SdkworkCallServiceDependencies = {}) {
    this.getClient = dependencies.getClient ?? ((session) => getImSdkClientWithSession(session));
    this.readSession = dependencies.readSession ?? readAppSdkSessionTokens;
    this.rtcMediaService = dependencies.rtcMediaService ?? rtcMediaService;
    this.configureRealtimeConnectionManager();
  }

  getSnapshot(): SdkworkCallSnapshot {
    return {
      ...this.snapshot,
    };
  }

  subscribe(handler: (snapshot: SdkworkCallSnapshot) => void): () => void {
    this.listeners.add(handler);
    handler(this.getSnapshot());
    return () => {
      this.listeners.delete(handler);
    };
  }

  async startOutgoingCall(options: StartOutgoingCallOptions): Promise<SdkworkCallSnapshot> {
    const sequence = ++this.sequence;
    const session = this.readSession();
    const participantId = resolveParticipantId(session);
    const rtcMode = toRtcMode(options.type);
    const rtcSessionId = createRuntimeId('call-pc', options.conversationId);
    const signalingStreamId = createRuntimeId('call-signal', rtcSessionId);

    this.applySnapshot({
      ...createIdleSnapshot(),
      state: 'ringing',
      controllerState: 'outgoing_ringing',
      conversationId: options.conversationId,
      direction: 'outgoing',
      isAudioMuted: this.snapshot.isAudioMuted,
      isVideoMuted: options.type === 'voice' ? true : this.snapshot.isVideoMuted,
      participantId,
      roomId: rtcSessionId,
      rtcMode,
      rtcSessionId,
      peerUserId: options.targetUserId,
      targetName: options.targetName,
      type: options.type,
    });

    try {
      const imClient = this.getClient(session);
      const created = await imClient.calls.start({
        conversationId: options.conversationId,
        rtcMode,
        rtcSessionId,
      });
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }
      await imClient.calls.invite(created.rtcSessionId, { signalingStreamId });
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }
      this.applySessionSnapshot(sessionFromMutation(created), {
        direction: 'outgoing',
        participantId,
        state: 'ringing',
        targetName: options.targetName,
        type: options.type,
      });
      return this.getSnapshot();
    } catch (error) {
      if (sequence === this.sequence) {
        this.applySnapshot({
          ...this.snapshot,
          state: 'errored',
          controllerState: 'errored',
          errorMessage: toErrorMessage(error),
        });
      }
      return this.getSnapshot();
    }
  }

  async setAudioMuted(muted: boolean): Promise<SdkworkCallSnapshot> {
    const previousSnapshot = this.snapshot;
    this.applySnapshot({
      ...this.snapshot,
      isAudioMuted: muted,
    });
    try {
      await this.rtcMediaService.muteAudio(muted);
    } catch (error) {
      this.applySnapshot(previousSnapshot);
      throw error;
    }
    return this.getSnapshot();
  }

  async setVideoMuted(muted: boolean): Promise<SdkworkCallSnapshot> {
    const previousSnapshot = this.snapshot;
    this.applySnapshot({
      ...this.snapshot,
      isVideoMuted: muted,
    });
    try {
      await this.rtcMediaService.muteVideo(muted);
    } catch (error) {
      this.applySnapshot(previousSnapshot);
      throw error;
    }
    return this.getSnapshot();
  }

  async bindLocalVideoElement(element: HTMLElement | null): Promise<void> {
    await this.rtcMediaService.bindLocalVideoElement(element);
  }

  async bindRemoteVideoElement(
    remoteUserId: string | null | undefined,
    element: HTMLElement | null,
  ): Promise<void> {
    await this.rtcMediaService.bindRemoteVideoElement(remoteUserId, element);
  }

  async watchIncomingCalls(conversationIds: string[]): Promise<SdkworkCallSnapshot> {
    const normalizedConversationIds = [...new Set(
      conversationIds
        .map((conversationId) => String(conversationId).trim())
        .filter((conversationId) => conversationId.length > 0),
    )];

    // Note: we do NOT early-return when hasActiveCall() is true. The
    // subscription's conversationIds filter must be refreshed so that
    // events for newly created conversations (e.g. a direct chat created
    // moments before placing an outgoing call) are not silently dropped.
    // The snapshot overwrite guard below (rtcSessionId && controllerState
    // !== 'watching') prevents clobbering an active call's state.

    try {
      const session = this.readSession();
      const principalId = resolveParticipantId(session);
      const imClient = this.getClient(session);
      this.configureRealtimeConnectionManager();
      this.incomingSubscription?.();
      this.incomingSubscription = undefined;
      this.incomingScopeSubscription?.();
      this.incomingScopeSubscription = subscribePcRealtimeScope(
        { scopeType: 'user', scopeId: principalId },
        () => undefined,
      );
      this.incomingSubscription = imClient.calls.subscribe((callSession) => {
        // 活跃通话的事件必须优先处理，不受 conversationIds 过滤限制。
        // 用户可能在通话期间切换到其他会话，此时活跃通话的 accepted/ended
        // 事件仍需正确投递，否则主叫会永远振铃或媒体无法释放。
        if (this.snapshot.rtcSessionId && this.snapshot.rtcSessionId === callSession.rtcSessionId) {
          const participantId = this.snapshot.participantId ?? resolveParticipantId(this.readSession());
          this.applySessionSnapshot(callSession, {
            direction: this.snapshot.direction,
            participantId,
          });
          if (this.snapshot.state === 'connected') {
            void this.ensureParticipantCredentialReady(callSession.rtcSessionId, this.readSession())
              .catch((error) => {
                if (this.snapshot.rtcSessionId !== callSession.rtcSessionId || this.snapshot.state !== 'connected') {
                  return;
                }
                this.applySnapshot({
                  ...this.snapshot,
                  state: 'errored',
                  controllerState: 'errored',
                  errorMessage: toErrorMessage(error),
                });
              });
          }
          return;
        }
        // 非活跃通话事件应用 conversationIds 过滤（仅处理当前关注的会话来电）。
        if (
          normalizedConversationIds.length > 0
          && !normalizedConversationIds.includes(callSession.conversationId ?? '')
        ) {
          return;
        }
        if (this.hasActiveCall()) {
          return;
        }
        const incomingState = toRecoveredServiceState(callSession.state);
        if (incomingState !== 'ringing') {
          return;
        }
        this.applySessionSnapshot(callSession, {
          direction: 'incoming',
          participantId: resolveParticipantId(this.readSession()),
          state: incomingState,
        });
      });
      this.incomingConnectionLease?.();
      this.incomingConnectionLease = acquirePcLiveConnectionLease('calls-incoming-watch', {
        conversationIds: normalizedConversationIds,
      });
      const connection = await ensurePcLiveConnection();
      const incoming = await imClient.calls.watchIncoming({
        connection,
        conversationIds: normalizedConversationIds,
        deviceId: resolveSdkworkChatPcClientId(),
        principalId,
      });
      if (this.snapshot.rtcSessionId && this.snapshot.controllerState !== 'watching') {
        return this.getSnapshot();
      }
      if (
        incoming
        && (
          normalizedConversationIds.length === 0
          || normalizedConversationIds.includes(incoming.conversationId ?? '')
        )
      ) {
        this.applySessionSnapshot(incoming, {
          direction: 'incoming',
          participantId: resolveParticipantId(session),
          state: 'ringing',
        });
      } else {
        this.applySnapshot({
          ...createIdleSnapshot(),
          controllerState: 'watching',
          participantId: resolveParticipantId(session),
        });
      }
    } catch (error) {
      this.incomingSubscription?.();
      this.incomingSubscription = undefined;
      this.incomingScopeSubscription?.();
      this.incomingScopeSubscription = undefined;
      this.incomingConnectionLease?.();
      this.incomingConnectionLease = undefined;
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        controllerState: 'errored',
        errorMessage: toErrorMessage(error),
      });
    }

    return this.getSnapshot();
  }

  async acceptIncomingCall(): Promise<SdkworkCallSnapshot> {
    const rtcSessionId = this.snapshot.rtcSessionId;
    if (!rtcSessionId) {
      this.applyUnavailableIncomingSnapshot();
      return this.getSnapshot();
    }

    try {
      const session = this.readSession();
      const imClient = this.getClient(session);
      const accepted = await imClient.calls.accept(rtcSessionId);
      this.applySessionSnapshot(sessionFromMutation(accepted), {
        direction: this.snapshot.direction ?? 'incoming',
        participantId: this.snapshot.participantId ?? resolveParticipantId(session),
        state: 'connected',
      });
      await this.ensureParticipantCredentialReady(rtcSessionId, session);
      return this.getSnapshot();
    } catch (error) {
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        controllerState: 'errored',
        errorMessage: toErrorMessage(error),
      });
      return this.getSnapshot();
    }
  }

  async rejectIncomingCall(_options: EndCallOptions = {}): Promise<SdkworkCallSnapshot> {
    const rtcSessionId = this.snapshot.rtcSessionId;
    if (!rtcSessionId) {
      this.applyUnavailableIncomingSnapshot();
      return this.getSnapshot();
    }

    try {
      const session = this.readSession();
      const imClient = this.getClient(session);
      const rejected = await imClient.calls.reject(rtcSessionId);
      this.applySessionSnapshot(sessionFromMutation(rejected), {
        direction: this.snapshot.direction ?? 'incoming',
        participantId: this.snapshot.participantId ?? resolveParticipantId(session),
        state: 'rejected',
      });
      await this.releaseRtcMedia();
      return this.getSnapshot();
    } catch (error) {
      // 当会话已被对端取消或超时清理时，reject 接口返回 404。
      // 此时应视为已拒绝而非错误。
      const treatAsRejected = isNotFoundHttpError(error);
      this.applySnapshot({
        ...this.snapshot,
        state: treatAsRejected ? 'rejected' : 'errored',
        controllerState: treatAsRejected ? 'rejected' : 'errored',
        errorMessage: treatAsRejected ? undefined : toErrorMessage(error),
      });
      if (treatAsRejected) {
        await this.releaseRtcMedia();
      }
      return this.getSnapshot();
    }
  }

  async endCall(_options: EndCallOptions = {}): Promise<void> {
    ++this.sequence;
    const rtcSessionId = this.snapshot.rtcSessionId;
    if (!rtcSessionId) {
      this.applySnapshot({
        ...this.snapshot,
        state: this.snapshot.state === 'idle' ? 'idle' : 'ended',
        controllerState: this.snapshot.state === 'idle' ? 'idle' : 'ended',
      });
      return;
    }

    try {
      const session = this.readSession();
      const imClient = this.getClient(session);
      const ended = await imClient.calls.end(rtcSessionId);
      this.applySessionSnapshot(sessionFromMutation(ended), {
        direction: this.snapshot.direction,
        participantId: this.snapshot.participantId ?? resolveParticipantId(session),
        state: 'ended',
      });
      await this.releaseRtcMedia();
    } catch (error) {
      await this.releaseRtcMedia();
      // 当会话已被对端结束或因超时被服务端清理时，end 接口返回 404。
      // 此时应视为已结束而非错误，避免 UI 进入 errored 状态。
      const treatAsEnded = isNotFoundHttpError(error);
      this.applySnapshot({
        ...this.snapshot,
        state: treatAsEnded ? 'ended' : 'errored',
        controllerState: treatAsEnded ? 'ended' : 'errored',
        errorMessage: treatAsEnded ? undefined : toErrorMessage(error),
        isParticipantCredentialReady: false,
        participantCredentialExpiresAt: undefined,
      });
    }
  }

  async recoverRtcSession(
    rtcSessionId: string,
    options: RecoverRtcSessionOptions = {},
  ): Promise<SdkworkCallSnapshot> {
    const sequence = ++this.sequence;
    const session = this.readSession();
    const participantId = resolveParticipantId(session);

    try {
      const imClient = this.getClient(session);
      const callSession = await imClient.calls.retrieve(rtcSessionId);
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }

      this.applySessionSnapshot(callSession, {
        direction: this.snapshot.direction,
        participantId,
        state: toRecoveredServiceState(callSession.state),
        targetName: options.targetName,
        type: options.type ?? toCallType(callSession.rtcMode),
      });
      if (this.snapshot.state === 'connected') {
        await this.ensureParticipantCredentialReady(rtcSessionId, session);
      }
      return this.getSnapshot();
    } catch (error) {
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }
      // 恢复时如果会话已不存在（404），视为已结束而非错误。
      const treatAsEnded = isNotFoundHttpError(error);
      this.applySnapshot({
        ...this.snapshot,
        state: treatAsEnded ? 'ended' : 'errored',
        controllerState: treatAsEnded ? 'ended' : 'errored',
        errorMessage: treatAsEnded ? undefined : toErrorMessage(error),
        isParticipantCredentialReady: false,
        participantCredentialExpiresAt: undefined,
      });
      return this.getSnapshot();
    }
  }

  async recoverActiveCall(): Promise<RecoverActiveCallResult> {
    const persisted = readPersistedActiveCall();
    if (!persisted) {
      return { recovered: false, snapshot: this.getSnapshot() };
    }
    // 如果当前已有活跃通话，不需要恢复。
    if (this.hasActiveCall()) {
      return { recovered: false, snapshot: this.getSnapshot() };
    }
    try {
      const snapshot = await this.recoverRtcSession(persisted.rtcSessionId, {
        targetName: persisted.targetName,
        type: persisted.type,
      });
      // 如果恢复后发现会话已结束（404 或 retrieve 返回终态），清除持久化。
      if (snapshot.state === 'ended' || snapshot.state === 'rejected' || snapshot.state === 'errored') {
        writePersistedActiveCall(null);
        return { recovered: false, snapshot };
      }
      // 恢复方向信息（recoverRtcSession 使用 snapshot.direction，初始为 undefined）
      if (!snapshot.direction && persisted.direction) {
        this.applySnapshot({
          ...this.snapshot,
          direction: persisted.direction,
          controllerState: toControllerState(this.snapshot.state, persisted.direction),
        });
      }
      return { recovered: true, snapshot: this.getSnapshot() };
    } catch {
      writePersistedActiveCall(null);
      return { recovered: false, snapshot: this.getSnapshot() };
    }
  }

  private applySessionSnapshot(
    session: ImCallSession,
    options: {
      direction?: 'incoming' | 'outgoing';
      participantId?: string;
      state?: SdkworkCallState;
      targetName?: string;
      type?: SdkworkCallType;
    } = {},
  ): void {
    const callType = options.type ?? toCallType(session.rtcMode);
    const state = options.state ?? toRecoveredServiceState(session.state);
    const participantId = options.participantId ?? this.snapshot.participantId;
    this.applySnapshot({
      ...this.snapshot,
      state,
      controllerState: toControllerState(state, options.direction ?? this.snapshot.direction),
      conversationId: session.conversationId ?? this.snapshot.conversationId,
      direction: options.direction ?? this.snapshot.direction,
      errorMessage: undefined,
      initiatorId: session.initiatorId ?? this.snapshot.initiatorId,
      accessEndpoint: resolveCallAccessEndpoint(session) ?? this.snapshot.accessEndpoint,
      isParticipantCredentialReady: state === 'connected' ? this.snapshot.isParticipantCredentialReady : false,
      participantCredentialExpiresAt: state === 'connected' ? this.snapshot.participantCredentialExpiresAt : undefined,
      participantId,
      peerUserId: resolvePeerUserId(session, participantId) ?? this.snapshot.peerUserId,
      providerKey: session.providerPluginId ?? this.snapshot.providerKey,
      providerRegion: session.providerRegion ?? this.snapshot.providerRegion,
      roomId: session.providerSessionId ?? session.rtcSessionId,
      rtcMode: session.rtcMode,
      rtcSessionId: session.rtcSessionId,
      targetName:
        options.targetName
        ?? this.snapshot.targetName,
      type: callType,
      isVideoMuted: callType === 'voice' ? true : this.snapshot.isVideoMuted,
    });
    if (state !== 'connected') {
      this.participantCredential = undefined;
      if (state === 'ended' || state === 'rejected' || state === 'errored') {
        void this.releaseRtcMedia().catch(() => undefined);
      }
    }
  }

  private hasActiveCall(): boolean {
    return Boolean(
      this.snapshot.rtcSessionId
        && this.snapshot.controllerState !== 'watching'
        && this.snapshot.state !== 'idle'
        && this.snapshot.state !== 'ended'
        && this.snapshot.state !== 'rejected'
        && this.snapshot.state !== 'errored',
    );
  }

  private configureRealtimeConnectionManager(): void {
    configurePcRealtimeConnectionManager({
      getClient: () => this.getClient(this.readSession()),
      getDeviceId: resolveSdkworkChatPcClientId,
      getSession: this.readSession,
    });
  }

  private applyUnavailableIncomingSnapshot(): void {
    this.applySnapshot({
      ...this.snapshot,
      state: 'errored',
      controllerState: 'errored',
      errorMessage: 'Incoming call is not available.',
    });
  }

  private async ensureParticipantCredentialReady(
    rtcSessionId: string,
    session: SdkworkChatSession | null,
  ): Promise<void> {
    if (
      this.snapshot.isParticipantCredentialReady
      && this.snapshot.rtcSessionId === rtcSessionId
      && this.participantCredential
    ) {
      await this.ensureRtcMediaReady(rtcSessionId, this.participantCredential);
      return;
    }
    const participantId = this.snapshot.participantId ?? resolveParticipantId(session);
    const credential = await this.getClient(session).calls.issueParticipantCredential(rtcSessionId, {
      participantId,
    });
    if (this.snapshot.rtcSessionId !== rtcSessionId || this.snapshot.state !== 'connected') {
      return;
    }
    this.applySnapshot({
      ...this.snapshot,
      isParticipantCredentialReady: true,
      participantCredentialExpiresAt: credential.expiresAt,
      participantId,
    });
    this.participantCredential = credential;
    await this.ensureRtcMediaReady(rtcSessionId, credential);
  }

  private async ensureRtcMediaReady(
    rtcSessionId: string,
    credential: ImCallParticipantCredential,
  ): Promise<void> {
    if (this.activeMediaRtcSessionId === rtcSessionId) {
      return;
    }
    const snapshot = this.getSnapshot();
    if (snapshot.rtcSessionId !== rtcSessionId || snapshot.state !== 'connected') {
      return;
    }
    const participantId = snapshot.participantId ?? credential.participantId;
    const roomId = snapshot.roomId ?? snapshot.rtcSessionId;
    if (!participantId || !roomId) {
      throw new Error('RTC media runtime requires a participant id and room id before joining.');
    }
    const joinOptions = {
      accessEndpoint: snapshot.accessEndpoint,
      credential,
      metadata: {
        conversationId: snapshot.conversationId,
        direction: snapshot.direction,
        type: snapshot.type,
      },
      participantId,
      providerKey: snapshot.providerKey,
      providerRegion: snapshot.providerRegion,
      roomId,
      rtcMode: snapshot.rtcMode,
      rtcSessionId,
    };
    try {
      await this.rtcMediaService.join(joinOptions);
      if (this.snapshot.rtcSessionId !== rtcSessionId || this.snapshot.state !== 'connected') {
        await this.releaseRtcMedia({ force: true });
        return;
      }
      this.activeMediaRtcSessionId = rtcSessionId;
      const publishKinds = resolveRtcMediaPublishKinds(joinOptions);
      await this.rtcMediaService.publish({
        kinds: publishKinds,
        rtcSessionId,
      });
      if (this.snapshot.isAudioMuted) {
        await this.rtcMediaService.muteAudio(true);
      }
      if (this.snapshot.isVideoMuted && publishKinds.includes('video')) {
        await this.rtcMediaService.muteVideo(true);
      }
    } catch (error) {
      const shouldReportMediaError =
        this.snapshot.rtcSessionId === rtcSessionId
        && this.snapshot.state === 'connected';
      await this.releaseRtcMedia({ force: true });
      if (!shouldReportMediaError) {
        return;
      }
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        controllerState: 'errored',
        errorMessage: toErrorMessage(error),
        isParticipantCredentialReady: false,
        participantCredentialExpiresAt: undefined,
      });
      await this.releaseRtcMedia();
    }
  }

  private async releaseRtcMedia(options: { force?: boolean } = {}): Promise<void> {
    if (!options.force && !this.activeMediaRtcSessionId && !this.participantCredential) {
      return;
    }
    this.activeMediaRtcSessionId = undefined;
    this.participantCredential = undefined;
    await this.rtcMediaService.leave().catch(() => undefined);
  }

  private applySnapshot(snapshot: SdkworkCallSnapshot): void {
    this.snapshot = {
      ...snapshot,
    };
    this.persistActiveCall();
    for (const listener of this.listeners) {
      listener(this.getSnapshot());
    }
  }

  private persistActiveCall(): void {
    const s = this.snapshot;
    const isTerminal = s.state === 'idle'
      || s.state === 'ended'
      || s.state === 'rejected'
      || s.state === 'errored';
    if (!s.rtcSessionId || isTerminal) {
      writePersistedActiveCall(null);
      return;
    }
    writePersistedActiveCall({
      rtcSessionId: s.rtcSessionId,
      conversationId: s.conversationId,
      direction: s.direction,
      targetName: s.targetName,
      type: s.type,
    });
  }
}

export function createSdkworkCallService(dependencies?: SdkworkCallServiceDependencies): CallService {
  return new SdkworkImCallService(dependencies);
}

export const callService = createSdkworkCallService();
