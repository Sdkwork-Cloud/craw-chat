import { existsSync, readFileSync, rmSync } from 'node:fs';
import path from 'node:path';

export function failTypescriptGeneratedBuildVerification({ prefix, message }) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function resolveTypescriptGeneratedBuildVerifyPaths({
  workspaceRoot,
  scriptDir,
}) {
  const generatedRoot = path.join(
    workspaceRoot,
    'sdkwork-im-sdk-typescript',
    'generated',
    'server-openapi',
  );
  const generatedTmpRoot = path.join(generatedRoot, '.sdkwork', 'tmp');

  return {
    generatedRoot,
    generatedTmpRoot,
    buildScriptPath: path.join(scriptDir, 'build-typescript-generated-package.mjs'),
    packageVerifierPath: path.join(scriptDir, 'verify-typescript-generated-package.mjs'),
    concurrencyVerifierPath: path.join(scriptDir, 'verify-typescript-generated-build-concurrency.mjs'),
    concurrencyPowerShellPath: path.join(scriptDir, 'verify-typescript-generated-build-concurrency.ps1'),
    concurrencyLogRoot: path.join(
      workspaceRoot,
      '.sdkwork',
      'tmp',
      'verify-typescript-generated-build-concurrency',
    ),
    mapPath: path.join(generatedRoot, 'dist', 'index.cjs.map'),
    tempVerifyTargets: [
      path.join(generatedTmpRoot, 'tsc-dts-verify'),
      path.join(generatedTmpRoot, 'tsc-js-verify'),
    ],
  };
}

export function readRequiredTypescriptBuildFile({
  prefix,
  filePath,
  missingMessage,
}) {
  if (!existsSync(filePath)) {
    failTypescriptGeneratedBuildVerification({
      prefix,
      message: missingMessage || `Required file is missing: ${filePath}`,
    });
  }

  return readFileSync(filePath, 'utf8');
}

export function removePathsIfPresent(targetPaths) {
  for (const targetPath of targetPaths) {
    if (!existsSync(targetPath)) {
      continue;
    }

    rmSync(targetPath, { recursive: true, force: true });
  }
}

export function collectExistingPaths(targetPaths) {
  return targetPaths.filter((targetPath) => existsSync(targetPath));
}
