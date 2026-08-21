#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const root = path.join(repoRoot, 'artifacts', 'perf', 'step-11');

const replacements = [
  ['"profile": "local-default"', '"profile": "standalone.development"'],
  ['"sourceProfile": "local-minimal"', '"sourceProfile": "standalone.development"'],
  [
    'CI Smoke Tier / local-minimal output',
    'CI Smoke Tier / standalone.development output',
  ],
  [
    'CI Smoke Tier / local-minimal evidence',
    'CI Smoke Tier / standalone.development evidence',
  ],
  ['local-default / capacity-dedicated', 'standalone.development / capacity-dedicated'],
  ['local-minimal evidence backfill', 'standalone.development evidence backfill'],
  ['for local-minimal evidence', 'for standalone.development evidence'],
  [
    'profile = local-default / capacity-dedicated',
    'profile = standalone.development / capacity-dedicated',
  ],
];

function walk(dir) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const absolute = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(absolute);
      continue;
    }
    if (!/\.(?:json|md)$/iu.test(entry.name)) {
      continue;
    }
    let content = fs.readFileSync(absolute, 'utf8');
    const original = content;
    for (const [from, to] of replacements) {
      content = content.replaceAll(from, to);
    }
    if (content !== original) {
      fs.writeFileSync(absolute, content);
      console.log(`updated ${path.relative(repoRoot, absolute)}`);
    }
  }
}

walk(root);
console.log('step-11 artifact profile migration complete');
