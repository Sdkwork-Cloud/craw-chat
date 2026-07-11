/**
 * UDP 传输实现（Node 环境）。
 *
 * 使用 Node.js dgram 模块建立无连接的 datagram 通信。
 * 每个 datagram 携带一个完整的 CCP envelope（JSON 字符串字节），
 * 对齐服务端 sdkwork-im-ccp-binding-udp（CCP_UDP_MAX_DATAGRAM_BYTES=64KB）。
 *
 * UDP 不可靠、无序，主要适合：
 * - CCP 控制帧握手（hello/auth_bind/auth_ok）
 * - 心跳保活
 * - 低延迟的轻量级命令
 *
 * 业务消息推送建议走 WebSocket/TCP。服务端 serve_udp_datagram 按每个 datagram
 * 独立处理，不维护长会话状态。
 *
 * 仅在 Node.js / Tauri 后端环境可用，浏览器不支持。
 */

import type {
  ImTransportCapabilities,
  ImTransportConnection,
  ImTransportConnectionState,
  ImTransportConnectOptions,
  ImTransportEndpoint,
  ImTransportFactory,
  ImTransportFrame,
  ImTransportCloseEvent,
  ImTransportErrorEvent,
} from '../transport.js';
import { TRANSPORT_CAPABILITIES } from '../transport.js';

/** 单个 datagram 最大 64KB，对齐服务端 CCP_UDP_MAX_DATAGRAM_BYTES。 */
const UDP_MAX_DATAGRAM_BYTES = 64 * 1024;

/** Node dgram 模块类型（最小化声明）。 */
interface NodeDgramSocket {
  send(
    msg: Uint8Array | string,
    port: number,
    address: string,
    callback?: (error: Error | null) => void,
  ): void;
  close(): void;
  bind(port?: number, address?: string): void;
  on(event: 'message', listener: (msg: Uint8Array, rinfo: { address: string; port: number }) => void): this;
  on(event: 'listening', listener: () => void): this;
  on(event: 'close', listener: () => void): this;
  on(event: 'error', listener: (error: Error) => void): this;
  off(event: 'listening', listener: () => void): this;
  off(event: 'error', listener: (error: Error) => void): this;
  ref(): void;
  unref(): void;
}

interface NodeDgramModule {
  createSocket(type: 'udp4' | 'udp6'): NodeDgramSocket;
}

type RuntimeBuffer = {
  alloc(size: number): Uint8Array;
  byteLength(value: string, encoding?: string): number;
};

interface BufferLike {
  readonly buffer: ArrayBuffer;
  readonly byteOffset: number;
  readonly byteLength: number;
}

function getRuntimeBuffer(): RuntimeBuffer | undefined {
  return (globalThis as { Buffer?: RuntimeBuffer }).Buffer;
}

function isBufferLike(value: unknown): value is BufferLike {
  return value !== null && typeof value === 'object' && 'buffer' in value && 'byteOffset' in value && 'byteLength' in value;
}

/** 从 udp://host:port URL 解析主机和端口。 */
function parseUdpEndpoint(url: string): { host: string; port: number } {
  const parsed = new URL(url);
  const host = parsed.hostname || '127.0.0.1';
  const port = Number.parseInt(parsed.port, 10);
  if (!Number.isFinite(port) || port <= 0 || port > 65535) {
    throw new Error(`Invalid UDP port in URL: ${url}`);
  }
  return { host, port };
}

/**
 * UDP 传输连接。
 *
 * UDP 是无连接协议，但为了适配 ImTransportConnection 接口，
 * 我们在 connect() 时创建 dgram socket 并视为"已连接"。
 * send() 时每个帧作为一个 datagram 发送到服务端。
 * onMessage() 监听来自服务端的 datagram。
 */
export class ImUdpTransportConnection implements ImTransportConnection {
  readonly kind = 'udp' as const;
  readonly capabilities: ImTransportCapabilities = TRANSPORT_CAPABILITIES.udp;

  private readonly socket: NodeDgramSocket;
  private readonly serverHost: string;
  private readonly serverPort: number;
  private readonly messageHandlers = new Set<(frame: ImTransportFrame) => void>();
  private readonly openHandlers = new Set<() => void>();
  private readonly closeHandlers = new Set<(event: ImTransportCloseEvent) => void>();
  private readonly errorHandlers = new Set<(event: ImTransportErrorEvent) => void>();
  private stateValue: ImTransportConnectionState = 'connecting';
  private openDispatched = false;
  private closeEvent: ImTransportCloseEvent | undefined;
  private requestedCloseEvent: ImTransportCloseEvent | undefined;

