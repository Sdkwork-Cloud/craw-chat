/**
 * H5 global TokenManager binding.
 *
 * The bootstrap creates one global TokenManager per authenticated session
 * context and binds it to appbase app SDK, application/dependency app SDKs,
 * Drive app SDK, IM app SDK, and other authenticated dependency app SDKs.
 */

import { getImAppAuthRuntime } from './iamRuntime';

export interface H5TokenManagerBinding {
  readonly accessToken: string | undefined;
  readonly authToken: string | undefined;
  readonly userId: string | undefined;
  readonly tenantId: string | undefined;
}

function resolveAuthRuntimeSession() {
  const authRuntime = getImAppAuthRuntime();
  return authRuntime.session as
    | {
        accessToken?: string;
        authToken?: string;
        userId?: string;
        tenantId?: string;
      }
    | undefined;
}

let cachedBinding: H5TokenManagerBinding | null = null;

export function resolveTokenManagerBinding(): H5TokenManagerBinding {
  if (cachedBinding) {
    return cachedBinding;
  }

  const session = resolveAuthRuntimeSession();
  cachedBinding = {
    accessToken: session?.accessToken,
    authToken: session?.authToken,
    userId: session?.userId,
    tenantId: session?.tenantId,
  };

  return cachedBinding;
}

export function getTokenManagerBinding(): H5TokenManagerBinding {
  return cachedBinding ?? resolveTokenManagerBinding();
}

export function resetTokenManagerBinding(): void {
  cachedBinding = null;
}

export function isTokenManagerBound(): boolean {
  const binding = getTokenManagerBinding();
  return Boolean(binding.accessToken || binding.authToken);
}
