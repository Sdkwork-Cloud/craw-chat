#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const crateRoot = path.join(repoRoot, 'crates', 'im-app-context');
const srcRoot = path.join(crateRoot, 'src');

const sourceFiles = fs
  .readdirSync(srcRoot)
  .filter((name) => name.endsWith('.rs'))
  .sort();

assert.deepEqual(
  sourceFiles,
  ['lib.rs'],
  'im-app-context must keep a single compiled source root (src/lib.rs) until a wired module tree is introduced.',
);

const libSource = fs.readFileSync(path.join(srcRoot, 'lib.rs'), 'utf8');

for (const symbol of [
  'pub fn resolve_app_context',
  'pub async fn inject_app_request_context_middleware',
  'pub fn allows_header_only_app_context_fallback',
  'DEV_JWT_SIGNING_SECRET_FALLBACK',
]) {
  assert.match(
    libSource,
    new RegExp(symbol.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `im-app-context lib.rs must export or enforce ${symbol}.`,
  );
}

assert.match(
  libSource,
  /Production environment must not use the built-in dev\/test JWT signing secret/u,
  'im-app-context lib.rs must fail-closed when production uses the public dev JWT signing secret.',
);

process.stdout.write('sdkwork-im app-context module standard passed\n');
