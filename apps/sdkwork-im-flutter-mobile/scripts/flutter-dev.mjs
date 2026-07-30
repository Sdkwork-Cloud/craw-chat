#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import {
  removeImRuntimeStateFile,
  resolveImTemporaryFilePath,
  writeImPrivateJsonFile,
} from '../../../scripts/lib/im-temporary-state.mjs';

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const repositoryRoot = path.resolve(appRoot, '..', '..');
const SUPPORTED_TARGETS = new Set(['android', 'ios']);
const PROFILE_ID = /^(?:standalone|cloud)\.(?:development|test|staging|production)$/u;
const DEVICE_ID = /^[A-Za-z0-9._:-]+$/u;

function option(args, name) {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
}

function requiredEnv(env, key) {
  const value = String(env[key] ?? '').trim();
  if (!value) throw new Error(`${key} is required from the selected topology profile`);
  return value;
}

export function createFlutterDefineConfig(env = process.env) {
  const applicationHttpUrl = requiredEnv(env, 'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL');
  const deploymentProfile = requiredEnv(env, 'SDKWORK_IM_DEPLOYMENT_PROFILE');
  const environment = requiredEnv(env, 'SDKWORK_IM_ENVIRONMENT');
  const profileId = requiredEnv(env, 'SDKWORK_IM_PROFILE_ID');
  if (profileId !== `${deploymentProfile}.${environment}`) {
    throw new Error('SDKWORK_IM_PROFILE_ID must match deployment profile and environment');
  }
  return {
    SDKWORK_APP_ID: 'sdkwork-im-flutter-mobile',
    SDKWORK_DEPLOYMENT_PROFILE: deploymentProfile,
    SDKWORK_ENVIRONMENT: environment,
    SDKWORK_PROFILE_ID: profileId,
    SDKWORK_RUNTIME_TARGET: 'flutter-android',
    SDKWORK_IM_DEPLOYMENT_PROFILE: deploymentProfile,
    SDKWORK_IM_ENVIRONMENT: environment,
    SDKWORK_IM_PROFILE_ID: profileId,
    SDKWORK_IM_RUNTIME_TARGET: 'flutter-android',
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationHttpUrl,
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: requiredEnv(
      env,
      'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL',
    ),
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: requiredEnv(
      env,
      'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
    ),
    SDKWORK_IAM_APP_API_BASE_URL: String(env.SDKWORK_IAM_APP_API_BASE_URL ?? applicationHttpUrl).trim(),
  };
}

export function createFlutterDevPlan({
  args = [],
  env = process.env,
  repoRoot = repositoryRoot,
  root = appRoot,
  runtimeStateOptions = {},
} = {}) {
  const target = option(args, '--target');
  if (!SUPPORTED_TARGETS.has(target)) throw new Error('--target must be android or ios');
  const profileId = requiredEnv(env, 'SDKWORK_IM_PROFILE_ID');
  if (!PROFILE_ID.test(profileId)) throw new Error('SDKWORK_IM_PROFILE_ID must be a canonical profile id');
  const flutterArgs = ['run'];
  const deviceId = String(env.SDKWORK_FLUTTER_DEVICE_ID ?? '').trim();
  if (deviceId && !DEVICE_ID.test(deviceId)) {
    throw new Error('SDKWORK_FLUTTER_DEVICE_ID contains unsupported characters');
  }
  if (deviceId) flutterArgs.push('--device-id', deviceId);
  const runtimeTarget = target === 'ios' ? 'flutter-ios' : 'flutter-android';
  const config = createFlutterDefineConfig(env);
  config.SDKWORK_RUNTIME_TARGET = runtimeTarget;
  config.SDKWORK_IM_RUNTIME_TARGET = runtimeTarget;
  const configPath = resolveImTemporaryFilePath({
    ...runtimeStateOptions,
    extension: '.json',
    fileName: `${profileId}.${target}.dart-define`,
    purpose: 'flutter-dev',
    repoRoot,
  });
  flutterArgs.push('--dart-define-from-file', configPath);
  return {
    command: process.platform === 'win32' ? 'flutter.bat' : 'flutter',
    config,
    configPath,
    flutterArgs,
    repoRoot,
    root,
    runtimeStateOptions,
    target,
  };
}

export function runFlutterDevelopment(plan, { spawn = spawnSync } = {}) {
  const stateOptions = { repoRoot: plan.repoRoot, ...plan.runtimeStateOptions };
  try {
    writeImPrivateJsonFile(plan.configPath, plan.config, stateOptions);
    const result = spawn(plan.command, plan.flutterArgs, {
      cwd: plan.root,
      env: process.env,
      stdio: 'inherit',
      shell: process.platform === 'win32',
      windowsHide: true,
    });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`flutter run exited with code ${result.status ?? 1}`);
  } finally {
    removeImRuntimeStateFile(plan.configPath, stateOptions);
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    runFlutterDevelopment(createFlutterDevPlan({ args: process.argv.slice(2) }));
  } catch (error) {
    console.error(`[sdkwork-im-flutter-dev] ${error.message}`);
    process.exitCode = 1;
  }
}
