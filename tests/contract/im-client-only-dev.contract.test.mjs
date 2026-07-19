import assert from 'node:assert/strict';
import test from 'node:test';

import {
  createSdkworkChatPcDevPlan,
  parseSdkworkChatPcDevArgs,
} from '../../scripts/lib/im-pc-dev.mjs';

const cloudClientEnv = {
  SDKWORK_IM_DEPLOYMENT_PROFILE: 'cloud',
  SDKWORK_IM_ENVIRONMENT: 'development',
  SDKWORK_IM_PROFILE_ID: 'cloud.development',
  SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://api-dev.sdkwork.com',
  SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://api-dev.sdkwork.com',
  SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'https://api-dev.sdkwork.com',
  SDKWORK_IM_PLATFORM_API_GATEWAY_AUTOSTART: 'false',
};

test('parses client-only independently from runtime target', () => {
  assert.deepEqual(
    parseSdkworkChatPcDevArgs(['--client-only', '--target', 'desktop']),
    {
      clientOnly: true,
      database: undefined,
      dryRun: false,
      envFile: undefined,
      target: 'desktop',
    },
  );
});

test('cloud client-only plan contains one renderer and no local API or database process', () => {
  const plan = createSdkworkChatPcDevPlan({
    argv: ['--client-only', '--target', 'browser', '--dry-run'],
    env: cloudClientEnv,
  });

  assert.equal(plan.processes.length, 1);
  assert.equal(plan.processes[0].label, 'sdkwork-im-pc-browser');
  assert.equal(plan.processes[0].env.SDKWORK_IM_DATABASE_URL, undefined);
  assert.equal(plan.processes[0].env.SDKWORK_CLAW_DATABASE_URL, undefined);
  assert.equal(
    plan.processes[0].env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
    'https://api-dev.sdkwork.com',
  );
});
