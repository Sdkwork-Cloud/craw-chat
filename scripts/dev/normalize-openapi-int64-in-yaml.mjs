#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const targets = process.argv.slice(2);
if (targets.length === 0) {
  console.error('usage: normalize-openapi-int64-in-yaml.mjs <file> [...]');
  process.exit(1);
}

const int64StringBlock = (indent, { nullable = false } = {}) => {
  const lines = [
    `${indent}type: string`,
    `${indent}format: int64`,
    `${indent}pattern: '^[0-9]+$'`,
    `${indent}x-sdkwork-int64-string: true`,
    `${indent}x-sdkwork-rust-type: i64`,
  ];
  if (nullable) {
    lines.push(`${indent}nullable: true`);
  }
  return lines.join('\n');
};

function normalizeInt64(content) {
  let updated = content;

  updated = updated.replace(
    /^([ \t]*)type: integer\r?\n\1format: int64\r?\n(?:\1minimum: 0\r?\n)?(\1nullable: true\r?\n)?/gmu,
    (_, indent, nullableLine) => `${int64StringBlock(indent, { nullable: Boolean(nullableLine) })}\n`,
  );

  updated = updated.replace(
    /^([ \t]*)format: int64\r?\n\1type: integer\r?\n(\1nullable: true\r?\n)?/gmu,
    (_, indent, nullableLine) => `${int64StringBlock(indent, { nullable: Boolean(nullableLine) })}\n`,
  );

  return updated;
}

for (const target of targets) {
  const filePath = path.resolve(repoRoot, target);
  const before = fs.readFileSync(filePath, 'utf8');
  const after = normalizeInt64(before);
  if (after !== before) {
    fs.writeFileSync(filePath, after);
    console.log(`normalized int64 strings in ${target}`);
  }
}
