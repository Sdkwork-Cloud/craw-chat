import {
  createDriveAppClient as createGeneratedDriveAppClient,
  type DriveAppClientOptions,
  type SdkworkAppConfig,
  type SdkworkDriveAppClient,
} from '@sdkwork/drive-app-sdk';

export type { SdkworkDriveAppClient, DriveAppClientOptions, SdkworkAppConfig };

let driveAppSdkClient: SdkworkDriveAppClient | null = null;

function resolveDriveAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_DRIVE_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_DRIVE_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

export function createDriveAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveDriveAppBaseUrl(),
    accessToken: config.accessToken,
    authToken: config.authToken,
    headers: config.headers,
    platform: 'h5',
    tokenManager: config.tokenManager,
  };
}

export function initDriveAppSdkClient(
  config: SdkworkAppConfig = createDriveAppSdkClientConfig(),
  options: DriveAppClientOptions = {},
): SdkworkDriveAppClient {
  driveAppSdkClient = createGeneratedDriveAppClient(config, options);
  return driveAppSdkClient;
}

export function getDriveAppSdkClient(): SdkworkDriveAppClient {
  return driveAppSdkClient ?? initDriveAppSdkClient();
}

export function getDriveAppSdkClientWithSession(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkDriveAppClient {
  return initDriveAppSdkClient(createDriveAppSdkClientConfig(config));
}

export function resetDriveAppSdkClient(): void {
  driveAppSdkClient = null;
}

export function createDriveAppClient(
  config: SdkworkAppConfig = createDriveAppSdkClientConfig(),
  options: DriveAppClientOptions = {},
): SdkworkDriveAppClient {
  return createGeneratedDriveAppClient(config, options);
}
