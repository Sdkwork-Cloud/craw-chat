/**
 * Fix formatting of Build Source Integrity notes in AGENTS.md files.
 *
 * Fixes:
 * 1. Remove extra blank line before the note (triple newline -> double newline)
 * 2. Add missing blank line between note and next ## section header
 */

import fs from 'node:fs';

const LIST_FILE = 'e:/sdkwork-space/agents_list.txt';
const SKIP_PATTERNS = [/node_modules/i, /external\//i, /target[\\/]/i, /hosted-runtime-tests/i, /\.codex/i];

const NOTE_LINE = 'Build scripts, dev runners, and `pnpm clean` must follow `CODE_STYLE_SPEC.md` §7 (Build Source Integrity And Self-Healing). Git-tracked build-critical source files must be verified before builds and self-healed from git when missing; `clean` must not delete them.';

function shouldSkip(filePath) {
  return SKIP_PATTERNS.some((pattern) => pattern.test(filePath));
}

function fixFormatting(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');
  let changed = false;

  // Fix 1: Remove extra blank line before the note
  // Pattern: \n\n\nBuild scripts -> \n\nBuild scripts
  if (content.includes('\n\n\n' + NOTE_LINE)) {
    content = content.replace('\n\n\n' + NOTE_LINE, '\n\n' + NOTE_LINE);
    changed = true;
  }

  // Fix 2: Add blank line after note if directly followed by ## header
  // Pattern: ...clean must not delete them.\n## -> ...\n\n##
  if (content.includes(NOTE_LINE + '\n## ')) {
    content = content.replace(NOTE_LINE + '\n## ', NOTE_LINE + '\n\n## ');
    changed = true;
  }

  if (changed) {
    fs.writeFileSync(filePath, content, 'utf8');
  }

  return changed;
}

const fileList = fs.readFileSync(LIST_FILE, 'utf8')
  .split(/\r?\n/)
  .map((line) => line.trim())
  .filter(Boolean);

let fixed = 0;
let skipped = 0;

for (const filePath of fileList) {
  if (shouldSkip(filePath) || !fs.existsSync(filePath)) {
    skipped++;
    continue;
  }

  try {
    if (fixFormatting(filePath)) {
      fixed++;
    }
  } catch (err) {
    console.error(`Error fixing ${filePath}: ${err.message}`);
  }
}

console.log(`\n=== Format Fix ===\n`);
console.log(`Fixed: ${fixed}`);
console.log(`Skipped: ${skipped}`);
console.log(`Done.`);
