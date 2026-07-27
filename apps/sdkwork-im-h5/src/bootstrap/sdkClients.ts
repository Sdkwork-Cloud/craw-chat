/**
 * H5 SDK client construction.
 *
 * Generated TypeScript app SDK clients are constructed here in bootstrap/core
 * code and injected into services or providers. UI packages MUST NOT construct
 * raw HTTP calls, manual auth headers, or generated SDK clients.
 */

import {
  getDriveAppSdkClient,
  initDriveAppSdkClient,
  resetDriveAppSdkClient,
  createDriveAppSdkClientConfig,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core';
import { resolveH5RuntimeEnvironment } from './environment';
import { getImAppAuthRuntime } from './iamRuntime';
import type { SdkworkAppbasePcAuthRuntimeComposition } from '@sdkwork/auth-runtime-pc-react';

export interface H5SdkClientComposition {
  readonly driveAppSdkClient: SdkworkDriveAppClient;
}

let sdkClientComposition: H5SdkClientComposition | null = null;

function resolveSessionTokens(
  authRuntime: SdkworkAppbasePcAuthRuntimeComposition,
): { accessToken?: string; authToken?: string } {
  const session = authRuntime.session as
    | { accessToken?: string; authToken?: string }
    | undefined;
  return {
    accessToken: session?.accessToken,
    authToken: session?.authToken,
  };
}

export function initSdkClients(): H5SdkClientComposition {
  if (sdkClientComposition) {
    return sdkClientComposition;
  }

  const environment = resolveH5RuntimeEnvironment();
  const authRuntime = getImAppAuthRuntime();
  const tokens = resolveSessionTokens(authRuntime);

  const driveAppSdkClient = initDriveAppSdkClient(
    createDriveAppSdkClientConfig({
      baseUrl: environment.driveAppApiBaseUrl,
      accessToken: tokens.accessToken,
      authToken: tokens.authToken,
    }),
  );

  sdkClientComposition = { driveAppSdkClient };
  return sdkClientComposition;
}

export function getSdkClients(): H5SdkClientComposition {
  return sdkClientComposition ?? initSdkClients();
}

export function getDriveAppSdkClientFromBootstrap(): SdkworkDriveAppClient {
  return getSdkClients().driveAppSdkClient;
}

export function resetSdkClients(): void {
  resetDriveAppSdkClient();
  sdkClientComposition = null;
}

export type { SdkworkDriveAppClient };
export { getDriveAppSdkClient };
