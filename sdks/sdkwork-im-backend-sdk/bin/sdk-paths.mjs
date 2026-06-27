#!/usr/bin/env node
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function isSdkworkGeneratorRoot(candidate) {
  return existsSync(path.join(candidate, 'bin', 'sdkgen.js'));
}

export function resolveSdkworkGeneratorRoot(workspaceRoot) {
  const override = process.env.SDKWORK_GENERATOR_ROOT;
  if (override && override.trim()) {
    const resolvedOverride = path.resolve(override.trim());
    if (!isSdkworkGeneratorRoot(resolvedOverride)) {
      throw new Error(`SDKWORK_GENERATOR_ROOT does not point to sdkwork-sdk-generator: ${resolvedOverride}`);
    }
    return resolvedOverride;
  }
  let current = path.resolve(workspaceRoot);
  while (true) {
    const candidate = path.join(current, 'sdk', 'sdkwork-sdk-generator');
    if (isSdkworkGeneratorRoot(candidate)) {
      return candidate;
    }
    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`Unable to resolve sdkwork-sdk-generator from workspace root: ${workspaceRoot}`);
}

const isDirectExecution =
  process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isDirectExecution) {
  const workspaceIndex = process.argv.indexOf('--workspace');
  const workspace = workspaceIndex >= 0 ? process.argv[workspaceIndex + 1] : path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
  process.stdout.write(resolveSdkworkGeneratorRoot(workspace));
}
