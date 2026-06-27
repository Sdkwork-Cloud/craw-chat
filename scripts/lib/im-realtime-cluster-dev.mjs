const ROUTE_STORE_ENV = 'SDKWORK_IM_REALTIME_ROUTE_STORE_URL';
const CLUSTER_BUS_ENV = 'SDKWORK_IM_REALTIME_CLUSTER_BUS_URL';
const NODE_ID_ENV = 'SDKWORK_IM_REALTIME_NODE_ID';
const SECRET_ENV = 'SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET';

const DEFAULT_DEV_NODE_ID = 'session_gateway_dev_1';
const DEFAULT_DEV_CLUSTER_BUS_SECRET = 'sdkwork-im-dev-realtime-cluster-bus-secret';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

export function isRealtimeClusterRouteStoreEnabled(env = process.env) {
  return Boolean(
    normalizeText(env[ROUTE_STORE_ENV]) || normalizeText(env[CLUSTER_BUS_ENV]),
  );
}

export function isDevelopmentRuntime(env = process.env) {
  const environment = normalizeText(env.SDKWORK_IM_ENVIRONMENT)
    ?? normalizeText(env.SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT)
    ?? 'development';
  return environment !== 'production';
}

/**
 * Supplies development-only realtime cluster defaults when Redis route store or cluster bus
 * URLs are configured. Production deployments must set explicit values.
 */
export function resolveRealtimeClusterDevEnv(env = process.env, { onInject } = {}) {
  if (!isRealtimeClusterRouteStoreEnabled(env) || !isDevelopmentRuntime(env)) {
    return {};
  }

  const injected = {};
  if (!normalizeText(env[NODE_ID_ENV])) {
    injected[NODE_ID_ENV] = DEFAULT_DEV_NODE_ID;
    onInject?.({ key: NODE_ID_ENV, value: DEFAULT_DEV_NODE_ID });
  }
  if (!normalizeText(env[SECRET_ENV])) {
    injected[SECRET_ENV] = DEFAULT_DEV_CLUSTER_BUS_SECRET;
    onInject?.({ key: SECRET_ENV, value: DEFAULT_DEV_CLUSTER_BUS_SECRET });
  }
  return injected;
}
