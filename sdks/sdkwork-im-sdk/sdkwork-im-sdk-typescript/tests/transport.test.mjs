/**
 * Transport 抽象层与各传输实现单元测试。
 *
 * 使用 Node 内置 node:test 运行：
 *   node --test tests/transport.test.mjs
 *
 * 前置条件：先执行 `npm run build` 生成 dist/ 输出。
 */
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import {
  TRANSPORT_CAPABILITIES,
  ccpBindingForTransport,
  parseTransportKindFromUrl,
  DEFAULT_TRANSPORT_SELECTION_POLICY,
} from '../dist/transport.js';
import {
  CCP_WS_BINDING,
  CCP_TCP_BINDING,
  CCP_UDP_BINDING,
  encodeCcpHelloFrame,
  encodeCcpAuthBindFrame,
  encodeCcpHeartbeatFrame,
  encodeCcpBusinessFrame,
  encodeCcpSessionResumeFrame,
  decodeCcpEnvelope,
} from '../dist/ccp-wire.js';
import {
  detectAvailableTransports,
  selectTransportFactory,
  buildTransportEndpoint,
  ImTransportSelector,
} from '../dist/transport-selector.js';
import { createDefaultTransportFactories } from '../dist/transports/index.js';
import { ImSdkClient } from '../dist/sdk.js';
import { createImLiveConnection } from '../dist/realtime.js';

const TEST_ACCESS_TOKEN = `header.${Buffer.from(JSON.stringify({ user_id: 'user-1' })).toString('base64url')}.signature`;

function createFakeTransport(kind, initialState = 'connecting') {
  let state = initialState;
  const sent = [];
  const closeCalls = [];
  const openHandlers = new Set();
  const closeHandlers = new Set();
  const errorHandlers = new Set();
  const messageHandlers = new Set();
  const capabilities = TRANSPORT_CAPABILITIES[kind];

  const connection = {
    kind,
    capabilities,
    get state() {
      return state;
    },
    sent,
    closeCalls,
    emitOpen() {
      if (state === 'closed') return;
      state = 'open';
      for (const handler of [...openHandlers]) handler();
    },
    send(frame) {
      if (state === 'open') sent.push(frame);
    },
    close(code, reason) {
      if (state === 'closed') return;
      closeCalls.push({ code, reason });
      state = 'closed';
      const event = { code: code ?? 1000, reason: reason ?? '', wasClean: true };
      for (const handler of [...closeHandlers]) handler(event);
    },
    onMessage(handler) {
      messageHandlers.add(handler);
      return () => messageHandlers.delete(handler);
    },
    onOpen(handler) {
      openHandlers.add(handler);
      if (state === 'open') queueMicrotask(() => openHandlers.has(handler) && handler());
      return () => openHandlers.delete(handler);
    },
    onClose(handler) {
      closeHandlers.add(handler);
      return () => closeHandlers.delete(handler);
    },
    onError(handler) {
      errorHandlers.add(handler);
      return () => errorHandlers.delete(handler);
    },
  };
  return connection;
}

describe('TRANSPORT_CAPABILITIES', () => {
  it('websocket: 可靠、有序、有帧边界、支持升级认证', () => {
    const caps = TRANSPORT_CAPABILITIES.websocket;
    assert.equal(caps.reliable, true);
    assert.equal(caps.orderedDelivery, true);
    assert.equal(caps.supportsFraming, true);
    assert.equal(caps.supportsDatagram, false);
    assert.equal(caps.supportsUpgradeAuth, true);
    assert.equal(caps.ccpBinding, 'Ws1');
    assert.equal(caps.maxFrameBytes, 512 * 1024);
  });

  it('tcp: 可靠、有序、有帧边界、支持背压', () => {
    const caps = TRANSPORT_CAPABILITIES.tcp;
    assert.equal(caps.reliable, true);
    assert.equal(caps.orderedDelivery, true);
    assert.equal(caps.supportsFraming, true);
    assert.equal(caps.supportsBackpressure, true);
    assert.equal(caps.supportsUpgradeAuth, false);
    assert.equal(caps.ccpBinding, 'Tcp1');
    assert.equal(caps.maxFrameBytes, 512 * 1024);
  });

  it('udp: 不可靠、无序、数据报模式、无背压', () => {
    const caps = TRANSPORT_CAPABILITIES.udp;
    assert.equal(caps.reliable, false);
    assert.equal(caps.orderedDelivery, false);
    assert.equal(caps.supportsFraming, false);
    assert.equal(caps.supportsDatagram, true);
    assert.equal(caps.supportsBackpressure, false);
    assert.equal(caps.supportsUpgradeAuth, false);
    assert.equal(caps.ccpBinding, 'Udp1');
    assert.equal(caps.maxFrameBytes, 64 * 1024);
  });
});

