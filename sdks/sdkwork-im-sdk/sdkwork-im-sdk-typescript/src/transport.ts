/**
 * 通用传输层抽象。
 *
 * 该模块定义了与具体传输协议（WebSocket/TCP/UDP/QUIC）无关的连接接口，
 * 使上层 CCP 协议状态机（realtime.ts）可以复用于任何传输实现。
 *
 * 设计目标：
 * - 浏览器环境只能提供 WebSocket transport（受 API 限制）。
 * - Node/Tauri 后端可提供 WebSocket/TCP/UDP 全部 transport。
 * - React Native / Flutter / 原生平台通过注入自定义 ImTransportFactory 支持原生 socket。
 * - 上层 realtime.ts 只依赖 ImTransportConnection 接口，不感知具体传输。
 */

/** 传输协议类型标识，对齐服务端 TransportBinding 枚举。 */
export type ImTransportKind = 'websocket' | 'tcp' | 'udp';

/** CCP binding 标识，用于协议信封的 binding 字段，对齐服务端 ccp-core TransportBinding.protocol_id。 */
export type ImCcpBindingId = 'Ws1' | 'Tcp1' | 'Udp1' | 'Quic1';

/** 传输能力描述，用于传输选择与协议行为决策。 */
export interface ImTransportCapabilities {
  /** 是否支持帧边界（WebSocket/TCP/QUIC true，UDP false）。 */
  readonly supportsFraming: boolean;
  /** 是否支持数据报模式（UDP true，其他 false）。 */
  readonly supportsDatagram: boolean;
  /** 是否可靠传输（WebSocket/TCP/QUIC true，UDP false）。 */
  readonly reliable: boolean;
  /** 是否有序投递（WebSocket/TCP/QUIC true，UDP false）。 */
  readonly orderedDelivery: boolean;
  /** 是否支持背压（TCP/QUIC true，WebSocket 视实现而定，UDP false）。 */
  readonly supportsBackpressure: boolean;
  /** 单帧最大字节数（TCP/QUIC 512KB，UDP 64KB，WebSocket 由实现决定）。 */
  readonly maxFrameBytes: number;
  /** 是否支持 HTTP 升级前认证（WebSocket true，TCP/UDP false）。 */
  readonly supportsUpgradeAuth: boolean;
  /** 对应的 CCP binding 标识。 */
  readonly ccpBinding: ImCcpBindingId;
}

/** 传输帧数据。 */
export interface ImTransportFrame {
  /** 帧数据，字符串（CCP JSON 编码）或二进制（CCP CBOR 编码，预留）。 */
  readonly data: string | Uint8Array;
  /** 是否为二进制帧。 */
  readonly isBinary: boolean;
}

/** 传输连接关闭事件。 */
export interface ImTransportCloseEvent {
  readonly code: number;
  readonly reason: string;
  readonly wasClean: boolean;
}

/** 传输连接状态。 */
export type ImTransportConnectionState = 'connecting' | 'open' | 'closing' | 'closed';

/** 传输连接错误事件。 */
export interface ImTransportErrorEvent {
  readonly error: unknown;
  readonly code?: string;
}

/**
 * 通用传输连接接口。
 *
 * 所有传输实现（WebSocket/TCP/UDP）都适配为此接口，
 * 上层 realtime.ts 通过该接口与传输层交互，不感知具体协议细节。
 */
export interface ImTransportConnection {
  /** 当前连接状态。 */
  readonly state: ImTransportConnectionState;
  /** 传输能力描述。 */
  readonly capabilities: ImTransportCapabilities;
  /** 传输类型。 */
  readonly kind: ImTransportKind;

  /** 发送一帧数据。连接未 open 时调用行为由实现决定（缓冲或丢弃）。 */
  send(frame: ImTransportFrame): void;
  /** 主动关闭连接。 */
  close(code?: number, reason?: string): void;

  /** 注册消息回调，返回取消订阅函数。 */
  onMessage(handler: (frame: ImTransportFrame) => void): () => void;
  /** 注册连接打开回调，返回取消订阅函数。 */
  onOpen(handler: () => void): () => void;
  /** 注册关闭回调，返回取消订阅函数。 */
  onClose(handler: (event: ImTransportCloseEvent) => void): () => void;
  /** 注册错误回调，返回取消订阅函数。 */
  onError(handler: (event: ImTransportErrorEvent) => void): () => void;
}

