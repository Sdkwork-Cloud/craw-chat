/**
 * WebSocket 传输实现。
 *
 * 将浏览器原生 WebSocket 或 Node ws 库适配为 ImTransportConnection 接口。
 * 这是浏览器环境唯一可用的传输；Node/Tauri 环境也可使用。
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

/** WebSocket 事件名，对齐浏览器 WebSocket 与 Node ws 库的公共事件接口。 */
type WebSocketEventName = 'open' | 'message' | 'close' | 'error';

/** 最小化的 WebSocket like 接口，兼容浏览器 WebSocket 和 Node ws 库。 */
export interface WebSocketLike {
  readonly readyState: number;
  addEventListener(type: WebSocketEventName, handler: (event: unknown) => void): void;
  removeEventListener?(type: WebSocketEventName, handler: (event: unknown) => void): void;
  close(code?: number, reason?: string): void;
  send(data: string | ArrayBuffer | Uint8Array): void;
}

/** WebSocket 工厂函数类型，接受 url、子协议和 headers。 */
export type WebSocketFactory = (
  url: string,
  options: { headers: Record<string, string>; protocols: string[] },
) => WebSocketLike;

/** WebSocket 连接状态常量，对齐浏览器 WebSocket readyState。 */
const WS_CONNECTING = 0;
const WS_OPEN = 1;
const WS_CLOSING = 2;
const WS_CLOSED = 3;

function extractMessageData(event: unknown): string | Uint8Array | undefined {
  if (typeof event === 'string') {
    return event;
  }
  if (event && typeof event === 'object') {
    const record = event as { data?: unknown };
    if (typeof record.data === 'string') {
      return record.data;
    }
    if (record.data instanceof Uint8Array) {
      return record.data;
    }
    if (ArrayBuffer.isView(record.data)) {
      return new Uint8Array(record.data.buffer, record.data.byteOffset, record.data.byteLength);
    }
    if (record.data instanceof ArrayBuffer) {
      return new Uint8Array(record.data);
    }
  }
  return undefined;
}

function readCloseCode(event: unknown): number {
  if (event && typeof event === 'object') {
    const record = event as { code?: unknown };
    return typeof record.code === 'number' ? record.code : 1000;
  }
  return 1000;
}

function readCloseReason(event: unknown): string {
  if (event && typeof event === 'object') {
    const record = event as { reason?: unknown };
    return typeof record.reason === 'string' ? record.reason : '';
  }
  return '';
}

function readWasClean(event: unknown, code: number): boolean {
  if (event && typeof event === 'object') {
    const value = (event as { wasClean?: unknown }).wasClean;
    if (typeof value === 'boolean') {
      return value;
    }
  }
  return code >= 1000 && code < 1004;
}

/**
 * WebSocket 传输连接，适配 WebSocketLike 为 ImTransportConnection。
 */
export class ImWebSocketTransportConnection implements ImTransportConnection {
  readonly kind = 'websocket' as const;
  readonly capabilities: ImTransportCapabilities = TRANSPORT_CAPABILITIES.websocket;

  private readonly socket: WebSocketLike;
  private readonly messageHandlers = new Set<(frame: ImTransportFrame) => void>();
  private readonly openHandlers = new Set<() => void>();
  private readonly closeHandlers = new Set<(event: ImTransportCloseEvent) => void>();
  private readonly errorHandlers = new Set<(event: ImTransportErrorEvent) => void>();
  private stateValue: ImTransportConnectionState = 'connecting';
  private openDispatched = false;
  private closeEvent: ImTransportCloseEvent | undefined;

