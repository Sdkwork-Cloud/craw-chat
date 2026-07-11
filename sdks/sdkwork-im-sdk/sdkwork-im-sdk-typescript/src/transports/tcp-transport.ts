/**
 * TCP 传输实现（Node 环境）。
 *
 * 使用 Node.js net.Socket 建立长连接，采用 4 字节大端长度前缀分帧，
 * 对齐服务端 sdkwork-im-ccp-binding-tcp（CCP_TCP_FRAME_HEADER_BYTES=4，
 * CCP_TCP_MAX_FRAME_BYTES=512KB）。
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

/** 4 字节大端长度前缀，对齐服务端 CCP_TCP_FRAME_HEADER_BYTES。 */
const TCP_FRAME_HEADER_BYTES = 4;
/** 单帧最大 512KB，对齐服务端 CCP_TCP_MAX_FRAME_BYTES。 */
const TCP_MAX_FRAME_BYTES = 512 * 1024;
const TCP_CLOSE_GRACE_MS = 1_000;

/** Node net 模块类型（最小化声明，避免依赖 @types/node）。 */
interface NodeNetSocket {
  write(data: Uint8Array | string): boolean;
  end(): void;
  destroy(error?: Error): void;
  on(event: 'connect', listener: () => void): this;
  on(event: 'data', listener: (data: Uint8Array) => void): this;
  on(event: 'close', listener: (hadError: boolean) => void): this;
  on(event: 'error', listener: (error: Error) => void): this;
  setKeepAlive(enable: boolean, initialDelay?: number): this;
  setNoDelay(noDelay: boolean): this;
  unref(): this;
  ref(): this;
  readonly destroyed: boolean;
  readonly writable: boolean;
}

interface NodeNetModule {
  createConnection(options: {
    host: string;
    port: number;
  }): NodeNetSocket;
}

