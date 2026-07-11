import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { assessAppReleaseEvidence } from './commercial-readiness.mjs';
import {
  applySdkworkImReleaseEvidenceSync,
  createSdkworkImReleaseEvidenceSyncPlan,
  parseReleaseEvidenceSyncArgs,
} from './sync-sdkwork-im-release-evidence.mjs';

test('release evidence sync fails closed when real package evidence files are missing', async () => {
  const tempRepoRoot = await createReleaseEvidenceFixture({
    includeEvidence: false,
  });

  try {
    const plan = await createSdkworkImReleaseEvidenceSyncPlan({
      root: tempRepoRoot,
    });

    assert.equal(plan.ok, false);
    assert.match(plan.issues.join('\n'), /signature evidence is missing/i);
    assert.match(plan.issues.join('\n'), /SBOM evidence is missing/i);
    assert.match(plan.issues.join('\n'), /provenance evidence is missing/i);
    assert.equal(plan.updatedManifest, null);
  } finally {
    await rm(tempRepoRoot, { recursive: true, force: true });
  }
});

test('release evidence sync updates manifest only from existing artifacts and evidence refs', async () => {
  const tempRepoRoot = await createReleaseEvidenceFixture({
    includeEvidence: true,
  });

  try {
    const plan = await createSdkworkImReleaseEvidenceSyncPlan({
      root: tempRepoRoot,
    });

    assert.equal(plan.ok, true, `unexpected sync issues: ${plan.issues.join('; ')}`);
    assert.ok(plan.updatedManifest, 'sync plan should include an updated manifest');

    const releasePackage = plan.updatedManifest.artifacts.installConfig.packages[0];
    assert.equal(releasePackage.checksum, plan.packages[0].archive.sha256);
    assert.equal(releasePackage.sizeBytes, plan.packages[0].archive.sizeBytes);
    assert.equal(
      releasePackage.url,
      'https://cdn.sdkwork.com/apps/chat/STABLE/0.1.0/sdkwork-im-web-universal-cloud-browser-zip-0.1.0.zip',
      'sync must not replace published package URLs with local file paths',
    );
    assert.deepEqual(releasePackage.metadata.artifact, {
      ref: 'dist/release-packages/sdkwork-im-web-universal-cloud-browser-zip-0.1.0.zip',
      manifestRef: 'dist/release-packages/sdkwork-im-web-universal-cloud-browser-zip-0.1.0.manifest.json',
    });
    assert.deepEqual(releasePackage.metadata.signing, {
      kind: 'detached-signature',
      ref: 'dist/release-evidence/web-universal-cloud-browser-zip/sdkwork-im-web-universal-cloud-browser-zip-0.1.0.zip.sig',
    });
    assert.deepEqual(releasePackage.metadata.sbom, {
      format: 'CycloneDX',
      ref: 'dist/release-evidence/web-universal-cloud-browser-zip/sdkwork-im-web-universal-cloud-browser-zip-0.1.0.zip.cdx.json',
    });
    assert.deepEqual(releasePackage.metadata.provenance, {
      format: 'in-toto',
      ref: 'dist/release-evidence/web-universal-cloud-browser-zip/sdkwork-im-web-universal-cloud-browser-zip-0.1.0.zip.intoto.jsonl',
    });

    const readiness = assessAppReleaseEvidence(plan.updatedManifest, {
      repoRoot: tempRepoRoot,
    });
    assert.equal(readiness.ok, true, readiness.blockers.join('\n'));
  } finally {
    await rm(tempRepoRoot, { recursive: true, force: true });
  }
});

test('release evidence sync writes sdkwork.app.config.json only when the plan is complete', async () => {
  const tempRepoRoot = await createReleaseEvidenceFixture({
    includeEvidence: true,
  });

  try {
    const plan = await createSdkworkImReleaseEvidenceSyncPlan({
      root: tempRepoRoot,
    });
    const result = await applySdkworkImReleaseEvidenceSync(plan, {
      write: true,
    });

    assert.equal(result.written, true);
    const manifestJson = JSON.parse(await readFile(path.join(tempRepoRoot, 'sdkwork.app.config.json'), 'utf8'));
    assert.equal(
      manifestJson.artifacts.installConfig.packages[0].checksum,
      plan.packages[0].archive.sha256,
    );
  } finally {
    await rm(tempRepoRoot, { recursive: true, force: true });
  }
});

