import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

export function resolveWorkspacePath(workspaceRoot, ...relativeSegments) {
  return path.join(workspaceRoot, ...relativeSegments);
}

export function makeRead(workspaceRoot) {
  return function read(relativePath) {
    return readFileSync(resolveWorkspacePath(workspaceRoot, relativePath), 'utf8');
  };
}

export function collectMissingPaths({ workspaceRoot, requiredPaths }) {
  return requiredPaths.filter(
    (relativePath) => !existsSync(resolveWorkspacePath(workspaceRoot, relativePath)),
  );
}

export function appendMissingPathFailures({
  workspaceRoot,
  requiredPaths,
  failures,
  formatFailure = (relativePath) => `Missing required path: ${relativePath}`,
}) {
  for (const relativePath of collectMissingPaths({ workspaceRoot, requiredPaths })) {
    failures.push(formatFailure(relativePath));
  }
}

export function ensureRequiredPaths({
  workspaceRoot,
  requiredPaths,
  prefix,
  header = 'Missing required workspace automation files:',
}) {
  const missing = collectMissingPaths({ workspaceRoot, requiredPaths });
  if (missing.length === 0) {
    return;
  }

  console.error(`[${prefix}] ${header}`);
  for (const relativePath of missing) {
    console.error(`- ${relativePath}`);
  }
  process.exit(1);
}

export function requireMatch({ source, pattern, message, failures }) {
  if (!pattern.test(source)) {
    failures.push(message);
  }
}

export function requireNotMatch({ source, pattern, message, failures }) {
  if (pattern.test(source)) {
    failures.push(message);
  }
}

export function requireIncludes({ source, value, message, failures }) {
  if (!source.includes(value)) {
    failures.push(message);
  }
}

export function finishVerification({ prefix, failures }) {
  if (failures.length > 0) {
    console.error(`[${prefix}] SDK automation verification failed:`);
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log(`[${prefix}] SDK automation verification passed.`);
}
