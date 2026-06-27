import type {
  MediaResource,
  MessageBody,
  MessageReplyReference,
  MessageType,
  Sender,
} from '@sdkwork/im-sdk-generated';

import {
  IM_CCP_WEBSOCKET_SUBPROTOCOL,
  ccpHelloAckNegotiatesSessionResume,
  encodeCcpAuthBindFrame,
  encodeCcpBusinessFrame,
  encodeCcpHeartbeatFrame,
  encodeCcpHelloFrame,
  encodeCcpSessionResumeFrame,
  isCcpAuthOkEnvelope,
  isCcpHelloAckEnvelope,
  isCcpSessionResumedEnvelope,
  resolveCcpAuthBindContext,
  unwrapInboundRealtimeFrame,
} from './ccp-wire';
import { IM_REALTIME_WS } from './realtime-api-paths';

export { IM_CCP_WEBSOCKET_SUBPROTOCOL } from './ccp-wire';

export interface ImMessageContext {
  ack(): Promise<void>;
  conversationId?: string;
  eventId?: string;
  eventType?: string;
  messageId?: string;
  payload?: Record<string, unknown>;
  rawEvent?: Record<string, unknown>;
  receivedAt: string;
  sender?: Sender;
  sequence: number;
}

export interface ImDecodedAttachment {
  resource?: MediaResource;
  [key: string]: unknown;
}

export interface ImDecodedMessage {
  attachments: ImDecodedAttachment[];
  body?: MessageBody;
  content?: Record<string, unknown>;
  conversationId?: string;
  deliveryMode?: string;
  messageId?: string;
  messageSeq?: number;
  messageType?: MessageType;
  occurredAt?: string;
  renderHints?: Record<string, unknown>;
  replyTo?: MessageReplyReference;
  sender?: Sender;
  summary?: string;
  text?: string;
  type?: string;
  [key: string]: unknown;
}

export interface ImRealtimeEventContext {
  ack(): Promise<void>;
  eventId?: string;
  eventType?: string;
  payload?: Record<string, unknown>;
  rawEvent?: Record<string, unknown>;
  receivedAt: string;
  scopeId?: string;
  scopeType?: string;
  sequence: number;
}

export type ImSubscription = () => void;

export interface ImRealtimeScopeSubscription {
  eventTypes?: string[];
  scopeId: string;
  scopeType: string;
}

export interface ImLiveConnectionState {
  status: 'connecting' | 'open' | 'closed' | 'error';
  reason?: string;
}

export interface ImLiveConnection {
  disconnect(code?: number, reason?: string): void;
  events: {
    onConversation(
      conversationId: string,
      handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void,
    ): ImSubscription;
    onScope(
      scopeType: string,
      scopeId: string,
      handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void,
    ): ImSubscription;
  };
  lifecycle: {
    onError(handler: (error: unknown) => void): ImSubscription;
    onStateChange(handler: (state: ImLiveConnectionState) => void): ImSubscription;
  };
  messages: {
    onConversation(
      conversationId: string,
      handler: (message: ImDecodedMessage, context: ImMessageContext) => void,
    ): ImSubscription;
  };
  subscriptions: {
    syncConversations(conversationIds: string[]): void;
    syncScopes(scopes: ImRealtimeScopeSubscription[]): void;
  };
}

export type ImWebSocketEventName = 'close' | 'error' | 'message' | 'open';

export interface ImWebSocketLike {
  readyState: number;
  addEventListener(type: ImWebSocketEventName, handler: (event: unknown) => void): void;
  close(code?: number, reason?: string): void;
  send(value: string): void;
}

export interface ImWebSocketFactoryOptions {
  headers: Record<string, string>;
  protocols: string[];
}

export type ImWebSocketFactory = (
  url: string,
  options: ImWebSocketFactoryOptions,
) => ImWebSocketLike;

export interface ImConnectOptions {
  connectionTimeoutMs?: number;
  deviceId?: string;
  heartbeat?: false | ImRealtimeHeartbeatOptions;
  subscriptions?: {
    conversations?: string[];
    scopes?: ImRealtimeScopeSubscription[];
  };
}

export interface ImRealtimeHeartbeatOptions {
  intervalMs?: number;
  timeoutMs?: number;
}

export interface ImWebSocketCredentialProvider {
  (): string | undefined;
}

export interface ImWebSocketAuthConfig {
  credentialProvider?: ImWebSocketCredentialProvider;
  mode: 'automatic' | 'none';
  timeoutMs?: number;
}

export class ImWebSocketAuthOptions {
  static automatic(options: Omit<ImWebSocketAuthConfig, 'mode'> = {}): ImWebSocketAuthConfig {
    return { ...options, mode: 'automatic' };
  }

  static none(): ImWebSocketAuthConfig {
    return { mode: 'none' };
  }
}

export interface ImCreateLiveConnectionParams {
  accessToken?: string;
  auth?: ImWebSocketAuthConfig;
  authToken?: string;
  headerProvider?: () => Record<string, string>;
  headers?: Record<string, string>;
  options: ImConnectOptions;
  tokenManager?: unknown;
  websocketBaseUrl: string;
  webSocketFactory?: ImWebSocketFactory;
}

