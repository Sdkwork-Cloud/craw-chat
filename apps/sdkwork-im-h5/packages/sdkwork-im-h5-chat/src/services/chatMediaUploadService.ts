import {
  getDriveAppSdkClient,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core/sdk';

export function getDriveAppSdkClientWithSession(): SdkworkDriveAppClient {
  return getDriveAppSdkClient();
}
