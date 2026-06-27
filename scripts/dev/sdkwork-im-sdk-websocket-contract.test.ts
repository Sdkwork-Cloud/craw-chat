import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

type ImDecodedMessage = import('../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index').ImDecodedMessage;
type ImCallSession = import('../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index').ImCallSession;
type ImMessageContext = import('../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index').ImMessageContext;
type ImRealtimeEventContext =
  import('../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index').ImRealtimeEventContext;
type ImWebSocketFactoryOptions =
  import('../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index').ImWebSocketFactoryOptions;
type ImWebSocketLike = import('../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index').ImWebSocketLike;

const tsxReentryEnv = 'SDKWORK_IM_SDK_WEBSOCKET_CONTRACT_TSX';
const scriptPath = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(scriptPath), '..', '..');
const appRoot = path.join(repoRoot, 'apps', 'sdkwork-im-pc');

function resolveTsxCliPath(): string {
  const candidates = [
    path.join(appRoot, 'node_modules', 'tsx', 'dist', 'cli.mjs'),
    path.join(repoRoot, 'node_modules', 'tsx', 'dist', 'cli.mjs'),
  ];
  for (const candidate of candidates) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }
  try {
    return createRequire(path.join(appRoot, 'package.json')).resolve('tsx/cli');
  } catch {
    return '';
  }
}

const tsxCliPath = resolveTsxCliPath();

if (process.env[tsxReentryEnv] !== '1') {
  if (!tsxCliPath) {
    console.error('[sdkwork-im-sdk-websocket-contract] missing tsx runner');
    console.error('[sdkwork-im-sdk-websocket-contract] run pnpm install from the sdkwork-im repository root first');
    process.exit(1);
  }

  const result = spawnSync(process.execPath, [tsxCliPath, scriptPath], {
    cwd: repoRoot,
    env: {
      ...process.env,
      [tsxReentryEnv]: '1',
    },
    stdio: 'inherit',
  });

  if (result.error) {
    throw result.error;
  }

  process.exit(result.status ?? (result.signal ? 1 : 0));
}

type FakeWebSocketEventName = 'close' | 'error' | 'message' | 'open';

class FakeWebSocket implements ImWebSocketLike {
  readonly sent: string[] = [];
  readonly url: string;
  readonly options: ImWebSocketFactoryOptions;
  readyState = 0;

  private readonly listeners = new Map<FakeWebSocketEventName, Set<(event: unknown) => void>>();

  constructor(url: string, options: ImWebSocketFactoryOptions) {
    this.url = url;
    this.options = options;
  }

  addEventListener(type: FakeWebSocketEventName, handler: (event: unknown) => void): void {
    const listeners = this.listeners.get(type) ?? new Set();
    listeners.add(handler);
    this.listeners.set(type, listeners);
  }

  close(code?: number, reason?: string): void {
    this.readyState = 3;
    this.emit('close', { code, reason });
  }

  emit(type: FakeWebSocketEventName, event: unknown): void {
    for (const handler of this.listeners.get(type) ?? []) {
      handler(event);
    }
  }

  open(): void {
    this.readyState = 1;
    this.emit('open', {});
  }

  send(value: string): void {
    this.sent.push(value);
  }
}

class FakeBrowserWebSocket implements ImWebSocketLike {
  static readonly instances: FakeBrowserWebSocket[] = [];

  readonly closeCalls: Array<{ code?: number; reason?: string }> = [];
  readonly sent: string[] = [];
  readonly protocols?: string | string[];
  readonly url: string;
  readyState = 0;

  private readonly listeners = new Map<FakeWebSocketEventName, Set<(event: unknown) => void>>();

  constructor(url: string, protocols?: string | string[]) {
    this.url = url;
    this.protocols = protocols;
    FakeBrowserWebSocket.instances.push(this);
  }

  addEventListener(type: FakeWebSocketEventName, handler: (event: unknown) => void): void {
    const listeners = this.listeners.get(type) ?? new Set();
    listeners.add(handler);
    this.listeners.set(type, listeners);
  }

  close(code?: number, reason?: string): void {
    this.closeCalls.push({ code, reason });
    this.readyState = 3;
    this.emit('close', { code, reason });
  }

  emit(type: FakeWebSocketEventName, event: unknown): void {
    for (const handler of this.listeners.get(type) ?? []) {
      handler(event);
    }
  }

  open(): void {
    this.readyState = 1;
    this.emit('open', {});
  }

  send(value: string): void {
    this.sent.push(value);
  }
}

function parseSent(socket: FakeWebSocket, index: number): Record<string, unknown> {
  const raw = socket.sent[index];
  assert.equal(typeof raw, 'string', `expected websocket frame ${index} to be sent`);
  return JSON.parse(raw) as Record<string, unknown>;
}

function parseBrowserSent(socket: FakeBrowserWebSocket, index: number): Record<string, unknown> {
  const raw = socket.sent[index];
  assert.equal(typeof raw, 'string', `expected browser websocket frame ${index} to be sent`);
  return JSON.parse(raw) as Record<string, unknown>;
}

const IM_CCP_WS_SUBPROTOCOL = 'sdkwork-im.ccp.ws.v1';

function fakeAccessToken(claims: Record<string, unknown>): string {
  const payload = Buffer.from(JSON.stringify(claims)).toString('base64url');
  return `header.${payload}.signature`;
}

const TEST_ACCESS_CLAIMS = {
  tenant_id: '100001',
  user_id: '30',
  organization_id: '0',
  session_id: 'session-1',
  subject_type: 'user',
};

function parseCcpPayload(raw: string): Record<string, unknown> {
  const envelope = JSON.parse(raw) as Record<string, unknown>;
  return JSON.parse(String(envelope.payload)) as Record<string, unknown>;
}

