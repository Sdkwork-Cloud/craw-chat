#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

import {
  resolveSharedSdkMode,
  SHARED_SDK_MODE_GIT,
} from './shared-sdk-mode.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const defaultRepoRoot = path.resolve(__dirname, '..', '..');

export const SHARED_SDK_RELEASE_CONFIG_PATH_ENV_VAR = 'SDKWORK_SHARED_SDK_RELEASE_CONFIG_PATH';
export const SHARED_SDK_GIT_REF_ENV_VAR = 'SDKWORK_SHARED_SDK_GIT_REF';
export const SHARED_SDK_GIT_FORCE_SYNC_ENV_VAR = 'SDKWORK_SHARED_SDK_GIT_FORCE_SYNC';
export const SHARED_SDK_GIT_PROTOCOL_ENV_VAR = 'SDKWORK_SHARED_SDK_GIT_PROTOCOL';
export const SHARED_SDK_GITHUB_TOKEN_ENV_VAR = 'SDKWORK_SHARED_SDK_GITHUB_TOKEN';

export const DEFAULT_SHARED_SDK_RELEASE_CONFIG_PATH = 'config/shared-sdk-release-sources.json';

const SOURCE_SPECS = Object.freeze([
  {
    id: 'sdkwork-appbase',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-appbase'),
    requiredPaths: [
      'packages/pc-react/foundation/sdkwork-appbase-pc-react/package.json',
      'apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react/package.json',
      'packages/pc-react/foundation/sdkwork-i18n-pc-react/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_APPBASE_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_APPBASE_GIT_REF',
  },
  {
    id: 'sdkwork-core',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-core'),
    requiredPaths: ['sdkwork-core-pc-react/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_CORE_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_CORE_GIT_REF',
  },
  {
    id: 'sdkwork-ui',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-ui'),
    requiredPaths: ['sdkwork-ui-pc-react/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_UI_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_UI_GIT_REF',
  },
  {
    id: 'sdkwork-drive',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-drive'),
    requiredPaths: [
      'package.json',
      'sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/package.json',
      'apps/sdkwork-drive-pc/packages/sdkwork-drive-pc-drive/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_DRIVE_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_DRIVE_GIT_REF',
  },
  {
    id: 'sdkwork-mail',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-mail'),
    requiredPaths: [
      'package.json',
      'sdks/sdkwork-mail-app-sdk/sdkwork-mail-app-sdk-typescript/generated/server-openapi/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_MAIL_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_MAIL_GIT_REF',
  },
  {
    id: 'sdkwork-community',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-community'),
    requiredPaths: [
      'package.json',
      'sdks/sdkwork-community-app-sdk/sdkwork-community-app-sdk-typescript/generated/server-openapi/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_COMMUNITY_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_COMMUNITY_GIT_REF',
  },
  {
    id: 'sdkwork-course',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-course'),
    requiredPaths: [
      'package.json',
      'sdks/sdkwork-course-app-sdk/sdkwork-course-app-sdk-typescript/generated/server-openapi/package.json',
      'apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_COURSE_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_COURSE_GIT_REF',
  },
  {
    id: 'sdkwork-im-app-sdk',
    repoRoot: path.resolve(defaultRepoRoot, 'sdks', 'sdkwork-im-app-sdk'),
    requiredPaths: ['sdkwork-im-app-sdk-typescript/generated/server-openapi/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_IM_APP_SDK_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_IM_APP_SDK_GIT_REF',
  },
  {
    id: 'sdkwork-im-backend-sdk',
    repoRoot: path.resolve(defaultRepoRoot, 'sdks', 'sdkwork-im-backend-sdk'),
    requiredPaths: ['sdkwork-im-backend-sdk-typescript/generated/server-openapi/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_IM_BACKEND_SDK_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_IM_BACKEND_SDK_GIT_REF',
  },
  {
    id: 'sdkwork-im-sdk',
    repoRoot: path.resolve(defaultRepoRoot, 'sdks', 'sdkwork-im-sdk'),
    requiredPaths: ['sdkwork-im-sdk-typescript/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_IM_SDK_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_IM_SDK_GIT_REF',
  },
  {
    id: 'sdkwork-notary',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-notary'),
    requiredPaths: [
      'package.json',
      'sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_NOTARY_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_NOTARY_GIT_REF',
  },
  {
    id: 'sdkwork-knowledgebase',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-knowledgebase'),
    requiredPaths: [
      'package.json',
      'sdks/sdkwork-knowledgebase-app-sdk/sdkwork-knowledgebase-app-sdk-typescript/package.json',
      'apps/sdkwork-knowledgebase-pc/packages/sdkwork-knowledgebase-pc-knowledge/package.json',
    ],
    repoUrlEnvVar: 'SDKWORK_SHARED_KNOWLEDGEBASE_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_KNOWLEDGEBASE_GIT_REF',
  },
  {
    id: 'sdkwork-clawrouter',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-clawrouter'),
    requiredPaths: ['package.json', 'apps/sdkwork-clawrouter-pc/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_CLAW_ROUTER_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_CLAW_ROUTER_GIT_REF',
  },
  {
    id: 'sdkwork-birdcoder',
    repoRoot: path.resolve(defaultRepoRoot, '..', 'sdkwork-birdcoder'),
    requiredPaths: ['package.json', 'packages/sdkwork-birdcoder-web/package.json'],
    repoUrlEnvVar: 'SDKWORK_SHARED_BIRDCODER_REPO_URL',
    refEnvVar: 'SDKWORK_SHARED_BIRDCODER_GIT_REF',
  },
]);

const GITHUB_TOKEN_ENV_VARS = Object.freeze([
  SHARED_SDK_GITHUB_TOKEN_ENV_VAR,
  'GITHUB_TOKEN',
  'GH_TOKEN',
]);

function parseBoolean(value) {
  return ['1', 'true', 'yes', 'on'].includes(String(value ?? '').trim().toLowerCase());
}

function isObject(value) {
  return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
}

function resolveGitCommand(platform = process.platform, env = process.env) {
  if (platform !== 'win32') {
    return 'git';
  }

  const configured = [env.GIT_EXE, env.GIT]
    .filter((value) => typeof value === 'string' && value.trim().length > 0);
  if (configured.length > 0) {
    return configured[0];
  }

  return 'git.exe';
}

function runGit(args, {
  captureStdout = false,
  cwd = defaultRepoRoot,
  env = process.env,
  spawnSyncImpl = spawnSync,
} = {}) {
  const result = spawnSyncImpl(resolveGitCommand(process.platform, env), args, {
    cwd,
    encoding: 'utf8',
    env,
    shell: false,
    stdio: captureStdout ? ['ignore', 'pipe', 'pipe'] : 'inherit',
  });

  if (result.error) {
    throw new Error(`git ${args.join(' ')} failed: ${result.error.message}`);
  }
  if (result.status !== 0) {
    const stderr = String(result.stderr ?? '').trim();
    throw new Error(
      `git ${args.join(' ')} failed with exit code ${result.status ?? 'unknown'}${stderr ? `: ${stderr}` : ''}`,
    );
  }

  return String(result.stdout ?? '').trim();
}

function resolveReleaseConfigPath(repoRoot = defaultRepoRoot, env = process.env) {
  const configured = String(env?.[SHARED_SDK_RELEASE_CONFIG_PATH_ENV_VAR] ?? '').trim();
  return path.resolve(repoRoot, configured || DEFAULT_SHARED_SDK_RELEASE_CONFIG_PATH);
}

export function readSharedSdkReleaseConfig(repoRoot = defaultRepoRoot, env = process.env) {
  const configPath = resolveReleaseConfigPath(repoRoot, env);
  if (!fs.existsSync(configPath)) {
    throw new Error(`Missing shared SDK release config: ${configPath}`);
  }

  const parsed = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  if (!isObject(parsed?.sources)) {
    throw new Error(`Invalid shared SDK release config, missing sources map: ${configPath}`);
  }

  return {
    configPath,
    sources: parsed.sources,
  };
}

function resolveConfiguredSource(spec, sourceMap, env = process.env) {
  const config = sourceMap[spec.id];
  if (!isObject(config)) {
    throw new Error(`Missing shared SDK release source: ${spec.id}`);
  }

  const repoUrl = String(env?.[spec.repoUrlEnvVar] ?? config.repoUrl ?? '').trim();
  const targetRef = String(
    env?.[spec.refEnvVar]
      ?? env?.[SHARED_SDK_GIT_REF_ENV_VAR]
      ?? config.ref
      ?? 'main',
  ).trim();

  if (!repoUrl) {
    throw new Error(`Missing repoUrl for shared SDK release source: ${spec.id}`);
  }
  if (!targetRef) {
    throw new Error(`Missing ref for shared SDK release source: ${spec.id}`);
  }

  return {
    repoUrl,
    targetRef,
  };
}

function normalizeRepoUrl(repoUrl) {
  const value = String(repoUrl ?? '').trim();
  const scpMatch = /^[^@]+@([^:]+):(.+)$/u.exec(value);
  if (scpMatch) {
    return `${scpMatch[1]}/${scpMatch[2]}`.replace(/\.git$/u, '').toLowerCase();
  }

  try {
    const parsed = new URL(value);
    return `${parsed.host}${parsed.pathname}`.replace(/\.git$/u, '').replace(/\/+$/u, '').toLowerCase();
  } catch {
    return path.resolve(value).replaceAll('\\', '/').replace(/\/+$/u, '').toLowerCase();
  }
}

function isGithubHttpsRepoUrl(repoUrl) {
  try {
    const parsed = new URL(String(repoUrl ?? '').trim());
    return parsed.protocol === 'https:' && parsed.hostname.toLowerCase() === 'github.com';
  } catch {
    return false;
  }
}

function toGithubSshRepoUrl(repoUrl) {
  if (!isGithubHttpsRepoUrl(repoUrl)) {
    return repoUrl;
  }

  const parsed = new URL(String(repoUrl).trim());
  return `git@github.com:${parsed.pathname.replace(/^\/+/u, '')}`;
}

function resolveTransportRepoUrl(repoUrl, env = process.env) {
  return String(env?.[SHARED_SDK_GIT_PROTOCOL_ENV_VAR] ?? '').trim().toLowerCase() === 'ssh'
    ? toGithubSshRepoUrl(repoUrl)
    : repoUrl;
}

function resolveGithubToken(env = process.env) {
  for (const envVar of GITHUB_TOKEN_ENV_VARS) {
    const token = String(env?.[envVar] ?? '').trim();
    if (token) {
      return token;
    }
  }

  return '';
}

function resolveEmptyGitConfigPath(env = process.env) {
  if (process.platform !== 'win32') {
    return os.devNull;
  }

  const tempRoot = String(env.RUNNER_TEMP ?? '').trim() || os.tmpdir();
  const configPath = path.join(tempRoot, 'sdkwork-im-shared-sdk-empty-gitconfig');
  fs.mkdirSync(path.dirname(configPath), { recursive: true });
  if (!fs.existsSync(configPath)) {
    fs.writeFileSync(configPath, '', 'utf8');
  }
  return configPath;
}

function createGitAuthEnv(repoUrl, env = process.env) {
  const transportUrl = resolveTransportRepoUrl(repoUrl, env);
  if (/^(?:git@github\.com:|ssh:\/\/git@github\.com\/)/iu.test(transportUrl)) {
    return {
      ...env,
      GIT_CONFIG_GLOBAL: resolveEmptyGitConfigPath(env),
      GIT_SSH_COMMAND: 'ssh -o StrictHostKeyChecking=accept-new',
    };
  }

  if (!isGithubHttpsRepoUrl(transportUrl)) {
    return env;
  }

  const token = resolveGithubToken(env);
  if (!token) {
    return env;
  }

  const authHeader = Buffer.from(`x-access-token:${token}`, 'utf8').toString('base64');
  const baseIndex = Number.parseInt(String(env.GIT_CONFIG_COUNT ?? '0'), 10);
  const index = Number.isFinite(baseIndex) && baseIndex >= 0 ? baseIndex : 0;
  return {
    ...env,
    GIT_CONFIG_COUNT: String(index + 1),
    [`GIT_CONFIG_KEY_${index}`]: 'http.https://github.com/.extraheader',
    [`GIT_CONFIG_VALUE_${index}`]: `AUTHORIZATION: basic ${authHeader}`,
  };
}

function isGitCheckout(repoRoot, options = {}) {
  if (!fs.existsSync(repoRoot)) {
    return false;
  }

  try {
    return runGit(['-C', repoRoot, 'rev-parse', '--is-inside-work-tree'], {
      ...options,
      captureStdout: true,
    }) === 'true';
  } catch {
    return false;
  }
}

function assertRequiredPaths(spec) {
  for (const relativePath of spec.requiredPaths) {
    const absolutePath = path.join(spec.repoRoot, relativePath);
    if (!fs.existsSync(absolutePath)) {
      throw new Error(`Shared SDK source ${spec.id} is missing required path: ${absolutePath}`);
    }
  }
}

function assertStandaloneCheckout(spec, options = {}) {
  const topLevel = path.resolve(runGit(['-C', spec.repoRoot, 'rev-parse', '--show-toplevel'], {
    ...options,
    captureStdout: true,
  }));
  if (topLevel !== path.resolve(spec.repoRoot)) {
    throw new Error(`Shared SDK source ${spec.id} must be a standalone checkout at ${spec.repoRoot}; git top-level is ${topLevel}`);
  }
}

function assertCleanCheckout(spec, options = {}) {
  const status = runGit(['-C', spec.repoRoot, 'status', '--porcelain'], {
    ...options,
    captureStdout: true,
  });
  if (status) {
    throw new Error(`Refusing to use dirty shared SDK checkout ${spec.id}: ${spec.repoRoot}`);
  }
}

function assertRemoteMatches(spec, repoUrl, options = {}) {
  const originUrl = runGit(['-C', spec.repoRoot, 'remote', 'get-url', 'origin'], {
    ...options,
    captureStdout: true,
  });
  if (normalizeRepoUrl(originUrl) !== normalizeRepoUrl(repoUrl)) {
    throw new Error(`Shared SDK source ${spec.id} origin ${originUrl} does not match configured repo ${repoUrl}`);
  }
}

function checkoutMatchesRef(spec, targetRef, options = {}) {
  if (/^[0-9a-f]{40}$/iu.test(targetRef)) {
    const head = runGit(['-C', spec.repoRoot, 'rev-parse', 'HEAD'], {
      ...options,
      captureStdout: true,
    });
    return head.toLowerCase() === targetRef.toLowerCase();
  }

  const branch = runGit(['-C', spec.repoRoot, 'branch', '--show-current'], {
    ...options,
    captureStdout: true,
  });
  return branch === targetRef;
}

function directoryHasEntries(targetPath) {
  return fs.existsSync(targetPath)
    && fs.statSync(targetPath).isDirectory()
    && fs.readdirSync(targetPath).length > 0;
}

function cloneSource(spec, repoUrl, targetRef, options = {}) {
  const transportUrl = resolveTransportRepoUrl(repoUrl, options.env);
  fs.mkdirSync(path.dirname(spec.repoRoot), { recursive: true });
  runGit(['clone', transportUrl, spec.repoRoot], {
    ...options,
    env: createGitAuthEnv(repoUrl, options.env),
  });
  runGit(['-C', spec.repoRoot, 'checkout', '--force', targetRef], options);
}

function syncSource(spec, repoUrl, targetRef, options = {}) {
  const transportUrl = resolveTransportRepoUrl(repoUrl, options.env);
  runGit(['-C', spec.repoRoot, 'remote', 'set-url', 'origin', transportUrl], options);
  runGit(['-C', spec.repoRoot, 'fetch', '--tags', '--prune', 'origin'], {
    ...options,
    env: createGitAuthEnv(repoUrl, options.env),
  });
  runGit(['-C', spec.repoRoot, 'checkout', '--force', targetRef], options);
  runGit(['-C', spec.repoRoot, 'clean', '-fd'], options);
}

function ensureSource(spec, {
  repoUrl,
  targetRef,
  syncExistingRepos,
  logger,
  ...options
}) {
  if (!fs.existsSync(spec.repoRoot)) {
    cloneSource(spec, repoUrl, targetRef, options);
    assertRequiredPaths(spec);
    logger.log(`[prepare-shared-sdk-git-sources] cloned ${spec.id} from ${repoUrl}#${targetRef}`);
    return { id: spec.id, repoRoot: spec.repoRoot, repoUrl, targetRef, changed: true, status: 'ready' };
  }

  if (!isGitCheckout(spec.repoRoot, options)) {
    if (directoryHasEntries(spec.repoRoot)) {
      throw new Error(`Shared SDK source ${spec.id} exists but is not a git checkout: ${spec.repoRoot}`);
    }
    cloneSource(spec, repoUrl, targetRef, options);
    assertRequiredPaths(spec);
    logger.log(`[prepare-shared-sdk-git-sources] cloned ${spec.id} from ${repoUrl}#${targetRef}`);
    return { id: spec.id, repoRoot: spec.repoRoot, repoUrl, targetRef, changed: true, status: 'ready' };
  }

  assertStandaloneCheckout(spec, options);
  assertRequiredPaths(spec);
  assertCleanCheckout(spec, options);
  assertRemoteMatches(spec, repoUrl, options);

  let changed = false;
  if (!checkoutMatchesRef(spec, targetRef, options)) {
    if (!syncExistingRepos) {
      throw new Error(`Shared SDK source ${spec.id} is not on configured ref ${targetRef}: ${spec.repoRoot}`);
    }
    syncSource(spec, repoUrl, targetRef, options);
    changed = true;
  }

  logger.log(`[prepare-shared-sdk-git-sources] ready ${spec.id} from ${repoUrl}#${targetRef}`);
  return { id: spec.id, repoRoot: spec.repoRoot, repoUrl, targetRef, changed, status: 'ready' };
}

export function ensureSharedSdkGitSources({
  repoRoot = defaultRepoRoot,
  env = process.env,
  logger = console,
  spawnSyncImpl = spawnSync,
  syncExistingRepos = parseBoolean(env?.[SHARED_SDK_GIT_FORCE_SYNC_ENV_VAR]),
} = {}) {
  const mode = resolveSharedSdkMode(env);
  if (mode !== SHARED_SDK_MODE_GIT) {
    logger.log('[prepare-shared-sdk-git-sources] shared SDK mode is source; skipping git materialization.');
    return { mode, changed: false, sources: [], status: 'skipped' };
  }

  const { sources: sourceMap } = readSharedSdkReleaseConfig(repoRoot, env);
  const sources = SOURCE_SPECS.map((spec) => {
    const configured = resolveConfiguredSource(spec, sourceMap, env);
    return ensureSource(spec, {
      ...configured,
      cwd: repoRoot,
      env,
      logger,
      spawnSyncImpl,
      syncExistingRepos,
    });
  });

  return {
    mode,
    changed: sources.some((source) => source.changed),
    sources,
    status: 'ready',
  };
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    console.log(JSON.stringify(ensureSharedSdkGitSources(), null, 2));
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
