#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import {
  copyFileSync,
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';
import { parseAllDocuments } from 'yaml';

const MODULE_PATH = fileURLToPath(import.meta.url);
const DEFAULT_REPO_ROOT = path.resolve(path.dirname(MODULE_PATH), '..', '..');
const DIGEST_PATTERN = /^sha256:[a-f0-9]{64}$/u;
const RELEASE_ID_PATTERN = /^[0-9A-Za-z][0-9A-Za-z._-]{0,127}$/u;
const REVISION_PATTERN = /^[a-f0-9]{40}$/u;

export function loadImageInventory(repoRoot = DEFAULT_REPO_ROOT) {
  const inventoryPath = path.join(
    repoRoot,
    'deployments',
    'kubernetes',
    'cloud',
    'image-inventory.json',
  );
  const inventory = JSON.parse(readFileSync(inventoryPath, 'utf8'));
  if (inventory?.schemaVersion !== 1 || !isRecord(inventory.images)) {
    throw new Error('cloud image inventory must use schemaVersion 1 and declare images');
  }
  return inventory;
}

export function validateImageLock(imageLock, inventory, { expectedRevision } = {}) {
  if (!isRecord(imageLock) || imageLock.schemaVersion !== 1) {
    throw new Error('image lock must be an object with schemaVersion 1');
  }
  if (!RELEASE_ID_PATTERN.test(imageLock.releaseId ?? '')) {
    throw new Error('image lock releaseId must be a package-safe value of at most 128 characters');
  }
  if (!REVISION_PATTERN.test(imageLock.sourceRevision ?? '')) {
    throw new Error('image lock sourceRevision must be a lowercase 40-character Git revision');
  }
  if (expectedRevision && imageLock.sourceRevision !== expectedRevision) {
    throw new Error(
      `image lock sourceRevision ${imageLock.sourceRevision} does not match ${expectedRevision}`,
    );
  }
  if (!isRecord(imageLock.images)) {
    throw new Error('image lock images must be an object');
  }

  const expectedServices = Object.keys(inventory.images).sort();
  const lockedServices = Object.keys(imageLock.images).sort();
  if (JSON.stringify(lockedServices) !== JSON.stringify(expectedServices)) {
    throw new Error(
      `image lock service set must exactly match inventory: ${expectedServices.join(', ')}`,
    );
  }

  for (const service of expectedServices) {
    const lockedImage = imageLock.images[service];
    if (!isRecord(lockedImage)) {
      throw new Error(`image lock entry ${service} must be an object`);
    }
    const expectedRepository = inventory.images[service];
    if (lockedImage.repository !== expectedRepository) {
      throw new Error(
        `image lock repository for ${service} must be ${expectedRepository}`,
      );
    }
    if (!DIGEST_PATTERN.test(lockedImage.digest ?? '')) {
      throw new Error(`image lock digest for ${service} must be sha256 followed by 64 lowercase hex characters`);
    }
    const extraKeys = Object.keys(lockedImage).filter(
      (key) => key !== 'repository' && key !== 'digest',
    );
    if (extraKeys.length > 0) {
      throw new Error(`image lock entry ${service} has unsupported keys: ${extraKeys.join(', ')}`);
    }
  }

  const extraRootKeys = Object.keys(imageLock).filter(
    (key) => !['schemaVersion', 'releaseId', 'sourceRevision', 'images'].includes(key),
  );
  if (extraRootKeys.length > 0) {
    throw new Error(`image lock has unsupported keys: ${extraRootKeys.join(', ')}`);
  }
}

