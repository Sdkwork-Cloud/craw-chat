#!/usr/bin/env node

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const baselinePath = path.join(repoRoot, 'tools', 'perf', 'step-11-cp11-3-local-drill-baseline.json');
const operatorDocPath = path.join(repoRoot, 'docs', '部署', '性能与灾备演练场景.md');
const drillTestPath = path.join(
  repoRoot,
  'services',
  'session-gateway',
  'tests',
  'performance_ha_dr_drill_test.rs',
);

assert.equal(fs.existsSync(baselinePath), true, 'missing CP11-3 drill baseline');
assert.equal(fs.existsSync(operatorDocPath), true, 'missing Step 11 operator doc');
assert.equal(fs.existsSync(drillTestPath), true, 'missing performance_ha_dr_drill_test.rs');

const result = spawnSync(
  'cargo',
  [
    'test',
    '-p',
    'session-gateway',
    '--test',
    'performance_ha_dr_drill_test',
    '--',
    '--nocapture',
  ],
  {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  },
);

if (result.stdout) {
  process.stdout.write(result.stdout);
}
if (result.stderr) {
  process.stderr.write(result.stderr);
}

assert.equal(
  result.status,
  0,
  `session-gateway HA/DR drill test failed with exit code ${result.status ?? 'unknown'}`,
);

console.log('sdkwork-im Step 11 HA/DR drill runner passed');