function parseCcpBusinessPayload(raw: string): Record<string, unknown> {
  return parseCcpPayload(raw);
}

function parseCcpControlType(raw: string): string | undefined {
  const payload = parseCcpPayload(raw);
  return typeof payload.type === 'string' ? payload.type : undefined;
}

function encodeCcpControlFrame(schema: string, controlType: string, data: Record<string, unknown>): string {
  return JSON.stringify({
    protocol: { family: 'ccp', major: 1, minor: 0 },
    binding: 'Ws1',
    kind: 'control',
    schema,
    scope: null,
    route: null,
    flags: [],
    payload: JSON.stringify({ type: controlType, data }),
  });
}

function emitCcpHelloAck(socket: FakeWebSocket | FakeBrowserWebSocket): void {
  socket.emit('message', {
    data: encodeCcpControlFrame('cc.control.hello_ack.v1', 'hello_ack', {
      protocol: { family: 'ccp', major: 1, minor: 0 },
      binding: 'Ws1',
      capabilities: { items: ['payload.json', 'session.resume'] },
      accepted: true,
    }),
  });
}

function emitCcpAuthOk(
  socket: FakeWebSocket | FakeBrowserWebSocket,
  context: {
    tenantId: string;
    principalId: string;
    deviceId?: string;
    sessionId?: string;
    actorKind?: string;
  },
): void {
  socket.emit('message', {
    data: encodeCcpControlFrame('cc.control.auth_ok.v1', 'auth_ok', {
      tenant_id: context.tenantId,
      principal_id: context.principalId,
      actor_kind: context.actorKind ?? 'user',
      device_id: context.deviceId ?? null,
      session_id: context.sessionId ?? null,
    }),
  });
}

function emitCcpSessionResumed(
  socket: FakeWebSocket | FakeBrowserWebSocket,
  sessionId: string,
): void {
  socket.emit('message', {
    data: encodeCcpControlFrame('cc.control.session_resumed.v1', 'session_resumed', {
      session_id: sessionId,
      resumed: true,
    }),
  });
}

function tryParseCcpControlType(raw: string | undefined): string | undefined {
  if (!raw) {
    return undefined;
  }
  try {
    const envelope = JSON.parse(raw) as Record<string, unknown>;
    if (typeof envelope.payload !== 'string') {
      return undefined;
    }
    return parseCcpControlType(raw);
  } catch {
    return undefined;
  }
}

function completeNegotiatedSessionResume(
  socket: FakeWebSocket | FakeBrowserWebSocket,
  sessionId: string,
): void {
  const resumeIndex = socket.sent.findIndex((frame) => tryParseCcpControlType(frame) === 'session_resume');
  assert.notEqual(resumeIndex, -1, 'client must send CCP session_resume when session.resume is negotiated');
  emitCcpSessionResumed(socket, sessionId);
}

function completeFactoryCcpHandshake(
  socket: FakeWebSocket,
  bindContext = {
    tenantId: '100001',
    principalId: '30',
    sessionId: 'session-1',
    deviceId: 'device-1',
    actorKind: 'user',
  },
): void {
  assert.equal(parseCcpControlType(socket.sent[0] ?? ''), 'hello', 'factory websocket must begin with CCP hello');
  emitCcpHelloAck(socket);
  assert.equal(parseCcpControlType(socket.sent[1] ?? ''), 'auth_bind', 'factory websocket must send CCP auth_bind after hello_ack');
  emitCcpAuthOk(socket, bindContext);
  completeNegotiatedSessionResume(socket, bindContext.sessionId ?? 'session-1');
}

function completeBrowserGatewayAuthAndCcp(
  browserSocket: FakeBrowserWebSocket,
  authOk: Record<string, unknown>,
): void {
  browserSocket.emit('message', {
    data: JSON.stringify({
      type: 'auth.ok',
      ...authOk,
    }),
  });
  assert.equal(parseCcpControlType(browserSocket.sent[1] ?? ''), 'hello', 'browser websocket must begin CCP hello after gateway auth.ok');
  emitCcpHelloAck(browserSocket);
  assert.equal(parseCcpControlType(browserSocket.sent[2] ?? ''), 'auth_bind', 'browser websocket must send CCP auth_bind after hello_ack');
  emitCcpAuthOk(browserSocket, {
    tenantId: String(authOk.tenantId),
    principalId: String(authOk.principalId),
    deviceId: typeof authOk.deviceId === 'string' ? authOk.deviceId : undefined,
    sessionId: typeof authOk.sessionId === 'string' ? authOk.sessionId : undefined,
  });
  completeNegotiatedSessionResume(
    browserSocket,
    typeof authOk.sessionId === 'string' ? authOk.sessionId : 'session_real',
  );
}

