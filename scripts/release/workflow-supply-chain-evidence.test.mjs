import assert from 'node:assert/strict';
import { generateKeyPairSync, verify } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  createSbomAndProvenance,
  signReleaseArtifact,
  stableUuid,
} from './workflow-supply-chain-evidence.mjs';

let tempRoot;

test.beforeEach(() => {
  tempRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-workflow-supply-chain-'));
});
test.afterEach(() => rmSync(tempRoot, { recursive: true, force: true }));

test('creates a verifiable detached signature and byte-bound SBOM/provenance', () => {
  const artifactRelativePath = 'dist/release-packages/sdkwork-im-demo-0.1.0.zip';
  const artifactPath = path.join(tempRoot, artifactRelativePath);
  mkdirSync(path.dirname(artifactPath), { recursive: true });
  const artifact = Buffer.from('immutable sdkwork release artifact');
  writeFileSync(artifactPath, artifact);
  const { privateKey, publicKey } = generateKeyPairSync('ed25519');
  const env = {
    SDKWORK_PACKAGE_ARTIFACT_PATH: artifactRelativePath,
    SDKWORK_PACKAGE_ID: 'demo-package',
    SDKWORK_PACKAGE_TARGET_ID: 'demo-package',
    SDKWORK_PACKAGE_VERSION: '0.1.0',
    SDKWORK_RUNTIME_TARGET: 'server',
    SDKWORK_PACKAGE_PLATFORM: 'linux',
    SDKWORK_RELEASE_SIGNING_PRIVATE_KEY: privateKey.export({ format: 'pem', type: 'pkcs8' }).toString(),
  };

  const signed = signReleaseArtifact({ env, root: tempRoot });
  const signatureEnvelope = JSON.parse(readFileSync(signed.signaturePath, 'utf8'));
  assert.equal(
    verify(null, artifact, publicKey, Buffer.from(signatureEnvelope.signatureBase64, 'base64')),
    true,
  );

  let evidenceContext = null;
  const result = createSbomAndProvenance({
    env,
    evidenceWriter: (context) => { evidenceContext = context; },
    root: tempRoot,
    sourceCommit: 'a'.repeat(40),
  });
  const sbom = JSON.parse(readFileSync(result.sbomPath, 'utf8'));
  const provenance = JSON.parse(readFileSync(result.provenancePath, 'utf8'));
  assert.equal(sbom.bomFormat, 'CycloneDX');
  assert.equal(provenance.subject[0].digest.sha256, result.digest.slice('sha256:'.length));
  assert.equal(evidenceContext.sourceCommit, 'a'.repeat(40));
});

test('stable UUID generation is deterministic and UUID-shaped', () => {
  const value = stableUuid('sdkwork-im');
  assert.equal(value, stableUuid('sdkwork-im'));
  assert.match(value, /^[a-f0-9]{8}-[a-f0-9]{4}-5[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/u);
});

test('signs RSA release artifacts with SHA-256', () => {
  const artifactRelativePath = 'dist/release-packages/sdkwork-im-rsa-0.1.0.zip';
  const artifactPath = path.join(tempRoot, artifactRelativePath);
  mkdirSync(path.dirname(artifactPath), { recursive: true });
  const artifact = Buffer.from('immutable RSA signed artifact');
  writeFileSync(artifactPath, artifact);
  const { privateKey, publicKey } = generateKeyPairSync('rsa', { modulusLength: 2048 });
  const signed = signReleaseArtifact({
    env: {
      SDKWORK_PACKAGE_ARTIFACT_PATH: artifactRelativePath,
      SDKWORK_PACKAGE_ID: 'rsa-package',
      SDKWORK_RELEASE_SIGNING_PRIVATE_KEY: privateKey.export({ format: 'pem', type: 'pkcs8' }).toString(),
    },
    root: tempRoot,
  });
  const envelope = JSON.parse(readFileSync(signed.signaturePath, 'utf8'));
  assert.equal(envelope.algorithm, 'rsa');
  assert.equal(envelope.hashAlgorithm, 'sha256');
  assert.equal(
    verify('sha256', artifact, publicKey, Buffer.from(envelope.signatureBase64, 'base64')),
    true,
  );
});

test('signing fails before evidence creation without real key material', () => {
  const artifactRelativePath = 'dist/release-packages/demo.zip';
  const artifactPath = path.join(tempRoot, artifactRelativePath);
  mkdirSync(path.dirname(artifactPath), { recursive: true });
  writeFileSync(artifactPath, 'artifact');
  assert.throws(
    () => signReleaseArtifact({
      env: {
        SDKWORK_PACKAGE_ARTIFACT_PATH: artifactRelativePath,
        SDKWORK_PACKAGE_ID: 'demo-package',
      },
      root: tempRoot,
    }),
    /release signing requires/u,
  );
});
