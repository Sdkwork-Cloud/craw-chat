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
  initDriveAppSdkClient,
  resetDriveAppSdkClient,
  createDriveAppSdkClientConfig,
  type ImSdkClient,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core/sdk';
import {
  createNotaryH5ComposedApi,
  initNotaryH5AppSdkClient,
  resetNotaryH5SdkClients,
  type NotaryH5ComposedApi,
} from '@sdkwork/notary-h5-core';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { resolveH5RuntimeEnvironment } from './environment';
import { getTokenManagerBinding } from './tokenManager';

export interface H5SdkClientComposition {
  readonly driveAppSdkClient: SdkworkDriveAppClient;
  readonly imSdkClient: ImSdkClient;
  readonly notaryAppSdkClient: ReturnType<typeof initNotaryH5AppSdkClient>;
  readonly notaryApi: NotaryH5ComposedApi;
}

let sdkClientComposition: H5SdkClientComposition | null = null;

export function initSdkClients(
  tokenManager: AuthTokenManager = getTokenManagerBinding(),
): H5SdkClientComposition {
  if (sdkClientComposition) {
    return sdkClientComposition;
  }

  const environment = resolveH5RuntimeEnvironment();
  const imSdkClient = initImSdkClient({
    apiBaseUrl: environment.imApiBaseUrl,
    platform: "h5",
    tokenManager,
  });

  const driveAppSdkClient = initDriveAppSdkClient(
    createDriveAppSdkClientConfig({
      baseUrl: environment.driveAppApiBaseUrl,
      tokenManager,
    }),
  );

  const notaryAppSdkClient = initNotaryH5AppSdkClient({
    baseUrl: environment.sdkGatewayApiBaseUrl,
    authMode: 'dual-token',
    platform: 'h5',
    tokenManager,
  });
  const notaryApi = createNotaryH5ComposedApi({
    drive: driveAppSdkClient,
    appbase: {},
  });

  sdkClientComposition = {
    driveAppSdkClient,
    imSdkClient,
    notaryAppSdkClient,
    notaryApi,
  };
  return sdkClientComposition;
}

export function getSdkClients(): H5SdkClientComposition {
  return sdkClientComposition ?? initSdkClients();
}

export function getDriveAppSdkClientFromBootstrap(): SdkworkDriveAppClient {
  return getSdkClients().driveAppSdkClient;
}

export function resetSdkClients(): void {
  resetNotaryH5SdkClients();
  resetDriveAppSdkClient();
  resetImSdkClient();
  sdkClientComposition = null;
}

export type { SdkworkDriveAppClient };
export { getDriveAppSdkClient };
