#!/usr/bin/env node

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const REPO_ROOT = path.resolve(path.dirname(__filename), '..', '..');
const MANIFEST_PATH = path.join(REPO_ROOT, 'sdkwork.app.config.json');
const DEFAULT_IM_PC_APP_ID = 'sdkwork-im-pc';
const DEFAULT_IAM_TENANT_ID = '100001';
const DEFAULT_IAM_ORGANIZATION_ID = '0';

export const SDKWORK_ACCESS_TOKEN_ENV_KEY = 'SDKWORK_ACCESS_TOKEN';

// ---------------------------------------------------------------------------
// createTestJwt — inlined from @sdkwork/runtime-bootstrap (sdkwork-appbase).
//
// Previously this module imported a source file from the sibling sdkwork-iam
// repository (`../../../sdkwork-iam/scripts/dev/create-dev-bootstrap-access-token-env.mjs`),
// which in turn imported `@sdkwork/runtime-bootstrap`. That cross-repo source
// import broke after pnpm cache/store cleanup because Node.js ESM resolution
// walks `node_modules` starting from the physical file location inside
// sdkwork-iam — a separate pnpm workspace whose `node_modules` may not contain
// `@sdkwork/runtime-bootstrap` after pruning.
//
// The `createTestJwt` function is a trivial unsigned-JWT factory used only for
// local dev bootstrap access tokens. Inlining it eliminates the cross-workspace
// dependency entirely and makes `pnpm dev` resilient to cache cleanup.
// ---------------------------------------------------------------------------

const SDKWORK_TOKEN_VERSION_CURRENT = 1;

function createTestJwt(claims) {
  const payload = {
    token_version: SDKWORK_TOKEN_VERSION_CURRENT,
    ...claims,
  };
  const header = btoa(JSON.stringify({ alg: 'none', typ: 'JWT' })).replace(/=+$/g, '');
  const encodedPayload = btoa(JSON.stringify(payload)).replace(/=+$/g, '');
  return `${header}.${encodedPayload}.signature`;
}

// ---------------------------------------------------------------------------
// Manifest helpers — inlined from sdkwork-iam create-dev-bootstrap-access-token-env.mjs
// ---------------------------------------------------------------------------

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function resolveAppIdFromManifest(manifest) {
  const appKey = manifest?.app?.key?.trim();
  if (appKey) {
    return appKey;
  }
  const legacyAppId = manifest?.app?.id?.trim();
  if (legacyAppId) {
    return legacyAppId;
  }
  throw new Error('sdkwork.app.config.json app.key is required for IAM bootstrap access token generation');
}

function resolveTenantIdFromManifest(manifest) {
  return normalizeText(manifest?.backend?.tenantId) ?? DEFAULT_IAM_TENANT_ID;
}

function resolveOrganizationIdFromManifest(manifest) {
  return normalizeText(manifest?.backend?.organizationId) ?? DEFAULT_IAM_ORGANIZATION_ID;
}

function readApplicationManifest(manifestPath) {
  return JSON.parse(readFileSync(manifestPath, 'utf8'));
}

// ---------------------------------------------------------------------------
// Bootstrap access token creation — self-contained, no cross-repo imports
// ---------------------------------------------------------------------------

function createDevBootstrapAccessTokenJwt(options = {}) {
  const manifest = options.manifest ?? {};
  const tenantId = normalizeText(options.tenantId) ?? resolveTenantIdFromManifest(manifest);
  const organizationId = normalizeText(options.organizationId)
    ?? resolveOrganizationIdFromManifest(manifest);
  const appId = normalizeText(options.appId) ?? resolveAppIdFromManifest(manifest);
  const nowUnixSeconds = Math.floor(Date.now() / 1000);

  return createTestJwt({
    app_id: appId,
    deployment_mode: options.deploymentMode ?? 'saas',
    environment: options.environment ?? 'dev',
    exp: nowUnixSeconds + 86_400,
    login_scope: organizationId === '0' ? 'TENANT' : 'ORGANIZATION',
    organization_id: organizationId,
    runtime_target: options.runtimeTarget ?? 'browser',
    session_id: options.sessionId ?? 'bootstrap-local-dev',
    tenant_id: tenantId,
    token_kind: 'access',
    user_id: options.userId ?? '0',
  });
}

function buildBootstrapAccessTokenEnvRecord(existingAccessToken, options = {}) {
  const normalized = normalizeText(existingAccessToken);
  return {
    [SDKWORK_ACCESS_TOKEN_ENV_KEY]: normalized || createDevBootstrapAccessTokenJwt(options),
  };
}

function mergeBootstrapAccessTokenEnv(env = {}, options = {}) {
  return {
    ...env,
    ...buildBootstrapAccessTokenEnvRecord(env[SDKWORK_ACCESS_TOKEN_ENV_KEY], options),
  };
}

function mergeBootstrapAccessTokenEnvFromManifest({
  env = {},
  manifestPath,
  ...options
} = {}) {
  const manifest = readApplicationManifest(manifestPath);
  return mergeBootstrapAccessTokenEnv(env, { ...options, manifest });
}

function resolveRepoApplicationManifestPath(repoRoot, manifestPath) {
  const normalizedRepoRoot = normalizeText(repoRoot);
  if (!normalizedRepoRoot) {
    throw new Error('resolveRepoApplicationManifestPath requires repoRoot');
  }

  const explicitManifestPath = normalizeText(manifestPath);
  if (explicitManifestPath) {
    return path.isAbsolute(explicitManifestPath)
      ? explicitManifestPath
      : path.join(normalizedRepoRoot, explicitManifestPath);
  }

  const defaultManifestPath = path.join(normalizedRepoRoot, 'sdkwork.app.config.json');
  if (existsSync(defaultManifestPath)) {
    return defaultManifestPath;
  }

  throw new Error(
    `sdkwork.app.config.json not found under ${normalizedRepoRoot}; pass manifestPath explicitly`,
  );
}

function mergeRepoDevBootstrapAccessTokenEnv({
  repoRoot,
  env = {},
  manifestPath,
  ...options
} = {}) {
  const resolvedManifestPath = resolveRepoApplicationManifestPath(repoRoot, manifestPath);
  return mergeBootstrapAccessTokenEnvFromManifest({
    env,
    manifestPath: resolvedManifestPath,
    ...options,
  });
}

// ---------------------------------------------------------------------------
// Public API — same exports as before, now fully self-contained
// ---------------------------------------------------------------------------

export function buildSdkworkImBootstrapAccessTokenEnv({
  existingAccessToken,
} = {}) {
  return buildBootstrapAccessTokenEnvRecord(existingAccessToken, {
    appId: DEFAULT_IM_PC_APP_ID,
  });
}

export function resolveSdkworkImBootstrapAccessTokenEnv(env = process.env) {
  return buildBootstrapAccessTokenEnvRecord(env[SDKWORK_ACCESS_TOKEN_ENV_KEY], {
    appId: DEFAULT_IM_PC_APP_ID,
  });
}

export function mergeSdkworkImBootstrapAccessTokenEnv(env = process.env) {
  return mergeRepoDevBootstrapAccessTokenEnv({
    repoRoot: REPO_ROOT,
    manifestPath: MANIFEST_PATH,
    appId: DEFAULT_IM_PC_APP_ID,
    env,
  });
}