async function main(): Promise<void> {
  const { ImSdkClient, ImWebSocketAuthOptions } = await import(
    '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index'
  );
  const originalWebSocket = Object.getOwnPropertyDescriptor(globalThis, 'WebSocket');
  Object.defineProperty(globalThis, 'WebSocket', {
    configurable: true,
    value: undefined,
  });

  try {
    const sockets: FakeWebSocket[] = [];
    const client = new ImSdkClient({
      accessToken: fakeAccessToken(TEST_ACCESS_CLAIMS),
      authToken: 'auth-token-1',
      headerProvider: () => ({
        'X-Trace-Id': 'trace-1',
      }),
      webSocketAuth: ImWebSocketAuthOptions.automatic({
        credentialProvider: () => 'auth-token-1',
      }),
      webSocketFactory: (url, options) => {
        const socket = new FakeWebSocket(url, options);
        sockets.push(socket);
        return socket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });

    const connection = await client.connect({
      deviceId: 'device-1',
      subscriptions: {
        conversations: ['conversation-1', 'conversation-2'],
      },
    });

    assert.equal(sockets.length, 1, 'IM SDK must use the injected websocket factory in non-browser runtimes');
    const socket = sockets[0];
    assert.equal(
      socket.url,
      'wss://chat.example.com/sdkwork/chat/im/v3/api/realtime/ws?deviceId=device-1',
      'IM websocket URL must preserve deployment base paths and keep subscriptions out of the handshake query',
    );
    assert.equal(socket.url.includes('conversationId='), false, 'IM websocket subscriptions must be sent as frames');
    assert.deepEqual(
      socket.options.protocols,
      [IM_CCP_WS_SUBPROTOCOL],
      'IM websocket must negotiate the canonical CCP subprotocol without embedding auth tokens',
    );
    assert.deepEqual(socket.options.headers, {
      Authorization: 'Bearer auth-token-1',
      'Access-Token': fakeAccessToken(TEST_ACCESS_CLAIMS),
      'X-Trace-Id': 'trace-1',
    });

    const lifecycleStates: string[] = [];
    const received: Array<{ context: ImMessageContext; message: ImDecodedMessage }> = [];
    const userScopeEvents: Array<{ context: ImRealtimeEventContext; event: Record<string, unknown> }> = [];
    connection.lifecycle.onStateChange((state) => lifecycleStates.push(state.status));
    connection.messages.onConversation('conversation-1', (message, context) => {
      received.push({ context, message });
    });
    connection.events.onScope('user', '30', (event, context) => {
      userScopeEvents.push({ context, event });
    });

    socket.open();
    assert.equal(
      socket.sent.some((frame) => frame.includes('subscriptions.sync')),
      false,
      'factory websocket must not send subscriptions before CCP handshake completes',
    );
    completeFactoryCcpHandshake(socket);

    assert.deepEqual(lifecycleStates, ['connecting', 'open']);
    assert.equal(parseCcpControlType(socket.sent[2] ?? ''), 'session_resume', 'factory websocket must send CCP session_resume when negotiated');
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[3]), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-1',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'conversation-1',
          eventTypes: ['message.posted'],
        },
        {
          scopeType: 'conversation',
          scopeId: 'conversation-2',
          eventTypes: ['message.posted'],
        },
      ],
    });
    connection.subscriptions.syncConversations(['conversation-1', 'conversation-3']);
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[4]), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-2',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'conversation-1',
          eventTypes: ['message.posted'],
        },
        {
          scopeType: 'conversation',
          scopeId: 'conversation-3',
          eventTypes: ['message.posted'],
        },
      ],
    });
    connection.subscriptions.syncConversations([]);
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[5]), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-3',
      items: [],
    });
    connection.subscriptions.syncConversations(['conversation-1', 'conversation-3']);
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[6]), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-4',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'conversation-1',
          eventTypes: ['message.posted'],
        },
        {
          scopeType: 'conversation',
          scopeId: 'conversation-3',
          eventTypes: ['message.posted'],
        },
      ],
    });
    connection.subscriptions.syncScopes([
      {
        scopeType: 'user',
        scopeId: '30',
        eventTypes: [
          'friend_request.submitted',
          'friend_request.accepted',
          'friend_request.declined',
          'friend_request.canceled',
        ],
      },
    ]);
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[7]), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-5',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'conversation-1',
          eventTypes: ['message.posted'],
        },
        {
          scopeType: 'conversation',
          scopeId: 'conversation-3',
          eventTypes: ['message.posted'],
        },
        {
          scopeType: 'user',
          scopeId: '30',
          eventTypes: [
            'friend_request.submitted',
            'friend_request.accepted',
            'friend_request.declined',
            'friend_request.canceled',
          ],
        },
      ],
    });

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        requestId: null,
        reason: 'push',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-friend-request-1',
              eventType: 'friend_request.submitted',
              occurredAt: '2026-06-06T09:59:00.000Z',
              payload: JSON.stringify({
                friendRequest: {
                  requestId: 'friend-request-1',
                  requesterUserId: 'user-2',
                  targetUserId: '30',
                  status: 'pending',
                },
              }),
              realtimeSeq: 6,
              scopeType: 'user',
              scopeId: '30',
            },
          ],
          nextAfterSeq: 6,
        },
      }),
    });

    assert.equal(userScopeEvents.length, 1, 'IM SDK must dispatch non-conversation user-scope realtime events');
    assert.equal(userScopeEvents[0].context.scopeType, 'user');
    assert.equal(userScopeEvents[0].context.scopeId, '30');
    assert.equal(userScopeEvents[0].context.eventType, 'friend_request.submitted');
    assert.equal(userScopeEvents[0].context.sequence, 6);
    assert.deepEqual(userScopeEvents[0].context.payload, {
      friendRequest: {
        requestId: 'friend-request-1',
        requesterUserId: 'user-2',
        targetUserId: '30',
        status: 'pending',
      },
    });
    await userScopeEvents[0].context.ack();
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[8]), {
      type: 'events.ack',
      requestId: 'sdkwork-im-events-ack-6',
      ackedSeq: 6,
    });

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        requestId: null,
        reason: 'push',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:00:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [{ kind: 'text', text: 'hello over websocket' }],
                  renderHints: {
                    sdkworkChatPcType: 'text',
                  },
                  summary: 'hello over websocket',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-1',
                messageSeq: 42,
                messageType: 'standard',
                occurredAt: '2026-06-06T10:00:00.000Z',
                sender: {
                  id: 'user-2',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'hello over websocket',
              }),
              realtimeSeq: 7,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 7,
        },
      }),
    });

    assert.equal(received.length, 1, 'IM SDK must decode realtime event windows into conversation messages');
    assert.equal(received[0].context.eventId, 'event-1');
    assert.equal(received[0].context.conversationId, 'conversation-1');
    assert.equal(received[0].context.sequence, 7);
    assert.equal(received[0].message.messageId, 'message-1');
    assert.equal(received[0].message.summary, 'hello over websocket');
    assert.deepEqual(received[0].message.content, {
      body: {
        parts: [{ kind: 'text', text: 'hello over websocket' }],
        renderHints: {
          sdkworkChatPcType: 'text',
        },
        summary: 'hello over websocket',
      },
      conversationId: 'conversation-1',
      deliveryMode: 'discrete',
      messageId: 'message-1',
      messageSeq: 42,
      messageType: 'standard',
      occurredAt: '2026-06-06T10:00:00.000Z',
      sender: {
        id: 'user-2',
        kind: 'user',
        metadata: {},
      },
      summary: 'hello over websocket',
    });

    const incomingSessions: ImCallSession[] = [];
    const unsubscribeIncoming = client.calls.subscribe((session) => {
      incomingSessions.push(session);
    });
    const initialIncoming = await client.calls.watchIncoming({
      conversationIds: ['conversation-1'],
      connection,
    });
    assert.equal(initialIncoming, null, 'IM calls watcher should start empty before RTC invite signaling arrives');

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        requestId: null,
        reason: 'push',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-invite-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:01:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        artifactMessageId: null,
                        conversationId: 'conversation-1',
                        rtcMode: 'video',
                        rtcSessionId: 'rtc-session-incoming-1',
                        signalingStreamId: 'signal-stream-1',
                        state: 'started',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.invite',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.invite',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-invite-1',
                messageSeq: 43,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:01:00.000Z',
                sender: {
                  id: 'user-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.invite',
              }),
              realtimeSeq: 8,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 8,
        },
      }),
    });

    assert.equal(incomingSessions.length, 1, 'IM SDK calls facade must publish RTC invite sessions from realtime signal messages');
    assert.equal(incomingSessions[0].rtcSessionId, 'rtc-session-incoming-1');
    assert.equal(incomingSessions[0].conversationId, 'conversation-1');
    assert.equal(incomingSessions[0].rtcMode, 'video');
    assert.equal(incomingSessions[0].state, 'started');
    assert.equal(incomingSessions[0].initiatorId, 'user-caller');
    assert.equal(incomingSessions[0].initiatorKind, 'user');
    const watchedIncoming = await client.calls.watchIncoming();
    assert.equal(watchedIncoming?.rtcSessionId, 'rtc-session-incoming-1');

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-end-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:02:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        artifactMessageId: null,
                        conversationId: 'conversation-1',
                        rtcMode: 'video',
                        rtcSessionId: 'rtc-session-incoming-1',
                        signalingStreamId: 'signal-stream-1',
                        state: 'ended',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.end',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.end',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-end-1',
                messageSeq: 44,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:02:00.000Z',
                sender: {
                  id: 'user-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.end',
              }),
              realtimeSeq: 9,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 9,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      2,
      'IM SDK calls facade must publish closing RTC signals so PC call state can sync accept/reject/end actions',
    );
    assert.equal(incomingSessions[1].rtcSessionId, 'rtc-session-incoming-1');
    assert.equal(incomingSessions[1].state, 'ended');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, undefined);

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-nested-invite-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:03:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        signalPayload: JSON.stringify({
                          artifactMessageId: null,
                          conversationId: 'conversation-1',
                          rtcMode: 'voice',
                          rtcSessionId: 'rtc-session-incoming-2',
                          signalingStreamId: 'signal-stream-2',
                          state: 'started',
                        }),
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.invite',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.invite',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-nested-invite-1',
                messageSeq: 45,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:03:00.000Z',
                sender: {
                  id: 'user-nested-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.invite',
              }),
              realtimeSeq: 10,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 10,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      3,
      'IM SDK calls facade must publish RTC invite sessions when signal fields are nested in signalPayload',
    );
    assert.equal(incomingSessions[2].rtcSessionId, 'rtc-session-incoming-2');
    assert.equal(incomingSessions[2].rtcMode, 'voice');
    assert.equal(incomingSessions[2].initiatorId, 'user-nested-caller');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, 'rtc-session-incoming-2');

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-nested-end-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:04:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        signalPayload: JSON.stringify({
                          rtcSessionId: 'rtc-session-incoming-2',
                          state: 'ended',
                        }),
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.end',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.end',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-nested-end-1',
                messageSeq: 46,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:04:00.000Z',
                sender: {
                  id: 'user-nested-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.end',
              }),
              realtimeSeq: 11,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 11,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      4,
      'IM SDK calls facade must reuse cached invite metadata when closing RTC signals only carry session id and state',
    );
    assert.equal(incomingSessions[3].rtcSessionId, 'rtc-session-incoming-2');
    assert.equal(incomingSessions[3].state, 'ended');
    assert.equal(incomingSessions[3].rtcMode, 'voice');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, undefined);

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-type-only-invite-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:05:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        conversationId: 'conversation-1',
                        rtcMode: 'video',
                        rtcSessionId: 'rtc-session-type-only-1',
                        signalingStreamId: 'signal-stream-type-only-1',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.invite',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.invite',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-type-only-invite-1',
                messageSeq: 47,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:05:00.000Z',
                sender: {
                  id: 'user-type-only-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.invite',
              }),
              realtimeSeq: 12,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 12,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      5,
      'IM SDK calls facade must infer a started RTC session from rtc.invite even when payload omits state',
    );
    assert.equal(incomingSessions[4].rtcSessionId, 'rtc-session-type-only-1');
    assert.equal(incomingSessions[4].state, 'started');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, 'rtc-session-type-only-1');

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-type-only-accept-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:06:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        rtcSessionId: 'rtc-session-type-only-1',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.accept',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.accept',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-type-only-accept-1',
                messageSeq: 48,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:06:00.000Z',
                sender: {
                  id: 'user-type-only-callee',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.accept',
              }),
              realtimeSeq: 13,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 13,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      6,
      'IM SDK calls facade must infer accepted state from rtc.accept when payload omits state',
    );
    assert.equal(incomingSessions[5].rtcSessionId, 'rtc-session-type-only-1');
    assert.equal(incomingSessions[5].state, 'accepted');
    assert.equal(incomingSessions[5].rtcMode, 'video');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, undefined);

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-type-only-end-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:06:30.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        rtcSessionId: 'rtc-session-type-only-1',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.end',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.end',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-type-only-end-1',
                messageSeq: 49,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:06:30.000Z',
                sender: {
                  id: 'user-type-only-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.end',
              }),
              realtimeSeq: 14,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 14,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      7,
      'IM SDK calls facade must keep accepted session metadata so a later rtc.end with only rtcSessionId can close the active call',
    );
    assert.equal(incomingSessions[6].rtcSessionId, 'rtc-session-type-only-1');
    assert.equal(incomingSessions[6].state, 'ended');
    assert.equal(incomingSessions[6].rtcMode, 'video');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, undefined);

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-type-only-invite-2',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:07:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        conversationId: 'conversation-1',
                        rtcMode: 'voice',
                        rtcSessionId: 'rtc-session-type-only-2',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.invite',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.invite',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-type-only-invite-2',
                messageSeq: 50,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:07:00.000Z',
                sender: {
                  id: 'user-type-only-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.invite',
              }),
              realtimeSeq: 15,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 15,
        },
      }),
    });

    assert.equal(incomingSessions.length, 8);
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, 'rtc-session-type-only-2');

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-rtc-type-only-end-2',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:08:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        rtcSessionId: 'rtc-session-type-only-2',
                      }),
                      schemaRef: 'rtc.signal.v1',
                      signalType: 'rtc.end',
                    },
                  ],
                  renderHints: {
                    channel: 'rtc',
                  },
                  summary: 'rtc.end',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-rtc-type-only-end-2',
                messageSeq: 51,
                messageType: 'signal',
                occurredAt: '2026-06-06T10:08:00.000Z',
                sender: {
                  id: 'user-type-only-caller',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'rtc.end',
              }),
              realtimeSeq: 16,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 16,
        },
      }),
    });

    assert.equal(
      incomingSessions.length,
      9,
      'IM SDK calls facade must infer ended state from rtc.end when payload omits state',
    );
    assert.equal(incomingSessions[8].rtcSessionId, 'rtc-session-type-only-2');
    assert.equal(incomingSessions[8].state, 'ended');
    assert.equal(incomingSessions[8].rtcMode, 'voice');
    assert.equal((await client.calls.watchIncoming())?.rtcSessionId, undefined);
    unsubscribeIncoming();

    await received[0].context.ack();
    assert.deepEqual(parseCcpBusinessPayload(socket.sent[9]), {
      type: 'events.ack',
      requestId: 'sdkwork-im-events-ack-7',
      ackedSeq: 7,
    });

    connection.disconnect(1000, 'test complete');
    assert.equal(socket.readyState, 3);

    const heartbeatSockets: FakeWebSocket[] = [];
    const heartbeatClient = new ImSdkClient({
      accessToken: fakeAccessToken({ ...TEST_ACCESS_CLAIMS, device_id: 'heartbeat-device-1' }),
      authToken: 'heartbeat-auth-token',
      webSocketAuth: ImWebSocketAuthOptions.none(),
      webSocketFactory: (url, options) => {
        const heartbeatSocket = new FakeWebSocket(url, options);
        heartbeatSockets.push(heartbeatSocket);
        return heartbeatSocket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const heartbeatConnection = await heartbeatClient.connect({
      deviceId: 'heartbeat-device-1',
      heartbeat: {
        intervalMs: 5,
        timeoutMs: 30,
      },
      subscriptions: {
        conversations: [],
      },
    });
    const heartbeatSocket = heartbeatSockets[0];
    const heartbeatErrors: unknown[] = [];
    heartbeatConnection.lifecycle.onError((error) => heartbeatErrors.push(error));
    heartbeatSocket.open();
    completeFactoryCcpHandshake(heartbeatSocket, {
      tenantId: '100001',
      principalId: '30',
      sessionId: 'session-1',
      deviceId: 'heartbeat-device-1',
    });
    await new Promise((resolve) => setTimeout(resolve, 12));
    assert.equal(parseCcpControlType(heartbeatSocket.sent[3] ?? ''), 'heartbeat', 'IM realtime connection must send CCP heartbeat frames after opening');
    heartbeatSocket.emit('message', {
      data: encodeCcpControlFrame('cc.control.heartbeat.v1', 'heartbeat', { sequence: 1 }),
    });
    await new Promise((resolve) => setTimeout(resolve, 18));
    assert.equal(heartbeatErrors.length, 0, 'IM realtime heartbeat must treat any inbound frame as connection liveness');
    assert.equal(heartbeatSocket.readyState, 1);
    heartbeatConnection.disconnect(1000, 'heartbeat test complete');

    const staleHeartbeatSockets: FakeWebSocket[] = [];
    const staleHeartbeatClient = new ImSdkClient({
      accessToken: fakeAccessToken({ ...TEST_ACCESS_CLAIMS, device_id: 'stale-heartbeat-device-1' }),
      authToken: 'stale-heartbeat-auth-token',
      webSocketAuth: ImWebSocketAuthOptions.none(),
      webSocketFactory: (url, options) => {
        const staleHeartbeatSocket = new FakeWebSocket(url, options);
        staleHeartbeatSockets.push(staleHeartbeatSocket);
        return staleHeartbeatSocket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const staleHeartbeatConnection = await staleHeartbeatClient.connect({
      deviceId: 'stale-heartbeat-device-1',
      heartbeat: {
        intervalMs: 5,
        timeoutMs: 12,
      },
      subscriptions: {
        conversations: [],
      },
    });
    const staleHeartbeatSocket = staleHeartbeatSockets[0];
    const staleHeartbeatStates: string[] = [];
    const staleHeartbeatErrors: unknown[] = [];
    staleHeartbeatConnection.lifecycle.onStateChange((state) => staleHeartbeatStates.push(state.status));
    staleHeartbeatConnection.lifecycle.onError((error) => staleHeartbeatErrors.push(error));
    staleHeartbeatSocket.open();
    completeFactoryCcpHandshake(staleHeartbeatSocket, {
      tenantId: '100001',
      principalId: '30',
      sessionId: 'session-1',
      deviceId: 'stale-heartbeat-device-1',
    });
    await new Promise((resolve) => setTimeout(resolve, 30));
    assert.deepEqual(staleHeartbeatStates, ['connecting', 'open', 'error', 'closed']);
    assert.deepEqual(staleHeartbeatErrors[0], {
      code: 'websocket_heartbeat_timeout',
      message: 'websocket heartbeat response was not received before timeout',
      requestId: 'sdkwork-im-heartbeat-1',
      type: 'error',
    });
    assert.equal(staleHeartbeatSocket.readyState, 3, 'IM realtime heartbeat timeout must close stale sockets');
    staleHeartbeatConnection.disconnect(1000, 'stale heartbeat test complete');

    const runtimeErrorSockets: FakeWebSocket[] = [];
    const runtimeErrorClient = new ImSdkClient({
      accessToken: fakeAccessToken({ ...TEST_ACCESS_CLAIMS, device_id: 'runtime-error-device-1' }),
      authToken: 'runtime-error-auth-token',
      webSocketAuth: ImWebSocketAuthOptions.none(),
      webSocketFactory: (url, options) => {
        const runtimeErrorSocket = new FakeWebSocket(url, options);
        runtimeErrorSockets.push(runtimeErrorSocket);
        return runtimeErrorSocket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const runtimeErrorConnection = await runtimeErrorClient.connect({
      deviceId: 'runtime-error-device-1',
      heartbeat: false,
      subscriptions: {
        conversations: [],
      },
    });
    const runtimeErrorSocket = runtimeErrorSockets[0];
    const runtimeErrors: unknown[] = [];
    runtimeErrorConnection.lifecycle.onError((error) => runtimeErrors.push(error));
    runtimeErrorSocket.open();
    completeFactoryCcpHandshake(runtimeErrorSocket, {
      tenantId: '100001',
      principalId: '30',
      sessionId: 'session-1',
      deviceId: 'runtime-error-device-1',
    });
    runtimeErrorSocket.emit('message', {
      data: JSON.stringify({
        type: 'error',
        requestId: 'server-error-1',
        code: 'subscription_forbidden',
        message: 'conversation access denied',
      }),
    });
    assert.deepEqual(runtimeErrors, [
      {
        code: 'subscription_forbidden',
        message: 'conversation access denied',
        requestId: 'server-error-1',
        type: 'error',
      },
    ], 'IM realtime SDK must surface server control error frames after the connection is open');
    assert.equal(runtimeErrorSocket.readyState, 1, 'non-fatal realtime control errors must not close an otherwise healthy socket');
    runtimeErrorConnection.disconnect(1000, 'runtime error test complete');

    const fatalRuntimeErrorSockets: FakeWebSocket[] = [];
    const fatalRuntimeErrorClient = new ImSdkClient({
      accessToken: fakeAccessToken({ ...TEST_ACCESS_CLAIMS, device_id: 'fatal-runtime-error-device-1' }),
      authToken: 'fatal-runtime-error-auth-token',
      webSocketAuth: ImWebSocketAuthOptions.none(),
      webSocketFactory: (url, options) => {
        const fatalRuntimeErrorSocket = new FakeWebSocket(url, options);
        fatalRuntimeErrorSockets.push(fatalRuntimeErrorSocket);
        return fatalRuntimeErrorSocket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const fatalRuntimeErrorConnection = await fatalRuntimeErrorClient.connect({
      deviceId: 'fatal-runtime-error-device-1',
      heartbeat: false,
      subscriptions: {
        conversations: [],
      },
    });
    const fatalRuntimeErrorSocket = fatalRuntimeErrorSockets[0];
    const fatalRuntimeErrorStates: string[] = [];
    const fatalRuntimeErrors: unknown[] = [];
    fatalRuntimeErrorConnection.lifecycle.onStateChange((state) => fatalRuntimeErrorStates.push(state.status));
    fatalRuntimeErrorConnection.lifecycle.onError((error) => fatalRuntimeErrors.push(error));
    fatalRuntimeErrorSocket.open();
    completeFactoryCcpHandshake(fatalRuntimeErrorSocket, {
      tenantId: '100001',
      principalId: '30',
      sessionId: 'session-1',
      deviceId: 'fatal-runtime-error-device-1',
    });
    fatalRuntimeErrorSocket.emit('message', {
      data: JSON.stringify({
        type: 'error',
        requestId: 'fatal-server-error-1',
        code: 'websocket_auth_failed',
        message: 'session expired after connection opened',
      }),
    });
    assert.deepEqual(fatalRuntimeErrorStates, ['connecting', 'open', 'error', 'closed']);
    assert.deepEqual(fatalRuntimeErrors, [
      {
        code: 'websocket_auth_failed',
        message: 'session expired after connection opened',
        requestId: 'fatal-server-error-1',
        type: 'error',
      },
    ]);
    assert.equal(fatalRuntimeErrorSocket.readyState, 3, 'fatal realtime control errors must close the socket so app reconnect can recover');

    const connectTimeoutSockets: FakeWebSocket[] = [];
    const connectTimeoutClient = new ImSdkClient({
      accessToken: fakeAccessToken({ ...TEST_ACCESS_CLAIMS, device_id: 'connect-timeout-device-1' }),
      authToken: 'connect-timeout-auth-token',
      webSocketAuth: ImWebSocketAuthOptions.none(),
      webSocketFactory: (url, options) => {
        const connectTimeoutSocket = new FakeWebSocket(url, options);
        connectTimeoutSockets.push(connectTimeoutSocket);
        return connectTimeoutSocket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const connectTimeoutConnection = await connectTimeoutClient.connect({
      connectionTimeoutMs: 1,
      deviceId: 'connect-timeout-device-1',
      heartbeat: false,
      subscriptions: {
        conversations: [],
      },
    });
    const connectTimeoutSocket = connectTimeoutSockets[0];
    const connectTimeoutStates: string[] = [];
    const connectTimeoutErrors: unknown[] = [];
    connectTimeoutConnection.lifecycle.onStateChange((state) => connectTimeoutStates.push(state.status));
    connectTimeoutConnection.lifecycle.onError((error) => connectTimeoutErrors.push(error));
    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.deepEqual(connectTimeoutStates, ['connecting', 'error']);
    assert.deepEqual(connectTimeoutErrors, [
      {
        code: 'websocket_connect_timeout',
        message: 'websocket connection was not established before timeout',
        type: 'error',
      },
    ]);
    assert.equal(
      connectTimeoutSocket.readyState,
      0,
      'IM realtime SDK must not call native close while a browser-compatible socket is still CONNECTING',
    );
    connectTimeoutSocket.open();
    assert.equal(connectTimeoutSocket.readyState, 3, 'pending connect-timeout close must be applied once the socket opens');

    FakeBrowserWebSocket.instances.length = 0;
    Object.defineProperty(globalThis, 'WebSocket', {
      configurable: true,
      value: FakeBrowserWebSocket,
    });

    const closingBrowserClient = new ImSdkClient({
      accessToken: 'closing-browser-access-token',
      authToken: 'closing-browser-auth-token',
      tokenManager: {
        getAccessToken: () => 'closing-browser-access-token',
        getAuthToken: () => 'closing-browser-auth-token',
      },
      webSocketAuth: ImWebSocketAuthOptions.automatic(),
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const closingBrowserConnection = await closingBrowserClient.connect({
      deviceId: 'closing-browser-device-1',
      subscriptions: {
        conversations: ['closing-browser-conversation-1'],
      },
    });
    const closingBrowserSocket = FakeBrowserWebSocket.instances[0];
    const closingBrowserStates: string[] = [];
    closingBrowserConnection.lifecycle.onStateChange((state) => closingBrowserStates.push(state.status));
    closingBrowserConnection.disconnect(1000, 'conversation subscription closed');

    assert.equal(
      closingBrowserSocket.closeCalls.length,
      0,
      'browser websocket must not call native close while the socket is still CONNECTING',
    );
    assert.deepEqual(closingBrowserStates, ['connecting']);
    closingBrowserSocket.open();
    assert.deepEqual(closingBrowserSocket.closeCalls, [
      {
        code: 1000,
        reason: 'conversation subscription closed',
      },
    ]);
    assert.deepEqual(closingBrowserStates, ['connecting', 'closed']);
    assert.equal(
      closingBrowserSocket.sent.length,
      0,
      'browser websocket must not send auth.init or subscriptions after a pending disconnect',
    );

    FakeBrowserWebSocket.instances.length = 0;
    const browserClient = new ImSdkClient({
      accessToken: 'stale-browser-access-token',
      authToken: 'stale-browser-auth-token',
      tokenManager: {
        getAccessToken: () => 'browser-access-token-1',
        getAuthToken: () => 'browser-auth-token-1',
      },
      webSocketAuth: ImWebSocketAuthOptions.automatic({
        credentialProvider: () => 'stale-browser-auth-token',
      }),
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const browserConnection = await browserClient.connect({
      deviceId: 'browser-device-1',
      subscriptions: {
        conversations: ['browser-conversation-1'],
      },
    });
    assert.equal(FakeBrowserWebSocket.instances.length, 1, 'browser runtime must use global WebSocket');
    const browserSocket = FakeBrowserWebSocket.instances[0];
    assert.equal(
      browserSocket.url,
      'wss://chat.example.com/sdkwork/chat/im/v3/api/realtime/ws?deviceId=browser-device-1',
      'browser websocket URL must not include subscription identifiers',
    );
    assert.deepEqual(browserSocket.protocols, [IM_CCP_WS_SUBPROTOCOL]);

    const browserStates: string[] = [];
    browserConnection.lifecycle.onStateChange((state) => browserStates.push(state.status));

    assert.deepEqual(browserStates, ['connecting']);
    browserSocket.open();
    assert.deepEqual(browserStates, ['connecting'], 'browser websocket must wait for auth.ok before reporting open');
    assert.deepEqual(parseBrowserSent(browserSocket, 0), {
      type: 'auth.init',
      requestId: 'sdkwork-im-auth-init-1',
      authToken: 'browser-auth-token-1',
      accessToken: 'browser-access-token-1',
      deviceId: 'browser-device-1',
    });
    assert.equal(
      browserSocket.sent.some((frame) => frame.includes('subscriptions.sync')),
      false,
      'browser websocket must not send subscriptions before auth.ok',
    );
    browserConnection.subscriptions.syncConversations(['browser-conversation-early']);
    assert.equal(
      browserSocket.sent.some((frame) => frame.includes('subscriptions.sync')),
      false,
      'browser websocket must not send subscriptions before CCP handshake completes',
    );

    completeBrowserGatewayAuthAndCcp(browserSocket, {
      requestId: 'sdkwork-im-auth-init-1',
      tenantId: '100001',
      principalId: '30',
      sessionId: 'session_real',
      deviceId: 'browser-device-1',
    });
    assert.deepEqual(browserStates, ['connecting', 'open']);
    assert.equal(parseCcpControlType(browserSocket.sent[3] ?? ''), 'session_resume', 'browser websocket must send CCP session_resume when negotiated');
    assert.deepEqual(parseCcpBusinessPayload(browserSocket.sent[4]), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-1',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'browser-conversation-early',
          eventTypes: ['message.posted'],
        },
      ],
    });

    browserConnection.disconnect(1000, 'browser test complete');

    FakeBrowserWebSocket.instances.length = 0;
    const failedAuthClient = new ImSdkClient({
      accessToken: 'failed-browser-access-token',
      authToken: 'failed-browser-auth-token',
      tokenManager: {
        getAccessToken: () => 'failed-browser-access-token',
        getAuthToken: () => 'failed-browser-auth-token',
      },
      webSocketAuth: ImWebSocketAuthOptions.automatic(),
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const failedAuthConnection = await failedAuthClient.connect({
      deviceId: 'failed-browser-device-1',
      subscriptions: {
        conversations: ['failed-browser-conversation-1'],
      },
    });
    const failedBrowserSocket = FakeBrowserWebSocket.instances[0];
    const failedBrowserStates: string[] = [];
    const failedBrowserErrors: unknown[] = [];
    failedAuthConnection.lifecycle.onStateChange((state) => failedBrowserStates.push(state.status));
    failedAuthConnection.lifecycle.onError((error) => failedBrowserErrors.push(error));

    assert.deepEqual(failedBrowserStates, ['connecting']);
    failedBrowserSocket.open();
    assert.deepEqual(parseBrowserSent(failedBrowserSocket, 0), {
      type: 'auth.init',
      requestId: 'sdkwork-im-auth-init-1',
      authToken: 'failed-browser-auth-token',
      accessToken: 'failed-browser-access-token',
      deviceId: 'failed-browser-device-1',
    });
    failedBrowserSocket.emit('message', {
      data: JSON.stringify({
        type: 'error',
        requestId: 'sdkwork-im-auth-init-1',
        code: 'websocket_auth_failed',
        message: 'session expired',
      }),
    });

    assert.deepEqual(failedBrowserStates, ['connecting', 'error', 'closed']);
    assert.equal(failedBrowserErrors.length, 1, 'browser auth error frame must notify lifecycle error handlers');
    assert.deepEqual(failedBrowserErrors[0], {
      code: 'websocket_auth_failed',
      message: 'session expired',
      requestId: 'sdkwork-im-auth-init-1',
      type: 'error',
    });
    assert.equal(
      failedBrowserSocket.sent.some((frame) => frame.includes('subscriptions.sync')),
      false,
      'browser websocket must not subscribe after auth error',
    );

    FakeBrowserWebSocket.instances.length = 0;
    const timeoutAuthClient = new ImSdkClient({
      accessToken: 'timeout-browser-access-token',
      authToken: 'timeout-browser-auth-token',
      tokenManager: {
        getAccessToken: () => 'timeout-browser-access-token',
        getAuthToken: () => 'timeout-browser-auth-token',
      },
      webSocketAuth: ImWebSocketAuthOptions.automatic({
        timeoutMs: 1,
      }),
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const timeoutAuthConnection = await timeoutAuthClient.connect({
      deviceId: 'timeout-browser-device-1',
      subscriptions: {
        conversations: ['timeout-browser-conversation-1'],
      },
    });
    const timeoutBrowserSocket = FakeBrowserWebSocket.instances[0];
    const timeoutBrowserStates: string[] = [];
    const timeoutBrowserErrors: unknown[] = [];
    timeoutAuthConnection.lifecycle.onStateChange((state) => timeoutBrowserStates.push(state.status));
    timeoutAuthConnection.lifecycle.onError((error) => timeoutBrowserErrors.push(error));
    timeoutBrowserSocket.open();
    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.deepEqual(timeoutBrowserStates, ['connecting', 'error', 'closed']);
    assert.equal(timeoutBrowserErrors.length, 1, 'browser auth timeout must notify lifecycle error handlers');
    assert.deepEqual(timeoutBrowserErrors[0], {
      code: 'websocket_auth_timeout',
      message: 'websocket auth.ok was not received before timeout',
      requestId: 'sdkwork-im-auth-init-1',
      type: 'error',
    });
    assert.equal(timeoutBrowserSocket.readyState, 3);
    assert.equal(
      timeoutBrowserSocket.sent.some((frame) => frame.includes('subscriptions.sync')),
      false,
      'browser websocket must not subscribe after auth timeout',
    );
    timeoutAuthConnection.disconnect(1000, 'timeout test complete');
  } finally {
    if (originalWebSocket) {
      Object.defineProperty(globalThis, 'WebSocket', originalWebSocket);
    } else {
      Reflect.deleteProperty(globalThis, 'WebSocket');
    }
  }

  console.log('sdkwork-im-sdk websocket contract passed');
}

void main();
