import process from 'node:process';

const WORKSPACE_DATABASE_PREFIX = 'SDKWORK_DATABASE_';
const WORKSPACE_DATABASE_GOVERNANCE_KEYS = [
  'SDKWORK_DATABASE_MAX_CONNECTIONS',
  'SDKWORK_DATABASE_MIN_CONNECTIONS',
  'SDKWORK_DATABASE_ACQUIRE_TIMEOUT',
  'SDKWORK_DATABASE_IDLE_TIMEOUT',
  'SDKWORK_DATABASE_MAX_LIFETIME',
  'SDKWORK_DATABASE_AUTO_MIGRATE',
  'SDKWORK_DATABASE_AUTO_SEED',
  'SDKWORK_DATABASE_TEMPORARY_ANY_POOL_EXCEPTION',
  'SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT',
];

function normalizeDatabaseField(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function encodePostgresPath(databaseName) {
  return encodeURIComponent(databaseName).replaceAll('%2F', '/');
}

export function rejectRetiredDatabaseKeys(env) {
  const retiredKeys = Object.keys(env).filter((key) => (
    /^SDKWORK_(?!DATABASE_)[A-Z0-9_]+_DATABASE_[A-Z0-9_]+$/u.test(key)
    || /^(?:DATABASE_URL|DATABASE_PROVIDER|DATABASE_SSLMODE)$/u.test(key)
    || /^SDKWORK_DATABASE_(?:PROVIDER|SSLMODE)$/u.test(key)
  ));
  if (retiredKeys.length > 0) {
    throw new Error(
      `retired database configuration ${retiredKeys.sort().join(', ')}; use ${WORKSPACE_DATABASE_PREFIX}*`,
    );
  }
}

function canonicalDatabaseProfile(database) {
  if (database === 'sdkwork_ai_dev') {
    return { environment: 'development', username: 'sdkwork_ai_dev' };
  }
  if (database === 'sdkwork_ai_test') {
    return { environment: 'test', username: 'sdkwork_ai_test' };
  }
  if (/^sdkwork_ai_test_[A-Za-z0-9_]+$/u.test(database)) {
    return { environment: 'test', username: 'sdkwork_ai_test' };
  }
  if (database === 'sdkwork_ai_staging') {
    return { environment: 'staging', username: 'sdkwork_ai_staging' };
  }
  if (database === 'sdkwork_ai_prod') {
    return { environment: 'production', username: 'sdkwork_ai_prod' };
  }
  return undefined;
}

export function validateWorkspacePostgresIdentity({ database, schema, username }) {
  const profile = canonicalDatabaseProfile(database);
  if (!profile) {
    throw new Error(
      `PostgreSQL database ${JSON.stringify(database)} is not a canonical SDKWork workspace identity`,
    );
  }
  if (schema !== database) {
    throw new Error(
      `SDKWORK_DATABASE_SCHEMA must equal workspace database ${JSON.stringify(database)}, got ${JSON.stringify(schema)}`,
    );
  }
  if (username !== profile.username) {
    throw new Error(
      `${profile.environment} database ${JSON.stringify(database)} requires username ${JSON.stringify(profile.username)}, got ${JSON.stringify(username)}`,
    );
  }
}

function resolvePostgresDatabaseUrlFromFields(env) {
  const engine = normalizeDatabaseField(env.SDKWORK_DATABASE_ENGINE);
  if (!engine) {
    return undefined;
  }
  if (!/^postgres(?:ql)?$/iu.test(engine)) {
    throw new Error(`unsupported SDKWork database engine: ${engine}`);
  }

  const host = normalizeDatabaseField(env.SDKWORK_DATABASE_HOST);
  const database = normalizeDatabaseField(env.SDKWORK_DATABASE_NAME);
  const schema = normalizeDatabaseField(env.SDKWORK_DATABASE_SCHEMA) ?? database;
  const username = normalizeDatabaseField(env.SDKWORK_DATABASE_USERNAME);
  const password = normalizeDatabaseField(env.SDKWORK_DATABASE_PASSWORD);
  const missing = [];
  for (const [key, value] of [
    ['SDKWORK_DATABASE_HOST', host],
    ['SDKWORK_DATABASE_NAME', database],
    ['SDKWORK_DATABASE_USERNAME', username],
    ['SDKWORK_DATABASE_PASSWORD', password],
  ]) {
    if (!value) {
      missing.push(key);
    }
  }
  if (missing.length > 0) {
    throw new Error(
      `SDKWORK_DATABASE_ENGINE=postgresql requires ${missing.join(', ')}`,
    );
  }
  validateWorkspacePostgresIdentity({ database, schema, username });

  const port = normalizeDatabaseField(env.SDKWORK_DATABASE_PORT);
  const credentials = `${encodeURIComponent(username)}:${encodeURIComponent(password)}`;
  const authority = `${credentials}@${host}${port ? `:${port}` : ''}`;
  const params = new URLSearchParams();
  const sslMode = normalizeDatabaseField(env.SDKWORK_DATABASE_SSL_MODE);
  if (sslMode) {
    params.set('sslmode', sslMode);
  }
  const query = params.toString();
  return `postgresql://${authority}/${encodePostgresPath(database)}${query ? `?${query}` : ''}`;
}

function parseWorkspacePostgresUrl(databaseUrl, env) {
  if (!/^postgres(?:ql)?:\/\//iu.test(databaseUrl)) {
    throw new Error(`unsupported SDKWork database URL; PostgreSQL is required: ${databaseUrl}`);
  }
  const parsed = new URL(databaseUrl);
  const database = decodeURIComponent(parsed.pathname.replace(/^\//u, ''));
  const schema = normalizeDatabaseField(env.SDKWORK_DATABASE_SCHEMA) ?? database;
  const username = decodeURIComponent(parsed.username || '');
  validateWorkspacePostgresIdentity({ database, schema, username });
  return {
    database,
    host: parsed.hostname,
    password: decodeURIComponent(parsed.password || ''),
    port: parsed.port,
    schema,
    sslmode: parsed.searchParams.get('sslmode') ?? undefined,
    username,
  };
}

function workspaceRuntimeDatabaseEnv(databaseUrl, postgres, env) {
  const runtimeEnv = {
    SDKWORK_DATABASE_ENGINE: 'postgresql',
    SDKWORK_DATABASE_URL: databaseUrl,
    SDKWORK_DATABASE_SCHEMA: postgres.schema,
  };
  for (const key of WORKSPACE_DATABASE_GOVERNANCE_KEYS) {
    const value = normalizeDatabaseField(env[key]);
    if (value) {
      runtimeEnv[key] = value;
    }
  }
  return runtimeEnv;
}

export function resolveSdkworkImSharedDatabaseConfig({
  env = process.env,
} = {}) {
  rejectRetiredDatabaseKeys(env);
  const databaseUrl = normalizeDatabaseField(env.SDKWORK_DATABASE_URL)
    ?? resolvePostgresDatabaseUrlFromFields(env);

  if (!databaseUrl) {
    throw new Error(
      'SDKWORK_DATABASE_URL or SDKWORK_DATABASE_ENGINE=postgresql configuration is required; server-side SQLite is not supported',
    );
  }

  const postgres = parseWorkspacePostgresUrl(databaseUrl, env);
  return {
    databaseUrl,
    env: workspaceRuntimeDatabaseEnv(databaseUrl, postgres, env),
    kind: 'postgresql',
    postgres,
  };
}