interface ListenerBag {
  errors: Set<(error: unknown) => void>;
  events: Map<string, Set<(event: Record<string, unknown>, context: ImRealtimeEventContext) => void>>;
  messages: Map<string, Set<(message: ImDecodedMessage, context: ImMessageContext) => void>>;
  states: Set<(state: ImLiveConnectionState) => void>;
}

interface ParsedRealtimeEvent {
  context: ImRealtimeEventContext;
  event: Record<string, unknown>;
}

interface ParsedRealtimeMessage {
  context: ImMessageContext;
  message: ImDecodedMessage;
}

interface ResolvedWebSocketCredentials {
  accessToken?: string;
  authToken?: string;
}

interface WebSocketTokenManagerLike {
  getAccessToken?: () => string | undefined;
  getAuthToken?: () => string | undefined;
  getTokens?: () => {
    accessToken?: string;
    authToken?: string;
  } | undefined;
}

type BrowserWebSocketConstructor = new (
  url: string,
  protocols?: string | string[],
) => ImWebSocketLike;

const SOCKET_CONNECTING_STATE = 0;
const SOCKET_OPEN_STATE = 1;
const SOCKET_CLOSING_STATE = 2;
const SOCKET_CLOSED_STATE = 3;
const DEFAULT_WEBSOCKET_CONNECTION_TIMEOUT_MS = 15_000;
const DEFAULT_WEBSOCKET_AUTH_TIMEOUT_MS = 10_000;
const DEFAULT_WEBSOCKET_HEARTBEAT_INTERVAL_MS = 30_000;
const DEFAULT_WEBSOCKET_HEARTBEAT_TIMEOUT_MS = 75_000;
const MIN_WEBSOCKET_CONNECTION_TIMEOUT_MS = 1;
const MIN_WEBSOCKET_HEARTBEAT_INTERVAL_MS = 1;
const MIN_WEBSOCKET_HEARTBEAT_TIMEOUT_MS = 1;

interface ImRealtimeControlError {
  code: string;
  message: string;
  requestId?: string;
  type: 'error';
}

type ImRealtimeConnectionPhase =
  | 'gateway_auth'
  | 'ccp_hello_ack'
  | 'ccp_auth_ok'
  | 'ccp_session_resume'
  | 'ready';

