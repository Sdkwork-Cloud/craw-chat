import { spawnSync } from 'node:child_process';
import http from 'node:http';
import net from 'node:net';

const DEFAULT_HTTP_RESPONSE_LIMIT_BYTES = 256 * 1024;
const DEFAULT_HTTP_REQUEST_TIMEOUT_MS = 2_000;
const TERMINATION_SIGNAL_EXIT_CODES = Object.freeze({
  SIGINT: 130,
  SIGTERM: 143,
});

function normalizePositiveInteger(value, label) {
  const normalized = Number(value);
  if (!Number.isSafeInteger(normalized) || normalized <= 0) {
    throw new Error(`${label} must be a positive integer`);
  }
  return normalized;
}

export function parseTcpPort(value, label = 'TCP port') {
  const port = normalizePositiveInteger(value, label);
  if (port > 65_535) {
    throw new Error(`${label} must be between 1 and 65535`);
  }
  return port;
}

export function waitForOwnedServerPort(child, {
  messageType = 'sdkwork-im-pc-server-listening',
  timeoutMs = 10_000,
} = {}) {
  if (!child || typeof child.on !== 'function' || typeof child.once !== 'function') {
    throw new Error('server port discovery requires an owned child process');
  }
  const normalizedTimeout = normalizePositiveInteger(timeoutMs, 'server port discovery timeout');
  if (childHasExited(child)) {
    return Promise.reject(new Error(
      `owned server exited before reporting its TCP port with ${formatChildExit(child.exitCode, child.signalCode)}`,
    ));
  }

  return new Promise((resolve, reject) => {
    let timer;
    const cleanup = () => {
      clearTimeout(timer);
      child.removeListener('error', onError);
      child.removeListener('exit', onExit);
      child.removeListener('message', onMessage);
    };
    const settle = (error, port) => {
      cleanup();
      if (error) {
        reject(error);
        return;
      }
      resolve(port);
    };
    const onError = () => settle(new Error('owned server failed before reporting its TCP port'));
    const onExit = (code, signal) => settle(new Error(
      `owned server exited before reporting its TCP port with ${formatChildExit(code, signal)}`,
    ));
    const onMessage = (message) => {
      if (!message || message.type !== messageType) {
        return;
      }
      try {
        settle(null, parseTcpPort(message.port, 'reported server TCP port'));
      } catch (error) {
        settle(error);
      }
    };

    child.once('error', onError);
    child.once('exit', onExit);
    child.on('message', onMessage);
    timer = setTimeout(() => settle(new Error(
      `owned server did not report its TCP port within ${normalizedTimeout}ms`,
    )), normalizedTimeout);
  });
}

function assertHostPortAvailable({ createServer, host, port }) {
  return new Promise((resolve, reject) => {
    const server = createServer();
    let settled = false;
    const settle = (error) => {
      if (settled) {
        return;
      }
      settled = true;
      server.removeAllListeners();
      if (error) {
        reject(error);
        return;
      }
      resolve();
    };

    server.unref?.();
    server.once('error', (error) => {
      if (error?.code === 'EADDRINUSE') {
        settle(new Error(`TCP port ${host}:${port} is already in use`));
        return;
      }
      settle(new Error(`failed to verify TCP port ${host}:${port}`));
    });
    server.listen({ exclusive: true, host, port }, () => {
      server.close((error) => {
        settle(error ? new Error(`failed to release TCP port ${host}:${port}`) : null);
      });
    });
  });
}

export async function assertPortAvailable({
  createServer = net.createServer,
  host = '127.0.0.1',
  port,
  readinessHosts = [],
} = {}) {
  const normalizedPort = parseTcpPort(port);
  if (!Array.isArray(readinessHosts)) {
    throw new Error('TCP readiness hosts must be an array');
  }

  const hosts = [...new Set([...readinessHosts, host])];
  if (hosts.some((candidate) => typeof candidate !== 'string' || candidate.length === 0)) {
    throw new Error('TCP port check hosts must be non-empty strings');
  }

  for (const candidate of hosts) {
    await assertHostPortAvailable({
      createServer,
      host: candidate,
      port: normalizedPort,
    });
  }
}

