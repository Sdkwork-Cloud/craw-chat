import assert from 'node:assert/strict';

import {
  isDevelopmentRuntime,
  isRealtimeClusterRouteStoreEnabled,
  resolveRealtimeClusterDevEnv,
} from '../lib/im-realtime-cluster-dev.mjs';

assert.equal(isRealtimeClusterRouteStoreEnabled({}), false);
assert.equal(
  isRealtimeClusterRouteStoreEnabled({
    SDKWORK_IM_REALTIME_ROUTE_STORE_URL: 'redis://127.0.0.1:6379/0',
  }),
  true,
);

assert.equal(isDevelopmentRuntime({ SDKWORK_IM_ENVIRONMENT: 'development' }), true);
assert.equal(isDevelopmentRuntime({ SDKWORK_IM_ENVIRONMENT: 'production' }), false);

assert.deepEqual(resolveRealtimeClusterDevEnv({}), {});

assert.deepEqual(
  resolveRealtimeClusterDevEnv({
    SDKWORK_IM_ENVIRONMENT: 'development',
    SDKWORK_IM_REALTIME_ROUTE_STORE_URL: 'redis://127.0.0.1:6379/0',
  }),
  {
    SDKWORK_IM_REALTIME_NODE_ID: 'session_gateway_dev_1',
    SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET: 'sdkwork-im-dev-realtime-cluster-bus-secret',
  },
);

assert.deepEqual(
  resolveRealtimeClusterDevEnv({
    SDKWORK_IM_ENVIRONMENT: 'production',
    SDKWORK_IM_REALTIME_ROUTE_STORE_URL: 'redis://127.0.0.1:6379/0',
  }),
  {},
);

assert.deepEqual(
  resolveRealtimeClusterDevEnv({
    SDKWORK_IM_ENVIRONMENT: 'development',
    SDKWORK_IM_REALTIME_ROUTE_STORE_URL: 'redis://127.0.0.1:6379/0',
    SDKWORK_IM_REALTIME_NODE_ID: 'custom_node',
    SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET: 'custom-secret',
  }),
  {},
);

console.log('sdkwork-im realtime cluster dev env contract passed');