function subscribe<T>(set: Set<T>, value: T): ImSubscription {
  set.add(value);
  return () => {
    set.delete(value);
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function pickStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value
    .filter((item): item is string => typeof item === 'string')
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
}

function realtimeScopeKey(scopeType: string, scopeId: string): string {
  return `${scopeType}:${scopeId}`;
}

function normalizeRealtimeScopeSubscriptions(
  value: unknown,
): ImRealtimeScopeSubscription[] {
  if (!Array.isArray(value)) {
    return [];
  }

  const deduped = new Map<string, ImRealtimeScopeSubscription>();
  for (const item of value) {
    if (!isRecord(item)) {
      continue;
    }
    const scopeType = pickString(item.scopeType, item.scope);
    const scopeId = pickString(item.scopeId, item.conversationId);
    if (!scopeType || !scopeId) {
      continue;
    }
    const normalized = {
      scopeId,
      scopeType,
      eventTypes: pickStringArray(item.eventTypes),
    };
    deduped.set(realtimeScopeKey(scopeType, scopeId), normalized);
  }
  return [...deduped.values()];
}

function mergeRealtimeScopeSubscriptions(
  conversations: string[],
  scopes: ImRealtimeScopeSubscription[],
): ImRealtimeScopeSubscription[] {
  const merged = new Map<string, ImRealtimeScopeSubscription>();
  for (const conversationId of conversations) {
    const item = {
      scopeType: 'conversation',
      scopeId: conversationId,
      eventTypes: ['message.posted'],
    };
    merged.set(realtimeScopeKey(item.scopeType, item.scopeId), item);
  }
  for (const scope of scopes) {
    merged.set(realtimeScopeKey(scope.scopeType, scope.scopeId), {
      scopeType: scope.scopeType,
      scopeId: scope.scopeId,
      eventTypes: [...(scope.eventTypes ?? [])],
    });
  }
  return [...merged.values()];
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

function parseRecordPayload(value: unknown): Record<string, unknown> | undefined {
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

function normalizeAttachments(value: unknown): ImDecodedAttachment[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter(isRecord) as ImDecodedAttachment[];
}

function normalizeMessage(
  record: Record<string, unknown>,
  contentFallback?: Record<string, unknown>,
): ImDecodedMessage {
  return {
    ...record,
    attachments: normalizeAttachments(record.attachments),
    content: parseRecordPayload(record.content) ?? contentFallback,
    renderHints: parseRecordPayload(record.renderHints),
  } as ImDecodedMessage;
}

function normalizeAuthorizationHeader(token: string): string {
  return /^Bearer\s+/iu.test(token) ? token : `Bearer ${token}`;
}

function addHeader(headers: Record<string, string>, key: string, value: unknown): void {
  if (typeof value !== 'string' || value.trim().length === 0) {
    return;
  }
  headers[key] = value.trim();
}

function buildWebSocketHeaders({
  accessToken,
  authToken,
  headerProvider,
  headers,
}: ResolvedWebSocketCredentials & Pick<ImCreateLiveConnectionParams, 'headerProvider' | 'headers'>): Record<string, string> {
  const resolvedHeaders: Record<string, string> = {};

  if (authToken) {
    resolvedHeaders.Authorization = normalizeAuthorizationHeader(authToken);
  }
  addHeader(resolvedHeaders, 'Access-Token', accessToken);

  for (const [key, value] of Object.entries(headers ?? {})) {
    addHeader(resolvedHeaders, key, value);
  }
  for (const [key, value] of Object.entries(headerProvider?.() ?? {})) {
    addHeader(resolvedHeaders, key, value);
  }

  return resolvedHeaders;
}

function hasUpgradeAuthHeaders(headers: Record<string, string>): boolean {
  for (const [key, value] of Object.entries(headers)) {
    if (!value.trim()) {
      continue;
    }
    const normalized = key.toLowerCase();
    if (normalized === 'authorization' || normalized === 'access-token') {
      return true;
    }
  }
  return false;
}

function resolveWebSocketCredentials({
  accessToken,
  auth,
  authToken,
  tokenManager,
}: Pick<ImCreateLiveConnectionParams, 'accessToken' | 'auth' | 'authToken' | 'tokenManager'>): ResolvedWebSocketCredentials {
  const manager = isRecord(tokenManager) ? tokenManager as WebSocketTokenManagerLike : undefined;
  const managerTokens = manager?.getTokens?.();
  const websocketCredential = auth?.mode === 'automatic' ? auth.credentialProvider?.() : undefined;
  return {
    accessToken: pickString(manager?.getAccessToken?.(), managerTokens?.accessToken, accessToken),
    authToken: pickString(manager?.getAuthToken?.(), managerTokens?.authToken, authToken, websocketCredential),
  };
}

function appendRealtimeRoutePath(pathname: string): string {
  const basePath = pathname.replace(/\/+$/u, '');
  if (basePath.endsWith(IM_REALTIME_WS)) {
    return basePath;
  }
  return `${basePath}${IM_REALTIME_WS}`;
}

function buildWebSocketUrl(websocketBaseUrl: string, options: ImConnectOptions): string {
  const url = new URL(websocketBaseUrl);
  url.pathname = appendRealtimeRoutePath(url.pathname);

  if (options.deviceId) {
    url.searchParams.set('deviceId', options.deviceId);
  }

  return url.toString();
}

function resolveWebSocketFactory(factory?: ImWebSocketFactory): ImWebSocketFactory {
  if (factory) {
    return factory;
  }

  const WebSocketConstructor = globalThis.WebSocket as unknown;
  if (typeof WebSocketConstructor !== 'function') {
    throw new Error('IM websocket transport is unavailable; provide ImSdkClientOptions.webSocketFactory.');
  }

  return (url, options) => new (WebSocketConstructor as BrowserWebSocketConstructor)(
    url,
    options.protocols,
  );
}

function unwrapWirePayload(parsed: Record<string, unknown>): Record<string, unknown> {
  const payload = parseRecordPayload(parsed.payload);
  if (pickString(parsed.schema) && payload && pickString(payload.type)) {
    return payload;
  }
  return parsed;
}

function extractMessageData(event: unknown): string | undefined {
  if (typeof event === 'string') {
    return event;
  }
  if (isRecord(event) && typeof event.data === 'string') {
    return event.data;
  }
  return undefined;
}

function resolveEventScopeType(event: Record<string, unknown>): string | undefined {
  return pickString(event.scopeType, event.scope);
}

function resolveConversationId(
  message: ImDecodedMessage,
  payload: Record<string, unknown> | undefined,
  event: Record<string, unknown>,
): string | undefined {
  const scopeType = resolveEventScopeType(event);
  return pickString(
    message.conversationId,
    payload?.conversationId,
    event.conversationId,
    scopeType === 'conversation' ? event.scopeId : undefined,
  );
}

function createNoopContextAck(): Promise<void> {
  return Promise.resolve();
}

function parseDirectRealtimePayload(frame: Record<string, unknown>): ParsedRealtimeMessage | null {
  const payload = parseRecordPayload(frame.payload);
  const messageRecord = payload ?? frame;
  const message = normalizeMessage(messageRecord, payload);
  const messageSender = isRecord(message.sender) ? message.sender as unknown as Sender : undefined;
  const payloadSender = payload && isRecord(payload.sender) ? payload.sender as unknown as Sender : undefined;
  const conversationId = resolveConversationId(message, payload, frame);
  const messageId = pickString(message.messageId, payload?.messageId, frame.messageId, frame.eventId);
  const sequence = pickNumber(
    frame.realtimeSeq,
    frame.sequence,
    message.messageSeq,
    payload?.messageSeq,
    payload?.sequence,
    frame.messageSeq,
  ) ?? 0;

  if (!conversationId) {
    return null;
  }

  return {
    context: {
      ack: createNoopContextAck,
      conversationId,
      eventId: pickString(frame.eventId),
      eventType: pickString(frame.eventType, frame.type),
      messageId,
      payload,
      rawEvent: frame,
      receivedAt: pickString(frame.receivedAt, frame.occurredAt, message.occurredAt) ?? new Date().toISOString(),
      sender: messageSender ?? payloadSender,
      sequence,
    },
    message,
  };
}

function parseRealtimeEventEnvelope(
  event: Record<string, unknown>,
  frame: Record<string, unknown> = event,
): ParsedRealtimeEvent | null {
  const scopeType = resolveEventScopeType(event);
  const scopeId = pickString(event.scopeId, event.conversationId);
  if (!scopeId) {
    return null;
  }
  const payload = parseRecordPayload(event.payload);
  const sequence = pickNumber(
    event.realtimeSeq,
    event.sequence,
    payload?.realtimeSeq,
    payload?.messageSeq,
    event.messageSeq,
  ) ?? 0;

  return {
    context: {
      ack: createNoopContextAck,
      eventId: pickString(event.eventId),
      eventType: pickString(event.eventType, event.type),
      payload,
      rawEvent: event,
      receivedAt: pickString(frame.receivedAt, event.receivedAt, event.occurredAt) ?? new Date().toISOString(),
      scopeId,
      scopeType: scopeType ?? 'conversation',
      sequence,
    },
    event,
  };
}

function parseRealtimeEventWindow(frame: Record<string, unknown>): ParsedRealtimeMessage[] {
  const window = isRecord(frame.window) ? frame.window : undefined;
  const items = Array.isArray(window?.items) ? window.items.filter(isRecord) : [];
  const messages: ParsedRealtimeMessage[] = [];

  for (const item of items) {
    const eventType = pickString(item.eventType, item.type);
    const scopeType = resolveEventScopeType(item);
    if (eventType && eventType !== 'message.posted') {
      continue;
    }
    if (scopeType && scopeType !== 'conversation') {
      continue;
    }

    const payload = parseRecordPayload(item.payload);
    const messageRecord = payload ?? item;
    const message = normalizeMessage(messageRecord, payload);
    const occurredAt = pickString(message.occurredAt, payload?.occurredAt, item.occurredAt);
    if (occurredAt) {
      message.occurredAt = occurredAt;
    }

    const messageSender = isRecord(message.sender) ? message.sender as unknown as Sender : undefined;
    const payloadSender = payload && isRecord(payload.sender) ? payload.sender as unknown as Sender : undefined;
    const conversationId = resolveConversationId(message, payload, item);
    if (!conversationId) {
      continue;
    }

    const sequence = pickNumber(
      item.realtimeSeq,
      item.sequence,
      payload?.realtimeSeq,
      payload?.messageSeq,
      item.messageSeq,
      window?.nextAfterSeq,
    ) ?? 0;
    const messageId = pickString(message.messageId, payload?.messageId, item.messageId, item.eventId);

    messages.push({
      context: {
        ack: createNoopContextAck,
        conversationId,
        eventId: pickString(item.eventId),
        eventType: eventType ?? 'message.posted',
        messageId,
        payload,
        rawEvent: item,
        receivedAt: pickString(frame.receivedAt, item.receivedAt, item.occurredAt, occurredAt) ?? new Date().toISOString(),
        sender: messageSender ?? payloadSender,
        sequence,
      },
      message,
    });
  }

  return messages;
}

function parseRealtimeEvents(raw: string): ParsedRealtimeEvent[] {
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!isRecord(parsed)) {
      return [];
    }
    const frame = unwrapWirePayload(parsed);
    if (pickString(frame.type) === 'event.window') {
      const window = isRecord(frame.window) ? frame.window : undefined;
      const items = Array.isArray(window?.items) ? window.items.filter(isRecord) : [];
      return items
        .map((item) => parseRealtimeEventEnvelope(item, frame))
        .filter((item): item is ParsedRealtimeEvent => Boolean(item));
    }
    const event = parseRealtimeEventEnvelope(frame);
    return event ? [event] : [];
  } catch {
    return [];
  }
}

function parseRealtimePayloads(raw: string): ParsedRealtimeMessage[] {
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!isRecord(parsed)) {
      return [];
    }
    const frame = unwrapWirePayload(parsed);
    if (pickString(frame.type) === 'event.window') {
      return parseRealtimeEventWindow(frame);
    }

    const directMessage = parseDirectRealtimePayload(frame);
    return directMessage ? [directMessage] : [];
  } catch {
    return [];
  }
}

