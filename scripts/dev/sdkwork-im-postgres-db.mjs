import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import {
  initializePostgresRoleAndDatabase,
  initializePostgresSchemaAndGrants,
} from './sdkwork-im-postgres-init-node.mjs';
import { repairPostgresMigrationChecksums } from './repair-postgres-migration-checksums.mjs';
import { resolvePostgresDevProfile } from './sdkwork-im-postgres-dev-profile.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const defaultRepoRoot = path.resolve(__dirname, '..', '..');
const defaultMigrationsDir = path.join(defaultRepoRoot, 'database', 'ddl', 'baseline', 'postgres');
const defaultFrameworkBaselineDir = defaultMigrationsDir;
const defaultAppbaseIamMigrationsDir = path.resolve(
  defaultRepoRoot,
  '..',
  'sdkwork-appbase',
  'packages',
  'native-rust',
  'iam',
  'sdkwork-iam-storage-sqlx-rust',
  'migrations',
);
const CANONICAL_DATABASE_PREFIX = 'SDKWORK_IM_DATABASE_';
const LEGACY_DATABASE_PREFIX = 'SDKWORK_CLAW_DATABASE_';

function normalizeField(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function stripInlineComment(value) {
  let quote = '';
  for (let index = 0; index < value.length; index += 1) {
    const char = value[index];
    if ((char === '"' || char === "'") && value[index - 1] !== '\\') {
      quote = quote === char ? '' : quote || char;
      continue;
    }
    if (char === '#' && !quote && /\s/u.test(value[index - 1] ?? ' ')) {
      return value.slice(0, index).trimEnd();
    }
  }
  return value;
}

function unquoteConfigValue(value) {
  const trimmed = stripInlineComment(String(value ?? '').trim());
  if (
    (trimmed.startsWith('"') && trimmed.endsWith('"'))
    || (trimmed.startsWith("'") && trimmed.endsWith("'"))
  ) {
    const inner = trimmed.slice(1, -1);
    return trimmed.startsWith('"')
      ? inner.replaceAll('\\"', '"').replaceAll('\\\\', '\\')
      : inner.replaceAll("''", "'");
  }
  return trimmed;
}

function parseDotEnv(text) {
  const result = {};
  for (const rawLine of String(text ?? '').split(/\r?\n/u)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    const normalizedLine = line.startsWith('export ') ? line.slice('export '.length).trim() : line;
    const equalsIndex = normalizedLine.indexOf('=');
    if (equalsIndex <= 0) {
      continue;
    }
    const key = normalizedLine.slice(0, equalsIndex).trim();
    const value = normalizedLine.slice(equalsIndex + 1);
    result[key] = unquoteConfigValue(value);
  }
  return result;
}

function parseCommandArgs(value) {
  const normalized = normalizeField(value);
  if (!normalized) {
    return [];
  }
  const args = [];
  let current = '';
  let quote = '';
  for (let index = 0; index < normalized.length; index += 1) {
    const char = normalized[index];
    if (char === '\\' && index + 1 < normalized.length) {
      index += 1;
      current += normalized[index];
      continue;
    }
    if ((char === '"' || char === "'") && !quote) {
      quote = char;
      continue;
    }
    if (char === quote) {
      quote = '';
      continue;
    }
    if (/\s/u.test(char) && !quote) {
      if (current) {
        args.push(current);
        current = '';
      }
      continue;
    }
    current += char;
  }
  if (quote) {
    throw new Error(`unterminated quote in PostgreSQL psql args: ${normalized}`);
  }
  if (current) {
    args.push(current);
  }
  return args;
}

function parseYamlScalar(value) {
  const normalized = unquoteConfigValue(value);
  if (/^(?:null|~)$/iu.test(normalized)) {
    return '';
  }
  return normalized;
}

function parseSimpleYaml(text) {
  const root = {};
  let currentSection;
  for (const rawLine of String(text ?? '').split(/\r?\n/u)) {
    if (!rawLine.trim() || rawLine.trimStart().startsWith('#')) {
      continue;
    }
    const indentLength = rawLine.match(/^\s*/u)?.[0].length ?? 0;
    const line = rawLine.trim();
    const colonIndex = line.indexOf(':');
    if (colonIndex <= 0) {
      continue;
    }
    const key = line.slice(0, colonIndex).trim();
    const value = line.slice(colonIndex + 1).trim();
    if (indentLength === 0) {
      if (!value) {
        root[key] = root[key] && typeof root[key] === 'object' ? root[key] : {};
        currentSection = key;
      } else {
        root[key] = parseYamlScalar(value);
        currentSection = undefined;
      }
      continue;
    }
    if (!currentSection) {
      continue;
    }
    root[currentSection][key] = parseYamlScalar(value);
  }
  return root;
}

function decodePostgresDatabasePath(pathname) {
  return decodeURIComponent(String(pathname ?? '').replace(/^\//u, ''));
}

function parsePostgresDatabaseUrl(value, schema) {
  const normalized = normalizeField(value);
  if (!normalized) {
    return undefined;
  }
  if (!/^postgres(?:ql)?:\/\//iu.test(normalized)) {
    throw new Error(`unsupported PostgreSQL database URL: ${sanitizePostgresDatabaseUrl(normalized)}`);
  }
  const parsed = new URL(normalized);
  return {
    database: decodePostgresDatabasePath(parsed.pathname),
    host: parsed.hostname,
    password: decodeURIComponent(parsed.password || ''),
    port: parsed.port || '5432',
    schema: normalizeField(schema) ?? decodePostgresDatabasePath(parsed.pathname),
    sslmode: parsed.searchParams.get('sslmode') ?? undefined,
    username: decodeURIComponent(parsed.username || ''),
  };
}

function resolveConfigPath(configPath, repoRoot = defaultRepoRoot) {
  if (!configPath) {
    return undefined;
  }
  return path.isAbsolute(configPath) ? configPath : path.resolve(repoRoot, configPath);
}

function readPasswordFile(passwordFile, configPath, repoRoot = defaultRepoRoot) {
  const normalized = normalizeField(passwordFile);
  if (!normalized) {
    return undefined;
  }
  const baseDir = configPath ? path.dirname(resolveConfigPath(configPath, repoRoot)) : repoRoot;
  const resolved = path.isAbsolute(normalized) ? normalized : path.resolve(baseDir, normalized);
  if (!fs.existsSync(resolved)) {
    return undefined;
  }
  return fs.readFileSync(resolved, 'utf8').trim();
}

function configFormatFor(configPath, configText) {
  const extension = path.extname(String(configPath ?? '')).toLowerCase();
  if (extension === '.yaml' || extension === '.yml') {
    return 'yaml';
  }
  const firstContentLine = String(configText ?? '')
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .find((line) => line && !line.startsWith('#'));
  return firstContentLine?.includes(':') && !firstContentLine.includes('=') ? 'yaml' : 'env';
}

function configFromYamlObject(yaml, configPath, repoRoot = defaultRepoRoot) {
  const connection = yaml.connection ?? {};
  const schema = yaml.schema ?? {};
  const admin = yaml.admin ?? yaml.adminConnection ?? {};
  const password = normalizeField(connection.password)
    ?? readPasswordFile(connection.passwordFile, configPath, repoRoot);
  const adminPassword = normalizeField(admin.password)
    ?? readPasswordFile(admin.passwordFile, configPath, repoRoot);
  const database = {
    database: normalizeField(connection.database),
    host: normalizeField(connection.host),
    password,
    port: normalizeField(connection.port) ?? '5432',
    schema: normalizeField(schema.name) ?? normalizeField(connection.database),
    sslmode: normalizeField(connection.sslmode),
    username: normalizeField(connection.username),
  };
  return {
    admin: {
      database: normalizeField(admin.database) ?? 'postgres',
      host: normalizeField(admin.host) ?? database.host,
      password: adminPassword,
      port: normalizeField(admin.port) ?? database.port,
      sslmode: normalizeField(admin.sslmode) ?? database.sslmode,
      username: normalizeField(admin.username) ?? 'postgres',
    },
    database,
    psql: {
      args: parseCommandArgs(yaml.psql?.args),
      command: normalizeField(yaml.psql?.command),
      pathStyle: normalizeField(yaml.psql?.pathStyle),
    },
    source: {
      format: 'yaml',
      path: configPath,
    },
  };
}

function configFromEnvObject(env, configPath) {
  if (normalizeField(env.SDKWORK_IM_DATABASE_PROVIDER)) {
    throw new Error('SDKWORK_IM_DATABASE_PROVIDER is not standard; use SDKWORK_IM_DATABASE_ENGINE');
  }
  if (normalizeField(env.SDKWORK_IM_DATABASE_SSLMODE)) {
    throw new Error('SDKWORK_IM_DATABASE_SSLMODE is not standard; use SDKWORK_IM_DATABASE_SSL_MODE');
  }
  const databaseFromUrl = parsePostgresDatabaseUrl(
    env.SDKWORK_IM_DATABASE_URL ?? env.SDKWORK_CLAW_DATABASE_URL,
    env.SDKWORK_IM_DATABASE_SCHEMA ?? env.SDKWORK_CLAW_DATABASE_SCHEMA,
  );
  const engine = normalizeField(env.SDKWORK_IM_DATABASE_ENGINE ?? env.SDKWORK_CLAW_DATABASE_PROVIDER);
  if (!databaseFromUrl && engine && !/^postgres(?:ql)?$/iu.test(engine)) {
    throw new Error(`unsupported Sdkwork IM database engine: ${engine}`);
  }
  const database = databaseFromUrl ?? {
    database: normalizeField(env.SDKWORK_IM_DATABASE_NAME ?? env.SDKWORK_CLAW_DATABASE_NAME),
    host: normalizeField(env.SDKWORK_IM_DATABASE_HOST ?? env.SDKWORK_CLAW_DATABASE_HOST),
    password: normalizeField(env.SDKWORK_IM_DATABASE_PASSWORD ?? env.SDKWORK_CLAW_DATABASE_PASSWORD),
    port: normalizeField(env.SDKWORK_IM_DATABASE_PORT ?? env.SDKWORK_CLAW_DATABASE_PORT) ?? '5432',
    schema: normalizeField(env.SDKWORK_IM_DATABASE_SCHEMA ?? env.SDKWORK_CLAW_DATABASE_SCHEMA)
      ?? normalizeField(env.SDKWORK_IM_DATABASE_NAME ?? env.SDKWORK_CLAW_DATABASE_NAME),
    sslmode: normalizeField(env.SDKWORK_IM_DATABASE_SSL_MODE ?? env.SDKWORK_CLAW_DATABASE_SSLMODE),
    username: normalizeField(env.SDKWORK_IM_DATABASE_USERNAME ?? env.SDKWORK_CLAW_DATABASE_USERNAME),
  };
  const adminFromUrl = parsePostgresDatabaseUrl(
    env.SDKWORK_IM_DATABASE_ADMIN_URL ?? env.SDKWORK_CLAW_DATABASE_ADMIN_URL,
    undefined,
  );
  const admin = adminFromUrl ?? {
    database: normalizeField(env.SDKWORK_IM_DATABASE_ADMIN_DATABASE ?? env.SDKWORK_CLAW_DATABASE_ADMIN_DATABASE) ?? 'postgres',
    host: normalizeField(env.SDKWORK_IM_DATABASE_ADMIN_HOST ?? env.SDKWORK_CLAW_DATABASE_ADMIN_HOST) ?? database.host,
    password: normalizeField(env.SDKWORK_IM_DATABASE_ADMIN_PASSWORD ?? env.SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD),
    port: normalizeField(env.SDKWORK_IM_DATABASE_ADMIN_PORT ?? env.SDKWORK_CLAW_DATABASE_ADMIN_PORT) ?? database.port,
    sslmode: normalizeField(env.SDKWORK_IM_DATABASE_ADMIN_SSL_MODE ?? env.SDKWORK_CLAW_DATABASE_ADMIN_SSLMODE) ?? database.sslmode,
    username: normalizeField(env.SDKWORK_IM_DATABASE_ADMIN_USERNAME ?? env.SDKWORK_CLAW_DATABASE_ADMIN_USERNAME) ?? 'postgres',
  };
  return {
    admin,
    database,
    psql: {
      args: parseCommandArgs(
        env.SDKWORK_IM_DATABASE_PSQL_ARGS ?? env.SDKWORK_CLAW_DATABASE_PSQL_ARGS,
      ),
      command: normalizeField(
        env.SDKWORK_IM_DATABASE_PSQL_COMMAND ?? env.SDKWORK_CLAW_DATABASE_PSQL_COMMAND,
      ),
      pathStyle: normalizeField(
        env.SDKWORK_IM_DATABASE_PSQL_PATH_STYLE ?? env.SDKWORK_CLAW_DATABASE_PSQL_PATH_STYLE,
      ),
    },
    source: {
      format: 'env',
      path: configPath,
    },
  };
}

function validateDatabaseConfig(database, prefix) {
  const missing = [];
  for (const field of ['host', 'database', 'username', 'password']) {
    if (!normalizeField(database[field])) {
      missing.push(`${prefix}${field.toUpperCase()}`);
    }
  }
  if (missing.length > 0) {
    throw new Error(`PostgreSQL configuration requires ${missing.join(', ')}`);
  }
}

function validateAdminConfig(admin) {
  const missing = [];
  for (const field of ['host', 'database', 'username']) {
    if (!normalizeField(admin[field])) {
      missing.push(`SDKWORK_CLAW_DATABASE_ADMIN_${field.toUpperCase()}`);
    }
  }
  if (missing.length > 0) {
    throw new Error(`PostgreSQL initialization requires ${missing.join(', ')}`);
  }
}

function validateAdminExecutionConfig(admin) {
  validateAdminConfig(admin);
  if (!normalizeField(admin.password)) {
    throw new Error(
      'PostgreSQL initialization requires SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD or SDKWORK_CLAW_DATABASE_ADMIN_URL',
    );
  }
}

export function parsePostgresConfig({
  configPath = '.env.postgres',
  configText,
  repoRoot = defaultRepoRoot,
} = {}) {
  const resolvedConfigPath = resolveConfigPath(configPath, repoRoot);
  const sourceText = configText ?? fs.readFileSync(resolvedConfigPath, 'utf8');
  const format = configFormatFor(configPath, sourceText);
  const config = format === 'yaml'
    ? configFromYamlObject(parseSimpleYaml(sourceText), configPath, repoRoot)
    : configFromEnvObject(parseDotEnv(sourceText), configPath);
  validateDatabaseConfig(config.database, 'SDKWORK_IM_DATABASE_');
  validateAdminConfig(config.admin);
  return config;
}

function encodePostgresDatabaseName(database) {
  return encodeURIComponent(database).replaceAll('%2F', '/');
}

export function buildPostgresDatabaseUrl(database) {
  const host = normalizeField(database.host);
  const dbName = normalizeField(database.database);
  const username = normalizeField(database.username);
  const password = normalizeField(database.password);
  const port = normalizeField(database.port);
  const credentials = `${encodeURIComponent(username)}${password ? `:${encodeURIComponent(password)}` : ''}`;
  const authority = `${credentials}@${host}${port ? `:${port}` : ''}`;
  const params = new URLSearchParams();
  if (normalizeField(database.sslmode)) {
    params.set('sslmode', normalizeField(database.sslmode));
  }
  const query = params.toString();
  return `postgresql://${authority}/${encodePostgresDatabaseName(dbName)}${query ? `?${query}` : ''}`;
}

export function sanitizePostgresDatabaseUrl(value) {
  try {
    const parsed = new URL(String(value));
    if (parsed.password) {
      parsed.password = '***';
    }
    return parsed.toString();
  } catch {
    return String(value ?? '').replace(/(:\/\/[^:\s]+:)([^@\s]+)(@)/u, '$1***$3');
  }
}

function sqlLiteral(value) {
  return `'${String(value ?? '').replaceAll("'", "''")}'`;
}

function sqlIdentifier(value) {
  return `"${String(value ?? '').replaceAll('"', '""')}"`;
}

function redactedValue(value, redactSecrets) {
  return redactSecrets && normalizeField(value) ? '***' : value;
}

function psqlConnectionArgs(connection) {
  const args = ['-v', 'ON_ERROR_STOP=1'];
  if (normalizeField(connection.host)) {
    args.push('-h', connection.host);
  }
  if (normalizeField(connection.port)) {
    args.push('-p', String(connection.port));
  }
  if (normalizeField(connection.username)) {
    args.push('-U', connection.username);
  }
  if (normalizeField(connection.database)) {
    args.push('-d', connection.database);
  }
  return args;
}

function psqlEnv(connection, redactSecrets) {
  return {
    ...(normalizeField(connection.sslmode) ? { PGSSLMODE: connection.sslmode } : {}),
    PGPASSWORD: redactedValue(connection.password, redactSecrets) ?? '',
  };
}

function psqlStepEnv(config, connection, redactSecrets) {
  const env = psqlEnv(connection, redactSecrets);
  if ((normalizeField(config.psql?.command) ?? '').toLowerCase() === 'wsl.exe') {
    return {
      ...env,
      WSLENV: 'PGPASSWORD/u:PGSSLMODE/u',
    };
  }
  return env;
}

function createRoleAndDatabaseSql(config, redactSecrets) {
  const databaseName = sqlLiteral(config.database.database);
  const username = sqlLiteral(config.database.username);
  const password = sqlLiteral(redactedValue(config.database.password, redactSecrets));
  return [
    '-- Sdkwork IM PostgreSQL role and database initialization.',
    '\\set ON_ERROR_STOP on',
    `SELECT format('CREATE ROLE %I LOGIN PASSWORD %L', ${username}, ${password})`,
    `WHERE NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = ${username})\\gexec`,
    '',
    `SELECT format('ALTER ROLE %I WITH LOGIN PASSWORD %L', ${username}, ${password})\\gexec`,
    '',
    `SELECT format('CREATE DATABASE %I OWNER %I', ${databaseName}, ${username})`,
    `WHERE NOT EXISTS (SELECT 1 FROM pg_database WHERE datname = ${databaseName})\\gexec`,
    '',
    `SELECT format('ALTER DATABASE %I OWNER TO %I', ${databaseName}, ${username})\\gexec`,
    '',
  ].join('\n');
}

function createSchemaAndGrantSql(config) {
  const schema = sqlLiteral(config.database.schema);
  const username = sqlLiteral(config.database.username);
  return [
    '-- Sdkwork IM PostgreSQL schema and grants initialization.',
    '\\set ON_ERROR_STOP on',
    `SELECT format('CREATE SCHEMA IF NOT EXISTS %I AUTHORIZATION %I', ${schema}, ${username})\\gexec`,
    `SELECT format('ALTER SCHEMA %I OWNER TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('GRANT CONNECT ON DATABASE %I TO %I', current_database(), ${username})\\gexec`,
    `SELECT format('GRANT TEMPORARY ON DATABASE %I TO %I', current_database(), ${username})\\gexec`,
    `SELECT format('GRANT USAGE, CREATE ON SCHEMA %I TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA %I TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA %I TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA %I TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('ALTER DEFAULT PRIVILEGES IN SCHEMA %I GRANT ALL PRIVILEGES ON TABLES TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('ALTER DEFAULT PRIVILEGES IN SCHEMA %I GRANT ALL PRIVILEGES ON SEQUENCES TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('ALTER DEFAULT PRIVILEGES IN SCHEMA %I GRANT ALL PRIVILEGES ON FUNCTIONS TO %I', ${schema}, ${username})\\gexec`,
    `SELECT format('ALTER ROLE %I SET search_path TO %I, public', ${username}, ${schema})\\gexec`,
    '',
  ].join('\n');
}

function migrationFilesFor(migrationsDir) {
  if (!fs.existsSync(migrationsDir)) {
    throw new Error(`PostgreSQL migrations directory does not exist: ${migrationsDir}`);
  }
  return fs
    .readdirSync(migrationsDir)
    .filter((entry) => entry.endsWith('.sql'))
    .sort((left, right) => left.localeCompare(right))
    .map((entry) => path.join(migrationsDir, entry));
}

export function listActivePostgresMigrationBasenames(migrationsDir = defaultMigrationsDir) {
  return migrationFilesFor(migrationsDir).map((migration) => path.basename(migration));
}

function normalizePathForWsl(value) {
  const normalized = path.resolve(value).replaceAll('\\', '/');
  const driveMatch = normalized.match(/^([A-Za-z]):\/(.*)$/u);
  if (driveMatch) {
    return `/mnt/${driveMatch[1].toLowerCase()}/${driveMatch[2]}`;
  }
  return normalized;
}

function normalizePathForPsql(value, pathStyle) {
  if ((normalizeField(pathStyle) ?? '').toLowerCase() === 'wsl') {
    return normalizePathForWsl(value);
  }
  return path.resolve(value).replaceAll('\\', '/');
}

function psqlStepCommand(config, psqlCommand) {
  return normalizeField(config.psql?.command) ?? normalizeField(psqlCommand) ?? 'psql';
}

function psqlStepArgs(config, args) {
  return [...(config.psql?.args ?? []), ...args];
}

function psqlStepShell(config) {
  if ((normalizeField(config.psql?.command) ?? '').toLowerCase() === 'wsl.exe') {
    return false;
  }
  return process.platform === 'win32';
}

function psqlPathStyle(config) {
  return config.psql?.pathStyle;
}

function shouldUsePsqlInit(config) {
  return Boolean(normalizeField(config.psql?.command));
}

function createNodeInitRoleStep(config) {
  return createStep({
    action: 'role-and-database',
    config,
    kind: 'node-init',
    label: 'initialize PostgreSQL role and database',
  });
}

function createNodeInitSchemaStep(config) {
  return createStep({
    action: 'schema-and-grants',
    config,
    kind: 'node-init',
    label: 'initialize PostgreSQL schema and grants',
  });
}

function createStep({
  action,
  args,
  command,
  config,
  cwd,
  env,
  input,
  kind = 'shell',
  label,
  shell,
}) {
  return {
    action,
    args,
    command,
    config,
    cwd,
    env,
    input,
    kind,
    label,
    shell,
  };
}

function createFrameworkBootstrapStep(repoRoot, config, redactSecrets) {
  const databaseUrl = buildPostgresDatabaseUrl(config.database);
  return createStep({
    args: [
      'run',
      '--manifest-path',
      path.join(repoRoot, '../sdkwork-database/Cargo.toml'),
      '-p',
      'sdkwork-database-cli',
      '--',
      '--app-root',
      repoRoot,
      'bootstrap',
    ],
    command: 'cargo',
    cwd: repoRoot,
    env: {
      SDKWORK_IM_DATABASE_AUTO_MIGRATE: 'true',
      SDKWORK_IM_DATABASE_URL: redactSecrets
        ? sanitizePostgresDatabaseUrl(databaseUrl)
        : databaseUrl,
    },
    kind: 'shell',
    label: 'bootstrap IM database lifecycle via sdkwork-database-cli',
  });
}

export function createPostgresDbPlan({
  appbaseIamMigrationsDir = defaultAppbaseIamMigrationsDir,
  config,
  migrationsDir = defaultMigrationsDir,
  mode = 'plan',
  psqlCommand = 'psql',
  redactSecrets = true,
  repoRoot = defaultRepoRoot,
} = {}) {
  if (!config) {
    throw new Error('createPostgresDbPlan requires parsed PostgreSQL config');
  }
  const normalizedMode = normalizeField(mode) ?? 'plan';
  if (!['init', 'migrate', 'plan'].includes(normalizedMode)) {
    throw new Error(`unsupported PostgreSQL database mode: ${normalizedMode}`);
  }
  const steps = [];
  if (normalizedMode === 'init' || normalizedMode === 'plan') {
    validateAdminExecutionConfig(config.admin);
    if (shouldUsePsqlInit(config)) {
      const adminDatabaseConnection = {
        ...config.admin,
        database: config.admin.database ?? 'postgres',
      };
      const targetDatabaseAdminConnection = {
        ...config.admin,
        database: config.database.database,
      };
      const command = psqlStepCommand(config, psqlCommand);
      const shell = psqlStepShell(config);
      steps.push(createStep({
        args: psqlStepArgs(config, psqlConnectionArgs(adminDatabaseConnection)),
        command,
        cwd: repoRoot,
        env: psqlStepEnv(config, adminDatabaseConnection, redactSecrets),
        input: createRoleAndDatabaseSql(config, redactSecrets),
        kind: 'shell',
        label: 'initialize PostgreSQL role and database',
        shell,
      }));
      steps.push(createStep({
        args: psqlStepArgs(config, psqlConnectionArgs(targetDatabaseAdminConnection)),
        command,
        cwd: repoRoot,
        env: psqlStepEnv(config, targetDatabaseAdminConnection, redactSecrets),
        input: createSchemaAndGrantSql(config),
        kind: 'shell',
        label: 'initialize PostgreSQL schema and grants',
        shell,
      }));
    } else {
      steps.push(createNodeInitRoleStep(config));
      steps.push(createNodeInitSchemaStep(config));
    }
  }
  if (normalizedMode === 'migrate' || normalizedMode === 'plan') {
    steps.push(createFrameworkBootstrapStep(repoRoot, config, redactSecrets));
  }

  return {
    mode: normalizedMode,
    source: config.source,
    steps,
    target: {
      database: config.database.database,
      databaseUrl: sanitizePostgresDatabaseUrl(buildPostgresDatabaseUrl(config.database)),
      host: config.database.host,
      port: config.database.port,
      schema: config.database.schema,
      username: config.database.username,
    },
  };
}

function parseArgs(argv) {
  const result = {
    configPath: '.env.postgres',
    dryRun: false,
    migrationsDir: defaultMigrationsDir,
    mode: 'plan',
    psqlCommand: 'psql',
  };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--help' || arg === '-h') {
      result.help = true;
      continue;
    }
    if (arg === '--dry-run') {
      result.dryRun = true;
      continue;
    }
    if (arg === '--mode') {
      result.mode = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--config') {
      result.configPath = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--migrations') {
      result.migrationsDir = path.resolve(defaultRepoRoot, argv[index + 1]);
      index += 1;
      continue;
    }
    if (arg === '--psql') {
      result.psqlCommand = argv[index + 1];
      index += 1;
      continue;
    }
    throw new Error(`unknown argument: ${arg}`);
  }
  return result;
}

function formatShellCommand(command, args) {
  return [command, ...args].map((part) => (/\s/u.test(part) ? `"${part.replaceAll('"', '\\"')}"` : part)).join(' ');
}

export function formatPostgresDbPlan(plan) {
  return [
    `Sdkwork IM PostgreSQL database plan (${plan.mode})`,
    `config: ${plan.source?.path ?? '<memory>'}`,
    `target: ${plan.target.databaseUrl}`,
    `schema: ${plan.target.schema}`,
    '',
    ...plan.steps.flatMap((step, index) => [
      `${index + 1}. ${step.label}`,
      step.kind === 'node-init'
        ? '   runtime: node (pg)'
        : `   command: ${formatShellCommand(step.command, step.args)}`,
      step.input ? `   stdin:\n${step.input.split('\n').map((line) => `     ${line}`).join('\n')}` : undefined,
      '',
    ].filter(Boolean)),
  ].join('\n');
}

function helpText() {
  return [
    'Usage:',
    '  node scripts/dev/sdkwork-im-postgres-db.mjs --mode plan --config .env.postgres --dry-run',
    '  node scripts/dev/sdkwork-im-postgres-db.mjs --mode init --config .env.postgres',
    '  node scripts/dev/sdkwork-im-postgres-db.mjs --mode migrate --config .env.postgres',
    '',
    'Modes:',
    '  plan      Print initialization and migration actions without touching the database.',
    '  init      Create/update app role, database, schema, grants, and default privileges.',
    '  migrate   Bootstrap IM schema via sdkwork-database-cli (database/ lifecycle).',
    '',
    'Config:',
    '  .env.postgres is the single source of truth for app startup and db:* commands.',
    '  Windows init uses node/pg by default; set SDKWORK_IM_DATABASE_PSQL_COMMAND for psql/WSL.',
    '',
  ].join('\n');
}

async function executePostgresDbStep(step) {
  if (step.kind === 'node-init') {
    if (step.action === 'role-and-database') {
      await initializePostgresRoleAndDatabase(step.config);
      return;
    }
    if (step.action === 'schema-and-grants') {
      await initializePostgresSchemaAndGrants(step.config);
      return;
    }
    throw new Error(`unsupported node init action: ${step.action}`);
  }

  const spawned = spawnSync(step.command, step.args, {
    cwd: step.cwd,
    encoding: 'utf8',
    env: step.env,
    input: step.input,
    shell: step.shell ?? process.platform === 'win32',
    stdio: ['pipe', 'pipe', 'pipe'],
  });
  if (spawned.stdout) {
    process.stdout.write(spawned.stdout);
  }
  if (spawned.stderr) {
    process.stderr.write(spawned.stderr);
  }
  if (spawned.error) {
    throw spawned.error;
  }
  if (spawned.status !== 0) {
    const detail = String(spawned.stderr ?? spawned.stdout ?? '').trim();
    throw new Error(
      detail
        ? `PostgreSQL step failed (${step.label}): ${detail}`
        : `PostgreSQL step failed with exit code ${spawned.status}: ${step.label}`,
    );
  }
}

export async function runPostgresDbCli({
  argv = process.argv.slice(2),
  env = process.env,
  repoRoot = defaultRepoRoot,
  stderr = process.stderr,
  stdout = process.stdout,
} = {}) {
  const args = parseArgs(argv);
  if (args.help) {
    stdout.write(helpText());
    return { status: 0 };
  }

  const profile = resolvePostgresDevProfile({ env, repoRoot });
  const config = profile.config;
  if (env.SDKWORK_IM_DATABASE_SCHEMA || env.SDKWORK_CLAW_DATABASE_SCHEMA) {
    config.database.schema = env.SDKWORK_IM_DATABASE_SCHEMA ?? env.SDKWORK_CLAW_DATABASE_SCHEMA;
  }
  const migrationsDir = path.isAbsolute(args.migrationsDir)
    ? args.migrationsDir
    : path.resolve(repoRoot, args.migrationsDir);
  const shouldExecute = args.mode !== 'plan' && !args.dryRun;
  const plan = createPostgresDbPlan({
    config,
    migrationsDir,
    mode: args.mode,
    psqlCommand: args.psqlCommand,
    redactSecrets: !shouldExecute,
    repoRoot,
  });
  plan.source = {
    format: 'env',
    path: profile.configPath,
  };
  plan.target.databaseUrl = profile.databaseUrl;
  const redactedPlan = shouldExecute
    ? createPostgresDbPlan({
      config,
      migrationsDir,
      mode: args.mode,
      psqlCommand: args.psqlCommand,
      redactSecrets: true,
      repoRoot,
    })
    : plan;
  redactedPlan.source = plan.source;
  redactedPlan.target.databaseUrl = sanitizePostgresDatabaseUrl(profile.databaseUrl);

  if (!shouldExecute) {
    stdout.write(`${formatPostgresDbPlan(redactedPlan)}\n`);
    return { plan: redactedPlan, status: 0 };
  }

  stdout.write(`${formatPostgresDbPlan(redactedPlan)}\n`);
  if (args.mode === 'migrate') {
    await repairPostgresMigrationChecksums({
      config,
      repoRoot,
      stdout,
    });
  }
  for (const step of plan.steps) {
    stdout.write(`[sdkwork-im-db] ${step.label}\n`);
    const stepEnv = {
      ...profile.env,
      ...step.env,
    };
    if (step.kind === 'shell') {
      const spawned = spawnSync(step.command, step.args, {
        cwd: step.cwd,
        encoding: 'utf8',
        env: {
          ...process.env,
          ...stepEnv,
        },
        input: step.input,
        shell: step.shell ?? process.platform === 'win32',
        stdio: ['pipe', 'inherit', 'inherit'],
      });
      if (spawned.error) {
        throw spawned.error;
      }
      if (spawned.status !== 0) {
        throw new Error(`PostgreSQL step failed with exit code ${spawned.status}: ${step.label}`);
      }
      continue;
    }
    await executePostgresDbStep({
      ...step,
      env: stepEnv,
    });
  }
  stdout.write('[sdkwork-im-db] PostgreSQL database task completed\n');
  return { plan: redactedPlan, status: 0 };
}

if (path.resolve(process.argv[1] ?? '') === __filename) {
  runPostgresDbCli().catch((error) => {
    process.stderr.write(`[sdkwork-im-db] ${error instanceof Error ? error.message : String(error)}\n`);
    process.exitCode = 1;
  });
}
