import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

export function resolveTypescriptGeneratedPackagePaths({
  workspaceRoot,
  relativeGeneratedRoot,
}) {
  const generatedRoot = path.join(workspaceRoot, relativeGeneratedRoot);
  return {
    generatedRoot,
    packageJsonPath: path.join(generatedRoot, 'package.json'),
    distRoot: path.join(generatedRoot, 'dist'),
  };
}

export function readTypescriptGeneratedPackageJson({ packageJsonPath }) {
  return JSON.parse(readFileSync(packageJsonPath, 'utf8'));
}

export function collectMissingTypescriptPackageArtifacts({
  distRoot,
  requiredArtifacts,
}) {
  return requiredArtifacts.filter(
    (relativePath) => !existsSync(path.join(distRoot, relativePath)),
  );
}

export function failTypescriptPackageVerification({ prefix, message }) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function finishTypescriptPackageVerification({
  prefix,
  failures,
  successMessage,
}) {
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`[${prefix}] ${failure}`);
    }
    process.exit(1);
  }

  console.log(successMessage || `[${prefix}] TypeScript generated package verification passed.`);
}
