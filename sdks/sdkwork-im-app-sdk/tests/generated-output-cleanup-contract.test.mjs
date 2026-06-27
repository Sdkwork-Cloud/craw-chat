import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { sdkFamilyConfig } from '../bin/sdk-family-config.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(__dirname, '..');

const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-app-sdk-cleanup-'));

try {
  const familyRoot = path.join(tempRoot, 'sdkwork-im-app-sdk');
  const binRoot = path.join(familyRoot, 'bin');
  const outputRoot = path.join(
    familyRoot,
    'sdkwork-im-app-sdk-csharp',
    'generated',
    'server-openapi',
  );
  const preservedCustomFile = path.join(outputRoot, 'custom', 'Preserved.cs');
  const staleManualBackupFile = path.join(outputRoot, '.sdkwork', 'manual-backups', 'Api', 'RtcApi.cs');
  const primaryClientFile = path.join(outputRoot, `${sdkFamilyConfig.primaryClient}.cs`);
  const legacyClientFile = path.join(outputRoot, `${sdkFamilyConfig.legacyClient}.cs`);

  fs.mkdirSync(binRoot, { recursive: true });
  fs.mkdirSync(path.dirname(preservedCustomFile), { recursive: true });
  fs.mkdirSync(path.dirname(staleManualBackupFile), { recursive: true });
  fs.copyFileSync(
    path.join(sdkRoot, 'bin', 'prepare-generated-output.mjs'),
    path.join(binRoot, 'prepare-generated-output.mjs'),
  );
  fs.copyFileSync(
    path.join(sdkRoot, 'bin', 'sdk-family-config.mjs'),
    path.join(binRoot, 'sdk-family-config.mjs'),
  );
  fs.writeFileSync(primaryClientFile, 'stale primary client', 'utf8');
  fs.writeFileSync(legacyClientFile, 'stale legacy client', 'utf8');
  fs.writeFileSync(staleManualBackupFile, 'stale dependency-owned RTC backup', 'utf8');
  fs.writeFileSync(preservedCustomFile, 'preserved custom scaffold', 'utf8');

  const result = spawnSync(
    process.execPath,
    [path.join(binRoot, 'prepare-generated-output.mjs'), '--language', 'csharp'],
    { encoding: 'utf8', stdio: 'pipe' },
  );

  assert.equal(result.status, 0, result.stderr || result.stdout);
  assert.equal(
    fs.existsSync(primaryClientFile),
    false,
    'C# generated-output cleanup must remove the current primary SDK client file.',
  );
  assert.equal(
    fs.existsSync(legacyClientFile),
    false,
    'C# generated-output cleanup must remove the legacy SDK client alias file.',
  );
  assert.equal(
    fs.existsSync(staleManualBackupFile),
    false,
    'C# generated-output cleanup must remove stale generator manual backups.',
  );
  assert.equal(
    fs.existsSync(preservedCustomFile),
    true,
    'C# generated-output cleanup must preserve generated custom scaffolds.',
  );
} finally {
  fs.rmSync(tempRoot, { force: true, recursive: true });
}

console.log('sdkwork-im-app-sdk generated-output cleanup contract passed');
