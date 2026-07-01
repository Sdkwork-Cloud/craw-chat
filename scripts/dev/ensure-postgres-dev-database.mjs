import process from 'node:process';

import { ensurePostgresDevDatabaseReady as ensureSharedPostgresDevDatabaseReady } from '../../../sdkwork-specs/tools/postgres/ensure-postgres-dev-database.mjs';

import { runPostgresDbCli } from './sdkwork-im-postgres-db.mjs';
import {
  isPostgresDevProfile,
  resolvePostgresDevProfile,
} from './sdkwork-im-postgres-dev-profile.mjs';
import { terminateStaleDevGatewayProcesses } from './terminate-stale-dev-gateway-processes.mjs';

export { isPostgresDevProfile, resolvePostgresDevProfile };

export async function ensurePostgresDevDatabaseReady({
  env = process.env,
  repoRoot,
  stdout = process.stdout,
  stderr = process.stderr,
} = {}) {
  if (isPostgresDevProfile(env)) {
    terminateStaleDevGatewayProcesses({ stdout });
  }

  return ensureSharedPostgresDevDatabaseReady({
    env,
    legacyDatabasePrefixes: ['SDKWORK_IM_DATABASE_'],
    repoRoot,
    runMigrations: async (profile) => {
      stdout.write('[sdkwork-im-db] applying IM database migrations via sdkwork-database-cli\n');
      return runPostgresDbCli({
        argv: ['--mode', 'migrate', '--config', '.env.postgres'],
        env: profile.env,
        repoRoot,
        stdout,
        stderr,
      });
    },
    stderr,
    stdout,
  });
}
