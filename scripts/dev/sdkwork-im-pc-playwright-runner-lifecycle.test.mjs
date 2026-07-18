#!/usr/bin/env node

import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { EventEmitter } from 'node:events';
import fs from 'node:fs';
import http from 'node:http';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import * as playwrightRunner from './sdkwork-im-pc-playwright-runner.mjs';
import {
  assertPortAvailable,
  probeHttp,
  stopServer,
  waitForOwnedHttpOk,
  waitForOwnedServerPort,
} from './sdkwork-im-pc-playwright-runner.mjs';

class FakeChild extends EventEmitter {
  constructor(pid = 4242) {
    super();
    this.exitCode = null;
    this.killed = false;
    this.pid = pid;
    this.signalCode = null;
    this.signals = [];
  }

  kill(signal = 'SIGTERM') {
    this.killed = true;
    this.signals.push(signal);
    if (signal === 'SIGKILL') {
      this.finish(null, signal);
    }
    return true;
  }

  finish(code = 0, signal = null) {
    this.exitCode = code;
    this.signalCode = signal;
    this.emit('exit', code, signal);
  }
}

class FakeAvailablePortServer extends EventEmitter {
  constructor(checkedHosts) {
    super();
    this.checkedHosts = checkedHosts;
  }

  unref() {}

  listen({ host }, callback) {
    this.checkedHosts.push(host);
    queueMicrotask(callback);
  }

  close(callback) {
    queueMicrotask(() => callback());
  }
}

const reportedPortChild = new FakeChild();
const reportedPortPromise = waitForOwnedServerPort(reportedPortChild);
reportedPortChild.emit('message', {
  port: 43_217,
  type: 'sdkwork-im-pc-server-listening',
});
assert.equal(
  await reportedPortPromise,
  43_217,
  'the runner must use the exact OS-assigned port reported by its owned server',
);

const exitedBeforePortChild = new FakeChild();
exitedBeforePortChild.finish(1);
await assert.rejects(
  waitForOwnedServerPort(exitedBeforePortChild),
  /exited before reporting its TCP port with code 1/u,
  'port discovery must fail immediately when the owned server already exited',
);

const invalidPortChild = new FakeChild();
const invalidPortPromise = waitForOwnedServerPort(invalidPortChild);
invalidPortChild.emit('message', {
  port: 0,
  type: 'sdkwork-im-pc-server-listening',
});
await assert.rejects(
  invalidPortPromise,
  /reported server TCP port must be a positive integer/u,
  'port discovery must reject malformed child readiness messages',
);

function waitForCondition(check, timeoutMs = 5_000) {
  const deadline = Date.now() + timeoutMs;
  return new Promise((resolve, reject) => {
    const attempt = () => {
      if (check()) {
        resolve();
        return;
      }
      if (Date.now() >= deadline) {
        reject(new Error(`condition was not satisfied within ${timeoutMs}ms`));
        return;
      }
      setTimeout(attempt, 20);
    };
    attempt();
  });
}

function isProcessAlive(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    if (error?.code === 'ESRCH') {
      return false;
    }
    throw error;
  }
}

function waitForProcessExit(child, timeoutMs = 10_000) {
  return new Promise((resolve, reject) => {
    let timer;
    const finish = (error, result) => {
      clearTimeout(timer);
      child.removeListener('error', onError);
      child.removeListener('exit', onExit);
      if (error) {
        reject(error);
        return;
      }
      resolve(result);
    };
    const onError = (error) => finish(error);
    const onExit = (code, signal) => finish(null, [code, signal]);
    child.once('error', onError);
    child.once('exit', onExit);
    timer = setTimeout(
      () => finish(new Error(`process did not exit within ${timeoutMs}ms`)),
      timeoutMs,
    );
  });
}

function waitForProcessMessage(child, timeoutMs = 10_000) {
  return new Promise((resolve, reject) => {
    let timer;
    const finish = (error, message) => {
      clearTimeout(timer);
      child.removeListener('error', onError);
      child.removeListener('exit', onExit);
      child.removeListener('message', onMessage);
      if (error) {
        reject(error);
        return;
      }
      resolve(message);
    };
    const onError = (error) => finish(error);
    const onExit = (code, signal) => finish(
      new Error(`process exited before sending readiness message with ${code ?? signal ?? 'unknown'} status`),
    );
    const onMessage = (message) => finish(null, message);
    child.once('error', onError);
    child.once('exit', onExit);
    child.once('message', onMessage);
    timer = setTimeout(
      () => finish(new Error(`process did not send readiness message within ${timeoutMs}ms`)),
      timeoutMs,
    );
  });
}

