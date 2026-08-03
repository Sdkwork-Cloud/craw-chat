#!/usr/bin/env node
// Ensures the IM H5 demo user accounts exist in the local standalone IAM.
//
// Accounts are created through the IAM registration API
// (POST /app/v3/api/auth/registrations) so password hashing and account
// lifecycle stay inside the IAM boundary. The script is idempotent: existing
// usernames are skipped.
//
// Usage:
//   node scripts/dev/ensure-im-h5-demo-users.mjs [--base-url http://127.0.0.1:18079]
//
// The IM demo seed (database/seeds/common/002_im_demo_data.sql) resolves these
// users by login name and fails closed when they are absent, so run this script
// before `pnpm db:seed`.

import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { createDevBootstrapAccessTokenJwt } from '../../../sdkwork-iam/scripts/dev/create-dev-bootstrap-access-token-env.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(__dirname, '..', '..');

const DEFAULT_BASE_URL = 'http://127.0.0.1:18079';

const DEMO_USERS = [
  { username: 'owner', password: 'Owner#2026', email: 'owner@sdkwork.local' },
  { username: 'guest', password: 'Guest#2026', email: 'guest@sdkwork.local' },
  { username: 'alice', password: 'Alice#2026', email: 'alice@sdkwork.local' },
  { username: 'bob', password: 'Bob#2026', email: 'bob@sdkwork.local' },
  { username: 'grace', password: 'Grace#2026', email: 'grace@sdkwork.local' },
];

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function bootstrapAccessToken() {
  return createDevBootstrapAccessTokenJwt({
    appId: 'sdkwork-im-pc',
    environment: 'development',
  });
}

function parseArgs(argv) {
  const settings = { baseUrl: DEFAULT_BASE_URL };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--base-url') {
      settings.baseUrl = normalizeText(argv[index + 1]) ?? DEFAULT_BASE_URL;
      index += 1;
      continue;
    }
    if (arg === '--help' || arg === '-h') {
      settings.help = true;
      continue;
    }
    throw new Error(`unknown argument: ${arg}`);
  }
  return settings;
}

async function existingDemoUsernames({ pg }) {
  const result = await pg.query(
    `SELECT username FROM iam_user WHERE username = ANY($1) AND is_deleted = 0`,
    [DEMO_USERS.map((user) => user.username)],
  );
  return new Set(result.rows.map((row) => row.username));
}

async function registerUser({ baseUrl, accessToken, user }) {
  const response = await fetch(`${baseUrl}/app/v3/api/auth/registrations`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      'access-token': accessToken,
    },
    body: JSON.stringify({
      username: user.username,
      email: user.email,
      password: user.password,
      confirmPassword: user.password,
    }),
  });
  if (response.status === 409 || response.status === 400) {
    // Account already exists or the tenant rejects duplicates; treat as
    // already-provisioned so the script stays idempotent.
    const body = await response.json().catch(() => ({}));
    const code = String(body?.code ?? '');
    if (code.includes('exists') || code.includes('conflict')) {
      return { status: 'already-exists', username: user.username };
    }
    if (code.includes('register')) {
      return { status: 'already-exists', username: user.username };
    }
  }
  if (!response.ok) {
    const detail = await response.text().catch(() => '');
    throw new Error(
      `register ${user.username} failed (${response.status}): ${detail.slice(0, 300)}`,
    );
  }
  return { status: 'registered', username: user.username };
}

async function main() {
  const settings = parseArgs(process.argv.slice(2));
  if (settings.help) {
    console.log('Ensure the IM H5 demo users exist in the local IAM.');
    console.log('Usage: node scripts/dev/ensure-im-h5-demo-users.mjs [--base-url URL]');
    return;
  }

  // Read the local PostgreSQL credentials so existing accounts can be
  // detected without touching the IAM write path.
  const envText = fs.readFileSync(path.join(REPO_ROOT, '.env.postgres'), 'utf8');
  const env = Object.fromEntries(
    envText
      .split(/\r?\n/u)
      .map((line) => line.trim())
      .filter((line) => line && !line.startsWith('#'))
      .map((line) => {
        const separator = line.indexOf('=');
        return [line.slice(0, separator), line.slice(separator + 1)];
      }),
  );
  const { default: pg } = await import('pg');
  const client = new pg.Client({
    host: env.SDKWORK_DATABASE_HOST ?? '127.0.0.1',
    port: Number(env.SDKWORK_DATABASE_PORT ?? 5432),
    user: env.SDKWORK_DATABASE_USERNAME ?? 'sdkwork_ai_dev',
    password: env.SDKWORK_DATABASE_PASSWORD,
    database: env.SDKWORK_DATABASE_NAME ?? 'sdkwork_ai_dev',
  });
  await client.connect();
  try {
    await client.query(`SET search_path TO ${env.SDKWORK_DATABASE_SCHEMA ?? 'sdkwork_ai_dev'}`);
    const existing = await existingDemoUsernames({ pg: client });
    const results = [];
    for (const user of DEMO_USERS) {
      if (existing.has(user.username)) {
        results.push({ status: 'exists', username: user.username });
        continue;
      }
      results.push(await registerUser({ baseUrl: settings.baseUrl, accessToken: bootstrapAccessToken(), user }));
    }
    for (const result of results) {
      console.log(`[demo-users] ${result.status.padEnd(14)} ${result.username}`);
    }
  } finally {
    await client.end();
  }
}

main().catch((error) => {
  console.error(`[demo-users] failed: ${error.message}`);
  process.exit(1);
});
