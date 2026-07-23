#!/usr/bin/env node
/**
 * Component-spec to workspace implementation consistency check.
 *
 * The check owns the contract-to-implementation boundary:
 *
 *  1. Every Cargo workspace member under crates/, services/, adapters/, and tools/
 *     ships a module README.
 *  2. Every component's canonical standard and permission manifest references resolve
 *     relative to the component root, as required by COMPONENT_SPEC.md.
 *  3. The repository component manifests exist at the repository root.
 *  4. Repository verification commands are non-empty, executable, and registered.
 *  5. Application and workflow security declarations remain aligned.
 */
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const ignoredDirectories = new Set(['.git', 'build', 'dist', 'node_modules', 'target']);

function abs(relativePath) {
  return path.join(repoRoot, relativePath);
}

function readText(relativePath) {
  const filePath = abs(relativePath);
  assert.ok(fs.existsSync(filePath), `${relativePath} must exist`);
  return fs.readFileSync(filePath, 'utf8').replace(/\r\n/gu, '\n');
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

function exists(relativePath) {
  return fs.existsSync(abs(relativePath));
}

function toPosix(value) {
  return value.replaceAll('\\', '/');
}

function listComponentSpecPaths(directory = repoRoot, records = []) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (!ignoredDirectories.has(entry.name)) {
        listComponentSpecPaths(path.join(directory, entry.name), records);
      }
      continue;
    }
    if (entry.name === 'component.spec.json') {
      records.push(path.join(directory, entry.name));
    }
  }
  return records.sort();
}

/**
 * Parse the workspace Cargo members list and return paths relative to the repo root.
 */
function parseWorkspaceMembers() {
  const cargoText = readText('Cargo.toml');
  const members = [];
  let inMembers = false;
  for (const line of cargoText.split('\n')) {
    if (/^members\s*=\s*\[/u.test(line)) {
      inMembers = true;
      continue;
    }
    if (inMembers && /^\]/u.test(line)) {
      inMembers = false;
      continue;
    }
    if (inMembers) {
      const match = line.match(/^\s*"([^"]+)"/u);
      if (match) {
        members.push(match[1]);
      }
    }
  }
  return members;
}

// --- 1. Every workspace member ships a module README ------------------------

const members = parseWorkspaceMembers();
assert.ok(members.length > 0, 'workspace Cargo.toml must declare members');

const membersWithoutReadme = members.filter((member) => !exists(`${member}/README.md`));
assert.deepEqual(
  membersWithoutReadme,
  [],
  `every Cargo workspace member must ship a module README (DOCUMENTATION_SPEC.md); missing for: ${membersWithoutReadme.join(', ') || '(none)'}`,
);

// --- 2. Component-local authority and permission paths resolve -------------

const componentSpecPaths = listComponentSpecPaths();
assert.ok(componentSpecPaths.length > 0, 'repository must declare component specs');

const missingCanonicalContracts = [];
const unresolvedSpecs = [];
const unresolvedPermissionManifests = [];
for (const componentSpecPath of componentSpecPaths) {
  const componentSpec = JSON.parse(fs.readFileSync(componentSpecPath, 'utf8'));
  const componentRoot = path.dirname(path.dirname(componentSpecPath));
  const relativeSpecPath = toPosix(path.relative(repoRoot, componentSpecPath));
  const canonicalSpecs = componentSpec.canonicalSpecs ?? [];

  if (componentSpec.component?.generated !== true && canonicalSpecs.length === 0) {
    missingCanonicalContracts.push(relativeSpecPath);
  }
  for (const entry of canonicalSpecs) {
    const resolved = path.resolve(componentRoot, entry.path);
    if (!fs.existsSync(resolved)) {
      unresolvedSpecs.push(`${relativeSpecPath}: ${entry.path}`);
    }
  }

  const permissionComposition = componentSpec.contracts?.permissionComposition;
  const permissionManifestRefs = [
    ...(permissionComposition?.moduleCatalogRefs ?? []).map((entry) => entry.manifestRef),
    permissionComposition?.applicationModule?.manifestRef,
  ].filter(Boolean);
  for (const manifestRef of permissionManifestRefs) {
    const manifestPath = manifestRef.split('#')[0];
    if (!fs.existsSync(path.resolve(componentRoot, manifestPath))) {
      unresolvedPermissionManifests.push(`${relativeSpecPath}: ${manifestRef}`);
    }
  }
}