function forceTerminateTestProcessTree(pid) {
  if (!Number.isSafeInteger(pid) || pid <= 0) {
    return;
  }
  if (process.platform === 'win32') {
    spawnSync('taskkill.exe', ['/PID', String(pid), '/T', '/F'], {
      stdio: 'ignore',
      windowsHide: true,
    });
    return;
  }
  try {
    process.kill(-pid, 'SIGKILL');
  } catch (error) {
    if (error?.code !== 'ESRCH') {
      throw error;
    }
  }
}

function listen(server, host = '127.0.0.1') {
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, host, () => {
      server.removeListener('error', reject);
      resolve(server.address());
    });
  });
}

function close(server) {
  return new Promise((resolve, reject) => {
    server.close((error) => {
      if (error) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

const occupiedServer = net.createServer();
const occupiedAddress = await listen(occupiedServer);
await assert.rejects(
  assertPortAvailable({ host: '127.0.0.1', port: occupiedAddress.port }),
  /already in use/u,
  'an existing listener must fail the Playwright gate before a child process is spawned',
);
await close(occupiedServer);
await assert.doesNotReject(
  assertPortAvailable({ host: '127.0.0.1', port: occupiedAddress.port }),
  'the same port must become available after the existing listener closes',
);

const checkedPortHosts = [];
await assert.doesNotReject(
  assertPortAvailable({
    createServer: () => new FakeAvailablePortServer(checkedPortHosts),
    host: '0.0.0.0',
    port: occupiedAddress.port,
    readinessHosts: ['127.0.0.1', '127.0.0.1'],
  }),
  'an available server port must check every distinct address used to bind or probe it',
);
assert.deepEqual(
  checkedPortHosts,
  ['127.0.0.1', '0.0.0.0'],
  'readiness-specific addresses must be checked before the wildcard bind address',
);

const loopbackOccupiedServer = net.createServer();
const loopbackOccupiedAddress = await listen(loopbackOccupiedServer, '127.0.0.1');
try {
  await assert.rejects(
    assertPortAvailable({
      host: '0.0.0.0',
      port: loopbackOccupiedAddress.port,
      readinessHosts: ['127.0.0.1'],
    }),
    /already in use/u,
    'a loopback listener must fail wildcard server preflight before a child process is spawned',
  );
} finally {
  await close(loopbackOccupiedServer);
}

const hangingServer = http.createServer(() => {});
const hangingAddress = await listen(hangingServer);
await assert.rejects(
  probeHttp(`http://127.0.0.1:${hangingAddress.port}/`, { requestTimeoutMs: 40 }),
  /timed out/u,
  'one unresponsive HTTP socket must not consume the entire readiness deadline',
);
await close(hangingServer);

const trickleServer = http.createServer((_request, response) => {
  response.writeHead(200, { 'content-type': 'text/plain' });
  const trickle = setInterval(() => response.write('x'), 5);
  const finish = setTimeout(() => {
    clearInterval(trickle);
    response.end('done');
  }, 150);
  response.once('close', () => {
    clearInterval(trickle);
    clearTimeout(finish);
  });
});
const trickleAddress = await listen(trickleServer);
await assert.rejects(
  probeHttp(`http://127.0.0.1:${trickleAddress.port}/`, { requestTimeoutMs: 40 }),
  /timed out after 40ms/u,
  'continuous trickle bytes must not extend the absolute HTTP readiness deadline',
);
await close(trickleServer);

const oversizedServer = http.createServer((_request, response) => {
  response.end('x'.repeat(4_096));
});
const oversizedAddress = await listen(oversizedServer);
await assert.rejects(
  probeHttp(`http://127.0.0.1:${oversizedAddress.port}/`, { maxResponseBytes: 128 }),
  /exceeded 128 bytes/u,
  'readiness must bound response buffering even when the endpoint returns HTTP 200',
);
await close(oversizedServer);

const earlyExitChild = new FakeChild();
const earlyExitWait = waitForOwnedHttpOk({
  child: earlyExitChild,
  probe: () => new Promise(() => {}),
  timeoutMs: 1_000,
  url: 'http://127.0.0.1:9/',
});
setTimeout(() => earlyExitChild.finish(23, null), 10);
await assert.rejects(
  earlyExitWait,
  /exited before readiness.*23/u,
  'readiness must fail as soon as the owned child exits',
);

const wrongOwnerChild = new FakeChild();
await assert.rejects(
  waitForOwnedHttpOk({
    child: wrongOwnerChild,
    probe: async () => ({ body: 'unrelated service', headers: {}, statusCode: 200 }),
    retryIntervalMs: 5,
    timeoutMs: 30,
    url: 'http://127.0.0.1:4173/',
    verifyResponse: ({ body }) => body.includes('expected application marker'),
  }),
  /did not expose the expected application response/u,
  'an arbitrary HTTP 200 must not prove that the spawned application owns the port',
);

const windowsChild = new FakeChild(9001);
windowsChild.killed = true;
const taskkillCalls = [];
await stopServer(windowsChild, {
  exitTimeoutMs: 50,
  platform: 'win32',
  spawnSyncImpl(command, args) {
    taskkillCalls.push({ args, command });
    windowsChild.finish(null, 'SIGTERM');
    return { status: 0 };
  },
});
assert.deepEqual(taskkillCalls, [{
  args: ['/PID', '9001', '/T', '/F'],
  command: 'taskkill.exe',
}]);

const unixChild = new FakeChild(9002);
unixChild.killed = true;
await stopServer(unixChild, {
  exitTimeoutMs: 50,
  graceMs: 10,
  platform: 'linux',
});
assert.deepEqual(
  unixChild.signals,
  ['SIGTERM', 'SIGKILL'],
  'a sent signal is not proof of exit; a live child must be escalated after the grace period',
);

const processGroupChild = new FakeChild(9003);
const processGroupSignals = [];
let processGroupAlive = true;
await stopServer(processGroupChild, {
  exitTimeoutMs: 50,
  graceMs: 50,
  isProcessGroupAliveImpl() {
    return processGroupAlive;
  },
  killProcessImpl(pid, signal) {
    processGroupSignals.push({ pid, signal });
    processGroupAlive = false;
    processGroupChild.finish(null, signal);
  },
  platform: 'linux',
  processGroup: true,
});
assert.deepEqual(
  processGroupSignals,
  [{ pid: -9003, signal: 'SIGTERM' }],
  'a detached Unix child must be stopped through its process group so descendants cannot survive',
);

const exitedGroupLeader = new FakeChild(9004);
exitedGroupLeader.finish(0, null);
const exitedLeaderGroupSignals = [];
let exitedLeaderGroupAlive = true;
await stopServer(exitedGroupLeader, {
  exitTimeoutMs: 50,
  graceMs: 50,
  isProcessGroupAliveImpl() {
    return exitedLeaderGroupAlive;
  },
  killProcessImpl(pid, signal) {
    exitedLeaderGroupSignals.push({ pid, signal });
    exitedLeaderGroupAlive = false;
  },
  platform: 'linux',
  processGroup: true,
});
assert.deepEqual(
  exitedLeaderGroupSignals,
  [{ pid: -9004, signal: 'SIGTERM' }],
  'an exited process-group leader must not hide surviving descendants from cleanup',
);

assert.equal(
  typeof playwrightRunner.createOwnedProcessLifecycle,
  'function',
  'the Playwright runner must expose one owned-process lifecycle for signal-safe cleanup',
);

const signalTarget = new EventEmitter();
signalTarget.exitCode = undefined;
const cleanupCalls = [];
let releaseCleanup;
const cleanupGate = new Promise((resolve) => {
  releaseCleanup = resolve;
});
const lifecycle = playwrightRunner.createOwnedProcessLifecycle({
  processTarget: signalTarget,
  async stopChild(child) {
    cleanupCalls.push(child.pid);
    await cleanupGate;
    child.finish(null, 'SIGTERM');
  },
});
const ownedChildren = [new FakeChild(9101), new FakeChild(9102), new FakeChild(9103)];
let releaseWork;
const workGate = new Promise((resolve) => {
  releaseWork = resolve;
});
const lifecycleRun = lifecycle.run(async () => {
  for (const child of ownedChildren) {
    lifecycle.track(child);
  }
  await workGate;
});

signalTarget.emit('SIGINT');
signalTarget.emit('SIGTERM');
await new Promise((resolve) => setImmediate(resolve));
assert.deepEqual(
  cleanupCalls.sort((left, right) => left - right),
  [9101, 9102, 9103],
  'the first termination signal must clean every owned server and command child exactly once',
);
assert.equal(signalTarget.exitCode, 130, 'SIGINT must preserve the conventional exit code');
releaseCleanup();
releaseWork();
await lifecycleRun;
assert.equal(
  signalTarget.listenerCount('SIGINT'),
  0,
  'SIGINT listeners must be removed after lifecycle completion',
);
assert.equal(
  signalTarget.listenerCount('SIGTERM'),
  0,
  'SIGTERM listeners must be removed after lifecycle completion',
);
assert.deepEqual(
  cleanupCalls.sort((left, right) => left - right),
  [9101, 9102, 9103],
  'finally cleanup must reuse the signal cleanup promise instead of stopping children twice',
);

const cleanupFailureTarget = new EventEmitter();
cleanupFailureTarget.exitCode = undefined;
const attemptedCleanupPids = [];
const reportedCleanupErrors = [];
let cleanupFailureWorkReady;
const cleanupFailureReady = new Promise((resolve) => {
  cleanupFailureWorkReady = resolve;
});
let releaseCleanupFailureWork;
const cleanupFailureWorkGate = new Promise((resolve) => {
  releaseCleanupFailureWork = resolve;
});
const cleanupFailureLifecycle = playwrightRunner.createOwnedProcessLifecycle({
  processTarget: cleanupFailureTarget,
  reportCleanupError(error) {
    reportedCleanupErrors.push(error);
  },
  async stopChild(child) {
    attemptedCleanupPids.push(child.pid);
    if (child.pid === 9201) {
      throw new Error('simulated cleanup failure');
    }
    child.finish(null, 'SIGTERM');
  },
});
const cleanupFailureRun = cleanupFailureLifecycle.run(async () => {
  cleanupFailureLifecycle.track(new FakeChild(9201));
  cleanupFailureLifecycle.track(new FakeChild(9202));
  cleanupFailureWorkReady();
  await cleanupFailureWorkGate;
});
await cleanupFailureReady;
cleanupFailureTarget.emit('SIGTERM');
releaseCleanupFailureWork();
await cleanupFailureRun;
assert.deepEqual(
  attemptedCleanupPids.sort((left, right) => left - right),
  [9201, 9202],
  'one cleanup failure must not skip the remaining owned children',
);
assert.equal(cleanupFailureTarget.exitCode, 143, 'SIGTERM cleanup must preserve exit code 143');
assert.equal(reportedCleanupErrors.length, 1, 'signal cleanup failures must be reported exactly once');
assert.match(reportedCleanupErrors[0].message, /simulated cleanup failure/u);
assert.equal(cleanupFailureTarget.listenerCount('SIGINT'), 0);
assert.equal(cleanupFailureTarget.listenerCount('SIGTERM'), 0);

const combinedFailureTarget = new EventEmitter();
combinedFailureTarget.exitCode = undefined;
const combinedFailureLifecycle = playwrightRunner.createOwnedProcessLifecycle({
  processTarget: combinedFailureTarget,
  async stopChild() {
    throw new Error('combined cleanup failure');
  },
});
await assert.rejects(
  combinedFailureLifecycle.run(async () => {
    combinedFailureLifecycle.track(new FakeChild(9301));
    throw new Error('combined work failure');
  }),
  (error) => {
    assert.equal(error instanceof AggregateError, true);
    assert.deepEqual(
      error.errors.map((nestedError) => nestedError.message),
      ['combined work failure', 'combined cleanup failure'],
      'combined work and cleanup failures must both remain diagnosable',
    );
    return true;
  },
);
assert.equal(combinedFailureTarget.listenerCount('SIGINT'), 0);
assert.equal(combinedFailureTarget.listenerCount('SIGTERM'), 0);

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const e2eWrapperSource = fs.readFileSync(
  path.join(scriptsDirectory, 'sdkwork-im-pc-playwright-e2e.test.mjs'),
  'utf8',
);
const smokeWrapperSource = fs.readFileSync(
  path.join(scriptsDirectory, 'sdkwork-im-pc-e2e-smoke.test.mjs'),
  'utf8',
);
for (const [label, source] of [
  ['Playwright e2e', e2eWrapperSource],
  ['production smoke', smokeWrapperSource],
]) {
  assert.match(
    source,
    /createOwnedProcessLifecycle/u,
    `${label} wrapper must install the shared signal-safe owned-process lifecycle`,
  );
  assert.match(
    source,
    /lifecycle\.run\s*\(/u,
    `${label} wrapper must execute its complete command inside the owned-process lifecycle`,
  );
  assert.match(
    source,
    /lifecycle\.track\s*\(/u,
    `${label} wrapper must register every spawned child for process-tree cleanup`,
  );
  assert.match(
    source,
    /readinessHosts:\s*\[['"]127\.0\.0\.1['"]\]/u,
    `${label} wrapper must preflight the loopback address used by HTTP readiness`,
  );
}
assert.match(
  smokeWrapperSource,
  /process\.env\.PLAYWRIGHT_PC_SMOKE_PORT[\s\S]*:\s*0;/u,
  'the smoke gate must default to an OS-assigned port for concurrency-safe execution',
);
assert.match(
  smokeWrapperSource,
  /stdio:\s*\[['"]inherit['"],\s*['"]inherit['"],\s*['"]inherit['"],\s*['"]ipc['"]\]/u,
  'the smoke gate must receive the OS-assigned server port over owned child IPC',
);
assert.match(
  smokeWrapperSource,
  /configuredServerPort\s*>\s*0[\s\S]*assertPortAvailable/u,
  'an explicitly configured smoke port must retain exclusive preflight checks',
);
assert.match(
  e2eWrapperSource,
  /runCommand[\s\S]*lifecycle\.track\s*\(/u,
  'the Playwright command child must be tracked in addition to the two HTTP servers',
);

const runnerModuleUrl = pathToFileURL(
  path.join(scriptsDirectory, 'sdkwork-im-pc-playwright-runner.mjs'),
).href;
const signalFixtureSource = `
import { spawn } from 'node:child_process';
import { createOwnedProcessLifecycle } from ${JSON.stringify(runnerModuleUrl)};

const lifecycle = createOwnedProcessLifecycle();
const onMessage = (message) => {
  if (message?.signal === 'SIGTERM') {
    process.emit('SIGTERM');
  }
};
process.on('message', onMessage);
try {
  await lifecycle.run(async ({ signal }) => {
    const processGroup = process.platform !== 'win32';
    const child = lifecycle.track(spawn(
      process.execPath,
      ['-e', 'setInterval(() => {}, 1000)'],
      {
        detached: processGroup,
        stdio: 'ignore',
        windowsHide: true,
      },
    ), { processGroup });
    process.send({ kind: 'ready', pid: child.pid });
    await new Promise((resolve) => signal.addEventListener('abort', resolve, { once: true }));
  });
} finally {
  process.removeListener('message', onMessage);
  process.disconnect();
}
`;
const signalFixture = spawn(
  process.execPath,
  ['--input-type=module', '-e', signalFixtureSource],
  {
    stdio: ['ignore', 'ignore', 'pipe', 'ipc'],
    windowsHide: true,
  },
);
let signalFixtureStderr = '';
signalFixture.stderr.setEncoding('utf8');
signalFixture.stderr.on('data', (chunk) => {
  signalFixtureStderr += chunk;
});
const readyMessage = await waitForProcessMessage(signalFixture);
assert.equal(readyMessage.kind, 'ready');
assert.equal(Number.isSafeInteger(readyMessage.pid), true);
assert.equal(isProcessAlive(readyMessage.pid), true, 'the fixture grandchild must be live before signal cleanup');
signalFixture.send({ signal: 'SIGTERM' });
const [signalFixtureExitCode, signalFixtureExitSignal] = await waitForProcessExit(signalFixture);
assert.equal(signalFixtureExitSignal, null, signalFixtureStderr);
assert.equal(signalFixtureExitCode, 143, signalFixtureStderr);
await waitForCondition(() => !isProcessAlive(readyMessage.pid));

if (process.platform === 'win32') {
  const hardTerminationFixture = spawn(
    process.execPath,
    ['--input-type=module', '-e', signalFixtureSource],
    {
      stdio: ['ignore', 'ignore', 'pipe', 'ipc'],
      windowsHide: true,
    },
  );
  let hardTerminationFixtureStderr = '';
  hardTerminationFixture.stderr.setEncoding('utf8');
  hardTerminationFixture.stderr.on('data', (chunk) => {
    hardTerminationFixtureStderr += chunk;
  });
  const hardTerminationReady = await waitForProcessMessage(hardTerminationFixture);
  try {
    assert.equal(
      isProcessAlive(hardTerminationReady.pid),
      true,
      'the hard-termination fixture child must be live before its parent is terminated',
    );
    hardTerminationFixture.kill('SIGTERM');
    const [_exitCode, exitSignal] = await waitForProcessExit(hardTerminationFixture);
    assert.equal(exitSignal, 'SIGTERM', hardTerminationFixtureStderr);
    await waitForCondition(
      () => !isProcessAlive(hardTerminationReady.pid),
      3_000,
    );
  } finally {
    forceTerminateTestProcessTree(hardTerminationReady.pid);
  }
}

console.log('sdkwork-im PC Playwright runner lifecycle contract passed');
