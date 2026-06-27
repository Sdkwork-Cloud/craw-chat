#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(scriptDir, '..');

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: docsRoot,
    stdio: 'inherit',
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 0) !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runNodeScript(relativePath, args = []) {
  run(process.execPath, [path.join(docsRoot, relativePath), ...args]);
}

function prepareApiDocs() {
  runNodeScript('scripts/standardize-api-docs.mjs');
  runNodeScript('scripts/generate-operation-pages.mjs');
}

function resolveVitepressCli() {
  try {
    const packageJsonPath = require.resolve('vitepress/package.json', { paths: [docsRoot] });
    return path.join(path.dirname(packageJsonPath), 'dist', 'node', 'cli.js');
  } catch (error) {
    const details = error instanceof Error ? error.message : String(error);
    throw new Error(`Unable to resolve vitepress from docs/sites. Run install first. ${details}`);
  }
}

function runVitepress(args) {
  run(process.execPath, [resolveVitepressCli(), ...args]);
}

async function assertEsbuildRuntime() {
  let esbuild;
  try {
    esbuild = require('esbuild');
  } catch (error) {
    const details = error instanceof Error ? error.message : String(error);
    throw new Error(`Unable to resolve esbuild from docs/sites. Run npm ci first. ${details}`);
  }

  try {
    await esbuild.transform('export default 1', { loader: 'js' });
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'EPERM') {
      throw new Error(
        'esbuild could not start its child process (spawn EPERM). This Windows shell blocks the VitePress/esbuild runtime. Use docs:verify for content validation here, or run docs:build/dev/preview from a normal local terminal with child-process execution enabled.',
      );
    }

    const details = error instanceof Error ? error.stack || error.message : String(error);
    throw new Error(`esbuild runtime check failed. ${details}`);
  }
}

const task = process.argv[2];

async function main() {
  switch (task) {
    case 'generate':
      prepareApiDocs();
      break;
    case 'verify':
      prepareApiDocs();
      runNodeScript('scripts/verify-api-docs.mjs');
      runNodeScript('sdk/verify-sdk-site-docs.mjs');
      break;
    case 'build':
      prepareApiDocs();
      runNodeScript('scripts/verify-api-docs.mjs');
      runNodeScript('sdk/verify-sdk-site-docs.mjs');
      await assertEsbuildRuntime();
      runVitepress(['build', '.']);
      break;
    case 'dev':
      prepareApiDocs();
      await assertEsbuildRuntime();
      runVitepress(['dev', '.']);
      break;
    case 'preview':
      await assertEsbuildRuntime();
      runVitepress(['preview', '.']);
      break;
    default:
      console.error(
        `Unknown docs task "${task}". Expected one of: generate, verify, build, dev, preview.`,
      );
      process.exit(1);
  }
}

main().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`[docs/sites] ${message}`);
  process.exit(1);
});
