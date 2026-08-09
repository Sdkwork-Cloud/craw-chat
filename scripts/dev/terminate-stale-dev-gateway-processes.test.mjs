import assert from 'node:assert/strict';

import {
  STALE_DEV_GATEWAY_PROCESS_NAMES,
  terminateStaleDevGatewayProcesses,
  terminateStaleGatewayPortListeners,
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
    if (args.includes('sdkwork-api-im-standalone-gateway.exe')) {
      return { status: 0, stdout: '', stderr: '' };
    }
    return { status: 128, stdout: '', stderr: 'not found' };
  },
  stdout,
});

assert.deepEqual(result.terminated, ['sdkwork-api-im-standalone-gateway.exe']);
assert.equal(calls.filter((entry) => typeof entry === 'object').length, 2);
assert.match(
  calls.join('\n'),
  /terminated stale sdkwork-api-im-standalone-gateway\.exe/u,
);
assert.deepEqual(
  STALE_DEV_GATEWAY_PROCESS_NAMES,
  [
    'sdkwork-api-im-standalone-gateway.exe',
    'sdkwork-cloudrouter-standalone-gateway.exe',
  ],
);

const skipped = terminateStaleDevGatewayProcesses({
  platform: 'linux',
  spawnSyncImpl() {
    throw new Error('taskkill should not run on non-Windows platforms');
  },
});
assert.deepEqual(skipped.terminated, []);

const portCalls = [];
const portStdout = {
  write(line) {
    portCalls.push(String(line));
  },
};

const portResult = await terminateStaleGatewayPortListeners({
  platform: 'win32',
  ports: [18089],
  spawnSyncImpl(command, args) {
    portCalls.push({ command, args });
    return { status: 0, stdout: '', stderr: '' };
  },
  listListeningPids(ports) {
    // Report the stale listener once, then release the port after the kill.
    if (portCalls.filter((entry) => typeof entry === 'object').length === 0) {
      return new Set([4242]);
    }
    return new Set();
  },
  stdout: portStdout,
  waitMs: 0,
});
assert.deepEqual(
  portResult.terminated,
  [{ port: 18089, pid: 4242 }],
  'port-based cleanup must force-kill stale listeners holding gateway ports',
);
assert.deepEqual(
  portCalls.filter((entry) => typeof entry === 'object'),
  [{ command: 'taskkill.exe', args: ['/F', '/PID', '4242'] }],
  'port-based cleanup must escalate to taskkill /F /PID for the stale listener',
);
assert.match(
  portCalls.join('\n'),
  /terminated stale gateway listener PID 4242 on port 18089/u,
);

const skippedPorts = await terminateStaleGatewayPortListeners({
  platform: 'linux',
  ports: [18089],
  spawnSyncImpl() {
    throw new Error('taskkill should not run on non-Windows platforms');
  },
});
assert.deepEqual(skippedPorts.terminated, []);

const deniedCalls = [];
const deniedStdout = {
  write(line) {
    deniedCalls.push(String(line));
  },
};
const deniedResult = await terminateStaleGatewayPortListeners({
  platform: 'win32',
  ports: [18089],
  spawnSyncImpl() {
    return { status: 1, stdout: '', stderr: '拒绝访问' };
  },
  listListeningPids() {
    return new Set([4243]);
  },
  stdout: deniedStdout,
  waitMs: 0,
  maxAttempts: 2,
});
assert.equal(
  deniedCalls.some((line) => /could not terminate stale gateway listener PID 4243/u.test(line)),
  true,
  'access-denied taskkill failures must surface in the dev log instead of being swallowed',
);
assert.ok(
  deniedResult.terminated.length > 0,
  'port-based cleanup must still report the attempted kill when taskkill is denied',
);

console.log('terminate-stale-dev-gateway-processes.test.mjs passed');
