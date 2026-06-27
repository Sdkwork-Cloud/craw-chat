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
  ensurePcLiveConnection,
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

export interface CallService {
  acceptIncomingCall(): Promise<SdkworkCallSnapshot>;
  bindLocalVideoElement(element: HTMLElement | null): Promise<void>;
  endCall(options?: EndCallOptions): Promise<void>;
  getSnapshot(): SdkworkCallSnapshot;
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
      return 'connected';
    case 'rejected':
      return 'rejected';
    case 'ended':
      return 'ended';
    case 'started':
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

class SdkworkImCallService implements CallService {
  private readonly listeners = new Set<(snapshot: SdkworkCallSnapshot) => void>();
  private readonly getClient: (session: SdkworkChatSession | null) => ImSdkClient;
  private readonly readSession: () => SdkworkChatSession | null;
  private readonly rtcMediaService: SdkworkRtcMediaService;
  private activeMediaRtcSessionId?: string;
  private incomingConnectionLease?: () => void;
  private incomingSubscription?: () => void;
  private participantCredential?: ImCallParticipantCredential;
  private snapshot: SdkworkCallSnapshot = createIdleSnapshot();
  private sequence = 0;

  constructor(dependencies: SdkworkCallServiceDependencies = {}) {
    this.getClient = dependencies.getClient ?? ((session) => getImSdkClientWithSession(session));
    this.readSession = dependencies.readSession ?? readAppSdkSessionTokens;
    this.rtcMediaService = dependencies.rtcMediaService ?? rtcMediaService;
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
      peerUserId: undefined,
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

  async watchIncomingCalls(conversationIds: string[]): Promise<SdkworkCallSnapshot> {
    const normalizedConversationIds = [...new Set(
      conversationIds
        .map((conversationId) => String(conversationId).trim())
        .filter((conversationId) => conversationId.length > 0),
    )];

    if (this.hasActiveCall()) {
      return this.getSnapshot();
    }

    if (normalizedConversationIds.length === 0) {
      this.incomingSubscription?.();
      this.incomingSubscription = undefined;
      this.incomingConnectionLease?.();
      this.incomingConnectionLease = undefined;
      this.applySnapshot(createIdleSnapshot());
      return this.getSnapshot();
    }

    try {
      const session = this.readSession();
      const imClient = this.getClient(session);
      this.incomingConnectionLease?.();
      this.incomingConnectionLease = acquirePcLiveConnectionLease('calls-incoming-watch', {
        conversationIds: normalizedConversationIds,
      });
      const connection = await ensurePcLiveConnection();
      this.incomingSubscription?.();
      this.incomingSubscription = imClient.calls.subscribe((callSession) => {
        if (!normalizedConversationIds.includes(callSession.conversationId ?? '')) {
          return;
        }
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
      const incoming = await imClient.calls.watchIncoming({
        connection,
        conversationIds: normalizedConversationIds,
        deviceId: resolveSdkworkChatPcClientId(),
      });
      if (this.snapshot.rtcSessionId && this.snapshot.controllerState !== 'watching') {
        return this.getSnapshot();
      }
      if (incoming && normalizedConversationIds.includes(incoming.conversationId ?? '')) {
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
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        controllerState: 'errored',
        errorMessage: toErrorMessage(error),
      });
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
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        controllerState: 'errored',
        errorMessage: toErrorMessage(error),
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
    for (const listener of this.listeners) {
      listener(this.getSnapshot());
    }
  }
}

export function createSdkworkCallService(dependencies?: SdkworkCallServiceDependencies): CallService {
  return new SdkworkImCallService(dependencies);
}

export const callService = createSdkworkCallService();
