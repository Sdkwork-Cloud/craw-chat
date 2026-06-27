#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveSdkworkChatIamCommandEnv } from '../../apps/sdkwork-im-pc/scripts/sdkwork-chat-iam-env.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

function pnpmCommand(platform = process.platform) {
  return platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

const RELEASE_BUILD_STEPS = Object.freeze([
  {
    args: ['sdk:shared:prepare'],
    cwd: '.',
    label: 'prepare shared SDK git sources',
  },
  {
    args: ['install', '--frozen-lockfile'],
    cwd: 'apps/sdkwork-im-pc',
    label: 'install sdkwork-im-pc workspace',
  },
  {
    args: ['build'],
    cwd: 'apps/sdkwork-im-pc',
    label: 'build sdkwork-im-pc Vite app',
  },
]);

function resolveOptionalEnvValue(...values) {
  for (const value of values) {
    const trimmedValue = String(value ?? '').trim();
    if (trimmedValue) {
      return trimmedValue;
    }
  }
  return undefined;
}

function resolveReleaseDependencyRefBridge(env = process.env) {
  const bridge = {};
  const driveRef = resolveOptionalEnvValue(env.SDKWORK_SHARED_DRIVE_GIT_REF, env.SDKWORK_DRIVE_REF);
  const notaryRef = resolveOptionalEnvValue(
    env.SDKWORK_SHARED_NOTARY_GIT_REF,
    env.SDKWORK_NOTARY_REF,
  );
  const mailRef = resolveOptionalEnvValue(
    env.SDKWORK_SHARED_MAIL_GIT_REF,
    env.SDKWORK_MAIL_REF,
  );
  const communityRef = resolveOptionalEnvValue(
    env.SDKWORK_SHARED_COMMUNITY_GIT_REF,
    env.SDKWORK_COMMUNITY_REF,
  );
  const courseRef = resolveOptionalEnvValue(
    env.SDKWORK_SHARED_COURSE_GIT_REF,
    env.SDKWORK_COURSE_REF,
  );
  const knowledgebaseRef = resolveOptionalEnvValue(
    env.SDKWORK_SHARED_KNOWLEDGEBASE_GIT_REF,
    env.SDKWORK_KNOWLEDGEBASE_REF,
  );

  if (driveRef) {
    bridge.SDKWORK_SHARED_DRIVE_GIT_REF = driveRef;
  }
  if (notaryRef) {
    bridge.SDKWORK_SHARED_NOTARY_GIT_REF = notaryRef;
  }
  if (mailRef) {
    bridge.SDKWORK_SHARED_MAIL_GIT_REF = mailRef;
  }
  if (communityRef) {
    bridge.SDKWORK_SHARED_COMMUNITY_GIT_REF = communityRef;
  }
  if (courseRef) {
    bridge.SDKWORK_SHARED_COURSE_GIT_REF = courseRef;
  }
  if (knowledgebaseRef) {
    bridge.SDKWORK_SHARED_KNOWLEDGEBASE_GIT_REF = knowledgebaseRef;
  }

  return bridge;
}

export function createSdkworkChatPcReleaseBuildPlan({
  env = process.env,
  repoRoot: resolvedRepoRoot = repoRoot,
} = {}) {
  const resolvedIamEnv = resolveSdkworkChatIamCommandEnv({
    env,
    target: 'web-build',
  });
  if (resolvedIamEnv.errors.length > 0) {
    throw new Error(resolvedIamEnv.errors.join('\n'));
  }

  return {
    command: pnpmCommand(),
    cwd: resolvedRepoRoot,
    env: {
      ...resolvedIamEnv.env,
      ...resolveReleaseDependencyRefBridge(env),
      SDKWORK_SHARED_SDK_MODE: 'git',
    },
    shell: false,
    steps: RELEASE_BUILD_STEPS.map((step) => ({
      args: [...step.args],
      cwd: path.resolve(resolvedRepoRoot, step.cwd),
      label: step.label,
    })),
  };
}

function runPnpm(step, plan) {
  const result = plan.spawnSyncImpl(plan.command, step.args, {
    cwd: step.cwd,
    env: plan.env,
    shell: false,
    stdio: 'inherit',
  });
  if (result.error) {
    throw new Error(`${step.label} failed: ${result.error.message}`);
  }
  if (result.status !== 0) {
    throw new Error(`${step.label} failed with exit code ${result.status ?? 'unknown'}`);
  }
}

export function runSdkworkChatPcReleaseBuild({
  env = process.env,
  repoRoot: resolvedRepoRoot = repoRoot,
  spawnSyncImpl = spawnSync,
} = {}) {
  const plan = createSdkworkChatPcReleaseBuildPlan({
    env,
    repoRoot: resolvedRepoRoot,
  });

  for (const step of plan.steps) {
    runPnpm(step, {
      ...plan,
      spawnSyncImpl,
    });
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
  try {
    runSdkworkChatPcReleaseBuild();
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
