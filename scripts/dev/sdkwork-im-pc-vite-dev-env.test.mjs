#!/usr/bin/env node

import assert from 'node:assert/strict';
import {
  discoverRunningSdkworkImGatewayHttpUrl,
  resolveSdkworkImPcViteDevEnv,
} from '../lib/im-pc-vite-dev-env.mjs';

const resolvedEnv = await resolveSdkworkImPcViteDevEnv({});
assert.equal(
  resolvedEnv.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18079',
  'vite dev env must default APPLICATION_PUBLIC to the local unified gateway',
);
assert.equal(
  resolvedEnv.VITE_SDKWORK_IAM_APP_API_BASE_URL,
  'http://127.0.0.1:18079',
  'vite dev env must align IAM app API base URL with the local unified gateway',
);

const discovered = await discoverRunningSdkworkImGatewayHttpUrl({
  startPort: 65500,
  maxAttempts: 1,
});
assert.equal(discovered, undefined, 'gateway discovery must return undefined when no gateway is listening');

console.log('sdkwork im pc vite dev env contract passed.');