export function renderCloudManifests({
  repoRoot = DEFAULT_REPO_ROOT,
  imageLock,
  outputRoot,
  expectedRevision,
}) {
  const inventory = loadImageInventory(repoRoot);
  validateImageLock(imageLock, inventory, { expectedRevision });

  const sourceRoot = path.join(repoRoot, 'deployments', 'kubernetes', 'cloud');
  const resolvedOutputRoot = path.resolve(outputRoot);
  if (isPathInsideOrSame(resolvedOutputRoot, sourceRoot)) {
    throw new Error('materialized output must not be inside the source manifest directory');
  }

  const temporaryRoot = `${resolvedOutputRoot}.tmp-${process.pid}`;
  rmSync(temporaryRoot, { recursive: true, force: true });
  mkdirSync(temporaryRoot, { recursive: true });

  const deploymentServices = new Set();
  const outputFiles = [];
  try {
    for (const sourcePath of listFiles(sourceRoot)) {
      const relativePath = toPortablePath(path.relative(sourceRoot, sourcePath));
      if (relativePath === 'image-inventory.json' || relativePath === 'image-lock.schema.json') {
        continue;
      }
      const destinationPath = path.join(temporaryRoot, relativePath);
      mkdirSync(path.dirname(destinationPath), { recursive: true });

      if (!/\.ya?ml$/u.test(sourcePath)) {
        copyFileSync(sourcePath, destinationPath);
        outputFiles.push(relativePath);
        continue;
      }

      const source = readFileSync(sourcePath, 'utf8');
      const documents = parseAllDocuments(source);
      for (const document of documents) {
        if (document.errors.length > 0) {
          throw new Error(`${relativePath} is invalid YAML: ${document.errors[0].message}`);
        }
        const resource = document.toJS();
        if (resource?.kind !== 'Deployment') {
          continue;
        }

        const service = resource?.metadata?.name;
        const lockedImage = imageLock.images[service];
        if (!lockedImage) {
          throw new Error(`${relativePath} contains unregistered Deployment ${service ?? '<missing-name>'}`);
        }
        if (deploymentServices.has(service)) {
          throw new Error(`Deployment ${service} is declared more than once`);
        }
        deploymentServices.add(service);

        const containers = resource?.spec?.template?.spec?.containers;
        if (!Array.isArray(containers) || containers.length !== 1) {
          throw new Error(`Deployment ${service} must declare exactly one application container`);
        }
        containers[0].image = `${lockedImage.repository}@${lockedImage.digest}`;
        containers[0].imagePullPolicy = 'IfNotPresent';
        document.contents = document.createNode(resource);
      }

      const rendered = documents.map((document) => document.toString()).join('---\n');
      writeFileSync(destinationPath, rendered, 'utf8');
      outputFiles.push(relativePath);
    }

    const expectedServices = Object.keys(inventory.images).sort();
    const renderedServices = [...deploymentServices].sort();
    if (JSON.stringify(renderedServices) !== JSON.stringify(expectedServices)) {
      throw new Error(
        `rendered Deployment set must exactly match inventory: ${expectedServices.join(', ')}`,
      );
    }

    writeFileSync(
      path.join(temporaryRoot, 'image-lock.json'),
      `${JSON.stringify(imageLock, null, 2)}\n`,
      'utf8',
    );
    outputFiles.push('image-lock.json');

    const bundleManifest = {
      schemaVersion: 1,
      releaseId: imageLock.releaseId,
      sourceRevision: imageLock.sourceRevision,
      files: outputFiles.sort().map((relativePath) => ({
        path: relativePath,
        sha256: sha256File(path.join(temporaryRoot, relativePath)),
      })),
    };
    writeFileSync(
      path.join(temporaryRoot, 'bundle-manifest.json'),
      `${JSON.stringify(bundleManifest, null, 2)}\n`,
      'utf8',
    );

    rmSync(resolvedOutputRoot, { recursive: true, force: true });
    cpSync(temporaryRoot, resolvedOutputRoot, { recursive: true, errorOnExist: true });
    rmSync(temporaryRoot, { recursive: true, force: true });
    return bundleManifest;
  } catch (error) {
    rmSync(temporaryRoot, { recursive: true, force: true });
    throw error;
  }
}

function readCliArgs(args) {
  const values = new Map();
  for (let index = 0; index < args.length; index += 2) {
    const key = args[index];
    const value = args[index + 1];
    if (!['--image-lock', '--output'].includes(key) || !value) {
      throw new Error('usage: materialize-sdkwork-im-kubernetes.mjs --image-lock <path> --output <dir>');
    }
    values.set(key, value);
  }
  if (values.size !== 2) {
    throw new Error('both --image-lock and --output are required');
  }
  return values;
}

function resolveCleanRevision(repoRoot) {
  const status = execFileSync('git', ['status', '--porcelain'], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  if (status.trim()) {
    throw new Error('cloud release materialization requires a clean Git worktree');
  }
  return execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: repoRoot,
    encoding: 'utf8',
  }).trim();
}

function listFiles(root) {
  const files = [];
  for (const entry of readdirSync(root)) {
    const entryPath = path.join(root, entry);
    if (statSync(entryPath).isDirectory()) {
      files.push(...listFiles(entryPath));
    } else {
      files.push(entryPath);
    }
  }
  return files.sort();
}

function sha256File(filePath) {
  return createHash('sha256').update(readFileSync(filePath)).digest('hex');
}

function toPortablePath(value) {
  return value.split(path.sep).join('/');
}

function isRecord(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function isPathInsideOrSame(candidatePath, parentPath) {
  const relative = path.relative(path.resolve(parentPath), path.resolve(candidatePath));
  return relative === '' || (relative && !relative.startsWith('..') && !path.isAbsolute(relative));
}

if (process.argv[1] && path.resolve(process.argv[1]) === MODULE_PATH) {
  try {
    const args = readCliArgs(process.argv.slice(2));
    const revision = resolveCleanRevision(DEFAULT_REPO_ROOT);
    const imageLockPath = path.resolve(args.get('--image-lock'));
    if (!existsSync(imageLockPath)) {
      throw new Error(`image lock does not exist: ${imageLockPath}`);
    }
    const imageLock = JSON.parse(readFileSync(imageLockPath, 'utf8'));
    const bundle = renderCloudManifests({
      repoRoot: DEFAULT_REPO_ROOT,
      imageLock,
      outputRoot: path.resolve(args.get('--output')),
      expectedRevision: revision,
    });
    console.log(
      `[kubernetes-materialize] rendered ${bundle.files.length} locked files for ${bundle.releaseId}`,
    );
  } catch (error) {
    console.error(`[kubernetes-materialize] ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  }
}
