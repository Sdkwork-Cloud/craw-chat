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
} from '../generated/server-openapi/dist/index.js';
import { requireStringIdentifier } from './identifier-boundary.js';
import type { ImConnectOptions, ImLiveConnection, ImRealtimeEventContext, ImSubscription } from './realtime.js';
import type { ImTransportClientLike } from './transport-client-like.js';

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
  participantIds?: string[];
}

export interface ImCallListSignalsOptions {
  /**
   * The server cursor is an int64. Strings preserve values above JS's safe
   * integer range; numbers remain supported for existing callers.
   */
  afterSignalSeq?: number | string;
  pageSize?: number;
  cursor?: string;
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
  principalId?: string;
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

const MAX_INT64 = 9223372036854775807n;

function normalizeAfterSignalSeq(value: number | string | undefined): string | undefined {
  if (value === undefined) {
    return undefined;
  }

  if (typeof value === 'number') {
    if (!Number.isSafeInteger(value) || value < 0) {
      throw new RangeError('afterSignalSeq must be a non-negative safe integer');
    }
    return String(value);
  }

  const normalized = value.trim();
  if (!/^\d+$/u.test(normalized)) {
    throw new TypeError('afterSignalSeq must be a non-negative integer string');
  }

  let sequence: bigint;
  try {
    sequence = BigInt(normalized);
  } catch {
    throw new TypeError('afterSignalSeq must be a non-negative integer string');
  }
  if (sequence > MAX_INT64) {
    throw new RangeError('afterSignalSeq exceeds the signed int64 range');
  }
  return sequence.toString();
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

function parseUserScopeRtcSession(
  envelope: Record<string, unknown>,
  cachedSession?: RtcSession,
): RtcSession | null {
  const inner = isRecord(envelope.payload) ? envelope.payload : envelope;
  const rtcSessionId = pickString(inner.rtc_session_id, inner.rtcSessionId, cachedSession?.rtcSessionId);
  const conversationId = pickString(inner.conversation_id, inner.conversationId, cachedSession?.conversationId);
  const rtcMode = pickString(inner.rtc_mode, inner.rtcMode, cachedSession?.rtcMode);
  const tenantId = pickString(inner.tenant_id, inner.tenantId, cachedSession?.tenantId) ?? '';
  const state = pickString(inner.state, cachedSession?.state) ?? 'started';
  if (!rtcSessionId || !rtcMode) {
    return null;
  }
  return {
    tenantId,
    rtcSessionId,
    conversationId: conversationId ?? null,
    rtcMode,
    initiatorId: pickString(inner.initiator_id, inner.initiatorId, cachedSession?.initiatorId) ?? '',
    initiatorKind: pickString(inner.initiator_kind, inner.initiatorKind, cachedSession?.initiatorKind) ?? 'user',
    state,
    signalingStreamId: pickString(inner.signaling_stream_id, inner.signalingStreamId, cachedSession?.signalingStreamId) ?? null,
    artifactMessageId: pickString(
      inner.artifact_message_id,
      inner.artifactMessageId,
      cachedSession?.artifactMessageId,
    ) ?? null,
    startedAt: pickString(inner.started_at, inner.startedAt, cachedSession?.startedAt) ?? '',
    ...(pickString(inner.ended_at, inner.endedAt, cachedSession?.endedAt)
      ? { endedAt: pickString(inner.ended_at, inner.endedAt, cachedSession?.endedAt) }
      : {}),
  };
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
    retrieve: (rtcSessionId: string): Promise<RtcSession> =>
      this.retrieve(rtcSessionId),
    invite: (rtcSessionId: string, body: InviteRtcSessionRequest): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.invite(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), body),
    accept: (rtcSessionId: string, body: UpdateRtcSessionRequest = {}): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.accept(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), body),
    reject: (rtcSessionId: string, body: UpdateRtcSessionRequest = {}): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.reject(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), body),
    end: (rtcSessionId: string, body: UpdateRtcSessionRequest = {}): Promise<RtcSessionMutationResponse> =>
      this.transportClient.calls.sessions.end(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), body),
    signals: {
      create: (rtcSessionId: string, body: PostRtcSignalRequest): Promise<RtcSignalEvent> =>
        this.transportClient.calls.sessions.signals.create(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), body),
    },
    credentials: {
      create: (
        rtcSessionId: string,
        body: IssueRtcParticipantCredentialRequest,
      ): Promise<RtcParticipantCredential> =>
        this.transportClient.calls.sessions.credentials.create(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), body),
    },
  };

  private readonly connect?: (options: ImConnectOptions) => Promise<ImLiveConnection>;
  private readonly incomingSessions = new Map<string, RtcSession>();
  private readonly outgoingSessionIds = new Set<string>();
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
    // 在 HTTP 调用前就标记为外呼会话，避免 `rtc.session.created` outbox 事件
    // 通过 WebSocket（relay 轮询 50ms）先于 HTTP 响应到达时，外呼会话被
    // `firstIncomingSession` 误识别为来电。rtcSessionId 由客户端提供，
    // 因此可以在请求发出前即加入追踪集合。
    this.outgoingSessionIds.add(options.rtcSessionId);
    return this.cacheSessionResult(
      this.transportClient.calls.sessions.create({
        conversationId: optionalString(options.conversationId),
        rtcMode: options.rtcMode,
        rtcSessionId: options.rtcSessionId,
      }),
    ).then((response) => {
      // 如果服务端返回的 ID 与客户端提供的不同（理论上不会发生，
      // 但防御性处理），修正追踪集合。
      if (response.rtcSessionId !== options.rtcSessionId) {
        this.outgoingSessionIds.delete(options.rtcSessionId);
        this.outgoingSessionIds.add(response.rtcSessionId);
      }
      return response;
    });
  }

  retrieve(rtcSessionId: string): Promise<RtcSession> {
    return this.cacheSessionResult(
      this.transportClient.calls.sessions.retrieve(requireStringIdentifier(rtcSessionId, 'rtcSessionId')),
    );
  }

  invite(
    rtcSessionId: string,
    options: ImCallInviteOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(
      this.transportClient.calls.sessions.invite(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), {
        signalingStreamId: optionalString(options.signalingStreamId),
      }),
    );
  }

  listSignals(
    rtcSessionId: string,
    options: ImCallListSignalsOptions = {},
  ): Promise<{ items: RtcSignalEvent[]; pageInfo: { mode: string; hasMore?: boolean; nextCursor?: string | null } }> {
    return this.transportClient.calls.sessions.signals.list(
      requireStringIdentifier(rtcSessionId, 'rtcSessionId'),
      {
        afterSignalSeq: normalizeAfterSignalSeq(options.afterSignalSeq),
        pageSize: options.pageSize,
        cursor: options.cursor,
      },
    );
  }

  accept(
    rtcSessionId: string,
    options: ImCallUpdateOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(
      this.transportClient.calls.sessions.accept(requireStringIdentifier(rtcSessionId, 'rtcSessionId'), {
        artifactMessageId: optionalString(options.artifactMessageId),
      }),
    );
  }

  reject(
    rtcSessionId: string,
    options: ImCallUpdateOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.reject(
      requireStringIdentifier(rtcSessionId, 'rtcSessionId'),
      {
        artifactMessageId: optionalString(options.artifactMessageId),
      },
    ), true).then((response) => {
      this.outgoingSessionIds.delete(response.rtcSessionId);
      return response;
    });
  }

  end(
    rtcSessionId: string,
    options: ImCallUpdateOptions = {},
  ): Promise<RtcSessionMutationResponse> {
    return this.cacheSessionResult(this.transportClient.calls.sessions.end(
      requireStringIdentifier(rtcSessionId, 'rtcSessionId'),
      {
        artifactMessageId: optionalString(options.artifactMessageId),
      },
    ), true).then((response) => {
      this.outgoingSessionIds.delete(response.rtcSessionId);
      return response;
    });
  }

  sendSignal(
    rtcSessionId: string,
    options: ImCallSignalOptions,
  ): Promise<RtcSignalEvent> {
    return this.transportClient.calls.sessions.signals.create(
      requireStringIdentifier(rtcSessionId, 'rtcSessionId'),
      {
        payload: options.payload,
        schemaRef: optionalString(options.schemaRef),
        signalingStreamId: optionalString(options.signalingStreamId),
        signalType: options.signalType,
      },
    );
  }

  issueParticipantCredential(
    rtcSessionId: string,
    options: ImCallCredentialOptions,
  ): Promise<RtcParticipantCredential> {
    return this.transportClient.calls.sessions.credentials.create(
      requireStringIdentifier(rtcSessionId, 'rtcSessionId'),
      {
        participantId: options.participantId,
      },
    );
  }

  refreshParticipantCredential(
    rtcSessionId: string,
    options: ImCallCredentialOptions,
  ): Promise<RtcParticipantCredential> {
    return this.transportClient.calls.sessions.credentials.refresh(
      requireStringIdentifier(rtcSessionId, 'rtcSessionId'),
      {
        participantId: options.participantId,
      },
    );
  }

  async watchIncoming(options: ImCallWatchIncomingOptions | string[] = {}): Promise<RtcSession | null> {
    const watchOptions = Array.isArray(options) ? { conversationIds: options } : options;
    const conversationIds = normalizeConversationIds(watchOptions.conversationIds);
    if (watchOptions.connection) {
      this.bindIncomingConnection(
        watchOptions.connection,
        conversationIds,
        false,
        watchOptions.principalId,
      );
    } else if (this.connect && (conversationIds.length > 0 || watchOptions.principalId)) {
      await this.ensureIncomingWatchConnection(
        conversationIds,
        watchOptions.deviceId,
        watchOptions.principalId,
      );
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
    principalId: string | undefined,
  ): Promise<void> {
    const conversationIdsKey = [conversationIds.join('\n'), principalId ?? ''].join('|');
    if (this.watchConnection && this.watchConversationIdsKey === conversationIdsKey) {
      return;
    }
    this.closeIncomingWatchConnection();
    const connection = await this.connect?.({
      ...(deviceId ? { deviceId } : {}),
      subscriptions: {
        conversations: conversationIds,
        ...(principalId ? { scopes: [{ scopeType: 'user', scopeId: principalId }] } : {}),
      },
    });
    if (!connection) {
      return;
    }
    this.bindIncomingConnection(connection, conversationIds, true, principalId);
  }

  private bindIncomingConnection(
    connection: ImLiveConnection,
    conversationIds: string[],
    closeWithModule: boolean,
    principalId?: string,
  ): void {
    this.watchUnsubscribers.splice(0).forEach((unsubscribe) => unsubscribe());
    this.pruneIncomingSessions(conversationIds);
    // 即使是外部传入的连接（closeWithModule === false），也需要记录
    // watchConnection 和 watchConversationIdsKey，这样后续调用
    // ensureIncomingWatchConnection 时能正确检测到已有监听并先清理旧订阅，
    // 避免创建重复连接导致事件重复投递。
    this.watchConnection = connection;
    this.watchConversationIdsKey = [conversationIds.join('\n'), principalId ?? ''].join('|');
    for (const conversationId of conversationIds) {
      this.watchUnsubscribers.push(
        connection.events.onConversation(conversationId, (_event, context) => {
          if (context.payload) {
            this.consumeRealtimePayload(context.payload, context);
          }
        }),
      );
    }
    if (principalId) {
      this.watchUnsubscribers.push(
        connection.events.onScope('user', principalId, (_event, context) => {
          if (context.payload) {
            this.consumeUserScopeRtcEnvelope(context.payload);
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

  private consumeUserScopeRtcEnvelope(messagePayload: Record<string, unknown>): void {
    const eventType = pickString(messagePayload.eventType);
    if (!eventType?.startsWith('rtc.')) {
      return;
    }

    // rtc.signal.posted 事件仅包含信号元数据（signal_seq、signal_type 等），
    // 不包含实际的 SDP/ICE 负载。实际信号通过会话消息流（conversation-scope）
    // 的 body.parts 投递，由 consumeRealtimePayload → parseCallSignals 处理。
    // 此处跳过以避免无负载的空处理和重复投递。
    if (eventType === 'rtc.signal.posted') {
      return;
    }

    const inner = isRecord(messagePayload.payload) ? messagePayload.payload : messagePayload;

    // rtc.credentials.revoked 负载使用 terminal_state 而非 state 字段，
    // 且缺少 rtc_mode 等会话字段。将 terminal_state 映射为 state，
    // 使 parseUserScopeRtcSession 能从缓存会话补全其余字段并正确解析终态。
    if (eventType === 'rtc.credentials.revoked') {
      const terminalState = pickString(inner.terminal_state);
      if (terminalState) {
        inner.state = terminalState;
      }
    }

    const rtcSessionId = pickString(inner.rtc_session_id, inner.rtcSessionId);
    const cachedSession = rtcSessionId ? this.incomingSessions.get(rtcSessionId) : undefined;
    const session = parseUserScopeRtcSession(messagePayload, cachedSession);
    if (!session) {
      return;
    }
    if (
      eventType === 'rtc.session.ended'
      || eventType === 'rtc.session.rejected'
      || eventType === 'rtc.session.revoked'
      || eventType === 'rtc.credentials.revoked'
    ) {
      this.emitIncoming(session);
      this.incomingSessions.delete(session.rtcSessionId);
      this.outgoingSessionIds.delete(session.rtcSessionId);
      return;
    }
    if (
      eventType === 'rtc.session.invited'
      || eventType === 'rtc.session.created'
      || eventType === 'rtc.session.accepted'
    ) {
      this.incomingSessions.set(session.rtcSessionId, session);
      this.emitIncoming(session);
    }
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
      // Skip sessions that were started locally (outgoing calls) so they
      // are not mistaken for incoming calls.
      if (this.outgoingSessionIds.has(session.rtcSessionId)) {
        continue;
      }
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