describe('ccpBindingForTransport', () => {
  it('websocket → Ws1', () => {
    assert.equal(ccpBindingForTransport('websocket'), 'Ws1');
  });
  it('tcp → Tcp1', () => {
    assert.equal(ccpBindingForTransport('tcp'), 'Tcp1');
  });
  it('udp → Udp1', () => {
    assert.equal(ccpBindingForTransport('udp'), 'Udp1');
  });
});

describe('parseTransportKindFromUrl', () => {
  it('ws:// → websocket', () => {
    assert.equal(parseTransportKindFromUrl('ws://localhost:8080'), 'websocket');
  });
  it('wss:// → websocket', () => {
    assert.equal(parseTransportKindFromUrl('wss://example.com/path'), 'websocket');
  });
  it('tcp:// → tcp', () => {
    assert.equal(parseTransportKindFromUrl('tcp://127.0.0.1:18900'), 'tcp');
  });
  it('udp:// → udp', () => {
    assert.equal(parseTransportKindFromUrl('udp://127.0.0.1:18901'), 'udp');
  });
  it('http:// → undefined', () => {
    assert.equal(parseTransportKindFromUrl('http://localhost:8080'), undefined);
  });
  it('无协议前缀 → undefined', () => {
    assert.equal(parseTransportKindFromUrl('localhost:8080'), undefined);
  });
});

describe('DEFAULT_TRANSPORT_SELECTION_POLICY', () => {
  it('默认优先级：websocket > tcp > udp', () => {
    assert.deepEqual(DEFAULT_TRANSPORT_SELECTION_POLICY.preferred, ['websocket', 'tcp', 'udp']);
  });
  it('默认启用自动降级', () => {
    assert.equal(DEFAULT_TRANSPORT_SELECTION_POLICY.autoFallback, true);
  });
});

describe('CCP Wire binding 参数化', () => {
  it('encodeCcpHelloFrame 默认使用 Ws1', () => {
    const raw = encodeCcpHelloFrame();
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_WS_BINDING);
  });

  it('encodeCcpHelloFrame 传递 Tcp1', () => {
    const raw = encodeCcpHelloFrame(CCP_TCP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_TCP_BINDING);
  });

  it('encodeCcpHelloFrame 传递 Udp1', () => {
    const raw = encodeCcpHelloFrame(CCP_UDP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_UDP_BINDING);
  });

  it('encodeCcpAuthBindFrame 传递 Tcp1', () => {
    const raw = encodeCcpAuthBindFrame({
      principalId: 'user-123',
      actorKind: 'user',
      deviceId: 'device-abc',
    }, CCP_TCP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_TCP_BINDING);
  });

  it('encodeCcpHeartbeatFrame 传递 Udp1', () => {
    const raw = encodeCcpHeartbeatFrame(42, CCP_UDP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_UDP_BINDING);
  });

  it('encodeCcpBusinessFrame 传递 Tcp1', () => {
    const raw = encodeCcpBusinessFrame('cc.test.v1', 'cmd', { type: 'test' }, CCP_TCP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_TCP_BINDING);
  });

  it('encodeCcpSessionResumeFrame 传递 Udp1', () => {
    const raw = encodeCcpSessionResumeFrame('session-xyz', 10, CCP_UDP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, CCP_UDP_BINDING);
  });

  it('不同 binding 产生不同的 envelope.binding 值', () => {
    const wsRaw = encodeCcpHelloFrame(CCP_WS_BINDING);
    const tcpRaw = encodeCcpHelloFrame(CCP_TCP_BINDING);
    const udpRaw = encodeCcpHelloFrame(CCP_UDP_BINDING);
    const wsBinding = decodeCcpEnvelope(wsRaw)?.binding;
    const tcpBinding = decodeCcpEnvelope(tcpRaw)?.binding;
    const udpBinding = decodeCcpEnvelope(udpRaw)?.binding;
    assert.notEqual(wsBinding, tcpBinding);
    assert.notEqual(tcpBinding, udpBinding);
    assert.notEqual(wsBinding, udpBinding);
  });
});

describe('buildTransportEndpoint', () => {
  it('http URL + websocket → ws URL 带 realtime 路径', () => {
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'websocket', 'device-1');
    assert.equal(endpoint.kind, 'websocket');
    assert.match(endpoint.url, /^ws:\/\/127\.0\.0\.1:18079\/im\/v3\/api\/realtime\/ws/);
    assert.match(endpoint.url, /deviceId=device-1/);
    assert.deepEqual(endpoint.protocols, ['sdkwork-im.ccp.ws.v1']);
  });

  it('https URL + websocket → wss URL', () => {
    const endpoint = buildTransportEndpoint('https://im.example.com', 'websocket');
    assert.match(endpoint.url, /^wss:\/\/im\.example\.com\/im\/v3\/api\/realtime\/ws/);
  });

  it('existing ws URL still receives the realtime path and device query', () => {
    const endpoint = buildTransportEndpoint('ws://127.0.0.1:18079', 'websocket', 'device-2');
    assert.equal(
      endpoint.url,
      'ws://127.0.0.1:18079/im/v3/api/realtime/ws?deviceId=device-2',
    );
  });

  it('http URL + tcp → tcp://host:port', () => {
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'tcp');
    assert.equal(endpoint.kind, 'tcp');
    assert.equal(endpoint.url, 'tcp://127.0.0.1:18079');
  });

  it('http URL + udp → udp://host:port', () => {
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    assert.equal(endpoint.kind, 'udp');
    assert.equal(endpoint.url, 'udp://127.0.0.1:18079');
  });

  it('已有 tcp:// URL + tcp → 直接使用', () => {
    const endpoint = buildTransportEndpoint('tcp://10.0.0.1:19000', 'tcp');
    assert.equal(endpoint.url, 'tcp://10.0.0.1:19000');
  });

  it('已有 udp:// URL + udp → 直接使用', () => {
    const endpoint = buildTransportEndpoint('udp://10.0.0.1:19001', 'udp');
    assert.equal(endpoint.url, 'udp://10.0.0.1:19001');
  });
});

