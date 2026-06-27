#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { resolveDeploymentDocsRoot } from '../lib/deployment-docs.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const deploymentDir = resolveDeploymentDocsRoot(repoRoot);
const markdownFiles = fs
  .readdirSync(deploymentDir)
  .filter((entry) => entry.endsWith('.md'))
  .sort((left, right) => left.localeCompare(right));

assert.ok(markdownFiles.length >= 10, 'deployment docs directory must publish the standard guide set');

for (const fileName of markdownFiles) {
  const source = fs.readFileSync(path.join(deploymentDir, fileName), 'utf8');
  assert.doesNotMatch(
    source,
    /\uFFFD/u,
    `${fileName} must not contain UTF-8 replacement characters (encoding corruption)`,
  );
  assert.doesNotMatch(
    source,
    /å…¼å®¹|ç›®çš„|æœ¬é¡µ/u,
    `${fileName} must not contain mojibake from mis-decoded UTF-8`,
  );
}

process.stdout.write('sdkwork-im deployment docs encoding standard passed\n');
