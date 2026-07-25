import {
  createDriveAppClient,
  type DriveUploaderBlobLike,
  type DriveUploaderClient,
  type DriveUploaderProfile,
  type DriveUploaderRequest,
  type DriveUploaderUploadResult,
  type SdkworkAppConfig,
  type SdkworkDriveAppClient as GeneratedSdkworkDriveAppClient,
} from '@sdkwork/drive-app-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';

import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkImH5RequestContextInterceptors,
  getSdkworkImH5GlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  IM_H5_IAM_SESSION_CHANGED_EVENT,
  type SdkworkImH5Session,
} from './session';

export type SdkworkDriveAppClient = GeneratedSdkworkDriveAppClient;
export type SdkworkDriveAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};
export type {
  DriveUploaderBlobLike,
  DriveUploaderClient,
  DriveUploaderProfile,
  DriveUploaderRequest,
  DriveUploaderUploadResult,
};
export type SdkworkDriveUploader = Pick<
  DriveUploaderClient,
  'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
>;

let driveAppSdkClient: SdkworkDriveAppClient | null = null;
let driveSessionListenerRegistered = false;

export function createDriveAppSdkClientConfig(
  session?: SdkworkImH5Session | null,
): SdkworkDriveAppClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkImH5RequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'h5',
    tokenManager: getSdkworkImH5GlobalTokenManager(),
  };
}

export function initDriveAppSdkClient(
  config: SdkworkDriveAppClientConfig = createDriveAppSdkClientConfig(),
): SdkworkDriveAppClient {
  driveAppSdkClient = createDriveAppClient(config);
  return driveAppSdkClient;
}

export function getDriveAppSdkClient(): SdkworkDriveAppClient {
  return driveAppSdkClient ?? initDriveAppSdkClient();
}

export function getDriveAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkDriveAppClient {
  return initDriveAppSdkClient(createDriveAppSdkClientConfig(session));
}

export function resetDriveAppSdkClient(): void {
  driveAppSdkClient = null;
}

export function syncImSessionToDriveH5(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetDriveAppSdkClient();
    return;
  }

  resetDriveAppSdkClient();
  void resolveAppSdkBaseUrl();
}

export function ensureDriveSessionListener(): void {
  if (driveSessionListenerRegistered || typeof window === 'undefined') {
    return;
  }
  driveSessionListenerRegistered = true;
  window.addEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, () => {
    syncImSessionToDriveH5();
  });
}

export { createDriveAppClient };
