/**
 * Generated from `crates/sdkwork-im-realtime-api-paths` — do not hand-edit.
 * Run `pnpm sdk:generate:realtime-api-paths` after changing Rust path constants.
 */

export const IM_REALTIME_API_PREFIX = '/im/v3/api/realtime' as const;
export const IM_REALTIME_SUBSCRIPTIONS_SYNC = '/im/v3/api/realtime/subscriptions/sync' as const;
export const IM_REALTIME_WS = '/im/v3/api/realtime/ws' as const;
export const IM_REALTIME_EVENTS_ACK = '/im/v3/api/realtime/events/ack' as const;
export const IM_REALTIME_EVENTS = '/im/v3/api/realtime/events' as const;
export const IM_PRESENCE_HEARTBEAT = '/im/v3/api/presence/heartbeat' as const;
export const IM_PRESENCE_ME = '/im/v3/api/presence/me' as const;

/** @deprecated Use {@link IM_REALTIME_WS}. */
export const IM_REALTIME_WEBSOCKET_PATH = IM_REALTIME_WS;
