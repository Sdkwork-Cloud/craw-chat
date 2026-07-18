#!/usr/bin/env node

import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { runSdkworkChatPcDev } from './lib/im-pc-dev.mjs';
import { ensureImSdkGeneratedTransportBuilt } from './dev/ensure-im-sdk-generated-built.mjs';
import { resolvePostgresDevProfile } from './dev/sdkwork-im-postgres-dev-profile.mjs';
import {
  DEFAULT_DEV_PROFILE_ID,
  IAM_APPLICATION_BOOTSTRAP_ENV,
  loadEnvFile,
  loadProfile,
  mergeRuntimeEnv,
  REPO_ROOT,
  resolveDevProfileId,
  resolveStandaloneGatewayConfigPath,
} from './lib/im-topology.mjs';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function parseArgs(argv) {
  const settings = {
    target: 'browser',
    database: undefined,
    deploymentProfile: 'standalone',
    environment: 'development',
    help: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--help' || arg === '-h') {
      settings.help = true;
      continue;
    }
    if (arg === '--target') {
      settings.target = argv[index + 1] ?? settings.target;
      index += 1;
      continue;
    }
    if (arg === '--database') {
      settings.database = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--deployment-profile') {
      settings.deploymentProfile = argv[index + 1] ?? settings.deploymentProfile;
      index += 1;
      continue;
    }
    if (arg === '--environment') {
      settings.environment = argv[index + 1] ?? settings.environment;
      index += 1;
      continue;
    }
    if (arg === '--hosting') {
      throw new Error(
        '--hosting is retired; use --deployment-profile (standalone or cloud)',
      );
    }
    if (arg === '--service-layout') {
      throw new Error(
        '--service-layout is retired; process layout is selected by the topology profile',
      );
    }
  }

  return settings;
}

function printHelp() {
  console.log(`Usage: node scripts/im-dev.mjs [options]

Topology-aware IM dev entry. Loads etc/topology profile env via @sdkwork/app-topology.

Options:
  --deployment-profile <standalone|cloud>           Default: standalone
  --environment <development|staging|production>    Default: development
  --target <browser|desktop>                        Default: browser
  --database <postgres>                             Default: postgres
  --help, -h
`);
}

async function main() {
  const settings = parseArgs(process.argv.slice(2));
  if (settings.help) {
    printHelp();
    process.exit(0);
  }

  const profileId = resolveDevProfileId(settings.deploymentProfile, settings.environment)
    || DEFAULT_DEV_PROFILE_ID;
  const profileEnv = loadProfile(profileId);
  const postgresProfile = resolvePostgresDevProfile({ env: process.env, repoRoot: REPO_ROOT });
  const envFile = undefined;
  const fileEnv = postgresProfile?.fileEnv ?? {};
  const childEnv = mergeRuntimeEnv(process.env, profileEnv, fileEnv, {
    SDKWORK_IM_PROFILE_ID: profileId,
    SDKWORK_IM_DEPLOYMENT_PROFILE: settings.deploymentProfile,
    SDKWORK_IM_STANDALONE_GATEWAY_CONFIG: resolveStandaloneGatewayConfigPath(
      { ...process.env, ...profileEnv, ...fileEnv, ...(postgresProfile?.env ?? {}) },
      REPO_ROOT,
    ),
    ...IAM_APPLICATION_BOOTSTRAP_ENV,
    ...(postgresProfile?.env ?? {}),
  });

  console.log(
    `[sdkwork-im] deploymentProfile=${settings.deploymentProfile} `
    + `environment=${settings.environment} profileId=${profileId}`,
  );

  const runnerArgv = ['--target', settings.target];
  if (settings.database) {
    runnerArgv.push('--database', settings.database);
  }

  ensureImSdkGeneratedTransportBuilt({ quiet: true });

  await runSdkworkChatPcDev({
    argv: runnerArgv,
    env: childEnv,
    repoRoot: REPO_ROOT,
  });
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
});
