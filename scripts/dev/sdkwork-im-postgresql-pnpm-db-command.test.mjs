import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import {
  DEPLOYMENT_DOC_FILES,
  readDeploymentDoc,
} from '../lib/deployment-docs.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const packageJson = readJson('package.json');
const envExample = read('.env.postgres.example');
const configIndex = readDeploymentDoc(repoRoot, DEPLOYMENT_DOC_FILES.postgresqlIndex);
const ubuntuWslGuide = readDeploymentDoc(repoRoot, DEPLOYMENT_DOC_FILES.ubuntuWslGuide);
const devGuide = readDeploymentDoc(repoRoot, DEPLOYMENT_DOC_FILES.developmentGuide);
const productionGuide = readDeploymentDoc(repoRoot, DEPLOYMENT_DOC_FILES.productionGuide);
const appbaseIamBaselinePath = path.join(
  workspaceRoot,
  'sdkwork-appbase',
  'database',
  'ddl',
  'baseline',
  'postgres',
  '0001_iam_baseline.sql',
);
const appbaseIamCrateMigrationPath = path.join(
  workspaceRoot,
  'sdkwork-appbase',
  'crates',
  'sdkwork-iam-directory-repository-sqlx',
  'migrations',
  '0001_iam_foundation.sql',
);

if (fs.existsSync(appbaseIamBaselinePath)) {
  const appbaseIamBaseline = fs.readFileSync(appbaseIamBaselinePath, 'utf8');
  assert.match(
    appbaseIamBaseline,
    /CREATE TABLE IF NOT EXISTS iam_organization_membership \(/u,
    'appbase IAM baseline must create the canonical organization membership table',
  );
  assert.match(
    appbaseIamBaseline,
    /CREATE TABLE IF NOT EXISTS iam_tenant_member \(/u,
    'appbase IAM baseline must create the canonical tenant member table',
  );
  assert.doesNotMatch(
    appbaseIamBaseline,
    /CREATE TABLE IF NOT EXISTS iam_organization_member \(/u,
    'appbase IAM baseline must not create the non-canonical iam_organization_member table',
  );
}

if (fs.existsSync(appbaseIamCrateMigrationPath)) {
  const appbaseIamCrateMigration = fs.readFileSync(appbaseIamCrateMigrationPath, 'utf8');
  assert.match(
    appbaseIamCrateMigration,
    /CREATE TABLE IF NOT EXISTS iam_tenant_signing_key \(/u,
    'appbase IAM crate migration must create tenant-bound signing key table',
  );
}

assert.equal(
  packageJson.scripts['db:postgres:plan'],
  'node scripts/dev/sdkwork-im-postgres-db.mjs --mode plan --config .env.postgres --dry-run',
  'root pnpm db:postgres:plan must print the audited PostgreSQL initialization and migration plan',
);
assert.equal(
  packageJson.scripts['db:postgres:init'],
  'node scripts/dev/sdkwork-im-postgres-db.mjs --mode init --config .env.postgres',
  'root pnpm db:postgres:init must initialize role/database/schema/grants from .env.postgres',
);
assert.equal(
  packageJson.scripts['db:postgres:migrate'],
  'node scripts/dev/sdkwork-im-postgres-db.mjs --mode migrate --config .env.postgres',
  'root pnpm db:postgres:migrate must run PostgreSQL migrations from .env.postgres',
);
assert.equal(
  packageJson.scripts['db:postgres:repair'],
  'node scripts/dev/repair-postgres-migration-checksums.cli.mjs --config .env.postgres',
  'root pnpm db:postgres:repair must reconcile local PostgreSQL migration checksum drift',
);

const dbScriptPath = path.join(repoRoot, 'scripts/dev/sdkwork-im-postgres-db.mjs');
const dbScript = read('scripts/dev/sdkwork-im-postgres-db.mjs');
assert.match(
  dbScript,
  /repairPostgresMigrationChecksums/u,
  'PostgreSQL migrate flow must repair local migration checksum drift before bootstrap',
);
assert.ok(fs.existsSync(dbScriptPath), 'PostgreSQL pnpm database script must exist');

const {
  buildPostgresDatabaseUrl,
  createPostgresDbPlan,
  parsePostgresConfig,
  sanitizePostgresDatabaseUrl,
} = await import(pathToFileURL(dbScriptPath).href);

const parsedSplitConfig = parsePostgresConfig({
  configText: [
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
    'SDKWORK_IM_DATABASE_PORT=15432',
    'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_PASSWORD=chat pass',
    'SDKWORK_IM_DATABASE_SSL_MODE=disable',
    'SDKWORK_IM_DATABASE_ADMIN_USERNAME=postgres',
    'SDKWORK_IM_DATABASE_ADMIN_PASSWORD=admin pass',
    'SDKWORK_IM_DATABASE_ADMIN_DATABASE=postgres',
    '',
  ].join('\n'),
  configPath: '.env.postgres',
});
assert.equal(parsedSplitConfig.database.host, '127.0.0.1');
assert.equal(parsedSplitConfig.database.port, '15432');
assert.equal(parsedSplitConfig.database.database, 'sdkwork_ai_dev');
assert.equal(parsedSplitConfig.database.schema, 'sdkwork_ai_dev');
assert.equal(parsedSplitConfig.database.username, 'sdkwork_ai_dev');
assert.equal(parsedSplitConfig.database.password, 'chat pass');
assert.equal(parsedSplitConfig.database.sslmode, 'disable');
assert.equal(parsedSplitConfig.admin.database, 'postgres');
assert.equal(parsedSplitConfig.admin.username, 'postgres');
assert.equal(parsedSplitConfig.admin.password, 'admin pass');

assert.equal(
  buildPostgresDatabaseUrl(parsedSplitConfig.database),
  'postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'PostgreSQL DB script must assemble a usable app connection URL from split config fields',
);
assert.equal(
  sanitizePostgresDatabaseUrl('postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable'),
  'postgresql://sdkwork_ai_dev:***@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'PostgreSQL DB script must redact connection-string passwords in logs and dry-run output',
);

const parsedUrlConfig = parsePostgresConfig({
  configText: [
    'SDKWORK_CLAW_DATABASE_URL=postgresql://url_user:url_pass@db.internal:5432/url_db?sslmode=require',
    'SDKWORK_CLAW_DATABASE_SCHEMA=url_schema',
    'SDKWORK_CLAW_DATABASE_ADMIN_URL=postgresql://postgres:admin_pass@db.internal:5432/postgres?sslmode=require',
    '',
  ].join('\n'),
  configPath: '.env.postgres',
});
assert.equal(parsedUrlConfig.database.host, 'db.internal');
assert.equal(parsedUrlConfig.database.database, 'url_db');
assert.equal(parsedUrlConfig.database.schema, 'url_schema');
assert.equal(parsedUrlConfig.database.username, 'url_user');
assert.equal(parsedUrlConfig.database.password, 'url_pass');
assert.equal(parsedUrlConfig.admin.username, 'postgres');
assert.equal(parsedUrlConfig.admin.password, 'admin_pass');

const parsedWslPsqlConfig = parsePostgresConfig({
  configText: [
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
    'SDKWORK_IM_DATABASE_PORT=5432',
    'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_PASSWORD=sdkworkdev123',
    'SDKWORK_IM_DATABASE_SSL_MODE=disable',
    'SDKWORK_IM_DATABASE_ADMIN_USERNAME=postgres',
    'SDKWORK_IM_DATABASE_ADMIN_PASSWORD=admin pass',
    'SDKWORK_IM_DATABASE_PSQL_COMMAND=wsl.exe',
    'SDKWORK_IM_DATABASE_PSQL_ARGS=-d Ubuntu-22.04 -- psql',
    'SDKWORK_IM_DATABASE_PSQL_PATH_STYLE=wsl',
    '',
  ].join('\n'),
  configPath: '.env.postgres',
});
assert.equal(parsedWslPsqlConfig.psql.command, 'wsl.exe');
assert.deepEqual(parsedWslPsqlConfig.psql.args, ['-d', 'Ubuntu-22.04', '--', 'psql']);
assert.equal(parsedWslPsqlConfig.psql.pathStyle, 'wsl');

const parsedYamlConfig = parsePostgresConfig({
  configText: [
    'provider: postgresql',
    'connection:',
    '  host: 10.0.0.8',
    '  port: 5432',
    '  database: sdkwork',
    '  username: sdkwork',
    '  password: yaml_pass',
    '  sslmode: require',
    'schema:',
    '  name: sdkwork',
    '',
  ].join('\n'),
  configPath: 'postgresql.yaml',
});
assert.equal(parsedYamlConfig.database.host, '10.0.0.8');
assert.equal(parsedYamlConfig.database.database, 'sdkwork');
assert.equal(parsedYamlConfig.database.schema, 'sdkwork');
assert.equal(parsedYamlConfig.database.username, 'sdkwork');
assert.equal(parsedYamlConfig.database.password, 'yaml_pass');
assert.equal(parsedYamlConfig.database.sslmode, 'require');

assert.throws(
  () => parsePostgresConfig({
    configText: [
      'SDKWORK_CLAW_DATABASE_PROVIDER=postgresql',
      'SDKWORK_CLAW_DATABASE_HOST=127.0.0.1',
      'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
      'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
      '',
    ].join('\n'),
    configPath: '.env.postgres',
  }),
  /SDKWORK_IM_DATABASE_PASSWORD/u,
  'PostgreSQL DB script must reject incomplete app connection config before invoking psql',
);

const initPlan = createPostgresDbPlan({
  config: parsedSplitConfig,
  mode: 'init',
  repoRoot,
});
assert.deepEqual(
  initPlan.steps.map((step) => step.label),
  ['initialize PostgreSQL role and database', 'initialize PostgreSQL schema and grants'],
  'init mode must provision role/database before target schema/grants',
);
assert.ok(
  initPlan.steps.every((step) => step.kind === 'node-init'),
  'default database init must use node/pg so Windows dev does not require psql on PATH',
);

const psqlInitPlan = createPostgresDbPlan({
  config: {
    ...parsedSplitConfig,
    psql: {
      command: 'psql',
    },
  },
  mode: 'init',
  repoRoot,
});
assert.ok(
  psqlInitPlan.steps.every((step) => step.kind === 'shell' && step.command === 'psql'),
  'explicit psql command config must keep shell-based init steps',
);
assert.ok(
  psqlInitPlan.steps.every((step) => step.env.PGPASSWORD && !step.args.join(' ').includes(step.env.PGPASSWORD)),
  'database init must pass passwords through PGPASSWORD instead of command-line arguments',
);
assert.match(
  psqlInitPlan.steps[0].input,
  /CREATE ROLE.*LOGIN PASSWORD|ALTER ROLE.*WITH LOGIN PASSWORD/su,
  'database init must create or update the app login role',
);
assert.match(
  psqlInitPlan.steps[0].input,
  /CREATE DATABASE/su,
  'database init must create the target database when it does not exist',
);
assert.match(
  psqlInitPlan.steps[1].input,
  /CREATE SCHEMA IF NOT EXISTS.*AUTHORIZATION/su,
  'database init must create the configured schema with app ownership',
);
assert.match(
  psqlInitPlan.steps[1].input,
  /ALTER DEFAULT PRIVILEGES/su,
  'database init must configure default privileges for future tables, sequences, and functions',
);
assert.doesNotMatch(
  JSON.stringify(psqlInitPlan),
  /chat pass|admin pass/u,
  'audited database plans must not leak raw passwords when serialized',
);
assert.throws(
  () => createPostgresDbPlan({
    config: {
      ...parsedSplitConfig,
      admin: {
        ...parsedSplitConfig.admin,
        password: undefined,
      },
    },
    mode: 'init',
    repoRoot,
  }),
  /SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD|SDKWORK_CLAW_DATABASE_ADMIN_URL/u,
  'database init must fail before invoking psql when the admin password is missing',
);

const migratePlan = createPostgresDbPlan({
  config: parsedSplitConfig,
  mode: 'migrate',
  repoRoot,
});
assert.equal(migratePlan.steps.length, 1, 'migrate mode must delegate schema to sdkwork-database-cli');
assert.match(
  migratePlan.steps[0].label,
  /bootstrap IM database lifecycle/u,
  'migrate mode must use the database framework bootstrap step',
);
assert.equal(migratePlan.steps[0].command, 'cargo');
assert.ok(
  migratePlan.steps[0].args.includes('bootstrap'),
  'migrate mode must invoke sdkwork-database-cli bootstrap',
);
assert.equal(
  migratePlan.steps[0].env.SDKWORK_IM_DATABASE_AUTO_MIGRATE,
  'true',
  'framework bootstrap must enable auto migrate for the CLI process',
);

const wslInitPlan = createPostgresDbPlan({
  config: parsedWslPsqlConfig,
  mode: 'init',
  repoRoot,
});
assert.ok(
  wslInitPlan.steps.every((step) => step.kind === 'shell' && step.command === 'wsl.exe'),
  'WSL developers initialize PostgreSQL through wsl.exe-wrapped psql steps',
);
assert.ok(
  wslInitPlan.steps.every((step) => step.shell === false),
  'WSL psql steps must bypass Windows shell quoting so psql arguments containing spaces stay intact',
);

const wslMigratePlan = createPostgresDbPlan({
  config: parsedWslPsqlConfig,
  mode: 'migrate',
  repoRoot,
});
assert.equal(wslMigratePlan.steps[0].command, 'cargo');
assert.ok(
  wslMigratePlan.steps[0].args.includes('bootstrap'),
  'WSL developers still bootstrap IM schema through sdkwork-database-cli',
);

const fullPlan = createPostgresDbPlan({
  config: parsedSplitConfig,
  mode: 'plan',
  repoRoot,
});
assert.deepEqual(
  fullPlan.steps.map((step) => step.label),
  [
    'initialize PostgreSQL role and database',
    'initialize PostgreSQL schema and grants',
    'bootstrap IM database lifecycle via sdkwork-database-cli',
  ],
  'plan mode must show initialization and database-framework bootstrap actions',
);

for (const required of [
  'SDKWORK_IM_DATABASE_ENGINE=postgresql',
  'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
  'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
  'SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_dev',
  'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
  'SDKWORK_CLAW_DATABASE_PASSWORD=sdkworkdev123',
  'SDKWORK_IM_DATABASE_SSL_MODE=disable',
  'SDKWORK_CLAW_DATABASE_ADMIN_USERNAME=postgres',
  'SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD=postgres_admin_pass',
  'SDKWORK_CLAW_DATABASE_ADMIN_DATABASE=postgres',
  'SDKWORK_CLAW_DATABASE_ADMIN_SSL_MODE=disable',
]) {
  assert.ok(envExample.includes(required), `.env.postgres.example must document ${required}`);
}

for (const doc of [configIndex, ubuntuWslGuide, devGuide]) {
  for (const required of [
    'pnpm db:postgres:plan',
    'pnpm db:postgres:init',
    'pnpm db:postgres:migrate',
    'SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD',
    'SDKWORK_CLAW_DATABASE_SCHEMA',
  ]) {
    assert.ok(doc.includes(required), `PostgreSQL docs must include ${required}`);
  }
}

for (const required of [
  'database: sdkwork',
  'schema: public',
  'username: sdkwork',
  'CREATE ROLE "sdkwork" LOGIN PASSWORD',
  'CREATE DATABASE sdkwork OWNER "sdkwork"',
  'CREATE SCHEMA IF NOT EXISTS public AUTHORIZATION "sdkwork"',
  'postgresql://sdkwork@postgres.internal.example.com:5432/sdkwork?sslmode=require',
]) {
  assert.ok(productionGuide.includes(required), `production PostgreSQL guide must include ${required}`);
}

console.log('sdkwork-im PostgreSQL pnpm database command contract passed');
