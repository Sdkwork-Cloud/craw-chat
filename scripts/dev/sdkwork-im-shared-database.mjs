import fs from 'node:fs';
import os from 'node:os';
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

function userPrivateChatRoot() {
  return path.join(os.homedir(), '.sdkwork', APP_CODE);
}

function defaultSharedSqliteUrl() {
  const sqlitePath = path.join(userPrivateChatRoot(), 'data', `${APP_CODE}.sqlite`);
  return `sqlite://${sqlitePath.replaceAll('\\', '/')}`;
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

function resolveConfiguredSqliteUrl(env) {
  assertNoCanonicalLegacyAliases(env);
  const engine = envValue(
    env,
    'SDKWORK_IM_DATABASE_ENGINE',
    'SDKWORK_CLAW_DATABASE_ENGINE',
    'SDKWORK_CLAW_DATABASE_PROVIDER',
  );
  const deploymentMode = normalizeDatabaseField(env.SDKWORK_IM_DEPLOYMENT_MODE)
    ?? normalizeDatabaseField(env.SDKWORK_CLAW_DEPLOYMENT_MODE);
  if (engine && engine.toLowerCase() !== 'sqlite') {
    return undefined;
  }
  if (engine?.toLowerCase() === 'sqlite' || deploymentMode?.toLowerCase() === 'desktop') {
    const sqliteFile = envValue(
      env,
      'SDKWORK_IM_DATABASE_FILE',
      'SDKWORK_CLAW_DATABASE_FILE',
    );
    if (sqliteFile) {
      return `sqlite://${path.resolve(sqliteFile).replaceAll('\\', '/')}`;
    }
    return defaultSharedSqliteUrl();
  }
  return undefined;
}

const AGENTS_DATABASE_ENV_KEYS = [
  'SDKWORK_AGENTS_DATABASE_URL',
  'SDKWORK_AGENTS_STORE_DATABASE_URL',
  'SDKWORK_AGENT_SERVER_DATABASE_URL',
];

const COMMERCE_T1_DATABASE_PREFIXES = [
  { prefix: 'SDKWORK_ACCOUNT', sqliteFile: 'account.sqlite' },
  { prefix: 'SDKWORK_CATALOG', sqliteFile: 'catalog.sqlite' },
  { prefix: 'SDKWORK_INVENTORY', sqliteFile: 'inventory.sqlite' },
  { prefix: 'SDKWORK_INVOICE', sqliteFile: 'invoice.sqlite' },
  { prefix: 'SDKWORK_MEMBERSHIP', sqliteFile: 'membership.sqlite' },
  { prefix: 'SDKWORK_MERCHANDISE', sqliteFile: 'merchandise.sqlite' },
  { prefix: 'SDKWORK_ORDER', sqliteFile: 'order.sqlite' },
  { prefix: 'SDKWORK_PAYMENT', sqliteFile: 'payment.sqlite' },
  { prefix: 'SDKWORK_PROMOTION', sqliteFile: 'promotion.sqlite' },
  { prefix: 'SDKWORK_SHOP', sqliteFile: 'shop.sqlite' },
];

function databaseBridgeEnv({
  databaseUrl,
  env,
  engine,
  maxConnections,
}) {
  const resolvedMaxConnections = maxConnections
    ?? envValue(env, 'SDKWORK_IM_DATABASE_MAX_CONNECTIONS', 'SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS');
  const bridged = {
    SDKWORK_IM_DATABASE_ENGINE: engine,
    SDKWORK_IM_DATABASE_URL: databaseUrl,
    SDKWORK_CLAW_DATABASE_URL: databaseUrl,
    ...(resolvedMaxConnections
      ? {
        SDKWORK_IM_DATABASE_MAX_CONNECTIONS: resolvedMaxConnections,
        SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS: resolvedMaxConnections,
      }
      : {}),
  };
    if (/^postgres(?:ql)?:\/\//iu.test(databaseUrl)) {
    bridged.SDKWORK_IAM_DATABASE_URL = databaseUrl;
    bridged.SDKWORK_DATABASE_URL = databaseUrl;
    bridged.SDKWORK_DRIVE_DATABASE_URL = databaseUrl;
    bridged.SDKWORK_KNOWLEDGEBASE_DATABASE_URL = databaseUrl;
    for (const key of AGENTS_DATABASE_ENV_KEYS) {
      bridged[key] = databaseUrl;
    }
    for (const module of COMMERCE_T1_DATABASE_PREFIXES) {
      bridged[`${module.prefix}_DATABASE_URL`] = databaseUrl;
    }
    bridged.SDKWORK_MAIL_DATABASE_URL = databaseUrl;
    bridged.SDKWORK_NOTARY_DATABASE_URL = databaseUrl;
  }
  if (/^sqlite:\/\//iu.test(databaseUrl)) {
    const sqlitePath = databaseUrl.replace(/^sqlite:\/\//iu, '');
    const absoluteSqlitePath = path.resolve(sqlitePath);
    const dataDir = path.dirname(absoluteSqlitePath);
    const driveSqlitePath = path.join(dataDir, 'drive.sqlite').replaceAll('\\', '/');
    const knowledgebaseSqlitePath = path.join(dataDir, 'knowledgebase.db').replaceAll('\\', '/');
    const mailSqlitePath = path.join(dataDir, 'mail.sqlite').replaceAll('\\', '/');
    const notarySqlitePath = path.join(dataDir, 'notary.sqlite').replaceAll('\\', '/');
    bridged.SDKWORK_DRIVE_DATABASE_ENGINE = 'sqlite';
    bridged.SDKWORK_DRIVE_DATABASE_SQLITE_URL = `sqlite://${driveSqlitePath}`;
    bridged.SDKWORK_DRIVE_DATABASE_URL = `sqlite://${driveSqlitePath}`;
    bridged.SDKWORK_KNOWLEDGEBASE_DATABASE_URL = `sqlite://${knowledgebaseSqlitePath}?mode=rwc`;
    for (const key of AGENTS_DATABASE_ENV_KEYS) {
      bridged[key] = databaseUrl;
    }
    for (const module of COMMERCE_T1_DATABASE_PREFIXES) {
      const sqlitePath = path.join(dataDir, module.sqliteFile).replaceAll('\\', '/');
      bridged[`${module.prefix}_DATABASE_URL`] = `sqlite://${sqlitePath}`;
    }
    bridged.SDKWORK_MAIL_DATABASE_URL = `sqlite://${mailSqlitePath}`;
    bridged.SDKWORK_NOTARY_DATABASE_URL = `sqlite://${notarySqlitePath}`;
  }
  return bridged;
}

export function resolveSdkworkImSharedDatabaseConfig({
  env = process.env,
  repoRoot: _root = repoRoot,
} = {}) {
  assertNoCanonicalLegacyAliases(env);
  const databaseUrl = normalizeDatabaseUrl(env.SDKWORK_IM_DATABASE_URL)
    ?? normalizeDatabaseUrl(env.SDKWORK_CLAW_DATABASE_URL)
    ?? resolveConfiguredSqliteUrl(env)
    ?? resolvePostgresDatabaseUrlFromFields(env)
    ?? defaultSharedSqliteUrl();

  if (/^sqlite:\/\//iu.test(databaseUrl)) {
    const sqlitePath = databaseUrl.replace(/^sqlite:\/\//iu, '');
    const absoluteSqlitePath = path.resolve(sqlitePath);
    fs.mkdirSync(path.dirname(absoluteSqlitePath), { recursive: true });
    return {
      databaseUrl,
      env: databaseBridgeEnv({
        databaseUrl,
        engine: 'sqlite',
        env,
        maxConnections: envValue(
          env,
          'SDKWORK_IM_DATABASE_MAX_CONNECTIONS',
          'SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS',
        ) ?? '1',
      }),
      kind: 'sqlite',
    };
  }

  if (/^postgres(?:ql)?:\/\//iu.test(databaseUrl)) {
    const parsed = new URL(databaseUrl);
    return {
      databaseUrl,
      env: databaseBridgeEnv({
        databaseUrl,
        engine: 'postgresql',
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

  return {
    databaseUrl,
    env: databaseBridgeEnv({
      databaseUrl,
      engine: 'custom',
      env,
    }),
    kind: 'custom',
  };
}