function sendSubscriptionSync(
  socket: ImWebSocketLike,
  scopes: ImRealtimeScopeSubscription[],
  requestId: string,
): void {
  if (socket.readyState !== SOCKET_OPEN_STATE) {
    return;
  }
  socket.send(encodeCcpBusinessFrame(
    'cc.realtime.subscriptions.sync.v1',
    'cmd',
    {
      type: 'subscriptions.sync',
      requestId,
      items: scopes.map((scope) => ({
        scopeType: scope.scopeType,
        scopeId: scope.scopeId,
        eventTypes: scope.eventTypes ?? [],
      })),
    },
  ));
}

function sendEventsAck(socket: ImWebSocketLike, requestId: string, ackedSeq: number): void {
  if (socket.readyState !== SOCKET_OPEN_STATE) {
    return;
  }
  socket.send(encodeCcpBusinessFrame(
    'cc.realtime.events.ack.v1',
    'ack',
    {
      type: 'events.ack',
      requestId,
      ackedSeq,
    },
  ));
}

function sendAuthInit(
  socket: ImWebSocketLike,
  credentials: Required<ResolvedWebSocketCredentials>,
  deviceId: string | undefined,
  requestId: string,
): void {
  if (socket.readyState !== SOCKET_OPEN_STATE) {
    return;
  }
  socket.send(JSON.stringify({
    type: 'auth.init',
    requestId,
    authToken: credentials.authToken,
    accessToken: credentials.accessToken,
    ...(deviceId ? { deviceId } : {}),
  }));
}

