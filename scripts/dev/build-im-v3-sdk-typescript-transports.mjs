import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const pnpmExecutable = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';

const transportPackages = [
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi',
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi',
  'sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi',
];

for (const relativePackageDir of transportPackages) {
  const packageDir = path.join(repoRoot, relativePackageDir);
  const packageJsonPath = path.join(packageDir, 'package.json');
  if (!existsSync(packageJsonPath)) {
    throw new Error(`missing generated transport package: ${relativePackageDir}`);
  }

  const result = spawnSync(pnpmExecutable, ['run', 'build'], {
    cwd: packageDir,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

process.stdout.write('im v3 typescript transport packages built\n');
