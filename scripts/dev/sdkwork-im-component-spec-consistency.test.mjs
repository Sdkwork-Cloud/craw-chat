#!/usr/bin/env node
/**
 * Component-spec ↔ workspace crate consistency check.
 *
 * Validates that the authored workspace stays aligned with `specs/component.spec.json`
 * and the SDKWork standards it references. This complements
 * `sdkwork-workspace-structure-standard.test.mjs` (which owns the directory dictionary
 * and governance artifacts) by focusing on the contract ↔ implementation boundary:
 *
 *  1. Every Cargo workspace member under crates/, services/, adapters/, tools/ ships a
 *     module README (DOCUMENTATION_SPEC.md module README rule).
 *  2. `specs/component.spec.json` `canonicalSpecs[].path` entries resolve to real files
 *     under `../sdkwork-specs/` (COMPONENT_SPEC.md authority-chain rule).
 *  3. `component.manifests` entries (sdkwork.app.config.json, package.json, Cargo.toml)
 *     exist at the repository root (COMPONENT_SPEC.md manifest rule).
 *  4. `verification.commands` referenced by the spec are non-empty and look executable.
 * Naming-alignment (the legacy sdkwork-im and im crate prefixes were consolidated to
 * sdkwork-im-) is governed by ADR-20260615-sdkwork-im-to-sdkwork-im-rebrand; the batched
 * rename is complete and intentionally NOT enforced here as a hard failure. This test only
 * hard-fails on structural drift that has no governance escape hatch.
 */
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

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

/**
 * Parse the workspace `Cargo.toml` members list and return each member directory
 * relative to the repo root. Handles quoted and bare member paths.
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

// --- 2. component.spec.json canonicalSpecs paths resolve --------------------

const componentSpec = readJson('specs/component.spec.json');
const unresolvedSpecs = [];
for (const entry of componentSpec.canonicalSpecs ?? []) {
  // Spec paths in component.spec.json are relative to the repository root
  // (e.g. "../sdkwork-specs/README.md" resolves to the sibling sdkwork-specs
  // checkout at <workspace>/sdkwork-specs), not relative to specs/.
  const resolved = path.resolve(repoRoot, entry.path);
  if (!fs.existsSync(resolved)) {
    unresolvedSpecs.push(entry.path);
  }
}
assert.deepEqual(
  unresolvedSpecs,
  [],
  `specs/component.spec.json canonicalSpecs paths must resolve under ../sdkwork-specs/; unresolved: ${unresolvedSpecs.join(', ') || '(none)'}`,
);

// --- 3. component.manifests entries exist at repo root ----------------------

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

// --- 4. verification.commands are declared and shaped -----------------------

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

// --- 5. verification.commands resolve to package.json scripts when using pnpm ---

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

// --- 6. sdkwork.app.config.json security aligns with sdkwork.workflow.json --------

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

process.stdout.write('sdkwork-chat component-spec consistency passed\n');