export function probeHttp(url, {
  getImpl = http.get,
  maxResponseBytes = DEFAULT_HTTP_RESPONSE_LIMIT_BYTES,
  requestTimeoutMs = DEFAULT_HTTP_REQUEST_TIMEOUT_MS,
} = {}) {
  const normalizedLimit = normalizePositiveInteger(maxResponseBytes, 'HTTP response byte limit');
  const normalizedTimeout = normalizePositiveInteger(requestTimeoutMs, 'HTTP request timeout');

  return new Promise((resolve, reject) => {
    let absoluteTimeout;
    let settled = false;
    const settle = (error, result) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(absoluteTimeout);
      if (error) {
        reject(error);
        return;
      }
      resolve(result);
    };

    let request;
    try {
      request = getImpl(url, (response) => {
        const chunks = [];
        let totalBytes = 0;
        response.once('error', (error) => settle(error));
        const declaredLength = Number(response.headers['content-length']);
        if (Number.isFinite(declaredLength) && declaredLength > normalizedLimit) {
          const error = new Error(
            `HTTP readiness response from ${url} exceeded ${normalizedLimit} bytes`,
          );
          response.destroy(error);
          settle(error);
          return;
        }
        response.on('data', (chunk) => {
          if (settled) {
            return;
          }
          const buffer = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
          totalBytes += buffer.length;
          if (totalBytes > normalizedLimit) {
            const error = new Error(
              `HTTP readiness response from ${url} exceeded ${normalizedLimit} bytes`,
            );
            response.destroy(error);
            settle(error);
            return;
          }
          chunks.push(buffer);
        });
        response.once('end', () => {
          if (settled) {
            return;
          }
          settle(null, {
            body: Buffer.concat(chunks, totalBytes).toString('utf8'),
            headers: response.headers,
            statusCode: response.statusCode ?? 0,
          });
        });
      });
    } catch (error) {
      settle(error);
      return;
    }

    request.once('error', (error) => settle(error));
    if (settled) {
      return;
    }
    absoluteTimeout = setTimeout(() => {
      const error = new Error(
        `HTTP readiness request to ${url} timed out after ${normalizedTimeout}ms`,
      );
      settle(error);
      request.destroy(error);
    }, normalizedTimeout);
  });
}

function childHasExited(child) {
  return Boolean(child) && (child.exitCode !== null || child.signalCode !== null);
}

function formatChildExit(code, signal) {
  if (code !== null && code !== undefined) {
    return `code ${code}`;
  }
  if (signal) {
    return `signal ${signal}`;
  }
  return 'an unknown status';
}

function monitorChildTermination(child) {
  if (!child || typeof child.once !== 'function') {
    throw new Error('readiness requires an owned child process');
  }
  if (childHasExited(child)) {
    return {
      cleanup() {},
      promise: Promise.resolve({
        code: child.exitCode,
        kind: 'exit',
        signal: child.signalCode,
      }),
    };
  }

  let onError;
  let onExit;
  const promise = new Promise((resolve) => {
    onExit = (code, signal) => resolve({ code, kind: 'exit', signal });
    onError = (error) => resolve({ error, kind: 'error' });
    child.once('exit', onExit);
    child.once('error', onError);
  });
  return {
    cleanup() {
      child.removeListener('exit', onExit);
      child.removeListener('error', onError);
    },
    promise,
  };
}

function readinessTimeoutError(url, lastResult) {
  if (lastResult?.kind === 'unexpected-response') {
    return new Error(`${url} returned HTTP 200 but did not expose the expected application response`);
  }
  if (lastResult?.kind === 'http-status') {
    return new Error(`expected HTTP 200 from ${url}, received ${lastResult.statusCode}`);
  }
  return new Error(`timed out waiting for the owned server at ${url}`);
}

