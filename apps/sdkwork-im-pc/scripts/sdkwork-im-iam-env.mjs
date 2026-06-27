#!/usr/bin/env node

import { readRustRealtimePathConstants } from '../../../scripts/dev/materialize-sdkwork-im-realtime-api-paths.mjs';

export const SDKWORK_IM_IAM_DEPLOYMENT_MODES = Object.freeze([
  'desktop-local',
  'server-private',
  'cloud-saas',
]);

export const SDKWORK_IM_IAM_MODES = Object.freeze([
  'private',
  'cloud',
]);

export const DEFAULT_SDKWORK_IM_LOCAL_PLATFORM_API_GATEWAY_HTTP_URL = 'http://127.0.0.1:18079';
export const DEFAULT_SDKWORK_IM_LOCAL_APPLICATION_PUBLIC_HTTP_URL = 'http://127.0.0.1:18079';
export const DEFAULT_SDKWORK_IM_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL = 'ws://127.0.0.1:18079';
export const DEFAULT_SDKWORK_IAM_DEV_FIXED_VERIFY_CODE = '123456';

const SDKWORK_IM_IAM_DEPLOYMENT_MODE_ENV = 'SDKWORK_IM_IAM_DEPLOYMENT_MODE';
const VITE_SDKWORK_IM_IAM_DEPLOYMENT_MODE_ENV = 'VITE_SDKWORK_IM_IAM_DEPLOYMENT_MODE';
const VITE_SDKWORK_DEPLOYMENT_MODE_ENV = 'VITE_SDKWORK_DEPLOYMENT_MODE';
const SDKWORK_IAM_MODE_ENV = 'SDKWORK_IAM_MODE';
const SDKWORK_IAM_DEV_FIXED_VERIFY_CODE_ENV = 'SDKWORK_IAM_DEV_FIXED_VERIFY_CODE';
const SDKWORK_IAM_APP_API_BASE_URL_ENV = 'SDKWORK_IAM_APP_API_BASE_URL';
const VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV = 'VITE_SDKWORK_IAM_APP_API_BASE_URL';
const SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV = 'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL';
const VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV = 'VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL';
const SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV = 'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL';
const VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV = 'VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL';
const SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV = 'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL';
const VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV = 'VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL';
const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';
const SDKWORK_IM_REALTIME_WEBSOCKET_PATH = readRustRealtimePathConstants().get('REALTIME_WS');

function readTrimmedValue(value) {
  const normalizedValue = String(value ?? '').trim();
  return normalizedValue || undefined;
}

function stripSdkOwnedPathSuffix(pathname, suffixes = []) {
  const normalizedPathname = pathname.replace(/\/+$/u, '');
  if (!normalizedPathname || normalizedPathname === '/') {
    return '';
  }

  for (const suffix of suffixes) {
    const normalizedSuffix = `/${String(suffix).replace(/^\/+|\/+$/gu, '')}`;
    if (normalizedPathname === normalizedSuffix) {
      return '';
    }
    if (normalizedPathname.endsWith(normalizedSuffix)) {
      return normalizedPathname.slice(0, -normalizedSuffix.length) || '';
    }
  }

  return normalizedPathname;
}

function normalizeHttpBaseUrl(value, sdkOwnedPathSuffixes = []) {
  const normalizedValue = readTrimmedValue(value);
  if (!normalizedValue) {
    return undefined;
  }

  try {
    const parsedUrl = new URL(normalizedValue);
    if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
      return undefined;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(
      parsedUrl.pathname,
      sdkOwnedPathSuffixes,
    );
    return `${parsedUrl.origin}${normalizedPathname === '/' ? '' : normalizedPathname}`;
  } catch {
    return undefined;
  }
}

