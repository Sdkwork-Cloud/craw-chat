#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const DIGEST_IMAGE_PATTERN = /^[a-z0-9][a-z0-9._/-]*(?::[a-z0-9._-]+)?@sha256:[a-f0-9]{64}$/u;
const RELEASE_TAG_PATTERN = /^[a-z0-9][a-z0-9._/-]*:[0-9A-Za-z][0-9A-Za-z._-]{0,127}$/u;

export function loadCloudServiceBuilds(root = repoRoot) {
  const buildInventory = JSON.parse(
    readFileSync(path.join(root, 'deployments', 'docker', 'cloud-service-builds.json'), 'utf8'),
  );
  if (buildInventory?.schemaVersion !== 1 || !isRecord(buildInventory.services)) {
    throw new Error('cloud service build inventory must use schemaVersion 1');
  }
  return buildInventory;
}

export function validateCloudImageBuildInput({ service, runtimeImage, tag }, inventory) {
  const build = inventory.services[service];
  if (!build) {
    throw new Error(`unknown cloud service: ${service}`);
  }
  if (!DIGEST_IMAGE_PATTERN.test(runtimeImage ?? '')) {
    throw new Error('runtime image must be an OCI repository reference pinned by sha256 digest');
  }
  if (!RELEASE_TAG_PATTERN.test(tag ?? '') || /:latest$/u.test(tag)) {
    throw new Error('output tag must be an explicit non-latest release tag');
  }
  return build;
}

function parseArgs(args) {
  const values = new Map();
  for (let index = 0; index < args.length; index += 2) {
    const key = args[index];
    const value = args[index + 1];
    if (!['--service', '--runtime-image', '--tag'].includes(key) || !value) {
      throw new Error(
        'usage: build-sdkwork-im-cloud-image.mjs --service <id> --runtime-image <repo@sha256:digest> --tag <repo:release>',
      );
    }
    values.set(key, value);
  }
  if (values.size !== 3) {
    throw new Error('service, runtime-image, and tag are required');
  }
  return {
    service: values.get('--service'),
    runtimeImage: values.get('--runtime-image'),
    tag: values.get('--tag'),
  };
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: false,
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(`${command} failed with exit code ${result.status ?? 1}`);
  }
}

function isRecord(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    if (process.platform !== 'linux') {
      throw new Error('cloud Linux images must be built by a Linux release runner');
    }
    const input = parseArgs(process.argv.slice(2));
    const build = validateCloudImageBuildInput(input, loadCloudServiceBuilds());
    run('cargo', [
      'build',
      '--locked',
      '--release',
      '--package',
      build.package,
      '--bin',
      build.binary,
    ]);

    const stageRoot = path.join(repoRoot, '.container-artifacts', input.service);
    rmSync(stageRoot, { recursive: true, force: true });
    mkdirSync(stageRoot, { recursive: true });
    copyFileSync(
      path.join(repoRoot, 'target', 'release', build.binary),
      path.join(stageRoot, 'service'),
    );

    run('docker', [
      'build',
      '--file',
      'deployments/docker/sdkwork-im-cloud-service.Dockerfile',
      '--build-arg',
      `RUNTIME_IMAGE=${input.runtimeImage}`,
      '--build-arg',
      `SERVICE_ARTIFACT=.container-artifacts/${input.service}/service`,
      '--build-arg',
      `SERVICE_NAME=${input.service}`,
      '--build-arg',
      `HEALTH_PORT=${build.port}`,
      '--tag',
      input.tag,
      '.',
    ]);
  } catch (error) {
    console.error(`[cloud-image-build] ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  }
}