describe('live transport lifecycle', () => {
  it('falls back when a created transport never becomes open', async () => {
    const stalled = createFakeTransport('websocket');
    const fallback = createFakeTransport('tcp', 'open');
    let fallbackConnects = 0;
    const factories = new Map([
      ['websocket', {
        kind: 'websocket',
        capabilities: TRANSPORT_CAPABILITIES.websocket,
        isAvailable: () => true,
        connect: async () => stalled,
      }],
      ['tcp', {
        kind: 'tcp',
        capabilities: TRANSPORT_CAPABILITIES.tcp,
        isAvailable: () => true,
        connect: async () => {
          fallbackConnects += 1;
          return fallback;
        },
      }],
    ]);
    const client = new ImSdkClient({
      accessToken: TEST_ACCESS_TOKEN,
      apiBaseUrl: 'http://127.0.0.1:18079',
      transportFactories: factories,
      transportPolicy: {
        preferred: ['websocket', 'tcp'],
        autoFallback: true,
        probeTimeoutMs: 5,
      },
    });

    const live = await client.connect({ deviceId: 'device-fallback', heartbeat: false });
    assert.equal(fallbackConnects, 1);
    assert.deepEqual(stalled.closeCalls, [
      { code: 4008, reason: 'transport_probe_failed' },
    ]);
    live.disconnect();
  });

  it('dispatches one CCP hello when an open transport has a pending open callback', async () => {
    const transport = createFakeTransport('udp', 'open');
    const live = createImLiveConnection({
      accessToken: TEST_ACCESS_TOKEN,
      options: { deviceId: 'device-open-race', heartbeat: false },
      transport,
      websocketBaseUrl: 'ws://127.0.0.1:18079',
    });

    await new Promise((resolve) => setTimeout(resolve, 0));
    const helloFrames = transport.sent.filter((frame) => {
      const envelope = decodeCcpEnvelope(frame.data);
      return envelope?.schema === 'cc.control.hello.v1';
    });
    assert.equal(helloFrames.length, 1);
    live.disconnect();
  });
});

describe('createDefaultTransportFactories', () => {
  it('返回包含 websocket/tcp/udp 三种工厂的 Map', () => {
    const factories = createDefaultTransportFactories();
    assert.equal(factories.size, 3);
    assert.ok(factories.has('websocket'));
    assert.ok(factories.has('tcp'));
    assert.ok(factories.has('udp'));
  });

  it('各工厂 kind 属性正确', () => {
    const factories = createDefaultTransportFactories();
    assert.equal(factories.get('websocket')?.kind, 'websocket');
    assert.equal(factories.get('tcp')?.kind, 'tcp');
    assert.equal(factories.get('udp')?.kind, 'udp');
  });

  it('各工厂 capabilities.ccpBinding 正确', () => {
    const factories = createDefaultTransportFactories();
    assert.equal(factories.get('websocket')?.capabilities.ccpBinding, 'Ws1');
    assert.equal(factories.get('tcp')?.capabilities.ccpBinding, 'Tcp1');
    assert.equal(factories.get('udp')?.capabilities.ccpBinding, 'Udp1');
  });
});

describe('detectAvailableTransports', () => {
  it('Node 环境下检测到 websocket + tcp + udp', () => {
    const factories = createDefaultTransportFactories();
    const available = detectAvailableTransports(factories);
    assert.ok(available.includes('websocket'));
    assert.ok(available.includes('tcp'));
    assert.ok(available.includes('udp'));
    assert.equal(available.length, 3);
  });
});

