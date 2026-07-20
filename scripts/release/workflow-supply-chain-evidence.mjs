#!/usr/bin/env node

import {
  createHash,
  createPrivateKey,
  createPublicKey,
  sign as signBytes,
} from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const MODULE_PATH = fileURLToPath(import.meta.url);
const REPO_ROOT = path.resolve(path.dirname(MODULE_PATH), '..', '..');

function requireText(value, label) {
  const normalized = String(value ?? '').trim();
  if (!normalized) throw new Error(`${label} is required`);
  return normalized;
}

function resolveArtifact(env = process.env, root = REPO_ROOT) {
  const relativePath = requireText(env.SDKWORK_PACKAGE_ARTIFACT_PATH, 'SDKWORK_PACKAGE_ARTIFACT_PATH');
  if (path.isAbsolute(relativePath) || relativePath.split(/[\\/]/u).includes('..')) {
    throw new Error('SDKWORK_PACKAGE_ARTIFACT_PATH must be a safe repository-relative path');
  }
  const artifactPath = path.resolve(root, relativePath);
  if (!existsSync(artifactPath)) throw new Error(`packaged artifact does not exist: ${artifactPath}`);
  return { artifactPath, relativePath: portable(relativePath) };
}

function releaseEvidencePaths({ env = process.env, root = REPO_ROOT } = {}) {
  const packageId = requireText(env.SDKWORK_PACKAGE_ID, 'SDKWORK_PACKAGE_ID');
  const { artifactPath, relativePath } = resolveArtifact(env, root);
  const evidenceRoot = path.join(root, 'dist', 'release-evidence', packageId);
  const artifactName = path.basename(artifactPath);
  return {
    artifactName,
    artifactPath,
    artifactRelativePath: relativePath,
    evidenceRoot,
    provenancePath: path.join(evidenceRoot, `${artifactName}.intoto.jsonl`),
    sbomPath: path.join(evidenceRoot, `${artifactName}.cdx.json`),
    signaturePath: path.join(evidenceRoot, `${artifactName}.sig`),
  };
}

function loadSigningKey(env = process.env) {
  const inlineKey = String(env.SDKWORK_RELEASE_SIGNING_PRIVATE_KEY ?? '').trim();
  const keyFile = String(env.SDKWORK_RELEASE_SIGNING_KEY_FILE ?? '').trim();
  if (!inlineKey && !keyFile) {
    throw new Error('release signing requires SDKWORK_RELEASE_SIGNING_PRIVATE_KEY or SDKWORK_RELEASE_SIGNING_KEY_FILE');
  }
  if (inlineKey && keyFile) throw new Error('configure exactly one release signing key source');
  if (keyFile && !existsSync(keyFile)) throw new Error(`release signing key file does not exist: ${keyFile}`);
  return createPrivateKey({
    key: inlineKey || readFileSync(keyFile),
    passphrase: String(env.SDKWORK_RELEASE_SIGNING_PRIVATE_KEY_PASSWORD ?? '') || undefined,
  });
}

function signReleaseArtifact({ env = process.env, root = REPO_ROOT } = {}) {
  const paths = releaseEvidencePaths({ env, root });
  const artifact = readFileSync(paths.artifactPath);
  const privateKey = loadSigningKey(env);
  const publicKey = createPublicKey(privateKey);
  const hashAlgorithm = ['ed25519', 'ed448'].includes(privateKey.asymmetricKeyType)
    ? null
    : 'sha256';
  const signature = signBytes(hashAlgorithm, artifact, privateKey);
  const publicKeyDer = publicKey.export({ format: 'der', type: 'spki' });
  const envelope = {
    schemaVersion: 1,
    algorithm: privateKey.asymmetricKeyType,
    hashAlgorithm: hashAlgorithm ?? 'none',
    artifact: paths.artifactRelativePath,
    digest: `sha256:${sha256(artifact)}`,
    publicKeyFingerprint: `sha256:${sha256(publicKeyDer)}`,
    signatureBase64: signature.toString('base64'),
  };
  mkdirSync(paths.evidenceRoot, { recursive: true });
  writeFileSync(paths.signaturePath, `${JSON.stringify(envelope, null, 2)}\n`, { mode: 0o600 });
  return { ...paths, signature: envelope };
}

