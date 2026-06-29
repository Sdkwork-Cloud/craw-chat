/**
 * Batch-update all SDKWork AGENTS.md files to add Build Source Integrity reference.
 *
 * Rules:
 * - Skip paths containing node_modules, external/, target/, or hosted-runtime-tests
 * - Skip files that already mention "Build Source Integrity" or "§7"
 * - For files with "## Code Style Rules" section: append the note at end of that section
 * - For files without that section but with CODE_STYLE_SPEC.md reference: append after the paragraph
 * - For files with neither: skip and report
 */

import fs from 'node:fs';
import path from 'node:path';

const LIST_FILE = 'e:/sdkwork-space/agents_list.txt';
const SKIP_PATTERNS = [/node_modules/i, /external\//i, /target\//i, /hosted-runtime-tests/i, /\.codex/i];

const BUILD_INTEGRITY_NOTE =
  '\nBuild scripts, dev runners, and `pnpm clean` must follow `CODE_STYLE_SPEC.md` §7 (Build Source Integrity And Self-Healing). Git-tracked build-critical source files must be verified before builds and self-healed from git when missing; `clean` must not delete them.';

function shouldSkip(filePath) {
  return SKIP_PATTERNS.some((pattern) => pattern.test(filePath));
}

function alreadyHasNote(content) {
  return /Build Source Integrity|build source integrity|§7/i.test(content);
}

function findCodeStyleRulesSectionEnd(content) {
  const sectionMatch = content.match(/^## Code Style Rules\s*$/m);
  if (!sectionMatch) return null;

  const startIndex = sectionMatch.index + sectionMatch[0].length;
  const nextSectionMatch = content.slice(startIndex).match(/^## /m);
  if (!nextSectionMatch) return content.length;

  return startIndex + nextSectionMatch.index;
}

function findCodeStyleSpecParagraphEnd(content) {
  const match = content.match(/CODE_STYLE_SPEC\.md[^\n]*\n/);
  if (!match) return null;

  const endIndex = match.index + match[0].length;
  // Find the end of the paragraph block (next blank line or next ##)
  const afterMatch = content.slice(endIndex);
  const nextBlankOrHeader = afterMatch.match(/\n\s*\n|\n## /);

  if (nextBlankOrHeader) {
    return endIndex + nextBlankOrHeader.index;
  }

  return content.length;
}

function updateFile(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');

  if (alreadyHasNote(content)) {
    return { status: 'skip', reason: 'already has build source integrity note' };
  }

  const codeStyleEnd = findCodeStyleRulesSectionEnd(content);

  if (codeStyleEnd !== null) {
    // Insert at the end of "## Code Style Rules" section, before the next ## section
    const before = content.slice(0, codeStyleEnd);
    const after = content.slice(codeStyleEnd);
    const needsNewlineBefore = !before.endsWith('\n\n');
    const newContent = before + (needsNewlineBefore ? '\n' : '') + BUILD_INTEGRITY_NOTE + '\n' + after;
    fs.writeFileSync(filePath, newContent, 'utf8');
    return { status: 'updated', reason: 'appended to Code Style Rules section' };
  }

  // No Code Style Rules section - try to find CODE_STYLE_SPEC.md reference
  const specParaEnd = findCodeStyleSpecParagraphEnd(content);
  if (specParaEnd !== null) {
    const before = content.slice(0, specParaEnd);
    const after = content.slice(specParaEnd);
    const newContent = before + BUILD_INTEGRITY_NOTE + '\n' + after;
    fs.writeFileSync(filePath, newContent, 'utf8');
    return { status: 'updated', reason: 'appended after CODE_STYLE_SPEC.md reference' };
  }

  return { status: 'skip', reason: 'no Code Style Rules section or CODE_STYLE_SPEC.md reference found' };
}

const fileList = fs.readFileSync(LIST_FILE, 'utf8')
  .split(/\r?\n/)
  .map((line) => line.trim())
  .filter(Boolean);

const results = { updated: [], skipped: [], errors: [] };

for (const filePath of fileList) {
  if (shouldSkip(filePath)) {
    results.skipped.push({ file: filePath, reason: 'path matches skip pattern' });
    continue;
  }

  if (!fs.existsSync(filePath)) {
    results.skipped.push({ file: filePath, reason: 'file not found' });
    continue;
  }

  try {
    const result = updateFile(filePath);
    if (result.status === 'updated') {
      results.updated.push({ file: filePath, reason: result.reason });
    } else {
      results.skipped.push({ file: filePath, reason: result.reason });
    }
  } catch (err) {
    results.errors.push({ file: filePath, error: err.message });
  }
}

console.log(`\n=== Build Source Integrity Batch Update ===\n`);
console.log(`Updated: ${results.updated.length}`);
console.log(`Skipped: ${results.skipped.length}`);
console.log(`Errors:  ${results.errors.length}`);

if (results.updated.length > 0) {
  console.log(`\n--- Updated files ---`);
  for (const item of results.updated) {
    console.log(`  ${item.file}`);
    console.log(`    (${item.reason})`);
  }
}

if (results.errors.length > 0) {
  console.log(`\n--- Errors ---`);
  for (const item of results.errors) {
    console.log(`  ${item.file}: ${item.error}`);
  }
}

const skippedNoMatch = results.skipped.filter((r) => !r.reason.includes('skip pattern') && !r.reason.includes('file not found') && !r.reason.includes('already has'));
if (skippedNoMatch.length > 0) {
  console.log(`\n--- Skipped (no Code Style section found) ---`);
  for (const item of skippedNoMatch) {
    console.log(`  ${item.file}`);
  }
}

console.log(`\nDone.`);