/** 运行时 Buffer 类型声明。 */
type RuntimeBuffer = {
  alloc(size: number): Uint8Array;
  concat(list: Uint8Array[], totalLength?: number): Uint8Array;
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

/** 从 tcp://host:port URL 解析主机和端口。 */
function parseTcpEndpoint(url: string): { host: string; port: number } {
  const parsed = new URL(url);
  const host = parsed.hostname || '127.0.0.1';
  const port = Number.parseInt(parsed.port, 10);
  if (!Number.isFinite(port) || port <= 0 || port > 65535) {
    throw new Error(`Invalid TCP port in URL: ${url}`);
  }
  return { host, port };
}

/**
 * 4 字节大端长度前缀帧编码器。
 *
 * 帧格式：[4字节大端长度][payload]
 * payload 为 UTF-8 编码的 CCP envelope JSON 字符串字节。
 */
class TcpFrameEncoder {
  private readonly buffer: RuntimeBuffer;

  constructor(buffer: RuntimeBuffer) {
    this.buffer = buffer;
  }

  encodeFrame(payload: string | Uint8Array): Uint8Array {
    const payloadBytes = typeof payload === 'string'
      ? this.buffer.byteLength(payload, 'utf8')
      : payload.byteLength;

    if (payloadBytes > TCP_MAX_FRAME_BYTES) {
      throw new Error(
        `TCP frame payload exceeds max ${TCP_MAX_FRAME_BYTES} bytes: got ${payloadBytes}`,
      );
    }

    const header = this.buffer.alloc(TCP_FRAME_HEADER_BYTES);
    const view = new DataView(header.buffer, header.byteOffset, header.byteLength);
    view.setUint32(0, payloadBytes, false); // 大端

    if (typeof payload === 'string') {
      // 使用 TextEncoder.encode() 而非 alloc + encodeInto，避免：
      // 1. encodeInto 写入不完整时静默失败
      // 2. 不同运行时 Buffer.alloc 返回的实例可能不兼容 encodeInto
      const encoder = new TextEncoder();
      const payloadBuffer = encoder.encode(payload);
      return this.buffer.concat([header, payloadBuffer], header.byteLength + payloadBuffer.byteLength);
    }

    return this.buffer.concat([header, payload], header.byteLength + payload.byteLength);
  }
}

/**
 * 4 字节大端长度前缀帧解码器。
 *
 * 状态机：先读 4 字节 header 得到 payload 长度，再读对应字节 payload，
 * 然后重置状态等待下一帧。
 */
class TcpFrameDecoder {
  private readonly buffer: RuntimeBuffer;
  private bufferQueue: Uint8Array[] = [];
  private bufferTotal = 0;
  private state: 'header' | 'payload' = 'header';
  private expectedPayloadLength = 0;

  constructor(buffer: RuntimeBuffer) {
    this.buffer = buffer;
  }

  /** 推入新数据，返回解码出的完整帧列表。 */
  push(data: Uint8Array): Uint8Array[] {
    this.bufferQueue.push(data);
    this.bufferTotal += data.byteLength;

    const frames: Uint8Array[] = [];
    while (true) {
      if (this.state === 'header') {
        if (this.bufferTotal < TCP_FRAME_HEADER_BYTES) {
          break;
        }
        const header = this.consume(TCP_FRAME_HEADER_BYTES);
        const view = new DataView(header.buffer, header.byteOffset, header.byteLength);
        this.expectedPayloadLength = view.getUint32(0, false); // 大端
        if (this.expectedPayloadLength > TCP_MAX_FRAME_BYTES) {
          throw new Error(
            `TCP frame length ${this.expectedPayloadLength} exceeds max ${TCP_MAX_FRAME_BYTES}`,
          );
        }
        this.state = 'payload';
      }

      if (this.state === 'payload') {
        if (this.bufferTotal < this.expectedPayloadLength) {
          break;
        }
        const payload = this.consume(this.expectedPayloadLength);
        frames.push(payload);
        this.state = 'header';
        this.expectedPayloadLength = 0;
      }
    }
    return frames;
  }

  /** 清理缓冲区，释放内存。连接关闭时调用。 */
  reset(): void {
    this.bufferQueue.length = 0;
    this.bufferTotal = 0;
    this.state = 'header';
    this.expectedPayloadLength = 0;
  }

  /** 从队列头部消费指定字节数。 */
  private consume(length: number): Uint8Array {
    const result = this.buffer.alloc(length);
    let written = 0;
    while (written < length) {
      const chunk = this.bufferQueue[0];
      if (!chunk) {
        break;
      }
      const remaining = length - written;
      if (chunk.byteLength <= remaining) {
        result.set(chunk, written);
        written += chunk.byteLength;
        this.bufferQueue.shift();
        this.bufferTotal -= chunk.byteLength;
      } else {
        const slice = chunk.subarray(0, remaining);
        result.set(slice, written);
        written += remaining;
        this.bufferQueue[0] = chunk.subarray(remaining);
        this.bufferTotal -= remaining;
      }
    }
    return result;
  }
}

/**
 * TCP 传输连接，封装 net.Socket 并实现长度前缀分帧。
 */
export class ImTcpTransportConnection implements ImTransportConnection {
  readonly kind = 'tcp' as const;
  readonly capabilities: ImTransportCapabilities = TRANSPORT_CAPABILITIES.tcp;

  private readonly socket: NodeNetSocket;
  private readonly encoder: TcpFrameEncoder;
  private readonly decoder: TcpFrameDecoder;
  private readonly messageHandlers = new Set<(frame: ImTransportFrame) => void>();
  private readonly openHandlers = new Set<() => void>();
  private readonly closeHandlers = new Set<(event: ImTransportCloseEvent) => void>();
  private readonly errorHandlers = new Set<(event: ImTransportErrorEvent) => void>();
  private stateValue: ImTransportConnectionState = 'connecting';
  private openDispatched = false;
  private closeEvent: ImTransportCloseEvent | undefined;
  private requestedCloseEvent: ImTransportCloseEvent | undefined;
  private closeFallbackTimer: ReturnType<typeof setTimeout> | undefined;

  constructor(socket: NodeNetSocket, buffer: RuntimeBuffer) {
    this.socket = socket;
    this.encoder = new TcpFrameEncoder(buffer);
    this.decoder = new TcpFrameDecoder(buffer);

    socket.on('connect', () => {
      if (this.stateValue !== 'connecting') {
        return;
      }
      this.stateValue = 'open';
      // 异步触发 open 回调，确保调用方在 factory.connect() 返回后已注册 onOpen
      queueMicrotask(() => {
        if (this.stateValue !== 'open') {
          return;
        }
        this.openDispatched = true;
        for (const handler of this.openHandlers) {
          handler();
        }
      });
    });

    socket.on('data', (data: Uint8Array) => {
      // 连接已关闭时不再处理数据，避免 decoder 状态混乱
      if (this.stateValue === 'closing' || this.stateValue === 'closed') {
        return;
      }
      const bytes = isBufferLike(data) ? new Uint8Array(data.buffer, data.byteOffset, data.byteLength) : data;
      try {
        const frames = this.decoder.push(bytes);
        for (const frame of frames) {
          const text = new TextDecoder().decode(frame);
          for (const handler of this.messageHandlers) {
            handler({ data: text, isBinary: false });
          }
        }
      } catch (error) {
        const errorEvent: ImTransportErrorEvent = {
          error,
          code: 'tcp_frame_decode_error',
        };
        for (const handler of this.errorHandlers) {
          handler(errorEvent);
        }
        this.socket.destroy(error instanceof Error ? error : new Error(String(error)));
      }
    });

    socket.on('close', (hadError: boolean) => {
      this.finalizeClose(hadError
        ? { code: 4000, reason: 'tcp_connection_error', wasClean: false }
        : (this.requestedCloseEvent
          ?? { code: 1000, reason: 'tcp_connection_closed', wasClean: true }));
    });

    socket.on('error', (error: Error) => {
      const errorEvent: ImTransportErrorEvent = { error, code: 'tcp_socket_error' };
      for (const handler of this.errorHandlers) {
        handler(errorEvent);
      }
      // 确保 error 后触发 close：Node net.Socket 通常 error 后会触发 close，
      // 但在某些极端情况下可能不会。主动 destroy 确保 close 事件被触发。
      if (this.stateValue !== 'closed') {
        try {
          this.socket.destroy(error);
        } catch {
          // 忽略重复 destroy
        }
      }
    });
  }

  private finalizeClose(event: ImTransportCloseEvent): void {
    if (this.stateValue === 'closed') {
      return;
    }
    if (this.closeFallbackTimer) {
      clearTimeout(this.closeFallbackTimer);
      this.closeFallbackTimer = undefined;
    }
    this.stateValue = 'closed';
    this.closeEvent = event;
    this.decoder.reset();
    for (const handler of [...this.closeHandlers]) {
      handler(event);
    }
    this.messageHandlers.clear();
    this.openHandlers.clear();
    this.errorHandlers.clear();
  }

  get state(): ImTransportConnectionState {
    return this.stateValue;
  }

  send(frame: ImTransportFrame): void {
    if (this.stateValue !== 'open' || this.socket.destroyed) {
      return;
    }
    try {
      const encoded = this.encoder.encodeFrame(frame.data);
      this.socket.write(encoded);
    } catch (error) {
      const errorEvent: ImTransportErrorEvent = { error, code: 'tcp_write_error' };
      for (const handler of this.errorHandlers) {
        handler(errorEvent);
      }
    }
  }

  close(code?: number, reason?: string): void {
    if (this.stateValue === 'closing' || this.stateValue === 'closed') {
      return;
    }
    this.stateValue = 'closing';
    this.requestedCloseEvent = {
      code: code ?? 1000,
      reason: reason ?? 'tcp_connection_closed',
      wasClean: true,
    };
    try {
      this.socket.end();
    } catch (error) {
      try {
        this.socket.destroy(error instanceof Error ? error : new Error(String(error)));
      } catch {
        this.finalizeClose({
          code: code ?? 1000,
          reason: reason ?? 'tcp_close_failed',
          wasClean: false,
        });
      }
    }
    this.closeFallbackTimer = setTimeout(() => {
      if (this.stateValue !== 'closing') {
        return;
      }
      try {
        this.socket.destroy();
      } catch {
        this.finalizeClose(this.requestedCloseEvent ?? {
          code: 1000,
          reason: 'tcp_connection_closed',
          wasClean: true,
        });
      }
    }, TCP_CLOSE_GRACE_MS);
  }

  onMessage(handler: (frame: ImTransportFrame) => void): () => void {
    this.messageHandlers.add(handler);
    return () => this.messageHandlers.delete(handler);
  }

  onOpen(handler: () => void): () => void {
    this.openHandlers.add(handler);
    // 仅在 open 事件已经派发后才调度回调；否则 connect 的 microtask 会负责调用
    if (this.stateValue === 'open' && this.openDispatched) {
      queueMicrotask(() => {
        if (this.openHandlers.has(handler) && this.stateValue === 'open') {
          handler();
        }
      });
    }
    return () => this.openHandlers.delete(handler);
  }

  onClose(handler: (event: ImTransportCloseEvent) => void): () => void {
    this.closeHandlers.add(handler);
    // late-registration：如果连接已经关闭，立即调度回调
    if (this.stateValue === 'closed') {
      queueMicrotask(() => {
        if (this.closeHandlers.has(handler)) {
          handler(this.closeEvent ?? { code: 1000, reason: 'tcp_connection_closed', wasClean: true });
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
 * TCP 传输工厂（Node 环境）。
 *
 * 通过动态 import('node:net') 加载 net 模块，避免在浏览器环境加载失败。
 */
export class ImTcpTransportFactory implements ImTransportFactory {
  readonly kind = 'tcp' as const;
  readonly capabilities: ImTransportCapabilities = TRANSPORT_CAPABILITIES.tcp;

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
      throw new Error('TCP transport requires Node.js Buffer; current environment is unsupported.');
    }

    // @ts-ignore - Node.js built-in module; dynamically imported for browser compatibility
    const netModule = await import('node:net');
    const net = (netModule as unknown as { default?: NodeNetModule }).default ?? (netModule as unknown as NodeNetModule);

    const { host, port } = parseTcpEndpoint(endpoint.url);
    const socket = net.createConnection({ host, port });

    // 启用 TCP keepalive（初始延迟 30 秒），避免中间设备因 idle 断开连接。
    // 启用 TCP_NODELAY，禁用 Nagle 算法，降低小帧延迟。
    try {
      socket.setKeepAlive(true, 30_000);
      socket.setNoDelay(true);
    } catch {
      // 某些环境可能不支持
    }

    const connection = new ImTcpTransportConnection(socket, buffer);

    // 如果连接在创建时就已经失败（极端情况），reject
    if (connection.state === 'closed') {
      return Promise.reject(new Error('TCP connection closed immediately after creation'));
    }

    // 连接超时
    const timeoutMs = options.connectionTimeoutMs;
    const timer = setTimeout(() => {
      if (connection.state === 'connecting') {
        socket.destroy(new Error(`TCP connect timeout after ${timeoutMs}ms`));
      }
    }, timeoutMs);

    connection.onOpen(() => clearTimeout(timer));
    connection.onClose(() => clearTimeout(timer));
    connection.onError(() => clearTimeout(timer));

    return connection;
  }
}
