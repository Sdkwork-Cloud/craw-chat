import type { IamDeploymentMode, IamEnvironment } from "@sdkwork/iam-contracts";
import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
  type SdkworkAppbasePcAuthRuntimeSdkClient,
} from "@sdkwork/auth-runtime-pc-react";
import {
  applyImH5IamSessionTokens,
  clearImH5IamSessionTokens,
  getImH5GlobalTokenManager,
  getImSdkClient,
  readImH5IamSessionTokens,
  resetImSdkClient,
  resolveAppSdkBaseUrl,
  type ImH5IamSession,
} from "@sdkwork/im-h5-core";
import { disposeChatLiveConnection } from "@sdkwork/im-h5-chat";

export interface CreateImAppAuthRuntimeOptions {
  appId: string;
  appbaseAppApiBaseUrl: string;
  deploymentMode?: IamDeploymentMode;
  environment?: IamEnvironment;
}

let imAppAuthRuntimeComposition: SdkworkAppbasePcAuthRuntimeComposition | null = null;

export function resetImH5AuthenticatedSdkClients(): void {
  disposeChatLiveConnection();
  resetImSdkClient();
}

export function createImAppAuthRuntime(
  options: CreateImAppAuthRuntimeOptions,
): SdkworkAppbasePcAuthRuntimeComposition {
  const composition = createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: options.appId,
      deploymentMode: options.deploymentMode ?? "saas",
      environment: options.environment ?? "dev",
      platform: "h5",
    },
    baseUrls: {
      appbaseAppApiBaseUrl: resolveAppSdkBaseUrl(options.appbaseAppApiBaseUrl),
    },
    hooks: {
      onSessionChanged: () => {
        resetImH5AuthenticatedSdkClients();
      },
    },
    sdkClients: [getImSdkClient() as SdkworkAppbasePcAuthRuntimeSdkClient],
    sessionBridge: {
      clearSession: clearImH5IamSessionTokens,
      commitSession: (session) => applyImH5IamSessionTokens(session as ImH5IamSession),
      readSession: readImH5IamSessionTokens,
    },
    tokenManager: getImH5GlobalTokenManager(),
  });

  imAppAuthRuntimeComposition = composition;
  return composition;
}

export function getImAppAuthRuntime(): SdkworkAppbasePcAuthRuntimeComposition | null {
  return imAppAuthRuntimeComposition;
}

export function getImIamRuntimeForAuth() {
  const composition = getImAppAuthRuntime();
  if (!composition) {
    throw new Error("IM H5 IAM runtime is not configured.");
  }
  return composition.getRuntime();
}
