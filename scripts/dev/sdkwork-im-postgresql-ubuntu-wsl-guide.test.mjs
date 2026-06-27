import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  DEPLOYMENT_DOC_FILES,
  readDeploymentDoc,
  resolveDeploymentDocsRoot,
} from '../lib/deployment-docs.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

assert.ok(
  fs.existsSync(
    path.join(resolveDeploymentDocsRoot(repoRoot), DEPLOYMENT_DOC_FILES.ubuntuWslGuide),
  ),
  'Ubuntu/WSL PostgreSQL initialization guide must exist',
);

const guide = readDeploymentDoc(repoRoot, DEPLOYMENT_DOC_FILES.ubuntuWslGuide);
const index = readDeploymentDoc(repoRoot, DEPLOYMENT_DOC_FILES.postgresqlIndex);

for (const required of [
  'sudo apt update',
  'sudo apt install -y postgresql postgresql-contrib',
  'sudo systemctl enable --now postgresql',
  'sudo apt install -y redis-server',
  'sudo service redis-server start',
  'redis-cli ping',
  'PONG',
  'sudo -u postgres psql',
  'CREATE DATABASE sdkwork_ai_dev OWNER sdkwork_ai_dev',
  'CREATE SCHEMA IF NOT EXISTS sdkwork_ai_dev AUTHORIZATION sdkwork_ai_dev',
  'ALTER DEFAULT PRIVILEGES',
  'database/ddl/baseline/postgres/0001_im_legacy_baseline.sql',
  'psql -h 127.0.0.1 -p 5432 -U sdkwork_ai_dev -d sdkwork_ai_dev',
  'listen_addresses',
  'pg_hba.conf',
  '192.168.1.0/24',
  'sudo ufw allow from 192.168.1.0/24 to any port 5432 proto tcp',
  'WSL2 NAT',
  'mirrored',
  'netsh interface portproxy',
  'GRANT CONNECT ON DATABASE sdkwork_ai_dev TO sdkwork_ai_dev',
  'GRANT USAGE, CREATE ON SCHEMA sdkwork_ai_dev TO sdkwork_ai_dev',
  'SELECT current_database(), current_user, current_schema()',
  'Windows \u8dd1\u5e94\u7528\uff0cPostgreSQL \u8dd1\u5728 WSL Ubuntu',
  'Test-NetConnection 127.0.0.1 -Port 5432',
  'wsl hostname -I',
  'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
  'SDKWORK_IM_DATABASE_ENGINE=postgresql',
  'SDKWORK_IM_DATABASE_SSL_MODE=disable',
  'SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_dev',
  'SDKWORK_IM_REDIS_HOST=127.0.0.1',
  'SDKWORK_IM_REDIS_PORT=6379',
  'SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD',
  'pnpm db:postgres:plan',
  'pnpm db:postgres:init',
  'pnpm db:postgres:migrate',
  'pnpm dev',
  'pnpm dev:browser',
  'pnpm dev:desktop',
  'pnpm dev:browser:sqlite',
  'pnpm dev:desktop:sqlite',
  'pnpm dev:desktop \u9ed8\u8ba4\u4f7f\u7528 PostgreSQL',
  'host    sdkwork_ai_dev    sdkwork_ai_dev    127.0.0.1/32',
  'host    sdkwork_ai_dev    postgres          127.0.0.1/32',
  'host    sdkwork_ai_dev    sdkwork_ai_dev    <WINDOWS_HOST_CIDR>',
  'host    sdkwork_ai_dev    postgres          <WINDOWS_HOST_CIDR>',
  'database: sdkwork',
  'username: sdkwork',
  'Windows \u5e94\u7528\u4e0d\u5e94\u8be5\u4f7f\u7528 WSL \u5185\u90e8\u7684 Unix socket',
]) {
  assert.ok(guide.includes(required), `Ubuntu/WSL PostgreSQL guide must include: ${required}`);
}

assert.ok(
  guide.includes('scram-sha-256') || guide.includes('md5'),
  'Ubuntu/WSL PostgreSQL guide must document pg_hba authentication method',
);

assert.ok(
  index.includes(`./${DEPLOYMENT_DOC_FILES.ubuntuWslGuide}`),
  'PostgreSQL configuration index must link the Ubuntu/WSL initialization guide',
);

console.log('sdkwork-im PostgreSQL Ubuntu/WSL guide contract passed');
