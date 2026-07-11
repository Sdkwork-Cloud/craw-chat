#!/usr/bin/env node
/**
 * Topology env file lint.
 *
 * Validates every `.env` file under `configs/topology/` enforces:
 *   - Each `KEY=VALUE` pair occupies its own line (no concatenated entries).
 *   - Line endings are `\n` or `\r\n` (bare `\r` between key pairs is rejected).
 *   - No inline `KEY=VALUE1KEY2=VALUE2` concatenations.
 *
 * This guards against the cloud.production.env line-break regression where
 * `SDKWORK_IM_DEPLOYMENT_PROFILE=cloud` and `SDKWORK_IM_ENVIRONMENT=production`
 * were joined by a bare `\r`, causing env loaders to register only the first
 * variable and silently drop `SDKWORK_IM_ENVIRONMENT`, which disabled every
 * production fail-closed guard (dev JWT secret rejection, JTI enforcement,
 * ALLOW_ALL_PRINCIPALS production ban).
 */
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const topologyDir = path.join(repoRoot, 'configs', 'topology');

const envFiles = fs
  .readdirSync(topologyDir)
  .filter((name) => name.endsWith('.env'))
  .map((name) => path.join(topologyDir, name));

assert.ok(envFiles.length > 0, 'configs/topology must contain .env profile files');

const violations = [];

for (const envFile of envFiles) {
  const raw = fs.readFileSync(envFile, null); // Buffer
  const bytes = Uint8Array.from(raw);

  // Reject bare \r (0x0D) that is NOT followed by \n (0x0A) between non-comment lines.
  for (let i = 0; i < bytes.length; i += 1) {
    if (bytes[i] === 0x0D && i + 1 < bytes.length && bytes[i + 1] !== 0x0A) {
      // Find line context for the error message.
      const lineStart = bytes.lastIndexOf(0x0A, i - 1) + 1;
      const lineEnd = bytes.indexOf(0x0A, i + 1);
      const end = lineEnd === -1 ? bytes.length : lineEnd;
      const context = Buffer.from(bytes.slice(lineStart, end)).toString('utf8');
      violations.push(
        `${path.basename(envFile)}: bare CR (0x0D) without LF at offset ${i} near: ${JSON.stringify(context)}`,
      );
    }
  }

  // Reject concatenated KEY=VALUE patterns: a value followed immediately (no newline) by an uppercase SDKWORK_ identifier.
  const text = raw.toString('utf8');
  const concatPattern = /=([^\r\n#]*?)(SDKWORK_[A-Z_]+=)/u;
  const match = text.match(concatPattern);
  if (match) {
    violations.push(
      `${path.basename(envFile)}: concatenated env entries detected — "${match[0]}" must be split onto separate lines`,
    );
  }

  // Ensure SDKWORK_IM_ENVIRONMENT is on its own line if present.
  if (text.includes('SDKWORK_IM_ENVIRONMENT')) {
    const envLinePattern = /^[ \t]*SDKWORK_IM_ENVIRONMENT=([^\r\n]*)$/mu;
    if (!envLinePattern.test(text)) {
      violations.push(
        `${path.basename(envFile)}: SDKWORK_IM_ENVIRONMENT must be on its own line as KEY=VALUE`,
      );
    }
  }
}

assert.equal(
  violations.length,
  0,
  `topology env lint violations:\n${violations.join('\n')}`,
);

console.log('sdkwork-im topology env lint passed');
