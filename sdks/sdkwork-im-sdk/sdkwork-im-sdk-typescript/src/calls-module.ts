import type {
  CreateRtcSessionRequest,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  PostRtcSignalRequest,
  RtcParticipantCredential,
  RtcSession,
  RtcSessionMutationResponse,
  RtcSignalEvent,
  UpdateRtcSessionRequest,
} from '@sdkwork/im-sdk-generated';
import type { ImConnectOptions, ImLiveConnection, ImRealtimeEventContext, ImSubscription } from './realtime';
import type { ImTransportClientLike } from './transport-client-like';

export type ImCallSession = RtcSession;
export type ImCallSessionMutationResponse = RtcSessionMutationResponse;
export type ImCallSignalEvent = RtcSignalEvent;
export type ImCallParticipantCredential = RtcParticipantCredential;

export interface ImCallStartOptions {
  conversationId?: string;
  rtcMode: string;
  rtcSessionId: string;
}

export interface ImCallInviteOptions {
  signalingStreamId?: string;
}

export interface ImCallUpdateOptions {
  artifactMessageId?: string;
}

export interface ImCallSignalOptions {
  payload: string;
  schemaRef?: string;
  signalingStreamId?: string;
  signalType: string;
}

export interface ImCallCredentialOptions {
  participantId: string;
}

export interface ImCallWatchIncomingOptions {
  connection?: ImLiveConnection;
  conversationIds?: string[];
  deviceId?: string;
}

interface ImCallsModuleOptions {
  connect?: (options: ImConnectOptions) => Promise<ImLiveConnection>;
}

type ImCallSessionListener = (session: RtcSession) => void;

interface ParsedCallSignal {
  payload: Record<string, unknown>;
  signalType: string;
}

function optionalString(value: string | undefined): string | null {
  return value === undefined ? null : value;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      return String(value);
    }
  }
  return undefined;
}

function normalizeConversationIds(values: string[] | undefined): string[] {
  return [...new Set(
    (values ?? [])
      .map((value) => value.trim())
      .filter((value) => value.length > 0),
  )].sort();
}

function parseJsonRecord(value: unknown): Record<string, unknown> | undefined {
  if (isRecord(value)) {
    return value;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(value);
    return isRecord(parsed) ? parsed : undefined;
  } catch {
    return undefined;
  }
}

function signalPartsFromMessagePayload(payload: Record<string, unknown>): Record<string, unknown>[] {
  const body = isRecord(payload.body) ? payload.body : undefined;
  const parts = Array.isArray(body?.parts) ? body.parts : [];
  return parts.filter((part): part is Record<string, unknown> =>
    isRecord(part) && pickString(part.kind) === 'signal',
  );
}

function parseCallSignals(payload: Record<string, unknown>): ParsedCallSignal[] {
  return signalPartsFromMessagePayload(payload)
    .map((part) => {
      const signalType = pickString(part.signalType);
      const partPayload = parseJsonRecord(part.payload);
      const nestedSignalPayload = parseJsonRecord(partPayload?.signalPayload);
      const signalPayload = nestedSignalPayload
        ? { ...partPayload, ...nestedSignalPayload }
        : partPayload;
      if (!signalType || !signalPayload) {
        return undefined;
      }
      return {
        payload: signalPayload,
        signalType,
      };
    })
    .filter((signal): signal is ParsedCallSignal => Boolean(signal));
}

function isOpenIncomingCallState(state: string): boolean {
  return state === 'started';
}

function isClosingCallSignal(signalType: string, state: string | undefined): boolean {
  return signalType === 'rtc.accept'
    || signalType === 'rtc.reject'
    || signalType === 'rtc.end'
    || state === 'accepted'
    || state === 'rejected'
    || state === 'ended';
}

function shouldRemoveCachedCallSession(signalType: string, state: string | undefined): boolean {
  return signalType === 'rtc.reject'
    || signalType === 'rtc.end'
    || state === 'rejected'
    || state === 'ended';
}

function normalizeCallSignalState(
  signalType: string,
  explicitState: string | undefined,
  cachedState: string | undefined,
): string {
  if (explicitState) {
    return explicitState;
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
      return cachedState ?? 'started';
  }
}

