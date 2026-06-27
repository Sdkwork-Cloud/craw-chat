#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function collectMarkdownFiles(relativeDir) {
  const absoluteDir = path.join(repoRoot, relativeDir);
  const results = [];
  for (const entry of fs.readdirSync(absoluteDir, { withFileTypes: true })) {
    const relativePath = path.join(relativeDir, entry.name);
    if (entry.isDirectory()) {
      results.push(...collectMarkdownFiles(relativePath));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith('.md')) {
      results.push(relativePath.replace(/\\/g, '/'));
    }
  }
  return results.sort();
}

const scanRoots = ['docs/review', 'docs/step'];

for (const root of scanRoots) {
  for (const relativePath of collectMarkdownFiles(root)) {
    const absolutePath = path.join(repoRoot, relativePath);
    const source = fs.readFileSync(absolutePath, 'utf8');
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
}

process.stdout.write('sdkwork-im review/step docs encoding standard passed\n');
