#!/usr/bin/env node

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');
const FALLBACK_RELEASE_VERSION = '0.1.0';

function normalizeSdkworkImReleaseVersion(version) {
  const normalized = String(version ?? '').trim().replace(/^v(?=\d)/iu, '');
  if (!/^[0-9A-Za-z][0-9A-Za-z._-]*$/u.test(normalized)) {
    throw new Error('release version must be a non-empty package-safe value');
  }
  return normalized;
}

function readPackageVersion(packageJsonPath) {
  if (!existsSync(packageJsonPath)) {
    return '';
  }
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
  const version = String(packageJson.version ?? '').trim();
  return version && version !== '0.0.0' ? version : '';
}

function resolveSdkworkImReleaseVersion({
  env = process.env,
  root = repoRoot,
} = {}) {
  const explicitVersion = String(env.SDKWORK_IM_RELEASE_VERSION ?? '').trim();
  if (explicitVersion) {
    return normalizeSdkworkImReleaseVersion(explicitVersion);
  }

  return normalizeSdkworkImReleaseVersion(
    readPackageVersion(path.join(root, 'package.json'))
      || readPackageVersion(path.join(root, 'apps', 'sdkwork-im-pc', 'package.json'))
      || FALLBACK_RELEASE_VERSION,
  );
}

const DEFAULT_RELEASE_VERSION = resolveSdkworkImReleaseVersion();

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
  try {
    console.log(resolveSdkworkImReleaseVersion());
  } catch (error) {
    console.error(`[sdkwork-im-release-version] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}

export {
  DEFAULT_RELEASE_VERSION,
  FALLBACK_RELEASE_VERSION,
  normalizeSdkworkImReleaseVersion,
  resolveSdkworkImReleaseVersion,
};
