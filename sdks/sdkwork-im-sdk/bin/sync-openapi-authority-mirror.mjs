#!/usr/bin/env node
import assert from 'node:assert/strict';
import { copyFileSync, existsSync, mkdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const repoRoot = path.resolve(workspaceRoot, '..', '..');
const authorityPath = path.join(repoRoot, 'apis', 'open-api', 'im', 'sdkwork-im-im.openapi.yaml');
const mirrorPath = path.join(workspaceRoot, 'openapi', 'sdkwork-im-im.openapi.yaml');

assert.ok(
  existsSync(authorityPath),
  `${path.relative(repoRoot, authorityPath)} must exist before syncing the SDK OpenAPI mirror`,
);

mkdirSync(path.dirname(mirrorPath), { recursive: true });
copyFileSync(authorityPath, mirrorPath);
process.stdout.write(
  `[sdkwork-im-sdk] synced OpenAPI mirror from ${path.relative(repoRoot, authorityPath).replaceAll('\\', '/')}\n`,
);