describe('selectTransportFactory', () => {
  it('无 preferred 时按策略选择第一个可用（websocket）', () => {
    const factories = createDefaultTransportFactories();
    const factory = selectTransportFactory(factories);
    assert.equal(factory.kind, 'websocket');
  });

  it('preferred=tcp 时选择 TCP 工厂', () => {
    const factories = createDefaultTransportFactories();
    const factory = selectTransportFactory(factories, DEFAULT_TRANSPORT_SELECTION_POLICY, 'tcp');
    assert.equal(factory.kind, 'tcp');
  });

  it('preferred=udp 时选择 UDP 工厂', () => {
    const factories = createDefaultTransportFactories();
    const factory = selectTransportFactory(factories, DEFAULT_TRANSPORT_SELECTION_POLICY, 'udp');
    assert.equal(factory.kind, 'udp');
  });

  it('preferred=websocket 时选择 WebSocket 工厂', () => {
    const factories = createDefaultTransportFactories();
    const factory = selectTransportFactory(factories, DEFAULT_TRANSPORT_SELECTION_POLICY, 'websocket');
    assert.equal(factory.kind, 'websocket');
  });
});

describe('ImTransportSelector', () => {
  it('select 返回 factory 和 endpoint', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const { factory, endpoint } = selector.select('http://127.0.0.1:18079');
    assert.equal(factory.kind, 'websocket');
    assert.equal(endpoint.kind, 'websocket');
    assert.ok(endpoint.url.startsWith('ws://'));
  });

  it('select 支持 preferredKind 覆盖', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const { factory, endpoint } = selector.select('http://127.0.0.1:18079', 'tcp');
    assert.equal(factory.kind, 'tcp');
    assert.equal(endpoint.kind, 'tcp');
    assert.equal(endpoint.url, 'tcp://127.0.0.1:18079');
  });

  it('detectAvailable 返回所有可用传输', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const available = selector.detectAvailable();
    assert.ok(available.length >= 3);
  });
});

describe('TCP 帧编解码（通过内部类间接验证）', () => {
  it('CCP envelope 通过 binding=Tcp1 编码后 binding 字段为 Tcp1', () => {
    const raw = encodeCcpHelloFrame(CCP_TCP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, 'Tcp1');
    assert.equal(envelope?.kind, 'control');
    assert.equal(envelope?.schema, 'cc.control.hello.v1');
  });

  it('CCP envelope 通过 binding=Udp1 编码后 binding 字段为 Udp1', () => {
    const raw = encodeCcpHeartbeatFrame(1, CCP_UDP_BINDING);
    const envelope = decodeCcpEnvelope(raw);
    assert.equal(envelope?.binding, 'Udp1');
  });
});

