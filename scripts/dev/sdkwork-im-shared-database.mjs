import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');
const APP_CODE = 'chat';
const CANONICAL_DATABASE_PREFIX = 'SDKWORK_IM_DATABASE_';
const LEGACY_DATABASE_PREFIX = 'SDKWORK_CLAW_DATABASE_';

function normalizeDatabaseUrl(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function normalizeDatabaseField(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function appendPostgresQueryParam(params, name, value) {
  const normalized = normalizeDatabaseField(value);
  if (normalized) {
    params.set(name, normalized);
  }
}

function encodePostgresPath(databaseName) {
  return encodeURIComponent(databaseName).replaceAll('%2F', '/');
}

function envValue(env, canonicalName, ...legacyNames) {
  const canonical = normalizeDatabaseField(env[canonicalName]);
  if (canonical) {
    return canonical;
  }
  for (const legacyName of legacyNames) {
    const legacy = normalizeDatabaseField(env[legacyName]);
    if (legacy) {
      return legacy;
    }
  }
  return undefined;
}

function assertNoCanonicalLegacyAliases(env) {
  if (normalizeDatabaseField(env.SDKWORK_IM_DATABASE_PROVIDER)) {
    throw new Error(
      'SDKWORK_IM_DATABASE_PROVIDER is not standard; use SDKWORK_IM_DATABASE_ENGINE',
    );
  }
  if (normalizeDatabaseField(env.SDKWORK_IM_DATABASE_SSLMODE)) {
    throw new Error(
      'SDKWORK_IM_DATABASE_SSLMODE is not standard; use SDKWORK_IM_DATABASE_SSL_MODE',
    );
  }
}

function resolvePostgresDatabaseUrlFromFields(env) {
  assertNoCanonicalLegacyAliases(env);
  const engine = envValue(
    env,
    'SDKWORK_IM_DATABASE_ENGINE',
    'SDKWORK_CLAW_DATABASE_ENGINE',
    'SDKWORK_CLAW_DATABASE_PROVIDER',
  );
  if (!engine) {
    return undefined;
  }
  if (!/^postgres(?:ql)?$/iu.test(engine)) {
    throw new Error(`unsupported Sdkwork IM database engine: ${engine}`);
  }

  const host = envValue(env, 'SDKWORK_IM_DATABASE_HOST', 'SDKWORK_CLAW_DATABASE_HOST');
  const database = envValue(env, 'SDKWORK_IM_DATABASE_NAME', 'SDKWORK_CLAW_DATABASE_NAME');
  const username = envValue(
    env,
    'SDKWORK_IM_DATABASE_USERNAME',
    'SDKWORK_CLAW_DATABASE_USERNAME',
  );
  const password = envValue(
    env,
    'SDKWORK_IM_DATABASE_PASSWORD',
    'SDKWORK_CLAW_DATABASE_PASSWORD',
  );
  const missing = [];
  if (!host) {
    missing.push('SDKWORK_IM_DATABASE_HOST');
  }
  if (!database) {
    missing.push('SDKWORK_IM_DATABASE_NAME');
  }
  if (!username) {
    missing.push('SDKWORK_IM_DATABASE_USERNAME');
  }
  if (!password) {
    missing.push('SDKWORK_IM_DATABASE_PASSWORD');
  }
  if (missing.length > 0) {
    throw new Error(
      `SDKWORK_IM_DATABASE_ENGINE=postgresql requires ${missing.join(', ')}`,
    );
  }

  const port = envValue(env, 'SDKWORK_IM_DATABASE_PORT', 'SDKWORK_CLAW_DATABASE_PORT');
  const credentials = `${encodeURIComponent(username)}${password ? `:${encodeURIComponent(password)}` : ''}`;
  const authority = `${credentials}@${host}${port ? `:${port}` : ''}`;
  const params = new URLSearchParams();
  appendPostgresQueryParam(
    params,
    'sslmode',
    envValue(
      env,
      'SDKWORK_IM_DATABASE_SSL_MODE',
      'SDKWORK_CLAW_DATABASE_SSL_MODE',
      'SDKWORK_CLAW_DATABASE_SSLMODE',
    ),
  );
  const query = params.toString();
  return `postgresql://${authority}/${encodePostgresPath(database)}${query ? `?${query}` : ''}`;
}

const AGENTS_DATABASE_ENV_KEYS = [
  'SDKWORK_AGENTS_DATABASE_URL',
  'SDKWORK_AGENTS_STORE_DATABASE_URL',
  'SDKWORK_AGENT_SERVER_DATABASE_URL',
];

const COMMERCE_T1_DATABASE_PREFIXES = [
  'SDKWORK_ACCOUNT',
  'SDKWORK_CATALOG',
  'SDKWORK_INVENTORY',
  'SDKWORK_INVOICE',
  'SDKWORK_MEMBERSHIP',
  'SDKWORK_MERCHANDISE',
  'SDKWORK_ORDER',
  'SDKWORK_PAYMENT',
  'SDKWORK_PROMOTION',
  'SDKWORK_SHOP',
];

function databaseBridgeEnv({
  databaseUrl,
  env,
  maxConnections,
}) {
  const resolvedMaxConnections = maxConnections
    ?? envValue(env, 'SDKWORK_IM_DATABASE_MAX_CONNECTIONS', 'SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS');
  const bridged = {
    SDKWORK_IM_DATABASE_ENGINE: 'postgresql',
    SDKWORK_IM_DATABASE_URL: databaseUrl,
    SDKWORK_CLAW_DATABASE_URL: databaseUrl,
    ...(resolvedMaxConnections
      ? {
        SDKWORK_IM_DATABASE_MAX_CONNECTIONS: resolvedMaxConnections,
        SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS: resolvedMaxConnections,
      }
      : {}),
  };
  bridged.SDKWORK_IAM_DATABASE_URL = databaseUrl;
  bridged.SDKWORK_DATABASE_URL = databaseUrl;
  bridged.SDKWORK_DRIVE_DATABASE_URL = databaseUrl;
  bridged.SDKWORK_KNOWLEDGEBASE_DATABASE_URL = databaseUrl;
  for (const key of AGENTS_DATABASE_ENV_KEYS) {
    bridged[key] = databaseUrl;
  }
  for (const prefix of COMMERCE_T1_DATABASE_PREFIXES) {
    bridged[`${prefix}_DATABASE_URL`] = databaseUrl;
  }
  bridged.SDKWORK_MAIL_DATABASE_URL = databaseUrl;
  bridged.SDKWORK_NOTARY_DATABASE_URL = databaseUrl;
  return bridged;
}

export function resolveSdkworkImSharedDatabaseConfig({
  env = process.env,
  repoRoot: _root = repoRoot,
} = {}) {
  assertNoCanonicalLegacyAliases(env);
  const databaseUrl = normalizeDatabaseUrl(env.SDKWORK_IM_DATABASE_URL)
    ?? normalizeDatabaseUrl(env.SDKWORK_CLAW_DATABASE_URL)
    ?? resolvePostgresDatabaseUrlFromFields(env);

  if (!databaseUrl) {
    throw new Error(
      'SDKWORK_IM_DATABASE_URL or SDKWORK_IM_DATABASE_ENGINE=postgresql configuration is required; IM SQLite default has been removed',
    );
  }

  if (/^postgres(?:ql)?:\/\//iu.test(databaseUrl)) {
    const parsed = new URL(databaseUrl);
    return {
      databaseUrl,
      env: databaseBridgeEnv({
        databaseUrl,
        env,
      }),
      kind: 'postgresql',
      postgres: {
        database: parsed.pathname.replace(/^\//u, ''),
        host: parsed.hostname,
        password: decodeURIComponent(parsed.password || ''),
        port: parsed.port,
        sslmode: parsed.searchParams.get('sslmode') ?? undefined,
        username: decodeURIComponent(parsed.username || ''),
      },
    };
  }

  throw new Error(`unsupported Sdkwork IM database URL; PostgreSQL is required: ${databaseUrl}`);
}
