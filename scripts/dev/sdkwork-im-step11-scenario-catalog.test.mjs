#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const catalogPath = path.join(repoRoot, 'tools', 'perf', 'step-11-scenario-catalog.json');
const preReleaseIndexPath = path.join(
  repoRoot,
  'artifacts',
  'perf',
  'step-11',
  'pre-release',
  'pre-release-tier-evidence-index.json',
);
const capacityIndexPath = path.join(
  repoRoot,
  'artifacts',
  'perf',
  'step-11',
  'capacity',
  'capacity-tier-evidence-index.json',
);

const catalog = JSON.parse(fs.readFileSync(catalogPath, 'utf8'));
const preReleaseIndex = JSON.parse(fs.readFileSync(preReleaseIndexPath, 'utf8'));
const capacityIndex = JSON.parse(fs.readFileSync(capacityIndexPath, 'utf8'));

const tierById = Object.fromEntries(catalog.tiers.map((tier) => [tier.id, tier]));

assert.equal(
  tierById['pre-release']?.state,
  preReleaseIndex.state,
  'pre-release tier state must match pre-release-tier-evidence-index.json',
);
assert.equal(
  tierById['capacity']?.state,
  capacityIndex.state,
  'capacity tier state must match capacity-tier-evidence-index.json',
);

for (const tier of catalog.tiers) {
  if (tier.artifactRoot) {
    assert.equal(
      fs.existsSync(path.join(repoRoot, tier.artifactRoot)),
      true,
      `missing tier artifact root: ${tier.artifactRoot}`,
    );
  }
}

const missingAssets = [];
for (const scenario of catalog.scenarioFamilies) {
  for (const relativePath of scenario.repoAssets ?? []) {
    if (relativePath.startsWith('artifacts/')) {
      continue;
    }
    const absolutePath = path.join(repoRoot, relativePath);
    if (!fs.existsSync(absolutePath)) {
      missingAssets.push(`${scenario.id}: ${relativePath}`);
    }
  }
}

assert.equal(
  missingAssets.length,
  0,
  `step-11 scenario catalog references missing repo assets:\n${missingAssets.join('\n')}`,
);

const operatorDoc = fs.readFileSync(path.join(repoRoot, catalog.operatorDocPath), 'utf8');
assert.match(operatorDoc, /performance_ha_dr_drill_test\.rs/u);
assert.match(operatorDoc, /STEP11_HA_DR/u);

console.log('sdkwork-im step-11 scenario catalog contract passed');