function parseRealtimeControlFrame(raw: string): Record<string, unknown> | undefined {
  try {
    const parsed: unknown = JSON.parse(raw);
    return isRecord(parsed) ? unwrapWirePayload(parsed) : undefined;
  } catch {
    return undefined;
  }
}

function isAuthOkFrame(raw: string, requestId: string): boolean {
  const frame = parseRealtimeControlFrame(raw);
  return pickString(frame?.type) === 'auth.ok'
    && (!pickString(frame?.requestId) || pickString(frame?.requestId) === requestId);
}

function parseRealtimeControlError(raw: string, requestId?: string): ImRealtimeControlError | undefined {
  const frame = parseRealtimeControlFrame(raw);
  if (pickString(frame?.type) !== 'error') {
    return undefined;
  }
  const frameRequestId = pickString(frame?.requestId);
  if (requestId && frameRequestId && frameRequestId !== requestId) {
    return undefined;
  }
  return {
    code: pickString(frame?.code) ?? 'websocket_error',
    message: pickString(frame?.message, frame?.detail) ?? 'websocket error',
    ...(frameRequestId ? { requestId: frameRequestId } : {}),
    type: 'error',
  };
}

function isFatalRealtimeControlError(error: ImRealtimeControlError): boolean {
  return /^websocket_(?:auth|upstream|connect)/u.test(error.code)
    || /(?:auth|session|token).*(?:failed|expired|invalid|required)/iu.test(error.code);
}

function websocketAuthTimeoutMs(auth: ImWebSocketAuthConfig | undefined): number {
  if (typeof auth?.timeoutMs === 'number' && Number.isFinite(auth.timeoutMs) && auth.timeoutMs > 0) {
    return auth.timeoutMs;
  }
  return DEFAULT_WEBSOCKET_AUTH_TIMEOUT_MS;
}

function websocketConnectionTimeoutMs(options: ImConnectOptions): number {
  return normalizePositiveDuration(
    options.connectionTimeoutMs,
    DEFAULT_WEBSOCKET_CONNECTION_TIMEOUT_MS,
    MIN_WEBSOCKET_CONNECTION_TIMEOUT_MS,
  );
}

function normalizePositiveDuration(
  value: unknown,
  fallback: number,
  minValue: number,
): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < minValue) {
    return fallback;
  }
  return value;
}

function resolveHeartbeatOptions(options: ImConnectOptions): Required<ImRealtimeHeartbeatOptions> | undefined {
  if (options.heartbeat === false) {
    return undefined;
  }
  return {
    intervalMs: normalizePositiveDuration(
      options.heartbeat?.intervalMs,
      DEFAULT_WEBSOCKET_HEARTBEAT_INTERVAL_MS,
      MIN_WEBSOCKET_HEARTBEAT_INTERVAL_MS,
    ),
    timeoutMs: normalizePositiveDuration(
      options.heartbeat?.timeoutMs,
      DEFAULT_WEBSOCKET_HEARTBEAT_TIMEOUT_MS,
      MIN_WEBSOCKET_HEARTBEAT_TIMEOUT_MS,
    ),
  };
}

function readCloseReason(event: unknown): string | undefined {
  return isRecord(event) ? pickString(event.reason) : undefined;
}

