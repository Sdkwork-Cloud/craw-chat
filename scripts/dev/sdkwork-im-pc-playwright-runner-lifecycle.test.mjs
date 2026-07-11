#!/usr/bin/env node

import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import fs from 'node:fs';
import http from 'node:http';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import * as playwrightRunner from './sdkwork-im-pc-playwright-runner.mjs';
import {
  assertPortAvailable,
  probeHttp,
  stopServer,
  waitForOwnedHttpOk,
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

const hangingServer = http.createServer(() => {});
const hangingAddress = await listen(hangingServer);
await assert.rejects(
  probeHttp(`http://127.0.0.1:${hangingAddress.port}/`, { requestTimeoutMs: 40 }),
  /timed out/u,
  'one unresponsive HTTP socket must not consume the entire readiness deadline',
);
await close(hangingServer);

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
}
assert.match(
  smokeWrapperSource,
  /process\.env\.PLAYWRIGHT_PC_SMOKE_PORT\s*\?\?\s*['"]3000['"]/u,
  'the smoke gate must allow a caller-owned free port without weakening exclusive port checks',
);
assert.match(
  e2eWrapperSource,
  /runCommand[\s\S]*lifecycle\.track\s*\(/u,
  'the Playwright command child must be tracked in addition to the two HTTP servers',
);

console.log('sdkwork-im PC Playwright runner lifecycle contract passed');
