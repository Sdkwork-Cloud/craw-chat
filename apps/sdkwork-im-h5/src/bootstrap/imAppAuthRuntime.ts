import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
  type SdkworkAppbasePcAuthRuntimeSdkClient,
} from '@sdkwork/auth-runtime-pc-react';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  disposeChatLiveConnection,
  getAppSdkClient,
  getDriveAppSdkClient,
  getImSdkClient,
  getSdkworkImH5GlobalTokenManager,
  readAppSdkSessionTokens,
  resetAppSdkClient,
  resetDriveAppSdkClient,
  resetImSdkClient,
  resolveAppSdkBaseUrl,
  type SdkworkImH5Session,
} from '@sdkwork/im-h5-core';

type IamEnvironment = 'dev' | 'prod' | 'test';
type IamDeploymentMode = 'local' | 'private' | 'saas';

let imAppAuthRuntimeComposition: SdkworkAppbasePcAuthRuntimeComposition | null = null;

function readEnvValue(...keys: string[]): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | boolean | undefined>;
  };

  for (const key of keys) {
    const value = meta.env?.[key];
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }

  return undefined;
}

function resolveIamEnvironment(): IamEnvironment {
  const value = readEnvValue(
    'VITE_SDKWORK_IM_IAM_ENVIRONMENT',
    'VITE_SDKWORK_IAM_ENVIRONMENT',
  );
  return value === 'prod' || value === 'production'
    ? 'prod'
    : value === 'test'
      ? 'test'
      : 'dev';
}

function resolveIamDeploymentMode(): IamDeploymentMode {
  const value = readEnvValue(
    'VITE_SDKWORK_IM_IAM_DEPLOYMENT_MODE',
    'VITE_SDKWORK_IAM_DEPLOYMENT_MODE',
  );
  return value === 'saas' || value === 'private' || value === 'local'
    ? value
    : 'saas';
}

export function resetImAppAuthenticatedSdkClients(): void {
  resetAppSdkClient();
  resetImSdkClient();
  resetDriveAppSdkClient();
}

export function clearImH5IamRuntimeSession(): void {
  clearAppSdkSessionTokens();
  disposeChatLiveConnection('iam session cleared');
  resetImAppAuthenticatedSdkClients();
}

function getAuthenticatedSdkClients(): SdkworkAppbasePcAuthRuntimeSdkClient[] {
  return [
    getAppSdkClient(),
    getImSdkClient(),
    getDriveAppSdkClient(),
  ] as SdkworkAppbasePcAuthRuntimeSdkClient[];
}

export function createImAppAuthRuntime(): SdkworkAppbasePcAuthRuntimeComposition {
  return createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: 'sdkwork-im-h5',
      deploymentMode: resolveIamDeploymentMode(),
      environment: resolveIamEnvironment(),
      platform: "h5",
    },
    baseUrls: {
      appbaseAppApiBaseUrl: resolveAppSdkBaseUrl(),
    },
    hooks: {
      onSessionChanged: () => {
        resetImAppAuthenticatedSdkClients();
      },
    },
    sdkClients: getAuthenticatedSdkClients(),
    sessionBridge: {
      clearSession: clearImH5IamRuntimeSession,
      commitSession: (session) => applyAppSdkSessionTokens(session as SdkworkImH5Session),
      readSession: readAppSdkSessionTokens,
    },
    tokenManager: getSdkworkImH5GlobalTokenManager(),
  });
}

export function getImAppAuthRuntime(): SdkworkAppbasePcAuthRuntimeComposition {
  if (!imAppAuthRuntimeComposition) {
    imAppAuthRuntimeComposition = createImAppAuthRuntime();
  }
  return imAppAuthRuntimeComposition;
}

export function resetImAppAuthRuntime(): void {
  imAppAuthRuntimeComposition = null;
}
