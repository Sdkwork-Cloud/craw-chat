#!/usr/bin/env node

import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  buildBootstrapAccessTokenEnvRecord,
  mergeRepoDevBootstrapAccessTokenEnv,
  SDKWORK_ACCESS_TOKEN_ENV_KEY,
} from '../../../sdkwork-iam/scripts/dev/create-dev-bootstrap-access-token-env.mjs';

const __filename = fileURLToPath(import.meta.url);
const REPO_ROOT = path.resolve(path.dirname(__filename), '..', '..');
const MANIFEST_PATH = path.join(REPO_ROOT, 'sdkwork.app.config.json');
const DEFAULT_IM_PC_APP_ID = 'sdkwork-im-pc';

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

export { SDKWORK_ACCESS_TOKEN_ENV_KEY };