  constructor(socket: NodeDgramSocket, host: string, port: number) {
    this.socket = socket;
    this.serverHost = host;
    this.serverPort = port;

    socket.on('message', (msg: Uint8Array) => {
      if (this.stateValue !== 'open') {
        return;
      }
      const bytes = isBufferLike(msg) ? new Uint8Array(msg.buffer, msg.byteOffset, msg.byteLength) : msg;
      const text = new TextDecoder().decode(bytes);
      for (const handler of [...this.messageHandlers]) {
        handler({ data: text, isBinary: false });
      }
    });

    socket.on('close', () => {
      this.finalizeClose(this.requestedCloseEvent
        ?? { code: 1000, reason: 'udp_socket_closed', wasClean: true });
    });

    socket.on('error', (error: Error) => {
      const errorEvent: ImTransportErrorEvent = { error, code: 'udp_socket_error' };
      for (const handler of this.errorHandlers) {
        handler(errorEvent);
      }
      // 确保 error 后触发 close：dgram socket error 后可能不会自动关闭
      if (this.stateValue !== 'closed') {
        try {
          this.socket.close();
        } catch {
          // 忽略重复关闭
        }
      }
    });

    // UDP 无连接，socket 创建后即可发送，视为立即 open。
    // 延迟一个 microtask 触发 open，确保调用方有机会在 open 前注册 onOpen 回调。
    this.open();
  }

  private finalizeClose(event: ImTransportCloseEvent): void {
    if (this.stateValue === 'closed') {
      return;
    }
    this.stateValue = 'closed';
    this.closeEvent = event;
    for (const handler of [...this.closeHandlers]) {
      handler(event);
    }
    this.messageHandlers.clear();
    this.openHandlers.clear();
    this.errorHandlers.clear();
  }

  private open(): void {
    if (this.stateValue === 'open') {
      return;
    }
    this.stateValue = 'open';
    // 异步触发 open 回调，避免调用方在 factory.connect() 返回前错过 open 事件
    queueMicrotask(() => {
      if (this.stateValue !== 'open') {
        return;
      }
      this.openDispatched = true;
      for (const handler of this.openHandlers) {
        handler();
      }
    });
  }

  get state(): ImTransportConnectionState {
    return this.stateValue;
  }

  onOpen(handler: () => void): () => void {
    this.openHandlers.add(handler);
    // 仅在 open 事件已经派发后才调度回调；否则 open() 的 microtask 会负责调用
    if (this.stateValue === 'open' && this.openDispatched) {
      queueMicrotask(() => {
        if (this.openHandlers.has(handler) && this.stateValue === 'open') {
          handler();
        }
      });
    }
    return () => this.openHandlers.delete(handler);
  }

  send(frame: ImTransportFrame): void {
    if (this.stateValue !== 'open') {
      return;
    }

    const bytes = typeof frame.data === 'string'
      ? new TextEncoder().encode(frame.data)
      : frame.data;

    if (bytes.byteLength > UDP_MAX_DATAGRAM_BYTES) {
      const errorEvent: ImTransportErrorEvent = {
        error: new Error(
          `UDP datagram exceeds max ${UDP_MAX_DATAGRAM_BYTES} bytes: got ${bytes.byteLength}`,
        ),
        code: 'udp_datagram_too_large',
      };
      for (const handler of this.errorHandlers) {
        handler(errorEvent);
      }
      return;
    }

    this.socket.send(bytes, this.serverPort, this.serverHost, (error) => {
      if (error) {
        const errorEvent: ImTransportErrorEvent = { error, code: 'udp_send_error' };
        for (const handler of this.errorHandlers) {
          handler(errorEvent);
        }
      }
    });
  }

  close(code?: number, reason?: string): void {
    if (this.stateValue === 'closing' || this.stateValue === 'closed') {
      return;
    }
    this.stateValue = 'closing';
    this.requestedCloseEvent = {
      code: code ?? 1000,
      reason: reason ?? 'udp_socket_closed',
      wasClean: true,
    };
    try {
      this.socket.close();
    } catch {
      this.finalizeClose({
        code: code ?? 1000,
        reason: reason ?? 'udp_close_failed',
        wasClean: false,
      });
    }
  }

