import {
  getDriveAppSdkClient,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core';

export function getDriveAppSdkClientWithSession(): SdkworkDriveAppClient {
  return getDriveAppSdkClient();
}