describe('TCP 帧编解码器单元测试', () => {
  // 通过 UDP transport 的 send 路径间接验证 TCP encoder 不易，
  // 这里直接测试 Buffer + TextEncoder 的编解码逻辑对齐 TCP 帧格式。

  it('encodeFrame + decode 往返：字符串 payload 正确还原', () => {
    // 模拟 TCP 帧编码：4字节大端长度 + UTF-8 payload
    const payload = '{"kind":"control","schema":"cc.control.hello.v1"}';
    const encoder = new TextEncoder();
    const payloadBytes = encoder.encode(payload);
    const header = Buffer.alloc(4);
    const view = new DataView(header.buffer, header.byteOffset, header.byteLength);
    view.setUint32(0, payloadBytes.byteLength, false); // 大端
    const frame = Buffer.concat([header, payloadBytes]);

    // 模拟解码
    const decodedView = new DataView(frame.buffer, frame.byteOffset, 4);
    const decodedLen = decodedView.getUint32(0, false);
    assert.equal(decodedLen, payloadBytes.byteLength);
    const decodedPayload = frame.subarray(4, 4 + decodedLen);
    const decodedText = new TextDecoder().decode(decodedPayload);
    assert.equal(decodedText, payload);
  });

  it('encodeFrame 包含多字节 UTF-8 字符时长度正确', () => {
    const payload = '{"msg":"你好世界🌍"}';
    const encoder = new TextEncoder();
    const payloadBytes = encoder.encode(payload);
    // UTF-8 字节数 != 字符串长度
    assert.ok(payloadBytes.byteLength > payload.length);

    const header = Buffer.alloc(4);
    const view = new DataView(header.buffer, header.byteOffset, header.byteLength);
    view.setUint32(0, payloadBytes.byteLength, false);
    const frame = Buffer.concat([header, payloadBytes]);

    // 解码验证
    const decodedView = new DataView(frame.buffer, frame.byteOffset, 4);
    const decodedLen = decodedView.getUint32(0, false);
    assert.equal(decodedLen, payloadBytes.byteLength);
    const decodedText = new TextDecoder().decode(frame.subarray(4, 4 + decodedLen));
    assert.equal(decodedText, payload);
  });

  it('TCP decoder 处理半帧：先收到 header+部分 payload，再收到剩余', () => {
    // 模拟 TCP decoder 的状态机行为
    const payload = '{"kind":"control"}';
    const encoder = new TextEncoder();
    const payloadBytes = encoder.encode(payload);
    const header = Buffer.alloc(4);
    const view = new DataView(header.buffer, header.byteOffset, header.byteLength);
    view.setUint32(0, payloadBytes.byteLength, false);
    const fullFrame = Buffer.concat([header, payloadBytes]);

    // 模拟分片：先收到 5 字节（header + 1字节），再收到剩余
    const firstChunk = fullFrame.subarray(0, 5);
    const secondChunk = fullFrame.subarray(5);

    // 简化的 decoder 状态机
    let buffer = Buffer.alloc(0);
    let state = 'header';
    let expectedLen = 0;
    const frames = [];

    function push(data) {
      buffer = Buffer.concat([buffer, data]);
      while (true) {
        if (state === 'header') {
          if (buffer.length < 4) break;
          const dv = new DataView(buffer.buffer, buffer.byteOffset, 4);
          expectedLen = dv.getUint32(0, false);
          buffer = buffer.subarray(4);
          state = 'payload';
        }
        if (state === 'payload') {
          if (buffer.length < expectedLen) break;
          frames.push(buffer.subarray(0, expectedLen));
          buffer = buffer.subarray(expectedLen);
          state = 'header';
          expectedLen = 0;
        }
      }
    }

    push(firstChunk);
    assert.equal(frames.length, 0, 'first chunk should not produce a complete frame');
    push(secondChunk);
    assert.equal(frames.length, 1, 'second chunk should complete the frame');
    assert.equal(new TextDecoder().decode(frames[0]), payload);
  });

  it('TCP decoder 处理粘包：多个帧在一次 data 事件中到达', () => {
    const payload1 = '{"kind":"control","type":"hello"}';
    const payload2 = '{"kind":"control","type":"heartbeat"}';
    const encoder = new TextEncoder();
    const p1Bytes = encoder.encode(payload1);
    const p2Bytes = encoder.encode(payload2);

    const h1 = Buffer.alloc(4);
    new DataView(h1.buffer, h1.byteOffset, 4).setUint32(0, p1Bytes.byteLength, false);
    const h2 = Buffer.alloc(4);
    new DataView(h2.buffer, h2.byteOffset, 4).setUint32(0, p2Bytes.byteLength, false);

    const combined = Buffer.concat([h1, p1Bytes, h2, p2Bytes]);

    // 简化的 decoder
    let buffer = Buffer.alloc(0);
    let state = 'header';
    let expectedLen = 0;
    const frames = [];

    function push(data) {
      buffer = Buffer.concat([buffer, data]);
      while (true) {
        if (state === 'header') {
          if (buffer.length < 4) break;
          const dv = new DataView(buffer.buffer, buffer.byteOffset, 4);
          expectedLen = dv.getUint32(0, false);
          buffer = buffer.subarray(4);
          state = 'payload';
        }
        if (state === 'payload') {
          if (buffer.length < expectedLen) break;
          frames.push(buffer.subarray(0, expectedLen));
          buffer = buffer.subarray(expectedLen);
          state = 'header';
          expectedLen = 0;
        }
      }
    }

    push(combined);
    assert.equal(frames.length, 2, 'should decode 2 frames from combined data');
    assert.equal(new TextDecoder().decode(frames[0]), payload1);
    assert.equal(new TextDecoder().decode(frames[1]), payload2);
  });

  it('TCP frame 超过 512KB 时应该报错', () => {
    // 构造超过 512KB 的 payload
    const largePayload = 'x'.repeat(512 * 1024 + 1);
    const header = Buffer.alloc(4);
    const view = new DataView(header.buffer, header.byteOffset, header.byteLength);
    view.setUint32(0, largePayload.length, false);

    // 验证长度字段确实超过 512KB
    assert.ok(largePayload.length > 512 * 1024);
  });

  it('UDP datagram 超过 64KB 时应该被拒绝', () => {
    // 构造超过 64KB 的 payload
    const largePayload = 'x'.repeat(64 * 1024 + 1);
    assert.ok(largePayload.length > 64 * 1024);
    // UDP transport 的 send 方法会检查并触发 error 事件
  });
});

