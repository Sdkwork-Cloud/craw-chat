import path from 'node:path';
import process from 'node:process';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import {
  isPostgresDevProfile,
  resolvePostgresDevProfile,
} from './sdkwork-im-postgres-dev-profile.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..', '..');
const databaseManifestPath = path.join(repoRoot, '../sdkwork-database/Cargo.toml');

function parseForwardedArgs(argv) {
  const separatorIndex = argv.indexOf('--');
  if (separatorIndex === -1) {
    return argv;
  }
  return argv.slice(separatorIndex + 1);
}

function resolveMergedEnv(env = process.env) {
  if (!isPostgresDevProfile(env)) {
    return env;
  }
  try {
    const profile = resolvePostgresDevProfile({ env, repoRoot });
    return {
      ...env,
      ...profile.env,
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    process.stderr.write(
      `[sdkwork-im-db] warning: failed to merge .env.postgres profile: ${message}\n`,
    );
    return env;
  }
}

function runDatabaseCli(forwardedArgs, env = process.env) {
  const args = [
    'run',
    '--manifest-path',
    databaseManifestPath,
    '-p',
    'sdkwork-database-cli',
    '--',
    '--app-root',
    repoRoot,
    ...forwardedArgs,
  ];
  const spawned = spawnSync('cargo', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    env,
    shell: process.platform === 'win32',
    stdio: 'inherit',
  });
  if (spawned.error) {
    throw spawned.error;
  }
  if (spawned.status !== 0) {
    process.exit(spawned.status ?? 1);
  }
}

export function runSdkworkImDatabaseCli({
  argv = process.argv.slice(2),
  env = process.env,
} = {}) {
  const forwardedArgs = parseForwardedArgs(argv);
  if (forwardedArgs.length === 0) {
    throw new Error('sdkwork-im-database-cli requires a sdkwork-database-cli subcommand');
  }
  runDatabaseCli(forwardedArgs, resolveMergedEnv(env));
}

if (path.resolve(process.argv[1] ?? '') === __filename) {
  try {
    runSdkworkImDatabaseCli();
  } catch (error) {
    process.stderr.write(
      `[sdkwork-im-db] ${error instanceof Error ? error.message : String(error)}\n`,
    );
    process.exitCode = 1;
  }
}
