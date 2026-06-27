import { existsSync, readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';

export function readWorkspaceSource({ workspaceRoot, relativePath }) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

export function readWorkspaceSources({ workspaceRoot, files }) {
  return Object.fromEntries(
    Object.entries(files).map(([key, relativePath]) => [
      key,
      readWorkspaceSource({ workspaceRoot, relativePath }),
    ]),
  );
}

export function readWorkspaceJson({ workspaceRoot, relativePath }) {
  return JSON.parse(readWorkspaceSource({ workspaceRoot, relativePath }));
}

export function readWorkspaceYamlScalar({ workspaceRoot, relativePath, key }) {
  const source = readWorkspaceSource({ workspaceRoot, relativePath });
  const match = source.match(new RegExp(`^${key}:\\s*(.+)$`, 'm'));
  return match ? match[1].trim() : '';
}

export function workspacePathExists({ workspaceRoot, relativePath }) {
  return existsSync(path.join(workspaceRoot, relativePath));
}

export function collectWorkspaceFiles({
  workspaceRoot,
  relativeRoot = '.',
  include = ({ entry }) => entry.isFile(),
}) {
  const rootDirectory = path.join(workspaceRoot, relativeRoot);
  const queue = [rootDirectory];
  const files = [];

  while (queue.length > 0) {
    const currentDirectory = queue.shift();
    for (const entry of readdirSync(currentDirectory, { withFileTypes: true })) {
      const absolutePath = path.join(currentDirectory, entry.name);
      const relativePath = path.relative(workspaceRoot, absolutePath).replace(/\\/g, '/');

      if (entry.isDirectory()) {
        queue.push(absolutePath);
        continue;
      }

      if (include({ absolutePath, relativePath, entry })) {
        files.push(relativePath);
      }
    }
  }

  return files;
}

export function collectExpectationFailures(expectations) {
  return expectations
    .filter((expectation) => {
      const matched = expectation.pattern.test(expectation.source);
      return expectation.negate ? matched : !matched;
    })
    .map((expectation) => expectation.description);
}

export function finishFileExpectationVerification({
  prefix,
  failures,
  failureHeader = 'File expectation verification failed:',
  successMessage,
}) {
  if (failures.length > 0) {
    console.error(`[${prefix}] ${failureHeader}`);
    for (const failure of failures) {
      console.error(`- Missing: ${failure}`);
    }
    process.exit(1);
  }

  console.log(successMessage || `[${prefix}] File expectation verification passed.`);
}