function createSbomAndProvenance({
  env = process.env,
  evidenceWriter = createWorkflowEvidence,
  root = REPO_ROOT,
  sourceCommit = gitHead(root),
} = {}) {
  const paths = releaseEvidencePaths({ env, root });
  if (!existsSync(paths.signaturePath)) throw new Error(`detached signature is missing: ${paths.signaturePath}`);
  const artifact = readFileSync(paths.artifactPath);
  const digest = sha256(artifact);
  const packageId = requireText(env.SDKWORK_PACKAGE_ID, 'SDKWORK_PACKAGE_ID');
  const version = requireText(env.SDKWORK_PACKAGE_VERSION, 'SDKWORK_PACKAGE_VERSION');
  const sbom = {
    bomFormat: 'CycloneDX',
    specVersion: '1.5',
    serialNumber: `urn:uuid:${stableUuid(`${packageId}:${version}:${digest}`)}`,
    version: 1,
    metadata: {
      component: {
        type: 'application',
        name: packageId,
        version,
        hashes: [{ alg: 'SHA-256', content: digest }],
      },
    },
    components: [{
      type: 'file',
      name: paths.artifactName,
      version,
      hashes: [{ alg: 'SHA-256', content: digest }],
      properties: [
        { name: 'sdkwork:artifactPath', value: paths.artifactRelativePath },
        { name: 'sdkwork:sizeBytes', value: String(artifact.length) },
      ],
    }],
  };
  const provenance = {
    _type: 'https://in-toto.io/Statement/v1',
    subject: [{ name: paths.artifactRelativePath, digest: { sha256: digest } }],
    predicateType: 'https://slsa.dev/provenance/v1',
    predicate: {
      buildDefinition: {
        buildType: 'https://sdkwork.com/buildtypes/github-workflow/v1',
        externalParameters: {
          packageId,
          runtimeTarget: requireText(env.SDKWORK_RUNTIME_TARGET, 'SDKWORK_RUNTIME_TARGET'),
          targetPlatform: String(env.SDKWORK_TARGET_PLATFORM ?? env.SDKWORK_PACKAGE_PLATFORM ?? '').trim() || null,
          clientArchitecture: String(env.SDKWORK_CLIENT_ARCHITECTURE ?? '').trim() || null,
        },
        internalParameters: { sourceCommit },
        resolvedDependencies: [{ uri: 'git+https://github.com/Sdkwork-Cloud/sdkwork-im', digest: { gitCommit: sourceCommit } }],
      },
      runDetails: {
        builder: { id: 'https://github.com/Sdkwork-Cloud/sdkwork-github-workflow' },
        metadata: { invocationId: String(env.GITHUB_RUN_ID ?? 'local-validation') },
      },
    },
  };
  mkdirSync(paths.evidenceRoot, { recursive: true });
  writeFileSync(paths.sbomPath, `${JSON.stringify(sbom, null, 2)}\n`, 'utf8');
  writeFileSync(paths.provenancePath, `${JSON.stringify(provenance)}\n`, 'utf8');
  evidenceWriter({ env, paths, root, sourceCommit });
  return { ...paths, digest: `sha256:${digest}`, sourceCommit };
}

function createWorkflowEvidence({ env, paths, root, sourceCommit }) {
  const cli = requireText(env.SDKWORK_WORKFLOW_CLI, 'SDKWORK_WORKFLOW_CLI');
  const evidencePaths = requireText(env.SDKWORK_ARTIFACT_EVIDENCE_PATHS, 'SDKWORK_ARTIFACT_EVIDENCE_PATHS')
    .split(/\r?\n/u)
    .map((value) => value.trim())
    .filter(Boolean);
  const relative = (value) => portable(path.relative(root, value));
  for (const evidencePath of evidencePaths) {
    const args = [
      cli,
      'evidence:create',
      '--config', 'sdkwork.workflow.json',
      '--target-id', requireText(env.SDKWORK_PACKAGE_TARGET_ID, 'SDKWORK_PACKAGE_TARGET_ID'),
      '--deployment-profile', requireText(env.SDKWORK_DEPLOYMENT_PROFILE, 'SDKWORK_DEPLOYMENT_PROFILE'),
      '--version', requireText(env.SDKWORK_PACKAGE_VERSION, 'SDKWORK_PACKAGE_VERSION'),
      '--source-commit', sourceCommit,
      '--artifact-id', requireText(env.SDKWORK_PACKAGE_ID, 'SDKWORK_PACKAGE_ID'),
      '--artifact', paths.artifactRelativePath,
      '--artifact-evidence', evidencePath,
      '--sbom', relative(paths.sbomPath),
      '--provenance', relative(paths.provenancePath),
      '--signature', relative(paths.signaturePath),
    ];
    const result = spawnSync(process.execPath, args, { cwd: root, env, stdio: 'inherit', shell: false });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`artifact evidence creation failed with exit code ${result.status ?? 1}`);
  }
}

function gitHead(root) {
  return execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
}

function stableUuid(seed) {
  const hex = createHash('sha256').update(seed).digest('hex').slice(0, 32).split('');
  hex[12] = '5';
  hex[16] = ['8', '9', 'a', 'b'][Number.parseInt(hex[16], 16) % 4];
  const value = hex.join('');
  return `${value.slice(0, 8)}-${value.slice(8, 12)}-${value.slice(12, 16)}-${value.slice(16, 20)}-${value.slice(20)}`;
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function portable(value) {
  return value.split(path.sep).join('/');
}

async function main(argv = process.argv.slice(2)) {
  const [command] = argv;
  const result = command === 'sign'
    ? signReleaseArtifact()
    : command === 'attest'
      ? createSbomAndProvenance()
      : null;
  if (!result) throw new Error('command must be sign or attest');
  console.log(JSON.stringify({
    ok: true,
    command,
    artifact: result.artifactRelativePath,
    signature: portable(path.relative(REPO_ROOT, result.signaturePath)),
    ...(result.sbomPath ? { sbom: portable(path.relative(REPO_ROOT, result.sbomPath)) } : {}),
    ...(result.provenancePath ? { provenance: portable(path.relative(REPO_ROOT, result.provenancePath)) } : {}),
  }, null, 2));
  return 0;
}

if (process.argv[1] && path.resolve(process.argv[1]) === MODULE_PATH) {
  main().then((code) => { process.exitCode = code; }).catch((error) => {
    console.error(`[sdkwork-im-supply-chain] ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  });
}

export {
  createSbomAndProvenance,
  loadSigningKey,
  releaseEvidencePaths,
  signReleaseArtifact,
  stableUuid,
};
