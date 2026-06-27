import assert from 'node:assert/strict';
import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const repoRoot = path.resolve(import.meta.dirname, '..');

const rootsToScan = [
  '.github',
  'scripts',
  'sdks',
  'package.json',
  'Cargo.toml',
  'sdkwork.workflow.json',
];

const textExtensions = new Set([
  '.json',
  '.mjs',
  '.js',
  '.ts',
  '.tsx',
  '.yml',
  '.yaml',
  '.toml',
  '.ps1',
  '.sh',
  '.md',
]);

const ignoredDirectoryNames = new Set([
  '.git',
  '.dart_tool',
  'node_modules',
  'dist',
  'target',
]);

const ignoredPathSegments = [
  '/generated/server-openapi/',
  '/pubspec.lock',
  '/package-lock.json',
  '/pnpm-lock.yaml',
];

function* walk(relativePath) {
  const absolutePath = path.join(repoRoot, relativePath);
  const entries = readdirSync(absolutePath, { withFileTypes: true });
  for (const entry of entries) {
    const childRelative = path.join(relativePath, entry.name);
    const childNormalized = childRelative.replace(/\\/g, '/');
    if (entry.isDirectory()) {
      if (ignoredDirectoryNames.has(entry.name)) {
        continue;
      }
      if (ignoredPathSegments.some((segment) => childNormalized.includes(segment))) {
        continue;
      }
      yield* walk(childRelative);
      continue;
    }
    if (!entry.isFile()) {
      continue;
    }
    if (ignoredPathSegments.some((segment) => childNormalized.includes(segment))) {
      continue;
    }
    if (!textExtensions.has(path.extname(entry.name).toLowerCase())) {
      continue;
    }
    yield childRelative;
  }
}

function filesToScan() {
  const files = [];
  for (const root of rootsToScan) {
    const absolutePath = path.join(repoRoot, root);
    const extension = path.extname(root).toLowerCase();
    if (extension) {
      files.push(root);
      continue;
    }
    for (const file of walk(root)) {
      files.push(file);
    }
  }
  return files;
}

test('source and build inputs do not contain machine-specific absolute paths', () => {
  const violations = [];
  const forbiddenPathPattern = /\b[A-Z]:[\\/]|\/home\/[^/\s"'`]+\/|\/Users\/[^/\s"'`]+\/|\/mnt\/[a-z]\//u;

  for (const relativePath of filesToScan()) {
    const source = readFileSync(path.join(repoRoot, relativePath), 'utf8');
    const match = forbiddenPathPattern.exec(source);
    if (match) {
      violations.push(`${relativePath.replace(/\\/g, '/')}: ${match[0]}`);
    }
  }

  assert.deepEqual(
    violations,
    [],
    'source/build paths must be repo-relative, runner-provided, temp-dir based, or documented placeholders',
  );
});