assert.deepEqual(
  missingCanonicalContracts,
  [],
  `authored component specs must declare canonicalSpecs; missing: ${missingCanonicalContracts.join(', ') || '(none)'}`,
);
assert.deepEqual(
  unresolvedSpecs,
  [],
  `component canonicalSpecs paths must resolve from each component root; unresolved: ${unresolvedSpecs.join(', ') || '(none)'}`,
);
assert.deepEqual(
  unresolvedPermissionManifests,
  [],
  `permission manifest references must resolve from each component root; unresolved: ${unresolvedPermissionManifests.join(', ') || '(none)'}`,
);

const componentPortBindingCheck = spawnSync(
  process.execPath,
  [
    path.resolve(repoRoot, '..', 'sdkwork-specs', 'tools', 'check-component-port-bindings.mjs'),
    '--root',
    repoRoot,
    '--strict',
  ],
  { cwd: repoRoot, encoding: 'utf8' },
);
assert.equal(
  componentPortBindingCheck.status,
  0,
  componentPortBindingCheck.stderr || componentPortBindingCheck.stdout,
);

// --- 3. Repository component manifests exist -------------------------------

const componentSpec = readJson('specs/component.spec.json');
const manifests = componentSpec.component?.manifests ?? [];
assert.ok(
  manifests.length > 0,
  'specs/component.spec.json component.manifests must declare at least one manifest',
);
const missingManifests = manifests.filter((manifest) => !exists(manifest));
assert.deepEqual(
  missingManifests,
  [],
  `specs/component.spec.json component.manifests must exist at the repo root; missing: ${missingManifests.join(', ') || '(none)'}`,
);

// --- 4. Verification commands are declared and shaped ----------------------

const verificationCommands = componentSpec.verification?.commands ?? [];
assert.ok(
  verificationCommands.length > 0,
  'specs/component.spec.json verification.commands must declare at least one command',
);
for (const command of verificationCommands) {
  assert.match(
    command,
    /^(cargo|node|pnpm)\s+\S/u,
    `specs/component.spec.json verification command must start with a known runner (cargo|node|pnpm): ${command}`,
  );
}

const rootPackageJson = readJson('package.json');
const rootScripts = rootPackageJson.scripts ?? {};
for (const command of verificationCommands) {
  const pnpmMatch = command.match(/^pnpm\s+([^\s]+)/u);
  if (!pnpmMatch) {
    continue;
  }
  const scriptName = pnpmMatch[1].replace(/^run:/u, '');
  assert.ok(
    Object.prototype.hasOwnProperty.call(rootScripts, scriptName),
    `package.json must expose script "${scriptName}" for verification command: ${command}`,
  );
}

// --- 5. Application and workflow security declarations align ---------------

const appManifest = readJson('sdkwork.app.config.json');
const workflowManifest = readJson('sdkwork.workflow.json');
const appSecurity = appManifest.security ?? {};
const workflowSecurity = workflowManifest.security ?? {};
if (appSecurity.sbomRequired === true) {
  assert.equal(
    workflowSecurity.sbomRequired,
    true,
    'sdkwork.workflow.json security.sbomRequired must be true when sdkwork.app.config.json requires SBOM',
  );
}
if (appSecurity.signatureRequired === true) {
  assert.equal(
    workflowSecurity.signingRequired,
    true,
    'sdkwork.workflow.json security.signingRequired must be true when sdkwork.app.config.json requires signatures',
  );
}
const lifecycle = workflowManifest.lifecycle ?? {};
if (workflowSecurity.sbomRequired === true) {
  assert.ok(
    Array.isArray(lifecycle.sbom) && lifecycle.sbom.length > 0,
    'sdkwork.workflow.json lifecycle.sbom must declare at least one step when security.sbomRequired is true',
  );
}
if (workflowSecurity.signingRequired === true) {
  assert.ok(
    Array.isArray(lifecycle.sign) && lifecycle.sign.length > 0,
    'sdkwork.workflow.json lifecycle.sign must declare at least one step when security.signingRequired is true',
  );
}

process.stdout.write('sdkwork-im component-spec consistency passed\n');