function childTerminationError(url, result) {
  if (result.kind === 'error') {
    return new Error(`owned server for ${url} failed before readiness`);
  }
  return new Error(
    `owned server for ${url} exited before readiness with ${formatChildExit(result.code, result.signal)}`,
  );
}

function wait(delayMs) {
  return new Promise((resolve) => setTimeout(() => resolve({ kind: 'delay' }), delayMs));
}

export async function waitForOwnedHttpOk({
  child,
  maxResponseBytes = DEFAULT_HTTP_RESPONSE_LIMIT_BYTES,
  probe = probeHttp,
  requestTimeoutMs = DEFAULT_HTTP_REQUEST_TIMEOUT_MS,
  retryIntervalMs = 250,
  timeoutMs = 30_000,
  url,
  verifyResponse = () => true,
} = {}) {
  const normalizedRetry = normalizePositiveInteger(retryIntervalMs, 'HTTP retry interval');
  const normalizedTimeout = normalizePositiveInteger(timeoutMs, 'HTTP readiness timeout');
  const deadline = Date.now() + normalizedTimeout;
  const childMonitor = monitorChildTermination(child);
  let lastResult;

  try {
    while (Date.now() < deadline) {
      if (childHasExited(child)) {
        throw childTerminationError(url, {
          code: child.exitCode,
          kind: 'exit',
          signal: child.signalCode,
        });
      }
      const remainingMs = Math.max(1, deadline - Date.now());
      const attempt = probe(url, {
        maxResponseBytes,
        requestTimeoutMs: Math.min(requestTimeoutMs, remainingMs),
      }).then(
        (snapshot) => ({ kind: 'probe', snapshot }),
        (error) => ({ error, kind: 'probe-error' }),
      );
      const result = await Promise.race([attempt, childMonitor.promise]);
      if (result.kind === 'exit' || result.kind === 'error') {
        throw childTerminationError(url, result);
      }
      if (result.kind === 'probe' && result.snapshot.statusCode === 200) {
        if (await verifyResponse(result.snapshot)) {
          if (childHasExited(child)) {
            throw childTerminationError(url, {
              code: child.exitCode,
              kind: 'exit',
              signal: child.signalCode,
            });
          }
          return result.snapshot;
        }
        lastResult = { kind: 'unexpected-response' };
      } else if (result.kind === 'probe') {
        lastResult = { kind: 'http-status', statusCode: result.snapshot.statusCode };
      } else {
        lastResult = result;
      }

      const retryDelayMs = Math.min(normalizedRetry, Math.max(1, deadline - Date.now()));
      const retryResult = await Promise.race([wait(retryDelayMs), childMonitor.promise]);
      if (retryResult.kind === 'exit' || retryResult.kind === 'error') {
        throw childTerminationError(url, retryResult);
      }
    }
    throw readinessTimeoutError(url, lastResult);
  } finally {
    childMonitor.cleanup();
  }
}

function waitForChildExit(child, timeoutMs) {
  if (childHasExited(child)) {
    return Promise.resolve(true);
  }
  return new Promise((resolve) => {
    let timer;
    const finish = (exited) => {
      clearTimeout(timer);
      child.removeListener('exit', onExit);
      resolve(exited);
    };
    const onExit = () => finish(true);
    child.once('exit', onExit);
    timer = setTimeout(() => finish(childHasExited(child)), timeoutMs);
  });
}

async function signalAndWait(child, signal, timeoutMs) {
  const exit = waitForChildExit(child, timeoutMs);
  child.kill(signal);
  return exit;
}

function defaultIsProcessGroupAlive(pid, killProcessImpl) {
  try {
    killProcessImpl(-pid, 0);
    return true;
  } catch (error) {
    if (error?.code === 'ESRCH') {
      return false;
    }
    throw error;
  }
}