/** 传输端点描述。 */
export interface ImTransportEndpoint {
  /** 传输类型。 */
  readonly kind: ImTransportKind;
  /**
   * 端点 URL。
   * - websocket: ws://host:port/im/v3/api/realtime/ws 或 wss://...
   * - tcp: tcp://host:port
   * - udp: udp://host:port
   */
  readonly url: string;
  /** 附加 HTTP 头（仅 WebSocket 升级时使用，TCP/UDP 忽略）。 */
  readonly headers?: Record<string, string>;
  /** WebSocket 子协议（仅 WebSocket 使用）。 */
  readonly protocols?: string[];
  /** 设备 ID（用于路由绑定）。 */
  readonly deviceId?: string;
}

/** 传输连接选项。 */
export interface ImTransportConnectOptions {
  /** 连接超时（毫秒）。 */
  readonly connectionTimeoutMs: number;
  /** 附加 HTTP 头。 */
  readonly headers?: Record<string, string>;
  /** WebSocket 子协议。 */
  readonly protocols?: string[];
}

/**
 * 传输工厂接口。
 *
 * 每种传输（WebSocket/TCP/UDP）实现一个工厂，
 * 通过 isAvailable() 声明当前环境是否可用，
 * 通过 connect() 建立连接并返回 ImTransportConnection。
 */
export interface ImTransportFactory {
  /** 传输类型。 */
  readonly kind: ImTransportKind;
  /** 传输能力描述。 */
  readonly capabilities: ImTransportCapabilities;

  /** 当前运行环境是否支持该传输。 */
  isAvailable(): boolean;

  /**
   * 建立传输连接。
   * @param endpoint 端点描述
   * @param options 连接选项
   * @returns 已建立的传输连接（状态为 connecting 或 open）
   */
  connect(endpoint: ImTransportEndpoint, options: ImTransportConnectOptions): Promise<ImTransportConnection>;
}

/** 传输选择策略。 */
export interface ImTransportSelectionPolicy {
  /** 期望的传输优先级列表，按顺序尝试。 */
  readonly preferred: ImTransportKind[];
  /** 首选传输不可用时是否自动降级到下一个。 */
  readonly autoFallback: boolean;
  /** 连接探测超时（毫秒），超时后尝试下一个传输。 */
  readonly probeTimeoutMs: number;
}

/** 默认传输选择策略：优先 WebSocket，自动降级。 */
export const DEFAULT_TRANSPORT_SELECTION_POLICY: ImTransportSelectionPolicy = {
  preferred: ['websocket', 'tcp', 'udp'],
  autoFallback: true,
  probeTimeoutMs: 15_000,
};

/** 各传输类型的默认能力描述，对齐服务端 ccp-core TransportBinding。 */
export const TRANSPORT_CAPABILITIES: Record<ImTransportKind, ImTransportCapabilities> = {
  websocket: {
    supportsFraming: true,
    supportsDatagram: false,
    reliable: true,
    orderedDelivery: true,
    supportsBackpressure: false,
    maxFrameBytes: 512 * 1024,
    supportsUpgradeAuth: true,
    ccpBinding: 'Ws1',
  },
  tcp: {
    supportsFraming: true,
    supportsDatagram: false,
    reliable: true,
    orderedDelivery: true,
    supportsBackpressure: true,
    maxFrameBytes: 512 * 1024,
    supportsUpgradeAuth: false,
    ccpBinding: 'Tcp1',
  },
  udp: {
    supportsFraming: false,
    supportsDatagram: true,
    reliable: false,
    orderedDelivery: false,
    supportsBackpressure: false,
    maxFrameBytes: 64 * 1024,
    supportsUpgradeAuth: false,
    ccpBinding: 'Udp1',
  },
};

/** 从传输类型获取对应的 CCP binding 标识。 */
export function ccpBindingForTransport(kind: ImTransportKind): ImCcpBindingId {
  return TRANSPORT_CAPABILITIES[kind].ccpBinding;
}

/** 从 URL 解析传输类型。 */
export function parseTransportKindFromUrl(url: string): ImTransportKind | undefined {
  if (/^wss?:\/\//i.test(url)) {
    return 'websocket';
  }
  if (/^tcp:\/\//i.test(url)) {
    return 'tcp';
  }
  if (/^udp:\/\//i.test(url)) {
    return 'udp';
  }
  return undefined;
}