function toRtcSession(
  signal: ParsedCallSignal,
  messagePayload: Record<string, unknown>,
  context: ImRealtimeEventContext,
  cachedSession?: RtcSession,
): RtcSession | null {
  const sender = isRecord(messagePayload.sender) ? messagePayload.sender : undefined;
  const rtcSessionId = pickString(signal.payload.rtcSessionId, cachedSession?.rtcSessionId);
  const conversationId = pickString(
    signal.payload.conversationId,
    messagePayload.conversationId,
    context.scopeId,
    cachedSession?.conversationId,
  );
  const rtcMode = pickString(signal.payload.rtcMode, cachedSession?.rtcMode);
  const state = normalizeCallSignalState(
    signal.signalType,
    pickString(signal.payload.state),
    cachedSession?.state,
  );
  if (!rtcSessionId || !conversationId || !rtcMode) {
    return null;
  }
  return {
    tenantId: pickString(signal.payload.tenantId, messagePayload.tenantId, cachedSession?.tenantId) ?? '',
    rtcSessionId,
    conversationId,
    initiatorId: pickString(signal.payload.initiatorId, cachedSession?.initiatorId, sender?.id) ?? '',
    initiatorKind: pickString(signal.payload.initiatorKind, cachedSession?.initiatorKind, sender?.kind) ?? 'user',
    providerPluginId: pickString(signal.payload.providerPluginId, cachedSession?.providerPluginId) ?? null,
    providerSessionId: pickString(signal.payload.providerSessionId, cachedSession?.providerSessionId) ?? null,
    accessEndpoint: pickString(signal.payload.accessEndpoint, cachedSession?.accessEndpoint) ?? null,
    providerRegion: pickString(signal.payload.providerRegion, cachedSession?.providerRegion) ?? null,
    rtcMode,
    state,
    signalingStreamId: pickString(signal.payload.signalingStreamId, cachedSession?.signalingStreamId) ?? null,
    artifactMessageId: pickString(signal.payload.artifactMessageId, cachedSession?.artifactMessageId) ?? null,
    startedAt: pickString(signal.payload.startedAt, cachedSession?.startedAt, messagePayload.occurredAt, context.receivedAt) ?? new Date().toISOString(),
    ...(pickString(signal.payload.endedAt, cachedSession?.endedAt) ? { endedAt: pickString(signal.payload.endedAt, cachedSession?.endedAt) } : {}),
  };
}

export class ImCallsModule {
  readonly sessions = {
    create: (body: CreateRtcSessionRequest): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.create(body),
    retrieve: (rtcSessionId: string | number): Promise<RtcSession> =>
      this.retrieve(rtcSessionId),
    invite: (rtcSessionId: string | number, body: InviteRtcSessionRequest): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.invite(rtcSessionId, body),
    accept: (rtcSessionId: string | number, body: UpdateRtcSessionRequest = {}): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.accept(rtcSessionId, body),
    reject: (rtcSessionId: string | number, body: UpdateRtcSessionRequest = {}): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.reject(rtcSessionId, body),
    end: (rtcSessionId: string | number, body: UpdateRtcSessionRequest = {}): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.end(rtcSessionId, body),
    signals: {
      create: (rtcSessionId: string | number, body: PostRtcSignalRequest): Promise<RtcSignalEvent> =>
        this.transportClient.calls.sessions.signals.create(rtcSessionId, body),
    },
    credentials: {
      create: (
        rtcSessionId: string | number,
        body: IssueRtcParticipantCredentialRequest,
      ): Promise<RtcParticipantCredential> =>
        this.transportClient.calls.sessions.credentials.create(rtcSessionId, body),
    },
  };

  private readonly connect?: (options: ImConnectOptions) => Promise<ImLiveConnection>;
  private readonly incomingSessions = new Map<string, RtcSession>();
  private readonly listeners = new Set<ImCallSessionListener>();
  private watchConnection?: ImLiveConnection;
  private watchConversationIdsKey = '';
  private watchUnsubscribers: ImSubscription[] = [];

  constructor(
    private readonly transportClient: ImTransportClientLike,
    options: ImCallsModuleOptions = {},
  ) {
    this.connect = options.connect;
  }