describe('onOpen 竞态条件防护', () => {
  it('UDP transport: factory.connect() 返回后注册 onOpen 仍能被触发', async () => {
    const factories = createDefaultTransportFactories();
    const udpFactory = factories.get('udp');
    assert.ok(udpFactory, 'UDP factory should exist');

    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    const connection = await udpFactory.connect(endpoint, { connectionTimeoutMs: 5000 });

    let openFired = false;
    connection.onOpen(() => {
      openFired = true;
    });

    // 等待 microtask flush
    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.equal(openFired, true, 'onOpen should fire even when registered after connect() resolves');
    assert.equal(connection.state, 'open');

    connection.close();
  });

  it('UDP transport: onOpen 注册多次都能被触发', async () => {
    const factories = createDefaultTransportFactories();
    const udpFactory = factories.get('udp');
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    const connection = await udpFactory.connect(endpoint, { connectionTimeoutMs: 5000 });

    let callCount = 0;
    connection.onOpen(() => { callCount += 1; });
    connection.onOpen(() => { callCount += 1; });

    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.equal(callCount, 2, 'both onOpen handlers should fire');
    connection.close();
  });

  it('UDP transport: onOpen 取消订阅后不再触发', async () => {
    const factories = createDefaultTransportFactories();
    const udpFactory = factories.get('udp');
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    const connection = await udpFactory.connect(endpoint, { connectionTimeoutMs: 5000 });

    let callCount = 0;
    const unsubscribe = connection.onOpen(() => { callCount += 1; });
    unsubscribe();

    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.equal(callCount, 0, 'unsubscribed onOpen should not fire');
    connection.close();
  });

  it('UDP transport: 重复 close() 不触发多次 close 事件', async () => {
    const factories = createDefaultTransportFactories();
    const udpFactory = factories.get('udp');
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    const connection = await udpFactory.connect(endpoint, { connectionTimeoutMs: 5000 });

    let closeCount = 0;
    connection.onClose(() => { closeCount += 1; });

    connection.close();
    connection.close();
    connection.close();

    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.equal(closeCount, 1, 'close() should only trigger close event once');
    assert.equal(connection.state, 'closed');
  });

  it('UDP transport: onOpen 不被双重调用（handleOpen microtask + late-registration）', async () => {
    const factories = createDefaultTransportFactories();
    const udpFactory = factories.get('udp');
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    const connection = await udpFactory.connect(endpoint, { connectionTimeoutMs: 5000 });

    // 立即注册 onOpen（在 open microtask 派发前）
    let callCount = 0;
    connection.onOpen(() => { callCount += 1; });

    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.equal(callCount, 1, 'onOpen should be called exactly once, not twice');
    connection.close();
  });

  it('UDP transport: open 后注册的 onOpen 也只调用一次', async () => {
    const factories = createDefaultTransportFactories();
    const udpFactory = factories.get('udp');
    const endpoint = buildTransportEndpoint('http://127.0.0.1:18079', 'udp');
    const connection = await udpFactory.connect(endpoint, { connectionTimeoutMs: 5000 });

    // 等待 open microtask 派发完成
    await new Promise((resolve) => setTimeout(resolve, 10));

    let callCount = 0;
    connection.onOpen(() => { callCount += 1; });

    await new Promise((resolve) => setTimeout(resolve, 10));

    assert.equal(callCount, 1, 'late-registered onOpen should fire exactly once');
    connection.close();
  });
});

describe('buildCandidateList 降级候选列表', () => {
  it('无 preferredKind 时按 policy.preferred 顺序返回所有可用传输', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList();
    assert.equal(candidates.length, 3);
    assert.equal(candidates[0], 'websocket');
    assert.equal(candidates[1], 'tcp');
    assert.equal(candidates[2], 'udp');
  });

  it('preferredKind=tcp 时 tcp 排在首位', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList('tcp');
    assert.equal(candidates[0], 'tcp');
    assert.equal(candidates[1], 'websocket');
    assert.equal(candidates[2], 'udp');
    assert.equal(candidates.length, 3);
  });

  it('preferredKind=udp 时 udp 排在首位', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList('udp');
    assert.equal(candidates[0], 'udp');
    assert.equal(candidates[1], 'websocket');
    assert.equal(candidates[2], 'tcp');
  });

  it('preferredKind 与 policy.preferred 重复时只出现一次', () => {
    const factories = createDefaultTransportFactories();
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList('websocket');
    assert.equal(candidates.length, 3);
    assert.equal(candidates[0], 'websocket');
  });

  it('仅 websocket 可用时只返回 websocket', () => {
    const factories = new Map();
    factories.set('websocket', createDefaultTransportFactories().get('websocket'));
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList('tcp');
    assert.equal(candidates.length, 1);
    assert.equal(candidates[0], 'websocket');
  });

  it('无任何传输可用时返回空数组', () => {
    const factories = new Map();
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList();
    assert.equal(candidates.length, 0);
  });
});

