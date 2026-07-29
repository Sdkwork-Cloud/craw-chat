import {
  createNotaryApi,
  createNotaryAppClient,
  type DriveAppSdkPort,
  type SdkworkAppConfig,
  type SdkworkNotaryAppClient,
} from '@sdkwork/notary-app-sdk';
import type { SdkworkDriveAppClient } from '@sdkwork/drive-app-sdk';

export type NotaryComposedApi = ReturnType<typeof createNotaryApi>;

let notaryAppSdkClient: SdkworkNotaryAppClient | null = null;
let notaryComposedApi: NotaryComposedApi | null = null;

export function initNotaryAppSdkClient(
  config: SdkworkAppConfig,
  driveAppSdkClient: SdkworkDriveAppClient,
): SdkworkNotaryAppClient {
  notaryAppSdkClient = createNotaryAppClient(config);
  notaryComposedApi = createNotaryApi({
    notary: notaryAppSdkClient.notary,
    drive: driveAppSdkClient as DriveAppSdkPort,
    appbase: {},
  });
  return notaryAppSdkClient;
}

export function getNotaryAppSdkClient(): SdkworkNotaryAppClient {
  if (!notaryAppSdkClient) {
    throw new Error('Notary App SDK client is not initialized');
  }
  return notaryAppSdkClient;
}

export function getNotaryComposedApi(): NotaryComposedApi {
  if (!notaryComposedApi) {
    throw new Error('Notary composed API is not initialized');
  }
  return notaryComposedApi;
}

export function resetNotaryAppSdkClient(): void {
  notaryAppSdkClient = null;
  notaryComposedApi = null;
}
