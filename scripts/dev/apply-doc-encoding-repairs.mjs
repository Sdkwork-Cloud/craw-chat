#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { collectMarkdownFiles, readRepoMarkdown, writeRepoMarkdown } from '../lib/doc-scan.mjs';
import { normalizeMarkdownEncoding } from '../lib/doc-encoding-normalize.mjs';
import { repairFffdContexts } from '../lib/doc-fffd-repair.mjs';
import { repairDocPhrases } from '../lib/doc-phrase-repair.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

let repairedCount = 0;
for (const relativePath of collectMarkdownFiles(repoRoot, 'docs')) {
  const absolutePath = path.join(repoRoot, relativePath);
  const buffer = fs.readFileSync(absolutePath);
  const rawUtf8 = buffer.toString('utf8');
  const before = normalizeMarkdownEncoding(buffer);
  const after = repairDocPhrases(repairFffdContexts(before));
  if (rawUtf8 !== after) {
    writeRepoMarkdown(repoRoot, relativePath, after);
    repairedCount += 1;
    process.stdout.write(`repaired ${relativePath}\n`);
  }
}

process.stdout.write(`doc encoding repairs applied to ${repairedCount} file(s)\n`);
