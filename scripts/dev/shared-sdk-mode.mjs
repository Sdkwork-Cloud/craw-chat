export const SHARED_SDK_MODE_ENV_VAR = 'SDKWORK_SHARED_SDK_MODE';
export const SHARED_SDK_MODE_SOURCE = 'source';
export const SHARED_SDK_MODE_GIT = 'git';

export function resolveSharedSdkMode(env = process.env) {
  const normalizedMode = String(env?.[SHARED_SDK_MODE_ENV_VAR] ?? '')
    .trim()
    .toLowerCase();

  return normalizedMode === SHARED_SDK_MODE_GIT
    ? SHARED_SDK_MODE_GIT
    : SHARED_SDK_MODE_SOURCE;
}

export function isSharedSdkSourceMode(env = process.env) {
  return resolveSharedSdkMode(env) === SHARED_SDK_MODE_SOURCE;
}
