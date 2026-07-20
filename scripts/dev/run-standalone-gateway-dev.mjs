#!/usr/bin/env node

import { spawn, spawnSync } from 'node:child_process';
import process from 'node:process';

import { terminateStaleDevGatewayProcesses } from './terminate-stale-dev-gateway-processes.mjs';
import {
  resolveStandaloneGatewayDevExecutable,
  resolveStandaloneGatewayDevTargetDir,
  waitForDevGatewayExecutableUnlock,
} from './wait-for-dev-gateway-exe-unlock.mjs';

function cargoCommand() {
  return process.platform === 'win32' ? 'cargo.exe' : 'cargo';
}

function parseArgs(argv) {
  return {
    release: argv.includes('--release'),
  };
}

async function main() {
  const { release } = parseArgs(process.argv.slice(2));
  const repoRoot = process.cwd();
  const profile = release ? 'release' : 'debug';
  const targetDir = resolveStandaloneGatewayDevTargetDir({
    env: process.env,
    repoRoot,
  });
  const gatewayEnv = {
    ...process.env,
    CARGO_TARGET_DIR: targetDir,
  };
  const executablePath = resolveStandaloneGatewayDevExecutable({
    env: gatewayEnv,
    repoRoot,
    profile,
  });

  terminateStaleDevGatewayProcesses({ stdout: process.stdout });
  const unlock = await waitForDevGatewayExecutableUnlock({ executablePath });
  if (unlock.waitedMs > 0) {
    process.stdout.write(
      `[sdkwork-api-im-standalone-gateway] waited ${unlock.waitedMs}ms for executable unlock\n`,
    );
  }

  const cargoArgs = [
    'build',
    '-p',
    'sdkwork-api-im-standalone-gateway',
    '--bin',
    'sdkwork-api-im-standalone-gateway',
  ];
  if (release) {
    cargoArgs.push('--release');
  }

  const build = spawnSync(cargoCommand(), cargoArgs, {
    cwd: repoRoot,
    env: gatewayEnv,
    stdio: 'inherit',
    shell: false,
  });
  if (build.status !== 0) {
    process.exit(build.status ?? 1);
  }

  const gateway = spawn(executablePath, [], {
    cwd: repoRoot,
    env: gatewayEnv,
    stdio: 'inherit',
    shell: false,
  });

  gateway.on('error', (error) => {
    process.stderr.write(
      `[sdkwork-api-im-standalone-gateway] ${error instanceof Error ? error.message : String(error)}\n`,
    );
    process.exit(1);
  });
  gateway.on('exit', (code, signal) => {
    if (signal) {
      process.exit(1);
    }
    process.exit(code ?? 0);
  });
}

main().catch((error) => {
  process.stderr.write(
    `[sdkwork-api-im-standalone-gateway] ${error instanceof Error ? error.message : String(error)}\n`,
  );
  process.exit(1);
});
