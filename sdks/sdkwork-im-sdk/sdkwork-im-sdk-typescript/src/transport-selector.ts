/**
 * 传输选择策略实现。
 *
 * 根据客户端运行环境能力，按优先级选择最优传输协议。
 * 支持：
 * - 自动检测当前环境可用的传输（浏览器仅 WebSocket，Node 全部）
 * - 按策略优先级选择，支持自动降级
 * - 手动覆盖（调用方强制指定传输类型）
 * - 从 base URL 推导传输端点
 */

import type {
  ImTransportEndpoint,
  ImTransportFactory,
  ImTransportKind,
  ImTransportSelectionPolicy,
} from './transport.js';
import {
  DEFAULT_TRANSPORT_SELECTION_POLICY,
  parseTransportKindFromUrl,
} from './transport.js';
import { IM_CCP_WEBSOCKET_SUBPROTOCOL } from './ccp-wire.js';
import { IM_REALTIME_WS } from './realtime-api-paths.js';

/**
 * 检测当前环境可用的传输类型列表。
 *
 * 浏览器环境：仅 websocket。
 * Node 环境：websocket + tcp + udp。
 * Tauri/Electron：取决于注入的工厂，至少 websocket。
 */
export function detectAvailableTransports(
  factories: Map<ImTransportKind, ImTransportFactory>,
): ImTransportKind[] {
  const available: ImTransportKind[] = [];
  for (const kind of ['websocket', 'tcp', 'udp'] as const) {
    const factory = factories.get(kind);
    if (factory?.isAvailable()) {
      available.push(kind);
    }
  }
  return available;
}

/**
 * 选择传输工厂。
 *
 * 选择逻辑：
 * 1. 如果指定了 preferred 且该传输可用，直接使用。
 * 2. 否则按 policy.preferred 顺序尝试，选择第一个可用的。
 * 3. 如果 autoFallback=false 且首选不可用，抛出错误。
 * 4. 如果全部不可用，抛出错误。
 *
 * @param factories 传输工厂集合
 * @param policy 选择策略
 * @param preferred 可选的手动覆盖传输类型
 */
export function selectTransportFactory(
  factories: Map<ImTransportKind, ImTransportFactory>,
  policy: ImTransportSelectionPolicy = DEFAULT_TRANSPORT_SELECTION_POLICY,
  preferred?: ImTransportKind,
): ImTransportFactory {
  if (preferred) {
    const factory = factories.get(preferred);
    if (factory?.isAvailable()) {
      return factory;
    }
    if (!policy.autoFallback) {
      throw new Error(
        `Preferred transport "${preferred}" is not available in the current environment.`,
      );
    }
  }

  for (const kind of policy.preferred) {
    const factory = factories.get(kind);
    if (factory?.isAvailable()) {
      return factory;
    }
  }

  throw new Error(
    'No transport is available in the current environment. ' +
      'Provide a custom ImTransportFactory or run in a supported runtime (browser/Node).',
  );
}

/**
 * 从 base URL 和传输类型构建传输端点。
 *
 * - websocket: ws://host:port/im/v3/api/realtime/ws
 * - tcp: tcp://host:port
 * - udp: udp://host:port
 *
 * @param baseUrl 基础 URL（可以是 http/https/ws/wss/tcp/udp 协议）
 * @param kind 传输类型
 * @param deviceId 可选设备 ID（WebSocket 路由绑定用）
 */
export function buildTransportEndpoint(
  baseUrl: string,
  kind: ImTransportKind,
  deviceId?: string,
): ImTransportEndpoint {
  const urlKind = parseTransportKindFromUrl(baseUrl);

  // 如果 URL 已经明确指定了传输类型且与 kind 匹配，直接使用
  if (urlKind === kind && kind !== 'websocket') {
    return {
      kind,
      url: baseUrl,
      deviceId,
    };
  }

  // 根据传输类型转换 URL 协议
  const url = convertBaseUrlForTransport(baseUrl, kind, deviceId);
  return {
    kind,
    url,
    deviceId,
    ...(kind === 'websocket' ? { protocols: [IM_CCP_WEBSOCKET_SUBPROTOCOL] } : {}),
  };
}

/**
 * 将基础 URL 转换为指定传输类型的 URL。
 *
 * - http(s)://host:port → ws(s)://host:port/im/v3/api/realtime/ws （websocket）
 * - http(s)://host:port → tcp://host:port （tcp）
 * - http(s)://host:port → udp://host:port （udp）
 */