async function waitForProcessGroupExit(pid, timeoutMs, isProcessGroupAlive) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (!isProcessGroupAlive(pid)) {
      return true;
    }
    await wait(Math.min(20, Math.max(1, deadline - Date.now())));
  }
  return !isProcessGroupAlive(pid);
}

async function signalProcessTreeAndWait(child, signal, timeoutMs, {
  isProcessGroupAlive,
  killProcessImpl,
  processGroup,
}) {
  if (!processGroup) {
    return signalAndWait(child, signal, timeoutMs);
  }
  if (!Number.isSafeInteger(child.pid) || child.pid <= 0) {
    throw new Error('detached Playwright server process is missing a valid process-group id');
  }
  const exit = waitForChildExit(child, timeoutMs);
  try {
    killProcessImpl(-child.pid, signal);
  } catch (error) {
    if (error?.code !== 'ESRCH') {
      throw error;
    }
  }
  const [leaderExited, groupExited] = await Promise.all([
    exit,
    waitForProcessGroupExit(child.pid, timeoutMs, isProcessGroupAlive),
  ]);
  return leaderExited && groupExited;
}

export async function stopServer(child, {
  exitTimeoutMs = 5_000,
  graceMs = 5_000,
  isProcessGroupAliveImpl,
  killProcessImpl = process.kill.bind(process),
  platform = process.platform,
  processGroup = false,
  spawnSyncImpl = spawnSync,
} = {}) {
  if (!child) {
    return;
  }
  if (childHasExited(child) && (!processGroup || platform === 'win32')) {
    return;
  }
  const normalizedExitTimeout = normalizePositiveInteger(exitTimeoutMs, 'server exit timeout');
  const normalizedGrace = normalizePositiveInteger(graceMs, 'server shutdown grace period');

  if (platform === 'win32') {
    if (!Number.isSafeInteger(child.pid) || child.pid <= 0) {
      return;
    }
    const exit = waitForChildExit(child, normalizedExitTimeout);
    const result = spawnSyncImpl(
      'taskkill.exe',
      ['/PID', String(child.pid), '/T', '/F'],
      {
        stdio: 'ignore',
        timeout: normalizedExitTimeout,
        windowsHide: true,
      },
    );
    if (await exit) {
      return;
    }
    if (result?.error) {
      throw new Error('failed to invoke taskkill for the Playwright server process tree');
    }
    throw new Error(`Playwright server process tree ${child.pid} did not exit after taskkill`);
  }

  const isProcessGroupAlive = isProcessGroupAliveImpl
    ?? ((pid) => defaultIsProcessGroupAlive(pid, killProcessImpl));
  if (childHasExited(child) && !isProcessGroupAlive(child.pid)) {
    return;
  }
  if (await signalProcessTreeAndWait(child, 'SIGTERM', normalizedGrace, {
    isProcessGroupAlive,
    killProcessImpl,
    processGroup,
  })) {
    return;
  }
  if (await signalProcessTreeAndWait(child, 'SIGKILL', normalizedExitTimeout, {
    isProcessGroupAlive,
    killProcessImpl,
    processGroup,
  })) {
    return;
  }
  const target = processGroup ? 'process group' : 'process';
  throw new Error(`Playwright server ${target} ${child.pid ?? 'unknown'} did not exit after SIGKILL`);
}

