/**
 * H5 global TokenManager binding.
 *
 * The bootstrap creates one global TokenManager per authenticated session
 * context and binds it to appbase app SDK, application/dependency app SDKs,
 * Drive app SDK, IM app SDK, and other authenticated dependency app SDKs.
 */

import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { getImAppAuthRuntime } from './iamRuntime';

export type H5TokenManagerBinding = AuthTokenManager;

let cachedBinding: H5TokenManagerBinding | null = null;

export function resolveTokenManagerBinding(): H5TokenManagerBinding {
  if (cachedBinding) {
    return cachedBinding;
  }

  cachedBinding = getImAppAuthRuntime().tokenManager;

  return cachedBinding;
}

export function getTokenManagerBinding(): H5TokenManagerBinding {
  return cachedBinding ?? resolveTokenManagerBinding();
}

export function resetTokenManagerBinding(): void {
  cachedBinding = null;
}

export function isTokenManagerBound(): boolean {
  return getTokenManagerBinding().hasToken();
}