  onMessage(handler: (frame: ImTransportFrame) => void): () => void {
    this.messageHandlers.add(handler);
    return () => this.messageHandlers.delete(handler);
  }

  onClose(handler: (event: ImTransportCloseEvent) => void): () => void {
    this.closeHandlers.add(handler);
    // late-registration：如果连接已经关闭，立即调度回调
    if (this.stateValue === 'closed') {
      queueMicrotask(() => {
        if (this.closeHandlers.has(handler)) {
          handler(this.closeEvent ?? { code: 1000, reason: 'udp_socket_closed', wasClean: true });
        }
      });
    }
    return () => this.closeHandlers.delete(handler);
  }

  onError(handler: (event: ImTransportErrorEvent) => void): () => void {
    this.errorHandlers.add(handler);
    return () => this.errorHandlers.delete(handler);
  }
}

/**
 * UDP 传输工厂（Node 环境）。
 *
 * 通过动态 import('node:dgram') 加载 dgram 模块，避免在浏览器环境加载失败。
 * UDP 无连接，connect() 创建 socket 后立即返回 open 状态的连接。
 */
export class ImUdpTransportFactory implements ImTransportFactory {
  readonly kind = 'udp' as const;
  readonly capabilities: ImTransportCapabilities = TRANSPORT_CAPABILITIES.udp;

  isAvailable(): boolean {
    return typeof (globalThis as { process?: { versions?: { node?: string } } }).process !== 'undefined'
      && (globalThis as { process?: { versions?: { node?: string } } }).process?.versions?.node !== undefined;
  }

  async connect(
    endpoint: ImTransportEndpoint,
    options: ImTransportConnectOptions,
  ): Promise<ImTransportConnection> {
    const buffer = getRuntimeBuffer();
    if (!buffer) {
      throw new Error('UDP transport requires Node.js Buffer; current environment is unsupported.');
    }

    // @ts-ignore - Node.js built-in module; dynamically imported for browser compatibility
    const dgramModule = await import('node:dgram');
    const dgram = (dgramModule as unknown as { default?: NodeDgramModule }).default ?? (dgramModule as unknown as NodeDgramModule);

    const { host, port } = parseUdpEndpoint(endpoint.url);
    const socket = dgram.createSocket('udp4');

    // UDP 必须先 bind 才能接收 message 事件。
    // bind 到随机端口（'0.0.0.0:0'），由操作系统分配。
    // unref 避免阻止 Node 进程退出。
    try {
      socket.unref();
    } catch {
      // 某些环境可能不支持 unref
    }

    // 等待 'listening' 事件确认 bind 成功后再返回连接。
    // 如果 bind 失败（如端口权限问题），'error' 事件会触发并 reject Promise。
    // 监听器在 Promise 完成后必须移除，避免泄漏到 ImUdpTransportConnection 的生命周期中
    // （connection constructor 会注册自己的 message/close/error 监听器）。
    await new Promise<void>((resolve, reject) => {
      let settled = false;
      const timer = setTimeout(() => {
        settled = true;
        socket.off('listening', onListening);
        socket.off('error', onError);
        try {
          socket.close();
        } catch {
          // Socket may already be closed after a bind error.
        }
        reject(new Error(`UDP bind timeout after ${options.connectionTimeoutMs}ms`));
      }, options.connectionTimeoutMs);

      const onListening = (): void => {
        if (settled) {
          return;
        }
        settled = true;
        clearTimeout(timer);
        socket.off('error', onError);
        resolve();
      };

      const onError = (error: Error): void => {
        if (settled) {
          return;
        }
        settled = true;
        clearTimeout(timer);
        socket.off('listening', onListening);
        try {
          socket.close();
        } catch {
          // Socket may already be closed after the error.
        }
        reject(error);
      };

      socket.on('listening', onListening);
      socket.on('error', onError);
      try {
        socket.bind(0, '0.0.0.0');
      } catch (error) {
        onError(error instanceof Error ? error : new Error(String(error)));
      }
    });

    return new ImUdpTransportConnection(socket, host, port);
  }
}
