import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
}

function deploymentDocsRoot() {
  const docsRoot = path.join(repoRoot, 'docs');
  for (const entry of fs.readdirSync(docsRoot, { withFileTypes: true })) {
    if (!entry.isDirectory()) {
      continue;
    }
    const marker = path.join(docsRoot, entry.name, 'postgresql-database-configuration.md');
    if (fs.existsSync(marker)) {
      return path.join(docsRoot, entry.name);
    }
  }
  throw new Error('deployment documentation directory must include postgresql-database-configuration.md');
}

function readDeploymentDoc(fileName) {
  const absolutePath = path.join(deploymentDocsRoot(), fileName);
  assert.ok(fs.existsSync(absolutePath), `deployment docs must include ${fileName}`);
  return fs.readFileSync(absolutePath, 'utf8');
}

function readDeploymentDocsMatching(pattern) {
  const deploymentDir = deploymentDocsRoot();
  return fs
    .readdirSync(deploymentDir)
    .filter((fileName) => pattern.test(fileName))
    .map((fileName) => fs.readFileSync(path.join(deploymentDir, fileName), 'utf8'));
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const manifest = readJson('sdkwork.app.config.json');
assert.equal(manifest.app.key, 'chat', 'SDKWork app key must be the canonical app code chat');
assert.equal(manifest.app.name, 'chat', 'PlusApp identity name must be the stable app code chat');
assert.equal(manifest.app.displayName, 'Sdkwork IM', 'display name may carry the product label');
assert.equal(manifest.publish.config.workspaceRoot, 'apps/sdkwork-im-pc');
assert.equal(manifest.devApp.sourceRoot, 'apps/sdkwork-im-pc');
assert.equal(manifest.app.identifiers.desktopAppId, 'com.sdkwork.chat.desktop');
assert.equal(manifest.app.identifiers.containerImage, 'registry.sdkwork.com/apps/chat');
assert.equal(manifest.artifacts.installConfig.installCommand, 'sdkwork install chat');
assert.equal(manifest.artifacts.installConfig.launchCommand, 'sdkwork open chat');
assert.equal(manifest.artifacts.installConfig.uninstallCommand, 'sdkwork uninstall chat');

const envExample = read('.env.postgres.example');
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
for (const required of [
  'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
  'SDKWORK_IM_REDIS_ENABLED=true',
  'SDKWORK_IM_REDIS_HOST=127.0.0.1',
  'SDKWORK_IM_REDIS_PORT=6379',
  'SDKWORK_IM_REDIS_DATABASE=0',
  'SDKWORK_IM_REDIS_KEY_PREFIX=chat',
  'SDKWORK_IM_REDIS_TLS=false',
  'SDKWORK_IM_DATABASE_MAX_CONNECTIONS=10',
]) {
  assert.ok(envExample.includes(required), `.env.postgres.example must document ${required}`);
}
assert.doesNotMatch(envExample, /SDKWORK_CLAW_DATABASE_PROVIDER|SDKWORK_CLAW_DATABASE_SSLMODE/u);

const sharedDbModule = await import(
  pathToFileURL(path.join(repoRoot, 'scripts/dev/sdkwork-im-shared-database.mjs')).href
);
const canonicalPostgres = sharedDbModule.resolveSdkworkImSharedDatabaseConfig({
  env: {
    SDKWORK_IM_DATABASE_ENGINE: 'postgresql',
    SDKWORK_IM_DATABASE_HOST: '127.0.0.1',
    SDKWORK_IM_DATABASE_PORT: '15432',
    SDKWORK_IM_DATABASE_NAME: 'sdkwork_ai_dev',
    SDKWORK_IM_DATABASE_USERNAME: 'sdkwork_ai_dev',
    SDKWORK_IM_DATABASE_PASSWORD: 'chat pass',
    SDKWORK_IM_DATABASE_SSL_MODE: 'disable',
    SDKWORK_IM_DATABASE_MAX_CONNECTIONS: '11',
  },
  repoRoot,
});
assert.equal(canonicalPostgres.kind, 'postgresql');
assert.equal(
  canonicalPostgres.env.SDKWORK_IM_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
);
assert.equal(
  canonicalPostgres.env.SDKWORK_CLAW_DATABASE_URL,
  canonicalPostgres.env.SDKWORK_IM_DATABASE_URL,
  'runtime bridge must keep the current Rust-compatible database URL during migration',
);
assert.equal(canonicalPostgres.env.SDKWORK_IM_DATABASE_MAX_CONNECTIONS, '11');
assert.equal(canonicalPostgres.env.SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS, '11');

assert.throws(
  () => sharedDbModule.resolveSdkworkImSharedDatabaseConfig({
    env: {
      SDKWORK_IM_DATABASE_PROVIDER: 'postgresql',
      SDKWORK_IM_DATABASE_HOST: '127.0.0.1',
      SDKWORK_IM_DATABASE_NAME: 'sdkwork_ai_dev',
      SDKWORK_IM_DATABASE_USERNAME: 'sdkwork_ai_dev',
      SDKWORK_IM_DATABASE_PASSWORD: 'sdkworkdev123',
    },
    repoRoot,
  }),
  /SDKWORK_IM_DATABASE_PROVIDER.*not standard/u,
  'new app config must reject legacy DATABASE_PROVIDER spelling',
);
assert.throws(
  () => sharedDbModule.resolveSdkworkImSharedDatabaseConfig({
    env: {
      SDKWORK_IM_DATABASE_ENGINE: 'postgresql',
      SDKWORK_IM_DATABASE_HOST: '127.0.0.1',
      SDKWORK_IM_DATABASE_NAME: 'sdkwork_ai_dev',
      SDKWORK_IM_DATABASE_USERNAME: 'sdkwork_ai_dev',
      SDKWORK_IM_DATABASE_PASSWORD: 'sdkworkdev123',
      SDKWORK_IM_DATABASE_SSLMODE: 'disable',
    },
    repoRoot,
  }),
  /SDKWORK_IM_DATABASE_SSLMODE.*not standard/u,
  'new app config must reject legacy DATABASE_SSLMODE spelling',
);

const canonicalSqlite = sharedDbModule.resolveSdkworkImSharedDatabaseConfig({
  env: {
    SDKWORK_IM_DEPLOYMENT_MODE: 'desktop',
  },
  repoRoot,
});
assert.equal(canonicalSqlite.kind, 'sqlite');
assert.match(
  canonicalSqlite.databaseUrl,
  /sqlite:\/\/.*[/\\]\.sdkwork[/\\]chat[/\\]data[/\\]chat\.sqlite$/u,
  'desktop/local default SQLite must live under the SDKWork user-private chat data directory',
);
assert.equal(canonicalSqlite.env.SDKWORK_IM_DATABASE_ENGINE, 'sqlite');
assert.equal(canonicalSqlite.env.SDKWORK_IM_DATABASE_MAX_CONNECTIONS, '1');
assert.equal(
  canonicalSqlite.env.SDKWORK_CLAW_DATABASE_URL,
  canonicalSqlite.env.SDKWORK_IM_DATABASE_URL,
  'desktop SQLite config must be bridged to the current Rust-compatible env name',
);
assert.match(
  canonicalSqlite.env.SDKWORK_DRIVE_DATABASE_SQLITE_URL,
  /sqlite:\/\/.*[/\\]drive\.sqlite$/u,
  'shared SQLite bridge must isolate Drive persistence from IM chat.sqlite',
);
assert.match(
  canonicalSqlite.env.SDKWORK_KNOWLEDGEBASE_DATABASE_URL,
  /sqlite:\/\/.*[/\\]knowledgebase\.db\?mode=rwc$/u,
  'shared SQLite bridge must isolate Knowledgebase persistence from IM chat.sqlite',
);
assert.match(
  canonicalSqlite.env.SDKWORK_CATALOG_DATABASE_URL,
  /sqlite:\/\/.*[/\\]catalog\.sqlite$/u,
  'shared SQLite bridge must isolate catalog persistence from IM chat.sqlite',
);
assert.match(
  canonicalSqlite.env.SDKWORK_ORDER_DATABASE_URL,
  /sqlite:\/\/.*[/\\]order\.sqlite$/u,
  'shared SQLite bridge must isolate order persistence from IM chat.sqlite',
);
assert.match(
  canonicalSqlite.env.SDKWORK_SHOP_DATABASE_URL,
  /sqlite:\/\/.*[/\\]shop\.sqlite$/u,
  'shared SQLite bridge must isolate shop persistence from IM chat.sqlite',
);
assert.match(
  canonicalSqlite.env.SDKWORK_MAIL_DATABASE_URL,
  /sqlite:\/\/.*[/\\]mail\.sqlite$/u,
  'shared SQLite bridge must isolate Mail persistence from IM chat.sqlite',
);

const planModule = await import(
  pathToFileURL(path.join(repoRoot, 'scripts/release/plan-sdkwork-im-install-packages.mjs')).href
);
const plan = planModule.createSdkworkImInstallPackagePlan({ version: '1.2.3' });
assert.equal(plan.appCode, 'chat');
assert.equal(plan.runtimeName, 'chat');
assert.equal(plan.product, 'chat');
assert.equal(plan.packageName, 'sdkwork-chat');

const linuxServer = plan.packages.find((item) => item.id === 'linux-x64-server-archive');
assert.deepEqual(linuxServer.runtimePaths, {
  installRoot: '/opt/sdkwork/chat',
  configDir: '/etc/sdkwork/chat',
  dataDir: '/var/lib/sdkwork/chat',
  logDir: '/var/log/sdkwork/chat',
  runDir: '/run/sdkwork/chat',
});
assert.equal(linuxServer.databasePolicy.defaultEngine, 'postgresql');
assert.equal(linuxServer.databasePolicy.configFile.path, '/etc/sdkwork/chat/chat.toml');
assert.equal(linuxServer.databasePolicy.passwordFile.path, '/etc/sdkwork/chat/database.secret');
assert.ok(linuxServer.databasePolicy.envOverrides.includes('SDKWORK_IM_DATABASE_ENGINE'));
assert.ok(linuxServer.databasePolicy.envOverrides.includes('SDKWORK_IM_LOG_DIR'));

const windowsServer = plan.packages.find((item) => item.id === 'windows-x64-server-archive');
assert.deepEqual(windowsServer.runtimePaths, {
  installRoot: '%ProgramFiles%/sdkwork/chat',
  configDir: '%ProgramData%/sdkwork/chat',
  dataDir: '%ProgramData%/sdkwork/chat/Data',
  logDir: '%ProgramData%/sdkwork/chat/Logs',
  runDir: '%ProgramData%/sdkwork/chat/Run',
});

const linuxDesktop = plan.packages.find((item) => item.id === 'linux-x64-desktop');
assert.equal(linuxDesktop.databasePolicy.defaultEngine, 'sqlite');
assert.equal(linuxDesktop.databasePolicy.requiresExternalDatabase, false);
assert.equal(linuxDesktop.databasePolicy.configFile.path, '~/.sdkwork/chat/config/chat.toml');
assert.equal(linuxDesktop.databasePolicy.dataDirectory.path, '~/.sdkwork/chat/data');
assert.equal(linuxDesktop.databasePolicy.defaultSqlitePath, '~/.sdkwork/chat/data/chat.sqlite');
assert.ok(linuxDesktop.databasePolicy.envOverrides.includes('SDKWORK_IM_DATABASE_URL'));
assert.ok(linuxDesktop.databasePolicy.envOverrides.includes('SDKWORK_IM_DATABASE_FILE'));

const serverEnvTemplate = read('deployments/templates/server.env.example');
for (const required of [
  'SDKWORK_IM_DEPLOYMENT_MODE=server',
  'SDKWORK_IM_CONFIG_FILE=/etc/sdkwork/chat/chat.toml',
  'SDKWORK_IM_DATA_DIR=/var/lib/sdkwork/chat',
  'SDKWORK_IM_LOG_DIR=/var/log/sdkwork/chat',
  'SDKWORK_IM_RUN_DIR=/run/sdkwork/chat',
  'SDKWORK_IM_DATABASE_ENGINE=postgresql',
  'SDKWORK_IM_DATABASE_PASSWORD_FILE=/etc/sdkwork/chat/database.secret',
  'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18080',
  'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com',
  'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com',
  'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com',
  'SDKWORK_IM_PC_API_UPSTREAM=https://im.sdkwork.com',
  'SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true',
  'SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET=replace-with-secret-manager-app-context-signature-secret',
  'SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID=100001',
  'SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID=bootstrap',
  'SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET=replace-with-secret-manager-jwt-signing-secret',
]) {
  assert.ok(serverEnvTemplate.includes(required), `server.env.example must document ${required}`);
}
assert.doesNotMatch(serverEnvTemplate, /SDKWORK_IM_POSTGRES_/u);

const serverConfigTemplate = read('deployments/templates/server.yaml.example');
for (const required of [
  'instance:',
  'name: default',
  'appCode: chat',
  'deploymentMode: server',
  'configFile: /etc/sdkwork/chat/chat.toml',
  'dataDirectory: /var/lib/sdkwork/chat',
  'dataDir: /var/lib/sdkwork/chat',
  'logDirectory: /var/log/sdkwork/chat',
  'logDir: /var/log/sdkwork/chat',
  'runtimeDirectory: /run/sdkwork/chat',
  'runDir: /run/sdkwork/chat',
  'baseUrl: https://im.sdkwork.com',
  'apiBaseUrl: https://im.sdkwork.com',
  'websocketBaseUrl: wss://im.sdkwork.com',
  'engine: postgresql',
  'passwordFile: /etc/sdkwork/chat/database.secret',
]) {
  assert.ok(serverConfigTemplate.includes(required), `server.yaml.example must document ${required}`);
}

const desktopConfigTemplate = read('deployments/templates/desktop.toml.example');
for (const required of [
  'deployment_mode = "desktop"',
  'app_code = "chat"',
  'engine = "sqlite"',
  'file = "~/.sdkwork/chat/data/chat.sqlite"',
  'max_connections = 1',
]) {
  assert.ok(desktopConfigTemplate.includes(required), `desktop.toml.example must document ${required}`);
}

const databasePrefixRegistry = readJson('specs/database-prefix-registry.json');
const databaseTableRegistry = readJson('specs/database-table-registry.json');
assert.equal(databasePrefixRegistry.appCode, 'chat');
assert.equal(databasePrefixRegistry.product, 'sdkwork-chat');
assert.ok(
  databasePrefixRegistry.prefixes.some(
    (entry) =>
      entry.prefix === 'im' &&
      entry.businessDomain === 'instant_messaging' &&
      entry.status === 'ACTIVE',
  ),
  'chat database prefix registry must register im for instant messaging tables',
);
assert.equal(databasePrefixRegistry.nonImPrefixPolicy.mustNotUseImPrefix, true);
assert.ok(
  databaseTableRegistry.tables.every(
    (entry) =>
      entry.tableName.startsWith('im_') &&
      entry.modulePrefix === 'im' &&
      ['instant_messaging', 'social', 'organization', 'messaging', 'user'].includes(
        entry.boundedContext,
      ),
  ),
  'checked-in chat IM table registry entries must all use the im_ prefix and registered bounded contexts',
);

const docs = [
  readDeploymentDoc('postgresql-database-configuration.md'),
  readDeploymentDoc('database-table-naming-standard.md'),
  ...readDeploymentDocsMatching(/PostgreSQL.*\.md$/u),
].join('\n');
for (const required of [
  'SDKWORK_IM_DATABASE_ENGINE',
  'SDKWORK_IM_DATABASE_SSL_MODE',
  '/etc/sdkwork/chat/chat.toml',
  '/etc/sdkwork/chat/database.secret',
  '/sdkwork/chat',
  '~/.sdkwork/chat/data/chat.sqlite',
  'desktop',
  'SQLite',
  'im_',
  'Non-IM',
]) {
  assert.ok(docs.includes(required), `database docs must include ${required}`);
}
assert.doesNotMatch(docs, /SDKWORK_CLAW_DATABASE_PROVIDER|SDKWORK_CLAW_DATABASE_SSLMODE/u);

console.log('sdkwork-chat runtime standard contract passed');
