import { randomUUID } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

import {
  ensurePrivateRuntimeStateDirectory,
  removeRuntimeStateFile,
  resolveRepositoryRuntimeStateDirectory,
  writePrivateJsonAtomically,
} from '@sdkwork/app-topology/runtime-state';

const PURPOSE_PATTERN = /^[a-z0-9][a-z0-9-]*$/u;
const FILE_NAME_PATTERN = /^[a-z0-9][a-z0-9.-]*$/u;

function assertPurpose(purpose) {
  if (!PURPOSE_PATTERN.test(String(purpose ?? ''))) {
    throw new Error('temporary state purpose must use lowercase kebab-case');
  }
}

function runtimeStateOptions(repoRoot, options = {}) {
  return {
    repoRoot,
    owner: 'sdkwork-im',
    ...(options.env ? { env: options.env } : {}),
    ...(options.platform ? { platform: options.platform } : {}),
    ...(options.temporaryDirectory ? { temporaryDirectory: options.temporaryDirectory } : {}),
  };
}

function assertOwnedRuntimeStatePath(filePath, { repoRoot, ...options } = {}) {
  if (!filePath) throw new Error('runtime state file path is required');
  const ownerRoot = path.resolve(resolveRepositoryRuntimeStateDirectory(runtimeStateOptions(repoRoot, options)));
  const resolvedFilePath = path.resolve(filePath);
  const relative = path.relative(ownerRoot, resolvedFilePath);
  if (!relative || relative.startsWith('..') || path.isAbsolute(relative)) {
    throw new Error('runtime state file must be inside the SDKWork IM runtime state root');
  }
  return { ownerRoot, resolvedFilePath };
}

export function createImTemporaryDirectory({ repoRoot, purpose, ...options } = {}) {
  assertPurpose(purpose);
  const parent = ensurePrivateRuntimeStateDirectory(runtimeStateOptions(repoRoot, options));
  return fs.mkdtempSync(path.join(parent, `${purpose}-`));
}

export function resolveImRuntimeStateDirectory({ repoRoot, purpose, ...options } = {}) {
  assertPurpose(purpose);
  return path.join(
    resolveRepositoryRuntimeStateDirectory(runtimeStateOptions(repoRoot, options)),
    purpose,
  );
}

export function resolveImTemporaryFilePath({
  extension = '',
  fileName,
  purpose,
  repoRoot,
  ...options
} = {}) {
  assertPurpose(purpose);
  if (!FILE_NAME_PATTERN.test(String(fileName ?? ''))) {
    throw new Error('temporary state file name must use lowercase letters, numbers, dots, or hyphens');
  }
  if (extension && !/^\.[a-z0-9][a-z0-9.-]*$/u.test(extension)) {
    throw new Error('temporary state file extension must start with a dot and use lowercase characters');
  }
  return path.join(
    resolveImRuntimeStateDirectory({ repoRoot, purpose, ...options }),
    `${fileName}-${process.pid}-${randomUUID()}${extension}`,
  );
}

export function writeImPrivateJsonFile(filePath, value, { repoRoot, ...options } = {}) {
  const { resolvedFilePath } = assertOwnedRuntimeStatePath(filePath, { repoRoot, ...options });
  writePrivateJsonAtomically(resolvedFilePath, value, options);
}

export function writeImPrivateFile(filePath, value, { repoRoot, ...options } = {}) {
  const { resolvedFilePath } = assertOwnedRuntimeStatePath(filePath, { repoRoot, ...options });
  const directory = path.dirname(resolvedFilePath);
  fs.mkdirSync(directory, { recursive: true, mode: 0o700 });
  if ((options.platform ?? process.platform) !== 'win32') fs.chmodSync(directory, 0o700);
  fs.writeFileSync(resolvedFilePath, value, { flag: 'wx', mode: 0o600 });
  if ((options.platform ?? process.platform) !== 'win32') fs.chmodSync(resolvedFilePath, 0o600);
}

export function removeImRuntimeStateFile(filePath, { repoRoot, ...options } = {}) {
  if (!filePath) return;
  const { ownerRoot, resolvedFilePath } = assertOwnedRuntimeStatePath(filePath, { repoRoot, ...options });
  removeRuntimeStateFile(resolvedFilePath);
  try {
    fs.rmdirSync(ownerRoot);
  } catch (error) {
    if (!['ENOENT', 'ENOTEMPTY'].includes(error.code)) throw error;
  }
}

export function removeImTemporaryDirectory(directory, { repoRoot, ...options } = {}) {
  if (!directory) return;
  const ownerRoot = path.resolve(resolveRepositoryRuntimeStateDirectory(runtimeStateOptions(repoRoot, options)));
  const resolvedDirectory = path.resolve(directory);
  const relative = path.relative(ownerRoot, resolvedDirectory);
  if (!relative || relative.startsWith('..') || path.isAbsolute(relative)) {
    throw new Error('temporary directory cleanup must target one child of the SDKWork IM runtime state root');
  }
  fs.rmSync(resolvedDirectory, { recursive: true, force: true });
  try {
    fs.rmdirSync(ownerRoot);
  } catch (error) {
    if (!['ENOENT', 'ENOTEMPTY'].includes(error.code)) throw error;
  }
}