test('release evidence sync argument parser keeps check and write modes explicit', () => {
  const settings = parseReleaseEvidenceSyncArgs([
    '--check',
    '--json',
    '--package-id',
    'web-universal-cloud-browser-zip',
    '--packages-dir',
    'dist/custom-packages',
    '--evidence-root',
    'dist/custom-evidence',
  ]);

  assert.deepEqual(settings, {
    check: true,
    evidenceRoot: 'dist/custom-evidence',
    help: false,
    json: true,
    manifestPath: null,
    packageIds: ['web-universal-cloud-browser-zip'],
    packagesDir: 'dist/custom-packages',
    write: false,
  });
});

test('release evidence sync argument parser rejects check and write together', () => {
  assert.throws(
    () => parseReleaseEvidenceSyncArgs(['--check', '--write']),
    /--check and --write cannot be used together/u,
  );
});

async function createReleaseEvidenceFixture({ includeEvidence }) {
  const tempRepoRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-release-evidence-sync-'));
  const packageId = 'web-universal-cloud-browser-zip';
  const archiveName = 'sdkwork-im-web-universal-cloud-browser-zip-0.1.0.zip';
  const archiveBytes = Buffer.from('real release artifact bytes');
  const archiveSha256 = sha256(archiveBytes);
  const releasePackagesDir = path.join(tempRepoRoot, 'dist', 'release-packages');
  const releaseEvidenceDir = path.join(tempRepoRoot, 'dist', 'release-evidence', packageId);

  await mkdir(releasePackagesDir, { recursive: true });
  await writeFile(path.join(releasePackagesDir, archiveName), archiveBytes);
  await writeFile(
    path.join(releasePackagesDir, archiveName.replace(/\.zip$/u, '.manifest.json')),
    JSON.stringify({
      product: 'chat',
      package: {
        id: packageId,
      },
      archive: {
        file: archiveName,
        packageId,
        sha256: archiveSha256,
      },
      files: [
        {
          path: 'web-manifest.json',
          size: 2,
          sha256: sha256(Buffer.from('{}')),
        },
      ],
    }, null, 2),
    'utf8',
  );
  await writeFile(
    path.join(releasePackagesDir, 'release-packages-manifest.json'),
    JSON.stringify({
      schemaVersion: '2026-06-04.sdkwork-im.release-packages-manifest.v1',
      product: 'chat',
      archives: [
        {
          file: archiveName,
          packageId,
          size: archiveBytes.length,
          sha256: archiveSha256,
        },
      ],
    }, null, 2),
    'utf8',
  );

  if (includeEvidence) {
    await mkdir(releaseEvidenceDir, { recursive: true });
    await writeFile(path.join(releaseEvidenceDir, `${archiveName}.sig`), 'signature');
    await writeFile(path.join(releaseEvidenceDir, `${archiveName}.cdx.json`), '{"bomFormat":"CycloneDX"}');
    await writeFile(path.join(releaseEvidenceDir, `${archiveName}.intoto.jsonl`), '{"predicateType":"https://slsa.dev/provenance/v1"}\n');
  }

  await writeFile(
    path.join(tempRepoRoot, 'sdkwork.app.config.json'),
    JSON.stringify(createManifestFixture(packageId, archiveName), null, 2),
    'utf8',
  );

  return tempRepoRoot;
}

function createManifestFixture(packageId, archiveName) {
  return {
    security: {
      checksumRequired: true,
      signatureRequired: true,
      sbomRequired: true,
    },
    media: {
      icons: {
        primary: {
          id: 'chat-primary-icon',
          enabled: true,
          metadata: {},
        },
        platform: [],
      },
      screenshots: [
        {
          id: 'chat-web-screenshot',
          enabled: true,
          metadata: {},
        },
      ],
      previews: [
        {
          id: 'chat-catalog-preview',
          enabled: true,
          metadata: {},
        },
      ],
    },
    artifacts: {
      installConfig: {
        packages: [
          {
            id: packageId,
            name: 'Sdkwork IM Web Bundle',
            sourceType: 'BINARY_URL',
            packageFormat: 'ZIP',
            platform: 'WEB',
            url: `https://cdn.sdkwork.com/apps/chat/STABLE/0.1.0/${archiveName}`,
            enabled: true,
            checksumAlgorithm: 'SHA-256',
            checksum: null,
            architecture: 'universal',
            deploymentProfile: 'cloud',
            runtimeTarget: 'browser',
            metadata: {
              workflowPlatform: 'web',
              packageProfile: 'browser',
            },
          },
        ],
      },
    },
  };
}

function sha256(data) {
  return createHash('sha256').update(data).digest('hex');
}