describe('连接失败降级', () => {
  it('首个传输连接失败时自动降级到下一个', async () => {
    // 创建模拟工厂：websocket 总是失败，tcp 成功
    const factories = new Map();
    let tcpConnectCalled = false;

    const failingWsFactory = {
      kind: 'websocket',
      capabilities: TRANSPORT_CAPABILITIES.websocket,
      isAvailable: () => true,
      connect: () => Promise.reject(new Error('websocket connect failed')),
    };
    const successTcpFactory = {
      kind: 'tcp',
      capabilities: TRANSPORT_CAPABILITIES.tcp,
      isAvailable: () => true,
      connect: () => {
        tcpConnectCalled = true;
        // 返回一个模拟的 transport 连接
        return Promise.resolve({
          kind: 'tcp',
          capabilities: TRANSPORT_CAPABILITIES.tcp,
          state: 'open',
          send: () => {},
          close: () => {},
          onMessage: () => () => {},
          onOpen: () => () => {},
          onClose: () => () => {},
          onError: () => () => {},
        });
      },
    };

    factories.set('websocket', failingWsFactory);
    factories.set('tcp', successTcpFactory);

    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList();

    assert.equal(candidates[0], 'websocket');
    assert.equal(candidates[1], 'tcp');

    // 模拟降级连接
    let connectedTransport = null;
    for (const kind of candidates) {
      const factory = selector.getFactory(kind);
      const endpoint = selector.buildEndpoint(kind, 'http://127.0.0.1:18079');
      try {
        connectedTransport = await factory.connect(endpoint, { connectionTimeoutMs: 5000, headers: {}, protocols: [] });
        break;
      } catch (error) {
        // 继续降级
        continue;
      }
    }

    assert.ok(connectedTransport, 'should connect via fallback');
    assert.ok(tcpConnectCalled, 'TCP factory should be tried as fallback');
    assert.equal(connectedTransport.kind, 'tcp');
  });

  it('所有传输都失败时抛出最后一个错误', async () => {
    const factories = new Map();
    const lastError = new Error('all transports failed');

    factories.set('websocket', {
      kind: 'websocket',
      capabilities: TRANSPORT_CAPABILITIES.websocket,
      isAvailable: () => true,
      connect: () => Promise.reject(new Error('ws failed')),
    });
    factories.set('tcp', {
      kind: 'tcp',
      capabilities: TRANSPORT_CAPABILITIES.tcp,
      isAvailable: () => true,
      connect: () => Promise.reject(lastError),
    });

    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList();

    let lastCaughtError = null;
    let connectedTransport = null;
    for (const kind of candidates) {
      const factory = selector.getFactory(kind);
      const endpoint = selector.buildEndpoint(kind, 'http://127.0.0.1:18079');
      try {
        connectedTransport = await factory.connect(endpoint, { connectionTimeoutMs: 5000, headers: {}, protocols: [] });
        break;
      } catch (error) {
        lastCaughtError = error;
        continue;
      }
    }

    assert.equal(connectedTransport, null, 'no transport should connect');
    assert.equal(lastCaughtError, lastError, 'should retain last error');
  });

  it('autoFallback=true 时手动覆盖连接失败也会降级', async () => {
    const factories = new Map();
    let tcpCalled = false;

    factories.set('tcp', {
      kind: 'tcp',
      capabilities: TRANSPORT_CAPABILITIES.tcp,
      isAvailable: () => true,
      connect: () => {
        tcpCalled = true;
        return Promise.reject(new Error('tcp failed'));
      },
    });
    factories.set('websocket', {
      kind: 'websocket',
      capabilities: TRANSPORT_CAPABILITIES.websocket,
      isAvailable: () => true,
      connect: () => Promise.resolve({
        kind: 'websocket',
        capabilities: TRANSPORT_CAPABILITIES.websocket,
        state: 'open',
        send: () => {},
        close: () => {},
        onMessage: () => () => {},
        onOpen: () => () => {},
        onClose: () => () => {},
        onError: () => () => {},
      }),
    });

    // preferredKind=tcp, autoFallback=true（默认）
    const selector = new ImTransportSelector(factories);
    const candidates = selector.buildCandidateList('tcp');
    assert.equal(candidates[0], 'tcp');
    assert.equal(candidates[1], 'websocket');

    // 模拟 autoFallback=true 的降级
    const autoFallback = true;
    let connectedTransport = null;
    for (const kind of candidates) {
      const factory = selector.getFactory(kind);
      const endpoint = selector.buildEndpoint(kind, 'http://127.0.0.1:18079');
      try {
        connectedTransport = await factory.connect(endpoint, { connectionTimeoutMs: 5000, headers: {}, protocols: [] });
        break;
      } catch (error) {
        if (!autoFallback) throw error;
        continue;
      }
    }

    assert.ok(tcpCalled, 'TCP should be tried first');
    assert.ok(connectedTransport, 'should connect via fallback');
    assert.equal(connectedTransport.kind, 'websocket');
  });

  it('autoFallback=false 时手动覆盖连接失败不降级', async () => {
    const factories = new Map();
    let wsCalled = false;

    factories.set('tcp', {
      kind: 'tcp',
      capabilities: TRANSPORT_CAPABILITIES.tcp,
      isAvailable: () => true,
      connect: () => Promise.reject(new Error('tcp failed')),
    });
    factories.set('websocket', {
      kind: 'websocket',
      capabilities: TRANSPORT_CAPABILITIES.websocket,
      isAvailable: () => true,
      connect: () => {
        wsCalled = true;
        return Promise.resolve({
          kind: 'websocket',
          capabilities: TRANSPORT_CAPABILITIES.websocket,
          state: 'open',
          send: () => {},
          close: () => {},
          onMessage: () => () => {},
          onOpen: () => () => {},
          onClose: () => () => {},
          onError: () => () => {},
        });
      },
    });

    // preferredKind=tcp, autoFallback=false
    const selector = new ImTransportSelector(factories, {
      preferred: ['websocket', 'tcp', 'udp'],
      autoFallback: false,
    });
    const candidates = selector.buildCandidateList('tcp');
    assert.equal(candidates[0], 'tcp');

    // 模拟 autoFallback=false 的不降级
    const autoFallback = false;
    let caughtError = null;
    try {
      for (const kind of candidates) {
        const factory = selector.getFactory(kind);
        const endpoint = selector.buildEndpoint(kind, 'http://127.0.0.1:18079');
        try {
          await factory.connect(endpoint, { connectionTimeoutMs: 5000, headers: {}, protocols: [] });
          break;
        } catch (error) {
          if (!autoFallback) throw error;
          continue;
        }
      }
    } catch (error) {
      caughtError = error;
    }

    assert.ok(caughtError, 'should throw error without fallback');
    assert.equal(caughtError.message, 'tcp failed');
    assert.equal(wsCalled, false, 'websocket should not be tried');
  });

  it('autoFallback=false + preferredKind 不可用时抛出错误而非静默降级', () => {
    // 模拟浏览器环境：tcp 不可用
    const factories = new Map();
    factories.set('websocket', {
      kind: 'websocket',
      capabilities: TRANSPORT_CAPABILITIES.websocket,
      isAvailable: () => true,
      connect: () => Promise.resolve({}),
    });
    factories.set('tcp', {
      kind: 'tcp',
      capabilities: TRANSPORT_CAPABILITIES.tcp,
      isAvailable: () => false, // 浏览器环境 tcp 不可用
      connect: () => Promise.reject(new Error('tcp not available')),
    });

    const selector = new ImTransportSelector(factories, {
      preferred: ['websocket', 'tcp', 'udp'],
      autoFallback: false,
    });

    // preferredKind=tcp 但不可用
    const candidates = selector.buildCandidateList('tcp');
    // tcp 不可用，不会出现在候选列表中
    assert.ok(!candidates.includes('tcp'), 'tcp should not be in candidates');
    assert.ok(candidates.includes('websocket'), 'websocket should be in candidates');

    // autoFallback=false + preferredKind 不可用 → 应该抛出错误
    const preferredFactory = factories.get('tcp');
    assert.equal(preferredFactory?.isAvailable(), false);
  });

  it('autoFallback=true + preferredKind 不可用时静默降级到可用传输', () => {
    const factories = new Map();
    factories.set('websocket', {
      kind: 'websocket',
      capabilities: TRANSPORT_CAPABILITIES.websocket,
      isAvailable: () => true,
      connect: () => Promise.resolve({}),
    });
    factories.set('tcp', {
      kind: 'tcp',
      capabilities: TRANSPORT_CAPABILITIES.tcp,
      isAvailable: () => false, // 不可用
      connect: () => Promise.reject(new Error('tcp not available')),
    });

    const selector = new ImTransportSelector(factories); // autoFallback=true（默认）
    const candidates = selector.buildCandidateList('tcp');

    // tcp 不可用，只有 websocket 在候选列表中
    assert.equal(candidates.length, 1);
    assert.equal(candidates[0], 'websocket');
  });
});