  start(options: ImCallStartOptions): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.create({
      conversationId: optionalString(options.conversationId),
      rtcMode: options.rtcMode,
      rtcSessionId: options.rtcSessionId,
    }));
  }

  retrieve(rtcSessionId: string | number): Promise<RtcSession> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.retrieve(rtcSessionId));
  }

  invite(
    rtcSessionId: string | number,
    options: ImCallInviteOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.invite(rtcSessionId, {
      signalingStreamId: optionalString(options.signalingStreamId),
    }));
  }

  accept(
    rtcSessionId: string | number,
    options: ImCallUpdateOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.accept(rtcSessionId, {
      artifactMessageId: optionalString(options.artifactMessageId),
    }));
  }

  reject(
    rtcSessionId: string | number,
    options: ImCallUpdateOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.reject(rtcSessionId, {
      artifactMessageId: optionalString(options.artifactMessageId),
    }), true);
  }

  end(
    rtcSessionId: string | number,
    options: ImCallUpdateOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.end(rtcSessionId, {
      artifactMessageId: optionalString(options.artifactMessageId),
    }), true);
  }

  sendSignal(
    rtcSessionId: string | number,
    options: ImCallSignalOptions,
  ): Promise<RtcSignalEvent> {
    return this.transportClient.calls.sessions.signals.create(rtcSessionId, {
      payload: options.payload,
      schemaRef: optionalString(options.schemaRef),
      signalingStreamId: optionalString(options.signalingStreamId),
      signalType: options.signalType,
    });
  }

  issueParticipantCredential(
    rtcSessionId: string | number,
    options: ImCallCredentialOptions,
  ): Promise<RtcParticipantCredential> {
    return this.transportClient.calls.sessions.credentials.create(rtcSessionId, {
      participantId: options.participantId,
    });
  }

  async watchIncoming(options: ImCallWatchIncomingOptions | string[] = {}): Promise<RtcSession | null> {
    const watchOptions = Array.isArray(options) ? { conversationIds: options } : options;
    const conversationIds = normalizeConversationIds(watchOptions.conversationIds);
    if (watchOptions.connection) {
      this.bindIncomingConnection(watchOptions.connection, conversationIds, false);
    } else if (this.connect && conversationIds.length > 0) {
      await this.ensureIncomingWatchConnection(conversationIds, watchOptions.deviceId);
    } else if (conversationIds.length > 0) {
      this.pruneIncomingSessions(conversationIds);
    }
    return this.firstIncomingSession(conversationIds);
  }

  subscribe(handler: ImCallSessionListener): () => void {
    this.listeners.add(handler);
    return () => {
      this.listeners.delete(handler);
    };
  }

  private async ensureIncomingWatchConnection(
    conversationIds: string[],
    deviceId: string | undefined,
  ): Promise<void> {
    const conversationIdsKey = conversationIds.join('\n');
    if (this.watchConnection && this.watchConversationIdsKey === conversationIdsKey) {
      return;
    }
    this.closeIncomingWatchConnection();
    const connection = await this.connect?.({
      ...(deviceId ? { deviceId } : {}),
      subscriptions: {
        conversations: conversationIds,
      },
    });
    if (!connection) {
      return;
    }
    this.watchConnection = connection;
    this.watchConversationIdsKey = conversationIdsKey;
    this.bindIncomingConnection(connection, conversationIds, true);
  }

  private bindIncomingConnection(
    connection: ImLiveConnection,
    conversationIds: string[],
    closeWithModule: boolean,
  ): void {
    this.watchUnsubscribers.splice(0).forEach((unsubscribe) => unsubscribe());
    this.pruneIncomingSessions(conversationIds);
    for (const conversationId of conversationIds) {
      this.watchUnsubscribers.push(
        connection.events.onConversation(conversationId, (_event, context) => {
          if (context.payload) {
            this.consumeRealtimePayload(context.payload, context);
          }
        }),
      );
    }
    if (closeWithModule) {
      this.watchUnsubscribers.push(() => {
        connection.disconnect(1000, 'IM calls incoming watch closed');
      });
    }
  }

  private closeIncomingWatchConnection(): void {
    this.watchUnsubscribers.splice(0).forEach((unsubscribe) => unsubscribe());
    this.watchConnection = undefined;
    this.watchConversationIdsKey = '';
  }

  private consumeRealtimePayload(
    messagePayload: Record<string, unknown>,
    context: ImRealtimeEventContext,
  ): void {
    for (const signal of parseCallSignals(messagePayload)) {
      const rtcSessionId = pickString(signal.payload.rtcSessionId);
      const cachedSession = rtcSessionId ? this.incomingSessions.get(rtcSessionId) : undefined;
      const session = toRtcSession(signal, messagePayload, context, cachedSession);
      if (!session) {
        continue;
      }
      if (isClosingCallSignal(signal.signalType, session.state)) {
        this.emitIncoming(session);
        if (shouldRemoveCachedCallSession(signal.signalType, session.state)) {
          this.incomingSessions.delete(session.rtcSessionId);
        } else {
          this.incomingSessions.set(session.rtcSessionId, session);
        }
        continue;
      }
      if (signal.signalType !== 'rtc.invite' && !isOpenIncomingCallState(session.state)) {
        continue;
      }
      this.incomingSessions.set(session.rtcSessionId, session);
      this.emitIncoming(session);
    }
  }

  private firstIncomingSession(conversationIds: string[]): RtcSession | null {
    for (const session of this.incomingSessions.values()) {
      if (conversationIds.length > 0 && !conversationIds.includes(session.conversationId ?? '')) {
        continue;
      }
      if (isOpenIncomingCallState(session.state)) {
        return session;
      }
    }
    return null;
  }

  private pruneIncomingSessions(conversationIds: string[]): void {
    if (conversationIds.length === 0) {
      return;
    }
    for (const [rtcSessionId, session] of this.incomingSessions) {
      if (!conversationIds.includes(session.conversationId ?? '')) {
        this.incomingSessions.delete(rtcSessionId);
      }
    }
  }

  private emitIncoming(session: RtcSession): void {
    for (const listener of this.listeners) {
      listener(session);
    }
  }

  private async cacheSessionResult<TSession extends RtcSession>(
    promise: Promise<TSession>,
    removeAfterCache = false,
  ): Promise<TSession> {
    const session = await promise;
    if (removeAfterCache) {
      this.incomingSessions.delete(session.rtcSessionId);
    } else {
      this.incomingSessions.set(session.rtcSessionId, session);
    }
    return session;
  }
}
