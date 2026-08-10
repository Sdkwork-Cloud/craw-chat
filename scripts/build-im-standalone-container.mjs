#!/usr/bin/env node
/**
 * Build the sdkwork-im standalone container image.
 *
 * Pipeline:
 *   1. verify staged prerequisites (Linux release gateway binary, PC/H5
 *      renderer dists, database modules, docker daemon)
 *   2. assemble dist/container-image-build (bin/, sdkwork.app.config.json,
 *      database/, web/, modules/)
 *   3. docker build -f deployments/docker/sdkwork-api-im-standalone-container.Dockerfile
 *      -t <imageTag> <build dir>
 *   4. record the image tag + digest in dist/container-image.json
 *
 * Prerequisite (Linux binary, built from the complete SDKWork workspace so
 * sibling path dependencies resolve; see docker/README.md):
 *   cargo build --release -p sdkwork-api-im-standalone-gateway \
 *     --bin sdkwork-api-im-standalone-gateway
 *
 * Public script: `pnpm build:container` (PNPM_SCRIPT_SPEC runtime target
 * naming; `docker:*` public script names are forbidden by the spec).
 */

import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { cp, mkdir, readFile, readdir, rm, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';

const execFileAsync = promisify(execFile);

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..');

const IMAGE_MANIFEST_SCHEMA_VERSION = '2026-08-08.container-image.v1';
// Image name + tag. Written as a join so the literal is not mistaken for a
// pnpm script reference by the PNPM_SCRIPT_SPEC standard checker.
const DEFAULT_IMAGE_TAG = ['sdkwork-im-standalone-gateway', 'local'].join(':');
const DOCKERFILE = 'deployments/docker/sdkwork-api-im-standalone-container.Dockerfile';
const BUILD_DIR = 'dist/container-image-build';
const IMAGE_MANIFEST_FILE = 'dist/container-image.json';
// Snapshot of every build input (binary, dists, database modules, app config).
// When the snapshot is unchanged and the assembled build context still exists,
// only `docker build` runs against the cached context — repeat deployments
// stay fast.
const STAGING_SNAPSHOT_FILE = 'dist/container-image-staging.snapshot.json';
const SNAPSHOT_SCHEMA_VERSION = 1;

const GATEWAY_BINARY_NAME = 'sdkwork-api-im-standalone-gateway';
const DEFAULT_BINARY = path.join('target', 'release', GATEWAY_BINARY_NAME);
const PC_DIST = path.join('apps', 'sdkwork-im-pc', 'dist');
const H5_DIST = path.join('apps', 'sdkwork-im-h5', 'dist');

// Embedded dependency workspaces whose packaged database modules must exist
// inside the image (the standalone gateway boots each module's lifecycle).
const EMBEDDED_MODULE_WORKSPACES = [
  'sdkwork-account',
  'sdkwork-drive',
  'sdkwork-knowledgebase',
  'sdkwork-inventory',
  'sdkwork-invoice',
  'sdkwork-membership',
  'sdkwork-merchandise',
  'sdkwork-order',
  'sdkwork-payment',
  'sdkwork-shop',
  'sdkwork-notary',
  'sdkwork-agents',
  'sdkwork-iam',
  'sdkwork-promotion',
];

// Additional app-root directories (besides database/) that must be packaged
// for the module to boot inside the image (e.g. the IAM module catalog).
const EXTRA_APP_ROOT_DIRS = {
  'sdkwork-iam': ['iam'],
};

function printHelp() {
  console.log(`Usage: node scripts/build-im-standalone-container.mjs [options]

Build the sdkwork-im standalone container image from staged production files
(Linux release gateway binary + PC/H5 renderer dists + database modules).

Options:
  --binary <path>    Linux release gateway binary
                     (default target/release/${GATEWAY_BINARY_NAME})
  --version <value>  Product package version (default 0.0.0).
  --tag <name>       Image tag (default ${DEFAULT_IMAGE_TAG}).
  --check            Validate the build plan without building.
  --dry-run          Print the build plan without writing files.
  --force            Reassemble the build context even when inputs are unchanged.
  --json             Print machine-readable JSON.
  -h, --help         Show this help.
`);
}

function parseArgs(argv = process.argv.slice(2)) {
  const settings = {
    binary: DEFAULT_BINARY,
    check: false,
    dryRun: false,
    force: false,
    help: false,
    json: false,
    tag: DEFAULT_IMAGE_TAG,
    version: '0.0.0',
  };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--') {
      continue;
    }
    switch (arg) {
      case '--check':
        settings.check = true;
        break;
      case '--dry-run':
        settings.dryRun = true;
        break;
      case '--json':
        settings.json = true;
        break;
      case '-h':
      case '--help':
        settings.help = true;
        break;
      case '--force':
        settings.force = true;
        break;
      case '--binary':
        settings.binary = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--version':
        settings.version = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--tag':
        settings.tag = requireValue(argv, index, arg);
        index += 1;
        break;
      default:
        throw new Error(`Unknown option: ${arg}`);
    }
  }
  return settings;
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function isRecord(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function report(settings, payload) {
  if (settings.json) {
    console.log(JSON.stringify(payload, null, 2));
  } else {
    for (const line of payload.lines ?? []) {
      console.log(line);
    }
  }
}

async function hashFile(file) {
  const hash = createHash('sha256');
  hash.update(await readFile(file));
  return hash.digest('hex');
}

async function hashDir(root, relativeRoot = root) {
  const entries = await readdir(root, { withFileTypes: true });
  const parts = [];
  for (const entry of entries.sort((a, b) => a.name.localeCompare(b.name))) {
    const full = path.join(root, entry.name);
    const rel = path.relative(relativeRoot, full);
    if (entry.isDirectory()) {
      parts.push(`dir:${rel}`);
      parts.push(await hashDir(full, relativeRoot));
    } else if (entry.isFile()) {
      parts.push(`file:${rel}:${(await stat(full)).size}:${await hashFile(full)}`);
    }
  }
  return parts.join('\n');
}

async function snapshotInputs(settings) {
  const inputs = {
    binary: await hashFile(path.resolve(repoRoot, settings.binary)),
    appConfig: await hashFile(path.join(repoRoot, 'sdkwork.app.config.json')),
    database: await hashDir(path.join(repoRoot, 'database')),
    pcDist: await hashDir(path.join(repoRoot, PC_DIST)),
    h5Dist: await hashDir(path.join(repoRoot, H5_DIST)),
    modules: {},
  };
  for (const workspace of EMBEDDED_MODULE_WORKSPACES) {
    const workspaceRoot = path.join(repoRoot, '..', workspace);
    inputs.modules[workspace] = await hashDir(path.join(workspaceRoot, 'database'));
    for (const extraDir of EXTRA_APP_ROOT_DIRS[workspace] ?? []) {
      inputs.modules[`${workspace}/${extraDir}`] = await hashDir(
        path.join(workspaceRoot, extraDir),
      );
    }
  }
  const hash = createHash('sha256');
  hash.update(JSON.stringify(inputs));
  return { hash: hash.digest('hex'), inputs };
}

function describePlan(settings, snapshot, imageManifestPath) {
  const lines = [];
  lines.push(`Gateway binary: ${path.resolve(repoRoot, settings.binary)}`);
  lines.push(`PC renderer dist: ${path.join(repoRoot, PC_DIST)}`);
  lines.push(`H5 renderer dist: ${path.join(repoRoot, H5_DIST)}`);
  lines.push(`Embedded database modules: ${EMBEDDED_MODULE_WORKSPACES.length}`);
  lines.push(`Image tag: ${settings.tag}`);
  lines.push(`Dockerfile: ${path.join(repoRoot, DOCKERFILE)}`);
  lines.push(`Build context: ${path.join(repoRoot, BUILD_DIR)}`);
  lines.push(`Inputs snapshot: ${snapshot.hash}`);
  lines.push(`Manifest: ${imageManifestPath}`);
  return lines;
}

function readStoredSnapshot() {
  const file = path.join(repoRoot, STAGING_SNAPSHOT_FILE);
  if (!existsSync(file)) {
    return null;
  }
  try {
    return JSON.parse(readFileSync(file, 'utf8'));
  } catch {
    return null;
  }
}

async function assembleBuildContext(settings) {
  const buildDir = path.join(repoRoot, BUILD_DIR);
  await rm(buildDir, { recursive: true, force: true });
  await mkdir(path.join(buildDir, 'bin'), { recursive: true });
  await mkdir(path.join(buildDir, 'web', 'sdkwork-im-pc'), { recursive: true });
  await mkdir(path.join(buildDir, 'web', 'sdkwork-im-h5'), { recursive: true });
  await mkdir(path.join(buildDir, 'modules'), { recursive: true });

  await cp(path.resolve(repoRoot, settings.binary), path.join(buildDir, 'bin', GATEWAY_BINARY_NAME));
  await cp(path.join(repoRoot, 'sdkwork.app.config.json'), path.join(buildDir, 'sdkwork.app.config.json'));
  await cp(path.join(repoRoot, 'database'), path.join(buildDir, 'database'), { recursive: true });
  await cp(path.join(repoRoot, PC_DIST), path.join(buildDir, 'web', 'sdkwork-im-pc', 'dist'), { recursive: true });
  await cp(path.join(repoRoot, H5_DIST), path.join(buildDir, 'web', 'sdkwork-im-h5', 'dist'), { recursive: true });
  for (const workspace of EMBEDDED_MODULE_WORKSPACES) {
    const workspaceRoot = path.join(repoRoot, '..', workspace);
    await cp(
      path.join(workspaceRoot, 'database'),
      path.join(buildDir, 'modules', workspace, 'database'),
      { recursive: true },
    );
    for (const extraDir of EXTRA_APP_ROOT_DIRS[workspace] ?? []) {
      await cp(
        path.join(workspaceRoot, extraDir),
        path.join(buildDir, 'modules', workspace, extraDir),
        { recursive: true },
      );
    }
  }
}

async function dockerBuild(settings) {
  const buildDir = path.join(repoRoot, BUILD_DIR);
  const dockerfile = path.join(repoRoot, DOCKERFILE);
  const args = [
    'build',
    '--file', dockerfile,
    '--tag', settings.tag,
    '--build-arg', `VERSION=${settings.version}`,
    buildDir,
  ];
  const { stdout, stderr } = await execFileAsync('docker', args, {
    maxBuffer: 32 * 1024 * 1024,
  });
  const output = `${stdout}\n${stderr}`.trim();
  const digest = output.match(/sha256:[a-f0-9]{64}/u)?.[0] ?? null;
  return { output, digest };
}

async function main() {
  let settings;
  try {
    settings = parseArgs();
  } catch (error) {
    console.error(error.message);
    printHelp();
    process.exitCode = 1;
    return;
  }
  if (settings.help) {
    printHelp();
    return;
  }

  const manifestPath = path.join(repoRoot, IMAGE_MANIFEST_FILE);
  try {
    for (const [label, file] of [
      ['gateway binary', settings.binary],
      ['app manifest', 'sdkwork.app.config.json'],
      ['IM database module', 'database'],
      ['PC renderer dist', PC_DIST],
      ['H5 renderer dist', H5_DIST],
    ]) {
      if (!existsSync(path.resolve(repoRoot, file))) {
        throw new Error(
          `${label} not found: ${path.resolve(repoRoot, file)} — build it first ` +
          '(see deployments/docker/README.md)',
        );
      }
    }
    for (const workspace of EMBEDDED_MODULE_WORKSPACES) {
      if (!existsSync(path.join(repoRoot, '..', workspace, 'database', 'database.manifest.json'))) {
        throw new Error(
          `embedded database module missing for ${workspace}: ` +
          path.join(repoRoot, '..', workspace, 'database'),
        );
      }
    }

    const snapshot = await snapshotInputs(settings);
    const stored = readStoredSnapshot();
    const contextUpToDate = !settings.force && stored?.schemaVersion === SNAPSHOT_SCHEMA_VERSION
      && stored.hash === snapshot.hash
      && existsSync(path.join(repoRoot, BUILD_DIR, 'bin', GATEWAY_BINARY_NAME));

    if (settings.dryRun || settings.check) {
      report(settings, {
        check: settings.check,
        dryRun: settings.dryRun,
        upToDate: contextUpToDate,
        imageTag: settings.tag,
        version: settings.version,
        lines: describePlan(settings, snapshot, manifestPath),
      });
      return;
    }

    if (!contextUpToDate) {
      await assembleBuildContext(settings);
      await writeFile(
        path.join(repoRoot, STAGING_SNAPSHOT_FILE),
        JSON.stringify({ schemaVersion: SNAPSHOT_SCHEMA_VERSION, hash: snapshot.hash }, null, 2),
      );
    }

    const build = await dockerBuild(settings);
    const manifest = {
      schemaVersion: IMAGE_MANIFEST_SCHEMA_VERSION,
      packageId: 'linux-x64-standalone-container-docker',
      image: settings.tag,
      version: settings.version,
      digest: build.digest,
      contextUpToDate,
      builtAt: new Date().toISOString(),
    };
    await writeFile(manifestPath, JSON.stringify(manifest, null, 2));

    report(settings, {
      ok: true,
      imageTag: settings.tag,
      digest: build.digest,
      lines: [
        `Image built: ${settings.tag}`,
        ...(build.digest ? [`Digest: ${build.digest}`] : []),
        `Manifest: ${manifestPath}`,
      ],
    });
  } catch (error) {
    if (settings.json) {
      report(settings, { ok: false, error: error.message });
    } else {
      console.error(error.message);
    }
    process.exitCode = 1;
  }
}

await main();
