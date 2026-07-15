import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

function isUnlockWaitError(error) {
  const code = error?.code;
  return code === 'EBUSY' || code === 'EPERM' || code === 'EACCES';
}

/**
 * Wait until a dev gateway executable is not locked by a stale process.
 * Missing executables are treated as unlocked so first-time builds can proceed.
 */
export async function waitForDevGatewayExecutableUnlock({
  executablePath,
  timeoutMs = 15_000,
  pollMs = 250,
} = {}) {
  if (!executablePath) {
    return { unlocked: true, waitedMs: 0 };
  }

  const startedAt = Date.now();
  const deadline = startedAt + timeoutMs;

  while (Date.now() < deadline) {
    try {
      const handle = await fs.open(executablePath, 'r+');
      await handle.close();
      return { unlocked: true, waitedMs: Date.now() - startedAt };
    } catch (error) {
      if (error?.code === 'ENOENT') {
        return { unlocked: true, waitedMs: Date.now() - startedAt };
      }
      if (!isUnlockWaitError(error)) {
        throw error;
      }
      await sleep(pollMs);
    }
  }

  throw new Error(
    `timed out after ${timeoutMs}ms waiting for dev gateway executable unlock: ${executablePath}`,
  );
}

export function resolveStandaloneGatewayDevExecutable({
  env = process.env,
  repoRoot,
  profile = 'debug',
} = {}) {
  const targetDir = resolveStandaloneGatewayDevTargetDir({ env, repoRoot });
  const executableName = process.platform === 'win32'
    ? 'sdkwork-im-standalone-gateway.exe'
    : 'sdkwork-im-standalone-gateway';
  return path.join(targetDir, profile, executableName);
}

export function resolveStandaloneGatewayDevTargetDir({
  env = process.env,
  repoRoot,
} = {}) {
  if (!repoRoot) {
    throw new Error('repoRoot is required for standalone gateway target resolution');
  }
  const configuredTargetDir = String(env.CARGO_TARGET_DIR ?? '').trim();
  return configuredTargetDir
    ? path.resolve(repoRoot, configuredTargetDir)
    : path.join(repoRoot, '.runtime', 'cargo-target', 'sdkwork-im-standalone-gateway-dev');
}