export function createImLiveConnection({
  accessToken,
  auth,
  authToken,
  headerProvider,
  headers,
  options,
  tokenManager,
  websocketBaseUrl,
  webSocketFactory,
}: ImCreateLiveConnectionParams): ImLiveConnection {
  const listeners: ListenerBag = {
    errors: new Set(),
    events: new Map(),
    messages: new Map(),
    states: new Set(),
  };
  let subscriptionConversations = pickStringArray(options.subscriptions?.conversations);
  let subscriptionScopes = normalizeRealtimeScopeSubscriptions(options.subscriptions?.scopes);
  const credentials = resolveWebSocketCredentials({ accessToken, auth, authToken, tokenManager });
  const url = buildWebSocketUrl(websocketBaseUrl, {
    ...options,
    subscriptions: {
      conversations: subscriptionConversations,
      scopes: subscriptionScopes,
    },
  });
  const usesBrowserWebSocket = !webSocketFactory;
  const resolvedHeaders = buildWebSocketHeaders({ ...credentials, headerProvider, headers });
  const socket = resolveWebSocketFactory(webSocketFactory)(url, {
    headers: resolvedHeaders,
    protocols: [IM_CCP_WEBSOCKET_SUBPROTOCOL],
  });
  const authInitRequestId = 'sdkwork-im-auth-init-1';
  const ccpHelloRequestId = 'sdkwork-im-ccp-hello-1';
  const frameAuthRequired = (usesBrowserWebSocket || !hasUpgradeAuthHeaders(resolvedHeaders)) && auth?.mode !== 'none';
  const frameAuthCredentials = credentials.accessToken && credentials.authToken
    ? { accessToken: credentials.accessToken, authToken: credentials.authToken }
    : undefined;
  let connectionPhase: ImRealtimeConnectionPhase = frameAuthRequired ? 'gateway_auth' : 'ccp_hello_ack';
  let authTimeout: ReturnType<typeof setTimeout> | undefined;
  let connectionTimeout: ReturnType<typeof setTimeout> | undefined;
  let currentState: ImLiveConnectionState = { status: 'connecting' };
  let pendingClose: { code: number; reason: string } | undefined;
  let suppressNextClosedState = false;
  let subscriptionSnapshotDirty = subscriptionConversations.length > 0 || subscriptionScopes.length > 0;
  let subscriptionSyncCounter = 0;
  const heartbeatOptions = resolveHeartbeatOptions(options);
  let heartbeatTimer: ReturnType<typeof setInterval> | undefined;
  let heartbeatCounter = 0;
  let lastHeartbeatRequestId: string | undefined;
  let lastInboundAt = Date.now();

  const emitState = (state: ImLiveConnectionState): void => {
    currentState = state;
    for (const handler of listeners.states) {
      handler(state);
    }
  };

  const emitError = (error: unknown): void => {
    for (const handler of listeners.errors) {
      handler(error);
    }
  };

  const clearAuthTimeout = (): void => {
    if (!authTimeout) {
      return;
    }
    clearTimeout(authTimeout);
    authTimeout = undefined;
  };

  const clearConnectionTimeout = (): void => {
    if (!connectionTimeout) {
      return;
    }
    clearTimeout(connectionTimeout);
    connectionTimeout = undefined;
  };

  const clearHeartbeatTimer = (): void => {
    if (!heartbeatTimer) {
      return;
    }
    clearInterval(heartbeatTimer);
    heartbeatTimer = undefined;
  };

  const closeSocket = (code: number, reason: string): void => {
    if (socket.readyState === SOCKET_CLOSING_STATE || socket.readyState === SOCKET_CLOSED_STATE) {
      return;
    }
    if (socket.readyState === SOCKET_CONNECTING_STATE) {
      pendingClose = { code, reason };
      return;
    }
    socket.close(code, reason);
  };

  const failAuth = (error: ImRealtimeControlError): void => {
    connectionPhase = 'gateway_auth';
    clearConnectionTimeout();
    clearAuthTimeout();
    clearHeartbeatTimer();
    emitState({ status: 'error', reason: error.message });
    emitError(error);
    closeSocket(4401, error.code);
  };

  const failCcpHandshake = (code: string, message: string): void => {
    connectionPhase = 'gateway_auth';
    clearConnectionTimeout();
    clearAuthTimeout();
    clearHeartbeatTimer();
    const error: ImRealtimeControlError = {
      code,
      message,
      type: 'error',
    };
    emitState({ status: 'error', reason: message });
    emitError(error);
    closeSocket(4401, code);
  };

  let pendingCcpAuthBindContext: ReturnType<typeof resolveCcpAuthBindContext> | undefined;
  let resumeNegotiated = false;

  const beginSessionResumeIfNeeded = (): void => {
    const sessionId = pendingCcpAuthBindContext?.sessionId;
    if (!resumeNegotiated || !sessionId) {
      pendingCcpAuthBindContext = undefined;
      emitOpenAndSyncSubscriptions();
      return;
    }
    connectionPhase = 'ccp_session_resume';
    socket.send(encodeCcpSessionResumeFrame(sessionId));
    startAuthTimeout();
  };

  const beginCcpHandshake = (authOk?: Record<string, unknown>): void => {
    const bindContext = resolveCcpAuthBindContext({
      accessToken: frameAuthCredentials?.accessToken ?? credentials.accessToken,
      authOk,
      deviceId: options.deviceId,
    });
    if (!bindContext) {
      failCcpHandshake(
        'websocket_ccp_auth_bind_unavailable',
        'websocket CCP auth_bind context is unavailable',
      );
      return;
    }
    pendingCcpAuthBindContext = bindContext;
    connectionPhase = 'ccp_hello_ack';
    socket.send(encodeCcpHelloFrame(ccpHelloRequestId));
    startAuthTimeout();
  };

  const failConnectionTimeout = (): void => {
    if (socket.readyState !== SOCKET_CONNECTING_STATE) {
      return;
    }
    suppressNextClosedState = true;
    const error: ImRealtimeControlError = {
      code: 'websocket_connect_timeout',
      message: 'websocket connection was not established before timeout',
      type: 'error',
    };
    emitState({ status: 'error', reason: error.message });
    emitError(error);
    closeSocket(4408, error.code);
  };

  const failHeartbeat = (): void => {
    clearHeartbeatTimer();
    const error: ImRealtimeControlError = {
      code: 'websocket_heartbeat_timeout',
      message: 'websocket heartbeat response was not received before timeout',
      ...(lastHeartbeatRequestId ? { requestId: lastHeartbeatRequestId } : {}),
      type: 'error',
    };
    emitState({ status: 'error', reason: error.message });
    emitError(error);
    closeSocket(4408, error.code);
  };

  const sendHeartbeat = (): void => {
    if (!heartbeatOptions || socket.readyState !== SOCKET_OPEN_STATE) {
      return;
    }
    if (lastHeartbeatRequestId && Date.now() - lastInboundAt > heartbeatOptions.timeoutMs) {
      failHeartbeat();
      return;
    }
    heartbeatCounter += 1;
    lastHeartbeatRequestId = `sdkwork-im-heartbeat-${heartbeatCounter}`;
    socket.send(encodeCcpHeartbeatFrame(heartbeatCounter));
  };

  const startHeartbeat = (): void => {
    clearHeartbeatTimer();
    if (!heartbeatOptions) {
      return;
    }
    lastInboundAt = Date.now();
    heartbeatTimer = setInterval(sendHeartbeat, heartbeatOptions.intervalMs);
  };

  const startAuthTimeout = (): void => {
    clearAuthTimeout();
    authTimeout = setTimeout(() => {
      if (connectionPhase === 'ready') {
        return;
      }
      if (connectionPhase === 'gateway_auth') {
        failAuth({
          code: 'websocket_auth_timeout',
          message: 'websocket auth.ok was not received before timeout',
          requestId: authInitRequestId,
          type: 'error',
        });
        return;
      }
      failCcpHandshake(
        'websocket_ccp_handshake_timeout',
        'websocket CCP handshake was not completed before timeout',
      );
    }, websocketAuthTimeoutMs(auth));
  };

  const emitOpenAndSyncSubscriptions = (): void => {
    clearConnectionTimeout();
    clearAuthTimeout();
    connectionPhase = 'ready';
    emitState({ status: 'open' });
    startHeartbeat();
    flushSubscriptionSync();
  };

  const flushSubscriptionSync = (): void => {
    if (
      !subscriptionSnapshotDirty
      || socket.readyState !== SOCKET_OPEN_STATE
      || connectionPhase !== 'ready'
    ) {
      return;
    }
    subscriptionSnapshotDirty = false;
    subscriptionSyncCounter += 1;
    sendSubscriptionSync(
      socket,
      mergeRealtimeScopeSubscriptions(subscriptionConversations, subscriptionScopes),
      `sdkwork-im-subscriptions-sync-${subscriptionSyncCounter}`,
    );
  };

  const syncConversations = (conversationIds: string[]): void => {
    subscriptionConversations = pickStringArray(conversationIds);
    subscriptionSnapshotDirty = true;
    if (connectionPhase === 'ready') {
      flushSubscriptionSync();
    }
  };

  const syncScopes = (scopes: ImRealtimeScopeSubscription[]): void => {
    subscriptionScopes = normalizeRealtimeScopeSubscriptions(scopes);
    subscriptionSnapshotDirty = true;
    if (connectionPhase === 'ready') {
      flushSubscriptionSync();
    }
  };

  connectionTimeout = setTimeout(failConnectionTimeout, websocketConnectionTimeoutMs(options));

  socket.addEventListener('open', () => {
    clearConnectionTimeout();
    if (pendingClose) {
      const { code, reason } = pendingClose;
      pendingClose = undefined;
      socket.close(code, reason);
      return;
    }

    if (frameAuthRequired) {
      if (!frameAuthCredentials) {
        failAuth({
          code: 'websocket_auth_tokens_not_ready',
          message: 'websocket auth tokens are not ready',
          requestId: authInitRequestId,
          type: 'error',
        });
        return;
      }
      sendAuthInit(socket, frameAuthCredentials, options.deviceId, authInitRequestId);
      startAuthTimeout();
      return;
    }

    beginCcpHandshake();
  });
  socket.addEventListener('close', (event: unknown) => {
    clearConnectionTimeout();
    clearAuthTimeout();
    clearHeartbeatTimer();
    if (suppressNextClosedState) {
      suppressNextClosedState = false;
      return;
    }
    emitState({ status: 'closed', reason: readCloseReason(event) });
  });
  socket.addEventListener('error', (event: unknown) => {
    clearConnectionTimeout();
    clearAuthTimeout();
    clearHeartbeatTimer();
    emitState({ status: 'error' });
    emitError(event);
  });
  socket.addEventListener('message', (event: unknown) => {
    const raw = extractMessageData(event);
    if (!raw) {
      return;
    }
    lastInboundAt = Date.now();
    if (connectionPhase === 'gateway_auth') {
      if (isAuthOkFrame(raw, authInitRequestId)) {
        const authOkFrame = parseRealtimeControlFrame(raw);
        beginCcpHandshake(authOkFrame);
        return;
      }
      const authError = parseRealtimeControlError(raw, authInitRequestId);
      if (authError) {
        failAuth(authError);
      }
      return;
    }
    if (connectionPhase === 'ccp_hello_ack') {
      if (isCcpHelloAckEnvelope(raw)) {
        resumeNegotiated = ccpHelloAckNegotiatesSessionResume(raw);
        const bindContext = pendingCcpAuthBindContext;
        if (!bindContext) {
          failCcpHandshake(
            'websocket_ccp_auth_bind_unavailable',
            'websocket CCP auth_bind context is unavailable',
          );
          return;
        }
        connectionPhase = 'ccp_auth_ok';
        socket.send(encodeCcpAuthBindFrame(bindContext));
        return;
      }
      const authError = parseRealtimeControlError(raw);
      if (authError) {
        failCcpHandshake(authError.code, authError.message);
      }
      return;
    }
    if (connectionPhase === 'ccp_auth_ok') {
      if (isCcpAuthOkEnvelope(raw)) {
        beginSessionResumeIfNeeded();
        return;
      }
      const authError = parseRealtimeControlError(raw);
      if (authError) {
        failCcpHandshake(authError.code, authError.message);
      }
      return;
    }
    if (connectionPhase === 'ccp_session_resume') {
      if (isCcpSessionResumedEnvelope(raw)) {
        pendingCcpAuthBindContext = undefined;
        emitOpenAndSyncSubscriptions();
        return;
      }
      const authError = parseRealtimeControlError(raw);
      if (authError) {
        failCcpHandshake(authError.code, authError.message);
      }
      return;
    }
    const inboundFrame = unwrapInboundRealtimeFrame(raw);
    const controlError = parseRealtimeControlError(inboundFrame);
    if (controlError) {
      if (isFatalRealtimeControlError(controlError)) {
        clearHeartbeatTimer();
        emitState({ status: 'error', reason: controlError.message });
        emitError(controlError);
        closeSocket(4401, controlError.code);
        return;
      }
      emitError(controlError);
      return;
    }
    const decodedEvents = parseRealtimeEvents(inboundFrame);
    for (const decoded of decodedEvents) {
      if (!decoded.context.scopeId || !decoded.context.scopeType) {
        continue;
      }
      decoded.context.ack = () => {
        if (socket.readyState === SOCKET_OPEN_STATE) {
          const ackedSeq = decoded.context.sequence;
          sendEventsAck(socket, `sdkwork-im-events-ack-${ackedSeq}`, ackedSeq);
        }
        return Promise.resolve();
      };
      const handlers = listeners.events.get(
        realtimeScopeKey(decoded.context.scopeType, decoded.context.scopeId),
      );
      if (!handlers) {
        continue;
      }
      for (const handler of handlers) {
        handler(decoded.event, decoded.context);
      }
    }
    const decodedMessages = parseRealtimePayloads(inboundFrame);
    for (const decoded of decodedMessages) {
      if (!decoded.context.conversationId) {
        continue;
      }
      decoded.context.ack = () => {
        if (socket.readyState === SOCKET_OPEN_STATE) {
          const ackedSeq = decoded.context.sequence;
          sendEventsAck(socket, `sdkwork-im-events-ack-${ackedSeq}`, ackedSeq);
        }
        return Promise.resolve();
      };
      const handlers = listeners.messages.get(decoded.context.conversationId);
      if (!handlers) {
        continue;
      }
      for (const handler of handlers) {
        handler(decoded.message, decoded.context);
      }
    }
  });

  return {
    disconnect(code = 1000, reason = 'client disconnect') {
      clearConnectionTimeout();
      clearAuthTimeout();
      clearHeartbeatTimer();
      closeSocket(code, reason);
    },
    events: {
      onConversation(conversationId, handler) {
        const key = realtimeScopeKey('conversation', conversationId);
        const handlers = listeners.events.get(key) ?? new Set();
        listeners.events.set(key, handlers);
        return subscribe(handlers, handler);
      },
      onScope(scopeType, scopeId, handler) {
        const key = realtimeScopeKey(scopeType, scopeId);
        const handlers = listeners.events.get(key) ?? new Set();
        listeners.events.set(key, handlers);
        return subscribe(handlers, handler);
      },
    },
    lifecycle: {
      onError(handler) {
        return subscribe(listeners.errors, handler);
      },
      onStateChange(handler) {
        const unsubscribe = subscribe(listeners.states, handler);
        handler(currentState);
        return unsubscribe;
      },
    },
    messages: {
      onConversation(conversationId, handler) {
        const handlers = listeners.messages.get(conversationId) ?? new Set();
        listeners.messages.set(conversationId, handlers);
        return subscribe(handlers, handler);
      },
    },
    subscriptions: {
      syncConversations,
      syncScopes,
    },
  };
}
