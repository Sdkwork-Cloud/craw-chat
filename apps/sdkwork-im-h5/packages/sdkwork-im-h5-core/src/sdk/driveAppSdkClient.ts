import {
  createDriveAppClient,
  type SdkworkAppConfig,
  type SdkworkDriveAppClient as GeneratedSdkworkDriveAppClient,
} from "@sdkwork/drive-app-sdk";
import type { Interceptors } from "@sdkwork/sdk-common";

import { resolveAppSdkBaseUrl } from "../config/resolveAppSdkBaseUrl";
import {
  getImH5GlobalTokenManager,
  readImH5IamSessionTokens,
  type ImH5IamSession,
} from "../session/iamSession";
import { resolveImSdkApiBaseUrl } from "./imSdkClient";

export type SdkworkDriveAppClient = GeneratedSdkworkDriveAppClient;
export type SdkworkDriveAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let driveAppSdkClient: SdkworkDriveAppClient | null = null;

function resolveDriveAccessToken(session?: ImH5IamSession | null): string | undefined {
  const token = session?.accessToken?.trim();
  return token || undefined;
}

function resolveDriveAuthToken(session?: ImH5IamSession | null): string | undefined {
  const token = session?.authToken?.trim();
  return token || undefined;
}

export function createDriveAppSdkClientConfig(
  session?: ImH5IamSession | null,
): SdkworkDriveAppClientConfig {
  const currentSession = session ?? readImH5IamSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(resolveImSdkApiBaseUrl()),
    accessToken: resolveDriveAccessToken(currentSession),
    authToken: resolveDriveAuthToken(currentSession),
    interceptors: { request: [], response: [], error: [] },
    platform: "h5",
    tokenManager: getImH5GlobalTokenManager(),
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
  session = readImH5IamSessionTokens(),
): SdkworkDriveAppClient {
  return initDriveAppSdkClient(createDriveAppSdkClientConfig(session));
}

export function resetDriveAppSdkClient(): void {
  driveAppSdkClient = null;
}