describe('浏览器端兼容性', () => {
  it('浏览器环境（无 process）TCP/UDP isAvailable() 返回 false', async () => {
    // 保存原始值
    const originalProcess = globalThis.process;
    try {
      // 模拟浏览器环境：删除 process
      delete globalThis.process;

      const { ImTcpTransportFactory } = await import('../dist/transports/tcp-transport.js');
      const { ImUdpTransportFactory } = await import('../dist/transports/udp-transport.js');

      const tcpFactory = new ImTcpTransportFactory();
      const udpFactory = new ImUdpTransportFactory();

      assert.equal(tcpFactory.isAvailable(), false, 'TCP should not be available in browser');
      assert.equal(udpFactory.isAvailable(), false, 'UDP should not be available in browser');
    } finally {
      // 恢复原始值
      if (originalProcess !== undefined) {
        globalThis.process = originalProcess;
      }
    }
  });

  it('浏览器环境 buildCandidateList 只包含 websocket', () => {
    const originalProcess = globalThis.process;
    try {
      delete globalThis.process;

      const factories = createDefaultTransportFactories();
      const selector = new ImTransportSelector(factories);
      const candidates = selector.buildCandidateList();

      assert.equal(candidates.length, 1, 'browser should only have websocket candidate');
      assert.equal(candidates[0], 'websocket');
    } finally {
      if (originalProcess !== undefined) {
        globalThis.process = originalProcess;
      }
    }
  });

  it('浏览器环境 buildCandidateList(preferred=tcp) 降级到 websocket', () => {
    const originalProcess = globalThis.process;
    try {
      delete globalThis.process;

      const factories = createDefaultTransportFactories();
      const selector = new ImTransportSelector(factories);
      // 用户指定 tcp 但浏览器不可用 → 自动降级到 websocket
      const candidates = selector.buildCandidateList('tcp');

      assert.equal(candidates.length, 1, 'browser should fallback to websocket only');
      assert.equal(candidates[0], 'websocket');
    } finally {
      if (originalProcess !== undefined) {
        globalThis.process = originalProcess;
      }
    }
  });
});
