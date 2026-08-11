#!/usr/bin/env node
// Re-mints the community commerce integration service token for the local
// standalone gateway.
//
// The circle tier publishing and paid-order verification flows call the
// membership/order backend business surfaces in-process (embedded in the IM
// standalone gateway). Those routes require a paired IAM session (dual-token:
// `Authorization` bearer auth token + `Access-Token` header) with
// `commerce.memberships.manage` / `commerce.orders.read`; the dev profile
// uses the demo `owner` account (granted the manage permission in dev data).
//
// The minted tokens are 30-day sessions; run this script to refresh them
// before expiry and it rewrites the four SDKWORK_*_BACKEND_*_TOKEN lines in
// `etc/topology/standalone.development.env`, then restart the gateway
// (`pnpm dev`) for the new tokens to take effect.

import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { createDevBootstrapAccessTokenJwt } from '../../../sdkwork-iam/scripts/dev/create-dev-bootstrap-access-token-env.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(__dirname, '..', '..');
const TOPOLOGY_ENV = path.join(REPO_ROOT, 'etc', 'topology', 'standalone.development.env');

const GATEWAY_BASE_URL = process.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL ?? 'http://127.0.0.1:18089';
const OWNER_USERNAME = 'owner';
const OWNER_PASSWORD = 'Owner#2026';

async function mintOwnerSessionTokens() {
  const bootstrapToken = createDevBootstrapAccessTokenJwt({
    appId: 'sdkwork-im-pc',
    environment: 'development',
  });
  const response = await fetch(`${GATEWAY_BASE_URL}/app/v3/api/auth/sessions`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      'access-token': bootstrapToken,
    },
    body: JSON.stringify({
      grantType: 'password',
      username: OWNER_USERNAME,
      password: OWNER_PASSWORD,
    }),
  });
  if (!response.ok) {
    const detail = await response.text().catch(() => '');
    throw new Error(`owner login failed (${response.status}): ${detail.slice(0, 300)}`);
  }
  const body = await response.json();
  const authToken = body?.data?.authToken;
  const accessToken = body?.data?.accessToken;
  if (typeof authToken !== 'string' || authToken.length === 0) {
    throw new Error('owner login did not return an auth token');
  }
  if (typeof accessToken !== 'string' || accessToken.length === 0) {
    throw new Error('owner login did not return an access token');
  }
  return { authToken, accessToken };
}

function rewriteTokenLines(envPath, authToken, accessToken) {
  const raw = fs.readFileSync(envPath, 'utf8');
  const lines = raw.split(/\r?\n/u);
  const remaining = lines.filter((line) => {
    const trimmed = line.trim();
    return (
      !trimmed.startsWith('SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN=') &&
      !trimmed.startsWith('SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN=') &&
      !trimmed.startsWith('SDKWORK_ORDER_BACKEND_AUTH_TOKEN=') &&
      !trimmed.startsWith('SDKWORK_ORDER_BACKEND_ACCESS_TOKEN=')
    );
  });
  remaining.push(
    `SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN=${authToken}`,
    `SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN=${accessToken}`,
    `SDKWORK_ORDER_BACKEND_AUTH_TOKEN=${authToken}`,
    `SDKWORK_ORDER_BACKEND_ACCESS_TOKEN=${accessToken}`,
    '',
  );
  fs.writeFileSync(envPath, remaining.join('\n'));
}

async function main() {
  const { authToken, accessToken } = await mintOwnerSessionTokens();
  rewriteTokenLines(TOPOLOGY_ENV, authToken, accessToken);
  console.log('minted 30-day commerce integration session (owner) and wrote:');
  console.log(`  ${TOPOLOGY_ENV}`);
  console.log('restart the gateway (`pnpm dev`) for the tokens to take effect.');
}

main().catch((error) => {
  console.error(error.message);
  process.exit(1);
});
