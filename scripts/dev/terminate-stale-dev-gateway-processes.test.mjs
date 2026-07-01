import assert from 'node:assert/strict';

import {
  STALE_DEV_GATEWAY_PROCESS_NAMES,
  terminateStaleDevGatewayProcesses,
} from './terminate-stale-dev-gateway-processes.mjs';

const calls = [];
const stdout = {
  write(line) {
    calls.push(String(line));
  },
};

const result = terminateStaleDevGatewayProcesses({
  platform: 'win32',
  spawnSyncImpl(command, args) {
    calls.push({ command, args });
    if (args.includes('sdkwork-im-standalone-gateway.exe')) {
      return { status: 0, stdout: '', stderr: '' };
    }
    return { status: 128, stdout: '', stderr: 'not found' };
  },
  stdout,
});

assert.deepEqual(result.terminated, ['sdkwork-im-standalone-gateway.exe']);
assert.equal(calls.filter((entry) => typeof entry === 'object').length, 2);
assert.match(
  calls.join('\n'),
  /terminated stale sdkwork-im-standalone-gateway\.exe/u,
);
assert.deepEqual(
  STALE_DEV_GATEWAY_PROCESS_NAMES,
  [
    'sdkwork-im-standalone-gateway.exe',
    'sdkwork-clawrouter-standalone-gateway.exe',
  ],
);

const skipped = terminateStaleDevGatewayProcesses({
  platform: 'linux',
  spawnSyncImpl() {
    throw new Error('taskkill should not run on non-Windows platforms');
  },
});
assert.deepEqual(skipped.terminated, []);

console.log('terminate-stale-dev-gateway-processes.test.mjs passed');