  constructor(socket: WebSocketLike) {
    this.socket = socket;

    const handleOpen = (): void => {
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
    };

    const handleMessage = (event: unknown): void => {
      if (this.stateValue !== 'open') {
        return;
      }
      const data = extractMessageData(event);
      if (data === undefined) {
        return;
      }
      const frame: ImTransportFrame = {
        data,
        isBinary: typeof data !== 'string',
      };
      for (const handler of this.messageHandlers) {
        handler(frame);
      }
    };

    const handleClose = (event: unknown): void => {
      const closeCode = readCloseCode(event);
      this.finalizeClose({
        code: closeCode,
        reason: readCloseReason(event),
        wasClean: readWasClean(event, closeCode),
      });
    };

    const handleError = (event: unknown): void => {
      if (this.stateValue === 'closed') {
        return;
      }
      const errorEvent: ImTransportErrorEvent = { error: event, code: 'websocket_error' };
      for (const handler of this.errorHandlers) {
        handler(errorEvent);
      }
      // 确保 error 后触发 close：浏览器 WebSocket error 后通常会触发 close，
      // 但主动关闭确保状态一致
      if (this.socket.readyState !== WS_CLOSED) {
        try {
          this.socket.close(4000, 'websocket_error');
        } catch {
          // 忽略重复关闭
        }
      }
    };

    this.socket.addEventListener('open', handleOpen);
    this.socket.addEventListener('message', handleMessage);
    this.socket.addEventListener('close', handleClose);
    this.socket.addEventListener('error', handleError);
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

  get state(): ImTransportConnectionState {
    // 优先使用内部状态（反映 close() 调用），避免与 socket.readyState 不同步
    if (this.stateValue === 'closed' || this.stateValue === 'closing') {
      return this.stateValue;
    }
    const readyState = this.socket.readyState;
    if (readyState === WS_CONNECTING) {
      return 'connecting';
    }
    if (readyState === WS_OPEN) {
      return 'open';
    }
    if (readyState === WS_CLOSING) {
      return 'closing';
    }
    return 'closed';
  }

  send(frame: ImTransportFrame): void {
    if (this.stateValue !== 'open' || this.socket.readyState !== WS_OPEN) {
      return;
    }
    this.socket.send(frame.data);
  }

  close(code?: number, reason?: string): void {
    if (this.stateValue === 'closing' || this.stateValue === 'closed') {
      return;
    }
    // socket 已在关闭中/已关闭：native close 事件可能尚未派发或已被 handleClose 处理
    if (this.socket.readyState === WS_CLOSING || this.socket.readyState === WS_CLOSED) {
      if (this.socket.readyState === WS_CLOSED) {
        this.finalizeClose({
          code: code ?? 1000,
          reason: reason ?? 'websocket_already_closed',
          wasClean: true,
        });
      }
      return;
    }
    this.stateValue = 'closing';
    try {
      this.socket.close(code ?? 1000, reason ?? '');
    } catch {
      this.finalizeClose({
        code: code ?? 1000,
        reason: reason ?? 'websocket_close_failed',
        wasClean: false,
      });
    }
  }

  onMessage(handler: (frame: ImTransportFrame) => void): () => void {
    this.messageHandlers.add(handler);
    return () => this.messageHandlers.delete(handler);
  }

  onOpen(handler: () => void): () => void {
    this.openHandlers.add(handler);
    // 仅在 open 事件已经派发后才调度回调；否则 handleOpen 的 microtask 会负责调用
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
          handler(this.closeEvent ?? { code: 1000, reason: 'websocket_already_closed', wasClean: true });
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
 * WebSocket 传输工厂。
 *
 * 优先使用注入的 webSocketFactory（Node ws 库或 Tauri bridge），
 * 回退到浏览器 globalThis.WebSocket。
 */
export class ImWebSocketTransportFactory implements ImTransportFactory {
  readonly kind = 'websocket' as const;
  readonly capabilities: ImTransportCapabilities = TRANSPORT_CAPABILITIES.websocket;

  private readonly webSocketFactory?: WebSocketFactory;

  constructor(webSocketFactory?: WebSocketFactory) {
    this.webSocketFactory = webSocketFactory;
  }

  isAvailable(): boolean {
    if (this.webSocketFactory) {
      return true;
    }
    return typeof globalThis.WebSocket === 'function';
  }

  async connect(
    endpoint: ImTransportEndpoint,
    options: ImTransportConnectOptions,
  ): Promise<ImTransportConnection> {
    const url = endpoint.url;
    const headers: Record<string, string> = { ...(options.headers ?? {}), ...(endpoint.headers ?? {}) };
    const protocols: string[] = endpoint.protocols ?? options.protocols ?? [];

    let socket: WebSocketLike;
    if (this.webSocketFactory) {
      socket = this.webSocketFactory(url, { headers, protocols });
    } else {
      const WebSocketConstructor = globalThis.WebSocket as unknown;
      if (typeof WebSocketConstructor !== 'function') {
        throw new Error(
          'WebSocket transport is unavailable; provide a webSocketFactory or run in a browser environment.',
        );
      }
      socket = new (WebSocketConstructor as new (
        url: string,
        protocols?: string | string[],
      ) => WebSocketLike)(url, protocols);
    }

    const connection = new ImWebSocketTransportConnection(socket);

    // 如果连接在创建时就已经关闭（极端情况），reject
    if (connection.state === 'closed') {
      return Promise.reject(new Error('WebSocket connection closed immediately after creation'));
    }

    // 连接超时：超时后关闭 socket 并通过 error 事件通知
    const timeoutMs = options.connectionTimeoutMs;
    const timer = setTimeout(() => {
      if (socket.readyState === WS_CONNECTING) {
        socket.close(4002, `websocket_connect_timeout_after_${timeoutMs}ms`);
      }
    }, timeoutMs);

    connection.onOpen(() => clearTimeout(timer));
    connection.onClose(() => clearTimeout(timer));
    connection.onError(() => clearTimeout(timer));

    return connection;
  }
}
