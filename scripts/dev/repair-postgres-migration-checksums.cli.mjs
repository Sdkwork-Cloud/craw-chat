import process from 'node:process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

import { parsePostgresConfig } from './sdkwork-im-postgres-db.mjs';
import { repairPostgresMigrationChecksums } from './repair-postgres-migration-checksums.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

const config = parsePostgresConfig({
  configPath: process.argv.includes('--config')
    ? process.argv[process.argv.indexOf('--config') + 1]
    : '.env.postgres',
  repoRoot,
});
const result = await repairPostgresMigrationChecksums({
  config,
  repoRoot,
  stdout: process.stdout,
});

if (result.repaired.length === 0) {
  process.stdout.write('[sdkwork-im-db] no PostgreSQL migration checksum repairs were needed\n');
}
