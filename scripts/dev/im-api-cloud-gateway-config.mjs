#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

export function resolveImApiCloudGatewayConfigPath(env = process.env, resolvedRepoRoot = repoRoot) {
  const explicit = normalizeText(env.SDKWORK_IM_API_CLOUD_GATEWAY_CONFIG);
  if (explicit) {
    return path.isAbsolute(explicit) ? explicit : path.resolve(resolvedRepoRoot, explicit);
  }

  const environment = normalizeText(env.SDKWORK_IM_API_CLOUD_GATEWAY_ENVIRONMENT)
    ?? normalizeText(env.SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT)
    ?? 'development';
  const configPath = path.join(
    resolvedRepoRoot,
    'configs',
    `sdkwork-api-cloud-gateway.sdkwork-im.${environment}.toml`,
  );
  if (fs.existsSync(configPath)) {
    return configPath;
  }

  return path.join(resolvedRepoRoot, 'configs', 'sdkwork-api-cloud-gateway.sdkwork-im.development.toml');
}
