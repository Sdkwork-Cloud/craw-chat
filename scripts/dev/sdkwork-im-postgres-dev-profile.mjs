import process from 'node:process';

import {
  ensurePostgresDevEnvFile,
  mergePostgresDevRuntimeEnv,
  POSTGRES_DEV_ENV_EXAMPLE_FILENAME,
  POSTGRES_DEV_ENV_FILENAME,
  readPostgresDevFileEnv,
  resolvePostgresDevEnvExamplePath,
  resolvePostgresDevEnvFilePath,
} from '../../../sdkwork-specs/tools/postgres/postgres-dev-profile.mjs';

import { resolveSdkworkImSharedDatabaseConfig } from './sdkwork-im-shared-database.mjs';
import { parsePostgresConfig } from './sdkwork-im-postgres-db.mjs';

export {
  POSTGRES_DEV_ENV_EXAMPLE_FILENAME,
  POSTGRES_DEV_ENV_FILENAME,
  ensurePostgresDevEnvFile,
  mergePostgresDevRuntimeEnv,
  readPostgresDevFileEnv,
  resolvePostgresDevEnvExamplePath,
  resolvePostgresDevEnvFilePath,
};

const DATABASE_ENV_PREFIXES = [
  'SDKWORK_IM_DATABASE_',
  'SDKWORK_CLAW_DATABASE_',
  'SDKWORK_IAM_DATABASE_',
  'SDKWORK_DATABASE_',
];

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function isDatabaseEnvKey(key) {
  return DATABASE_ENV_PREFIXES.some((prefix) => key.startsWith(prefix));
}

export function resolvePostgresDevProfile({
  env = process.env,
  extraEnv = {},
  repoRoot,
} = {}) {
  if (!repoRoot) {
    throw new Error('resolvePostgresDevProfile requires repoRoot');
  }
  const { configPath, fileEnv } = readPostgresDevFileEnv(repoRoot);
  const mergedEnv = mergePostgresDevRuntimeEnv({ env, fileEnv, extraEnv });
  const sharedDatabase = resolveSdkworkImSharedDatabaseConfig({
    env: mergedEnv,
    repoRoot,
  });
  const config = parsePostgresConfig({
    configPath,
    repoRoot,
  });
  return {
    config,
    configPath,
    databaseUrl: sharedDatabase.databaseUrl,
    env: {
      ...mergedEnv,
      ...sharedDatabase.env,
      SDKWORK_IM_STORAGE_PROVIDER: mergedEnv.SDKWORK_IM_STORAGE_PROVIDER ?? 'postgresql',
    },
    fileEnv,
    kind: sharedDatabase.kind,
    postgres: sharedDatabase.postgres,
  };
}

export function isPostgresDevProfile(env = process.env) {
  const engine = normalizeText(env.SDKWORK_IM_DATABASE_ENGINE)
    ?? normalizeText(env.SDKWORK_CLAW_DATABASE_ENGINE)
    ?? normalizeText(env.SDKWORK_CLAW_DATABASE_PROVIDER);
  if (!engine) {
    return true;
  }
  return /^postgres(?:ql)?$/iu.test(engine);
}