function convertBaseUrlForTransport(
  baseUrl: string,
  kind: ImTransportKind,
  deviceId?: string,
): string {
  const trimmed = baseUrl.trim().replace(/\/+$/u, '');

  if (kind === 'websocket') {
    let wsUrl: string;
    if (trimmed.startsWith('https://')) {
      wsUrl = `wss://${trimmed.slice('https://'.length)}`;
    } else if (trimmed.startsWith('http://')) {
      wsUrl = `ws://${trimmed.slice('http://'.length)}`;
    } else if (trimmed.startsWith('wss://') || trimmed.startsWith('ws://')) {
      wsUrl = trimmed;
    } else {
      wsUrl = `ws://${trimmed}`;
    }
    // 追加 realtime ws 路径
    const parsed = new URL(wsUrl);
    const basePath = parsed.pathname.replace(/\/+$/u, '');
    parsed.pathname = basePath.endsWith(IM_REALTIME_WS)
      ? basePath
      : `${basePath}${IM_REALTIME_WS}`;
    if (deviceId) {
      parsed.searchParams.set('deviceId', deviceId);
    }
    return parsed.toString();
  }

  if (kind === 'tcp') {
    const hostPort = extractHostPort(trimmed);
    return `tcp://${hostPort}`;
  }

  if (kind === 'udp') {
    const hostPort = extractHostPort(trimmed);
    return `udp://${hostPort}`;
  }

  return trimmed;
}

/** 从各种协议的 URL 中提取 host:port。 */
function extractHostPort(url: string): string {
  const match = url.match(/^[a-z]+:\/\/([^/]+)/i);
  if (match) {
    return match[1];
  }
  return url;
}

/**
 * 传输选择器，封装完整的传输选择流程。
 */
export class ImTransportSelector {
  private readonly factories: Map<ImTransportKind, ImTransportFactory>;
  private readonly policy: ImTransportSelectionPolicy;

  constructor(
    factories: Map<ImTransportKind, ImTransportFactory>,
    policy: ImTransportSelectionPolicy = DEFAULT_TRANSPORT_SELECTION_POLICY,
  ) {
    this.factories = factories;
    this.policy = policy;
  }

  /** 检测当前环境可用的传输类型。 */
  detectAvailable(): ImTransportKind[] {
    return detectAvailableTransports(this.factories);
  }

  /** 获取指定类型的传输工厂。 */
  getFactory(kind: ImTransportKind): ImTransportFactory | undefined {
    return this.factories.get(kind);
  }

  /** 构建指定类型的传输端点。 */
  buildEndpoint(kind: ImTransportKind, baseUrl: string, deviceId?: string): ImTransportEndpoint {
    return buildTransportEndpoint(baseUrl, kind, deviceId);
  }

  /**
   * 构建候选传输列表，用于连接失败降级。
   *
   * 排序规则：
   * 1. 如果指定了 preferredKind 且可用，放在首位
   * 2. 然后按 policy.preferred 顺序追加其他可用传输（排除已添加的）
   *
   * @param preferredKind 可选的手动覆盖传输类型
   * @returns 候选传输类型列表，按优先级排序
   */
  buildCandidateList(preferredKind?: ImTransportKind): ImTransportKind[] {
    const candidates: ImTransportKind[] = [];
    const seen = new Set<ImTransportKind>();

    const tryAdd = (kind: ImTransportKind): void => {
      if (seen.has(kind)) {
        return;
      }
      const factory = this.factories.get(kind);
      if (factory?.isAvailable()) {
        candidates.push(kind);
        seen.add(kind);
      }
    };

    // 手动覆盖优先
    if (preferredKind) {
      tryAdd(preferredKind);
    }
    // 然后按 policy.preferred 顺序追加
    for (const kind of this.policy.preferred) {
      tryAdd(kind);
    }

    return candidates;
  }

  /**
   * 选择传输并构建端点（同步，仅用于已确定传输类型的场景）。
   *
   * @param baseUrl 基础 URL
   * @param preferredKind 可选的手动覆盖传输类型
   * @param deviceId 可选设备 ID
   */
  select(
    baseUrl: string,
    preferredKind?: ImTransportKind,
    deviceId?: string,
  ): { factory: ImTransportFactory; endpoint: ImTransportEndpoint } {
    const factory = selectTransportFactory(this.factories, this.policy, preferredKind);
    const endpoint = buildTransportEndpoint(baseUrl, factory.kind, deviceId);
    return { factory, endpoint };
  }
}
