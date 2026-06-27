#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { runGenerateSdkFamily } from '../../workspace-im-v3-sdk-family.mjs';
import { sdkFamilyConfig } from './sdk-family-config.mjs';

function selectedLanguages(argv) {
  const values = [];
  for (let index = 0; index < argv.length; index += 1) {
    if (argv[index] === '--language') {
      values.push(...String(argv[index + 1] || '').split(',').map((value) => value.trim().toLowerCase()).filter(Boolean));
      index += 1;
    }
  }
  return [...new Set(values)];
}

function shouldAssembleTypeScriptRoot(argv) {
  const languages = selectedLanguages(argv);
  return languages.length === 0 || languages.includes('typescript');
}

function run(step, args, cwd) {
  const result = spawnSync('node', args, { cwd, stdio: 'inherit', shell: false });
  if (result.error) {
    console.error(`[sdkwork-im-sdk] ${step} failed to start: ${result.error.message}`);
    process.exit(1);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    console.error(`[sdkwork-im-sdk] ${step} failed with exit code ${result.status}`);
    process.exit(result.status);
  }
  if (result.signal) {
    console.error(`[sdkwork-im-sdk] ${step} terminated with signal ${result.signal}`);
    process.exit(1);
  }
}

const argv = process.argv.slice(2);
const scriptDir = path.dirname(fileURLToPath(import.meta.url));

run('materialize-local-openapi-seed.mjs', [
  path.join(scriptDir, 'materialize-local-openapi-seed.mjs'),
], path.resolve(scriptDir, '..'));

run('sync-openapi-authority-mirror.mjs', [
  path.join(scriptDir, 'sync-openapi-authority-mirror.mjs'),
], path.resolve(scriptDir, '..'));

run('materialize-im-v3-openapi-boundaries.mjs', [
  path.resolve(scriptDir, '..', '..', 'materialize-im-v3-openapi-boundaries.mjs'),
], path.resolve(scriptDir, '..', '..'));

await runGenerateSdkFamily(sdkFamilyConfig, argv);

if (shouldAssembleTypeScriptRoot(argv)) {
  run('assemble-single-package.mjs', [
    path.join(scriptDir, 'assemble-single-package.mjs'),
  ], path.resolve(scriptDir, '..'));
}

run('assemble-sdk.mjs', [
  path.join(scriptDir, 'assemble-sdk.mjs'),
  ...argv.filter((value, index) => value !== '--fixed-sdk-version' && argv[index - 1] !== '--fixed-sdk-version'),
], path.resolve(scriptDir, '..'));
