#!/usr/bin/env node

import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  SDKWORK_ACCESS_TOKEN_ENV_KEY,
  buildBootstrapAccessTokenEnvRecord,
  mergeRepoDevBootstrapAccessTokenEnv,
} from '../../../sdkwork-iam/scripts/dev/create-dev-bootstrap-access-token-env.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const manifestPath = path.join(repoRoot, 'sdkwork.app.config.json');
const defaultImPcAppId = 'sdkwork-im-pc';

export function buildSdkworkImBootstrapAccessTokenEnv({ existingAccessToken } = {}) {
  return buildBootstrapAccessTokenEnvRecord(existingAccessToken, {
    appId: defaultImPcAppId,
    environment: 'development',
  });
}

export function resolveSdkworkImBootstrapAccessTokenEnv(env = process.env) {
  return buildSdkworkImBootstrapAccessTokenEnv({
    existingAccessToken: env[SDKWORK_ACCESS_TOKEN_ENV_KEY],
  });
}

export function mergeSdkworkImBootstrapAccessTokenEnv(env = process.env) {
  return mergeRepoDevBootstrapAccessTokenEnv({
    repoRoot,
    manifestPath,
    appId: defaultImPcAppId,
    env,
  });
}

export { SDKWORK_ACCESS_TOKEN_ENV_KEY };
