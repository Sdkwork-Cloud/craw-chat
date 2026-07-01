import { spawnSync } from 'node:child_process';
import process from 'node:process';

export const STALE_DEV_GATEWAY_PROCESS_NAMES = [
  'sdkwork-im-standalone-gateway.exe',
  'sdkwork-clawrouter-standalone-gateway.exe',
];

/**
 * Terminate stale Windows standalone gateway processes that keep PostgreSQL
 * connections open and block sdkwork-database bootstrap during dev startup.
 */
export function terminateStaleDevGatewayProcesses({
  platform = process.platform,
  processNames = STALE_DEV_GATEWAY_PROCESS_NAMES,
  spawnSyncImpl = spawnSync,
  stdout = process.stdout,
} = {}) {
  if (platform !== 'win32') {
    return { terminated: [] };
  }

  const terminated = [];
  for (const imageName of processNames) {
    const result = spawnSyncImpl('taskkill.exe', ['/F', '/IM', imageName], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    if (result.status === 0) {
      terminated.push(imageName);
      stdout.write(`[sdkwork-postgres] terminated stale ${imageName} process(es)\n`);
    }
  }
  return { terminated };
}
