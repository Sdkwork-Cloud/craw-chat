import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import pg from 'pg';

import { buildPostgresDatabaseUrl } from './sdkwork-im-postgres-db.mjs';

const MODULE_ID = 'im';
const ENGINE = 'postgres';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function fileChecksum(filePath) {
  const bytes = fs.readFileSync(filePath);
  return crypto.createHash('sha256').update(bytes).digest('hex');
}

function listPostgresMigrationFiles(repoRoot) {
  const migrationsDir = path.join(repoRoot, 'database', 'migrations', 'postgres');
  if (!fs.existsSync(migrationsDir)) {
    return [];
  }
  return fs
    .readdirSync(migrationsDir)
    .filter((fileName) => fileName.endsWith('.up.sql'))
    .sort()
    .map((fileName) => {
      const version = fileName.slice(0, fileName.indexOf('_'));
      const name = fileName.slice(fileName.indexOf('_') + 1, -'.up.sql'.length);
      return {
        fileName,
        name,
        path: path.join(migrationsDir, fileName),
        version,
      };
    });
}

export async function repairPostgresMigrationChecksums({
  config,
  repoRoot,
  stdout = process.stdout,
} = {}) {
  if (!config) {
    throw new Error('repairPostgresMigrationChecksums requires parsed PostgreSQL config');
  }
  if (!repoRoot) {
    throw new Error('repairPostgresMigrationChecksums requires repoRoot');
  }

  const migrations = listPostgresMigrationFiles(repoRoot);
  if (migrations.length === 0) {
    return { repaired: [] };
  }

  const adminUrl = buildPostgresDatabaseUrl({
    ...config.admin,
    database: config.database.database,
  });
  const client = new pg.Client({ connectionString: adminUrl });
  await client.connect();

  try {
    const schema = normalizeText(config.database.schema) ?? 'public';
    await client.query(`SET search_path TO ${schema}`);

    const repaired = [];
    for (const migration of migrations) {
      const currentChecksum = fileChecksum(migration.path);
      const existing = await client.query(
        `SELECT checksum
         FROM ops_schema_migration_history
         WHERE module_id = $1 AND version = $2 AND engine = $3`,
        [MODULE_ID, migration.version, ENGINE],
      );
      if (existing.rows.length === 0) {
        continue;
      }
      const appliedChecksum = existing.rows[0].checksum;
      if (appliedChecksum === currentChecksum) {
        continue;
      }

      const update = await client.query(
        `UPDATE ops_schema_migration_history
         SET checksum = $1
         WHERE module_id = $2 AND version = $3 AND engine = $4`,
        [currentChecksum, MODULE_ID, migration.version, ENGINE],
      );
      if (update.rowCount > 0) {
        repaired.push({
          appliedChecksum,
          currentChecksum,
          version: migration.version,
        });
        stdout.write(
          `[sdkwork-im-db] repaired migration ${migration.version} checksum in schema ${schema}\n`
          + `  applied: ${appliedChecksum}\n`
          + `  current: ${currentChecksum}\n`,
        );
      }
    }

    return { repaired };
  } finally {
    await client.end();
  }
}