function normalizeWebSocketBaseUrl(value, sdkOwnedPathSuffixes = []) {
  const normalizedValue = readTrimmedValue(value);
  if (!normalizedValue) {
    return undefined;
  }

  try {
    const parsedUrl = new URL(normalizedValue);
    if (parsedUrl.protocol !== 'ws:' && parsedUrl.protocol !== 'wss:') {
      return undefined;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(
      parsedUrl.pathname,
      sdkOwnedPathSuffixes,
    );
    return `${parsedUrl.origin}${normalizedPathname === '/' ? '' : normalizedPathname}`;
  } catch {
    return undefined;
  }
}

function deriveWebSocketBaseUrlFromHttpBaseUrl(value) {
  const normalizedValue = normalizeHttpBaseUrl(value, [
    SDKWORK_APP_API_PREFIX,
    SDKWORK_IM_API_PREFIX,
  ]);
  if (!normalizedValue) {
    return undefined;
  }

  const parsedUrl = new URL(normalizedValue);
  parsedUrl.protocol = parsedUrl.protocol === 'https:' ? 'wss:' : 'ws:';
  return normalizeWebSocketBaseUrl(parsedUrl.toString());
}

function setEnvValue(env, key, value) {
  const normalizedValue = readTrimmedValue(value);
  if (!normalizedValue) {
    delete env[key];
    return;
  }

  env[key] = normalizedValue;
}

function setEnvDefault(env, key, value) {
  if (!readTrimmedValue(env[key])) {
    setEnvValue(env, key, value);
  }
}

function firstConfiguredEnvKey(env, keys) {
  return keys.find((key) => readTrimmedValue(env[key]));
}

function validateConfiguredHttpEnv(env, keys, errors) {
  const configuredKey = firstConfiguredEnvKey(env, keys);
  if (!configuredKey) {
    return;
  }
  if (!normalizeHttpBaseUrl(env[configuredKey], [
    SDKWORK_APP_API_PREFIX,
    SDKWORK_IM_API_PREFIX,
  ])) {
    errors.push(`${configuredKey} must be a valid absolute http(s) URL.`);
  }
}

function validateConfiguredWebSocketEnv(env, keys, errors) {
  const configuredKey = firstConfiguredEnvKey(env, keys);
  if (!configuredKey) {
    return;
  }
  if (!normalizeWebSocketBaseUrl(env[configuredKey], [
    SDKWORK_IM_REALTIME_WEBSOCKET_PATH,
    SDKWORK_IM_API_PREFIX,
  ])) {
    errors.push(`${configuredKey} must be a valid absolute ws(s) URL.`);
  }
}

function normalizeDeploymentMode(value, fallback = 'desktop-local') {
  const normalizedValue = readTrimmedValue(value)?.toLowerCase();
  if (!normalizedValue) {
    return fallback;
  }
  return SDKWORK_IM_IAM_DEPLOYMENT_MODES.includes(normalizedValue)
    ? normalizedValue
    : fallback;
}

function resolveSdkworkIamMode(iamMode) {
  if (iamMode === 'cloud-saas') {
    return 'cloud';
  }
  if (iamMode === 'server-private' || iamMode === 'desktop-local') {
    return 'private';
  }
  return 'private';
}

function resolvePublicDeploymentMode(iamMode) {
  if (iamMode === 'cloud-saas') {
    return 'saas';
  }
  if (iamMode === 'server-private') {
    return 'private';
  }
  // desktop-local: local gateway URLs, but IAM identity still comes from dual-token JWT claims.
  return 'saas';
}

function resolveDefaultDeploymentMode(target) {
  if (target === 'web-build' || target === 'server-build' || target === 'server-dev') {
    return 'server-private';
  }
  return 'desktop-local';
}

function shouldUseLocalEndpointDefaults(target, resolvedDeploymentMode) {
  if (resolvedDeploymentMode === 'desktop-local') {
    return true;
  }
  return target === 'desktop-dev' || target === 'desktop-build' || target === 'browser-dev';
}

function resolveConfiguredPlatformApiBaseUrl(env) {
  return normalizeHttpBaseUrl(
    env[VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV]
      ?? env[SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV]
      ?? env[VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV]
      ?? env[SDKWORK_IAM_APP_API_BASE_URL_ENV],
    [SDKWORK_APP_API_PREFIX, SDKWORK_IM_API_PREFIX],
  );
}

function resolveConfiguredApplicationHttpUrl(env) {
  return normalizeHttpBaseUrl(
    env[VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV]
      ?? env[SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV],
    [SDKWORK_APP_API_PREFIX, SDKWORK_IM_API_PREFIX],
  );
}

function resolveConfiguredApplicationWebSocketUrl(env) {
  return normalizeWebSocketBaseUrl(
    env[VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]
      ?? env[SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV],
    [SDKWORK_IM_REALTIME_WEBSOCKET_PATH, SDKWORK_IM_API_PREFIX],
  );
}

export function resolveSdkworkChatIamCommandEnv({
  env = process.env,
  iamMode,
  target = 'desktop-dev',
} = {}) {
  const nextEnv = { ...env };
  const resolvedDeploymentMode = normalizeDeploymentMode(
    iamMode ?? nextEnv[SDKWORK_IM_IAM_DEPLOYMENT_MODE_ENV],
    resolveDefaultDeploymentMode(target),
  );
  const errors = [];
  const sdkworkIamMode = resolveSdkworkIamMode(resolvedDeploymentMode);
  validateConfiguredHttpEnv(nextEnv, [
    VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV,
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV,
    VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV,
    SDKWORK_IAM_APP_API_BASE_URL_ENV,
  ], errors);
  validateConfiguredHttpEnv(nextEnv, [
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV,
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV,
  ], errors);
  validateConfiguredWebSocketEnv(nextEnv, [
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV,
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV,
  ], errors);
  const useLocalEndpointDefaults = shouldUseLocalEndpointDefaults(target, resolvedDeploymentMode);
  const configuredPlatformApiBaseUrl = resolveConfiguredPlatformApiBaseUrl(nextEnv);
  const configuredApplicationHttpUrl = resolveConfiguredApplicationHttpUrl(nextEnv);
  const platformApiBaseUrl = configuredPlatformApiBaseUrl
    ?? (useLocalEndpointDefaults ? DEFAULT_SDKWORK_IM_LOCAL_PLATFORM_API_GATEWAY_HTTP_URL : undefined);
  const applicationHttpUrl = configuredApplicationHttpUrl
    ?? (useLocalEndpointDefaults ? DEFAULT_SDKWORK_IM_LOCAL_APPLICATION_PUBLIC_HTTP_URL : undefined);
  const applicationWebSocketUrl = resolveConfiguredApplicationWebSocketUrl(nextEnv)
    ?? deriveWebSocketBaseUrlFromHttpBaseUrl(configuredApplicationHttpUrl)
    ?? (useLocalEndpointDefaults ? DEFAULT_SDKWORK_IM_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL : undefined);

  setEnvValue(nextEnv, SDKWORK_IM_IAM_DEPLOYMENT_MODE_ENV, resolvedDeploymentMode);
  setEnvValue(nextEnv, VITE_SDKWORK_IM_IAM_DEPLOYMENT_MODE_ENV, resolvedDeploymentMode);
  setEnvValue(nextEnv, VITE_SDKWORK_DEPLOYMENT_MODE_ENV, resolvePublicDeploymentMode(resolvedDeploymentMode));
  setEnvValue(nextEnv, SDKWORK_IAM_MODE_ENV, sdkworkIamMode);

  setEnvValue(nextEnv, SDKWORK_IAM_APP_API_BASE_URL_ENV, platformApiBaseUrl);
  setEnvValue(nextEnv, VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV, platformApiBaseUrl);
  setEnvValue(nextEnv, SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV, platformApiBaseUrl);
  setEnvValue(nextEnv, VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV, platformApiBaseUrl);
  setEnvValue(nextEnv, SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV, applicationHttpUrl);
  setEnvValue(nextEnv, VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV, applicationHttpUrl);
  setEnvValue(nextEnv, SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV, applicationWebSocketUrl);
  setEnvValue(nextEnv, VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV, applicationWebSocketUrl);

  if (sdkworkIamMode === 'private' || sdkworkIamMode === 'cloud') {
    setEnvDefault(
      nextEnv,
      SDKWORK_IAM_DEV_FIXED_VERIFY_CODE_ENV,
      DEFAULT_SDKWORK_IAM_DEV_FIXED_VERIFY_CODE,
    );
  }

  if (!SDKWORK_IM_IAM_MODES.includes(sdkworkIamMode)) {
    errors.push(`${SDKWORK_IAM_MODE_ENV} must be one of: ${SDKWORK_IM_IAM_MODES.join(', ')}.`);
  }
  if (platformApiBaseUrl && !normalizeHttpBaseUrl(platformApiBaseUrl)) {
    errors.push(`${VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL_ENV} must be a valid http(s) URL.`);
  }
  if (applicationHttpUrl && !normalizeHttpBaseUrl(applicationHttpUrl)) {
    errors.push(`${VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL_ENV} must be a valid http(s) URL.`);
  }
  if (applicationWebSocketUrl && !normalizeWebSocketBaseUrl(applicationWebSocketUrl)) {
    errors.push(`${VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV} must be a valid ws(s) URL.`);
  }

  return {
    env: nextEnv,
    errors,
    iamMode: resolvedDeploymentMode,
    sdkworkIamMode,
  };
}
