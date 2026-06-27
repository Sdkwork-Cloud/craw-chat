#!/usr/bin/env node
import assert from 'node:assert/strict';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { collectMarkdownFiles, readRepoMarkdown } from '../lib/doc-scan.mjs';
import { findStripDamageHits } from '../lib/doc-strip-damage-patterns.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const scanRoots = ['docs'];

const allHits = [];
for (const root of scanRoots) {
  for (const relativePath of collectMarkdownFiles(repoRoot, root)) {
    const source = readRepoMarkdown(repoRoot, relativePath);
    allHits.push(...findStripDamageHits(source, relativePath));
  }
}

if (allHits.length > 0) {
  const summary = allHits
    .slice(0, 20)
    .map((hit) => `${hit.relativePath}: ${hit.label} (sample: ${hit.sample})`)
    .join('\n');
  const suffix = allHits.length > 20 ? `\n... and ${allHits.length - 20} more` : '';
  assert.fail(`docs strip-damage standard failed (${allHits.length} hit(s)):\n${summary}${suffix}`);
}

process.stdout.write('sdkwork-im docs strip-damage standard passed\n');