export function createOwnedProcessLifecycle({
  processTarget = process,
  reportCleanupError = (error) => console.error(error),
  stopChild = stopServer,
} = {}) {
  if (typeof processTarget?.on !== 'function' || typeof processTarget?.removeListener !== 'function') {
    throw new Error('owned-process lifecycle requires an event-emitting process target');
  }
  if (typeof stopChild !== 'function') {
    throw new Error('owned-process lifecycle requires a child cleanup function');
  }

  const ownedChildren = new Map();
  const stoppingChildren = new Map();
  const cleanupErrors = [];
  const abortController = new AbortController();
  let cleanupPromise;
  let firstSignal;
  let installed = false;
  let resolveSignal;
  let runStarted = false;
  const signalPromise = new Promise((resolve) => {
    resolveSignal = resolve;
  });

  const rememberCleanupError = (error) => {
    cleanupErrors.push(error instanceof Error ? error : new Error(String(error)));
  };

  const scheduleStop = (child, stopOptions) => {
    const existing = stoppingChildren.get(child);
    if (existing) {
      return existing;
    }
    const stopping = Promise.resolve()
      .then(() => stopChild(child, stopOptions))
      .catch(rememberCleanupError)
      .finally(() => {
        ownedChildren.delete(child);
        stoppingChildren.delete(child);
      });
    stoppingChildren.set(child, stopping);
    return stopping;
  };

  const drainOwnedChildren = async () => {
    do {
      for (const [child, stopOptions] of ownedChildren) {
        scheduleStop(child, stopOptions);
      }
      await Promise.all([...stoppingChildren.values()]);
    } while (ownedChildren.size > 0 || stoppingChildren.size > 0);
    return [...cleanupErrors];
  };

  const cleanup = () => {
    cleanupPromise ??= drainOwnedChildren();
    return cleanupPromise;
  };

  const handleSignal = (signal) => {
    if (firstSignal) {
      return;
    }
    firstSignal = signal;
    processTarget.exitCode = TERMINATION_SIGNAL_EXIT_CODES[signal];
    abortController.abort(signal);
    resolveSignal(signal);
    void cleanup();
  };
  const onSigint = () => handleSignal('SIGINT');
  const onSigterm = () => handleSignal('SIGTERM');

  const install = () => {
    if (installed) {
      return;
    }
    installed = true;
    processTarget.on('SIGINT', onSigint);
    processTarget.on('SIGTERM', onSigterm);
  };

  const dispose = () => {
    if (!installed) {
      return;
    }
    installed = false;
    processTarget.removeListener('SIGINT', onSigint);
    processTarget.removeListener('SIGTERM', onSigterm);
  };

  const reportSignalCleanupErrors = () => {
    for (const error of cleanupErrors) {
      reportCleanupError(error);
    }
  };

  return {
    cleanup,
    get signal() {
      return abortController.signal;
    },
    track(child, stopOptions) {
      if (!child || typeof child.once !== 'function') {
        throw new Error('owned-process lifecycle can only track child processes');
      }
      if (childHasExited(child)) {
        return child;
      }
      ownedChildren.set(child, stopOptions);
      child.once('exit', () => {
        if (!stopOptions?.processGroup) {
          ownedChildren.delete(child);
        }
      });
      if (firstSignal) {
        scheduleStop(child, stopOptions);
      }
      return child;
    },
    async run(work) {
      if (runStarted) {
        throw new Error('owned-process lifecycle can only run once');
      }
      if (typeof work !== 'function') {
        throw new Error('owned-process lifecycle requires a work function');
      }
      runStarted = true;
      install();
      const workOutcome = Promise.resolve()
        .then(() => work({ signal: abortController.signal }))
        .then(
          (value) => ({ kind: 'success', value }),
          (error) => ({ error, kind: 'failure' }),
        );

      try {
        const outcome = await Promise.race([
          workOutcome,
          signalPromise.then((signal) => ({ kind: 'signal', signal })),
        ]);
        await cleanup();
        if (outcome.kind === 'signal' || firstSignal) {
          await workOutcome;
          await drainOwnedChildren();
          reportSignalCleanupErrors();
          return undefined;
        }
        if (cleanupErrors.length > 0) {
          const failures = outcome.kind === 'failure'
            ? [outcome.error, ...cleanupErrors]
            : cleanupErrors;
          throw new AggregateError(
            failures,
            outcome.kind === 'failure'
              ? 'Playwright work and owned-process cleanup both failed'
              : 'failed to clean one or more owned child processes',
          );
        }
        if (outcome.kind === 'failure') {
          throw outcome.error;
        }
        return outcome.value;
      } finally {
        dispose();
      }
    },
  };
}
