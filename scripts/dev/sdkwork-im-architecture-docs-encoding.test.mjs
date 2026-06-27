#!/usr/bin/env node
import assert from 'node:assert/strict';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { collectMarkdownFiles, readRepoMarkdown } from '../lib/doc-scan.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

for (const relativePath of collectMarkdownFiles(repoRoot, 'docs/架构')) {
  const source = readRepoMarkdown(repoRoot, relativePath);
  assert.doesNotMatch(
    source,
    /\uFFFD/u,
    `${relativePath} must not contain UTF-8 replacement characters (encoding corruption)`,
  );
  assert.doesNotMatch(
    source,
    /å…¼å®¹|ç›®çš„|æœ¬é¡µ|ï¼ˆ|ä¾|æœ¬è½®/u,
    `${relativePath} must not contain latin1 mojibake from mis-decoded UTF-8`,
  );
}

process.stdout.write('sdkwork-im architecture docs encoding standard passed\n');
