/**
 * 传输实现导出与默认注册表。
 */

export {
  ImWebSocketTransportFactory,
  ImWebSocketTransportConnection,
  type WebSocketFactory,
  type WebSocketLike,
} from './websocket-transport.js';
export {
  ImTcpTransportFactory,
  ImTcpTransportConnection,
} from './tcp-transport.js';
export {
  ImUdpTransportFactory,
  ImUdpTransportConnection,
} from './udp-transport.js';

import type { ImTransportFactory, ImTransportKind } from '../transport.js';
import { ImWebSocketTransportFactory } from './websocket-transport.js';
import { ImTcpTransportFactory } from './tcp-transport.js';
import { ImUdpTransportFactory } from './udp-transport.js';

/**
 * 创建默认的传输工厂集合。
 *
 * 包含 WebSocket（浏览器+Node）、TCP（Node）、UDP（Node）三种传输。
 * 各工厂通过 isAvailable() 声明当前环境是否可用。
 *
 * @param webSocketFactory 可选的 WebSocket 工厂注入（Node ws 库或 Tauri bridge）
 */
export function createDefaultTransportFactories(
  webSocketFactory?: import('./websocket-transport.js').WebSocketFactory,
): Map<ImTransportKind, ImTransportFactory> {
  const factories = new Map<ImTransportKind, ImTransportFactory>();
  factories.set('websocket', new ImWebSocketTransportFactory(webSocketFactory));
  factories.set('tcp', new ImTcpTransportFactory());
  factories.set('udp', new ImUdpTransportFactory());
  return factories;
}
