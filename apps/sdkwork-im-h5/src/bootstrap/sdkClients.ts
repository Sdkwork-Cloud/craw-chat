/**
 * H5 SDK client construction.
 *
 * Generated TypeScript app SDK clients are constructed here in bootstrap/core
 * code and injected into services or providers. UI packages MUST NOT construct
 * raw HTTP calls, manual auth headers, or generated SDK clients.
 */

import {
  getDriveAppSdkClient,
  initImSdkClient,
  resetImSdkClient,
  getNotaryComposedApi,
  initDriveAppSdkClient,
  initNotaryAppSdkClient,
  resetNotaryAppSdkClient,
  resetDriveAppSdkClient,
  createDriveAppSdkClientConfig,
  type NotaryComposedApi,
  type ImSdkClient,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core';
import { resolveH5RuntimeEnvironment } from './environment';
import { getImAppAuthRuntime } from './iamRuntime';
import type { SdkworkAppbasePcAuthRuntimeComposition } from '@sdkwork/auth-runtime-pc-react';

export interface H5SdkClientComposition {
  readonly driveAppSdkClient: SdkworkDriveAppClient;
  readonly imSdkClient: ImSdkClient;
  readonly notaryApi: NotaryComposedApi;
}

let sdkClientComposition: H5SdkClientComposition | null = null;

export function initSdkClients(): H5SdkClientComposition {
  if (sdkClientComposition) {
    return sdkClientComposition;
  }

  const environment = resolveH5RuntimeEnvironment();
  const authRuntime = getImAppAuthRuntime();
  const imSdkClient = initImSdkClient({
    apiBaseUrl: environment.imApiBaseUrl,
    platform: "h5",
    tokenManager: authRuntime.tokenManager,
  });

  const driveAppSdkClient = initDriveAppSdkClient(
    createDriveAppSdkClientConfig({
      baseUrl: environment.driveAppApiBaseUrl,
      tokenManager: authRuntime.tokenManager,
    }),
  );

  initNotaryAppSdkClient(
    {
      baseUrl: environment.sdkGatewayApiBaseUrl,
      authMode: 'dual-token',
      platform: 'h5',
      tokenManager: authRuntime.tokenManager,
    },
    driveAppSdkClient,
  );
  const notaryApi = getNotaryComposedApi();

  sdkClientComposition = { driveAppSdkClient, imSdkClient, notaryApi };
  return sdkClientComposition;
}

export function getSdkClients(): H5SdkClientComposition {
  return sdkClientComposition ?? initSdkClients();
}

export function getDriveAppSdkClientFromBootstrap(): SdkworkDriveAppClient {
  return getSdkClients().driveAppSdkClient;
}

export function resetSdkClients(): void {
  resetNotaryAppSdkClient();
  resetDriveAppSdkClient();
  resetImSdkClient();
  sdkClientComposition = null;
}

export type { SdkworkDriveAppClient };
export { getDriveAppSdkClient };
