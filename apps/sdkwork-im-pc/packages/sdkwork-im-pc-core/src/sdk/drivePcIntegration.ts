import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';
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
import type { DriveCapabilitySdkPorts, HostCapabilitySessionSnapshot } from './hostCapabilitySession';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
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
let drivePcRuntimeBootstrapped = false;
let driveSessionListenerRegistered = false;
let imDrivePcPorts: DriveCapabilitySdkPorts | null = null;

export function createDriveAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkDriveAppClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc',
    tokenManager: getSdkworkChatGlobalTokenManager(),
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

export function syncImSessionToDrivePc(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetDriveAppSdkClient();
    return;
  }

  resetDriveAppSdkClient();
  void resolveAppSdkBaseUrl();
}

function mapImSessionToDriveSnapshot(session: SdkworkChatSession | null): HostCapabilitySessionSnapshot | null {
  if (!session?.authToken || !session.accessToken || !session.context?.tenantId || !session.context?.userId) {
    return null;
  }

  return {
    authToken: session.authToken,
    accessToken: session.accessToken,
    refreshToken: session.refreshToken,
    sessionId: session.sessionId,
    user: session.user?.id
      ? {
          id: String(session.user.userId ?? session.user.id),
          displayName: session.user.displayName ?? session.user.name ?? session.user.nickname,
          avatarUrl: session.user.avatar,
          email: session.user.email,
        }
      : undefined,
    context: {
      tenantId: session.context.tenantId,
      userId: session.context.userId,
      organizationId: session.context.organizationId,
      sessionId: session.context.sessionId ?? session.sessionId,
      appId: session.context.appId,
      environment: session.context.environment,
      deploymentMode: session.context.deploymentMode,
      actorId: session.context.actorId,
      actorKind: session.context.actorKind,
      deviceId: session.context.deviceId,
      dataScope: session.context.dataScope,
      permissionScope: session.context.permissionScope,
      authLevel: session.context.authLevel,
    },
  };
}

function createImDrivePcSdkPorts(): DriveCapabilitySdkPorts {
  const hostLanguageBridge = createImPcHostLanguageBridge();
  return {
    getDriveClient: getDriveAppSdkClient,
    readHostSession: () => mapImSessionToDriveSnapshot(readAppSdkSessionTokens()),
    subscribeHostSession(listener) {
      const handler = () => listener();
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
    },
    resolveHostLanguage: hostLanguageBridge.resolveInitialLanguage,
    subscribeHostLanguage: hostLanguageBridge.onLanguageChange,
  };
}

export type DrivePcRuntimeConfigurator = (options: {
  sdkPorts: DriveCapabilitySdkPorts;
}) => void;

function resolveImDrivePcSdkPorts(): DriveCapabilitySdkPorts {
  imDrivePcPorts ??= createImDrivePcSdkPorts();
  return imDrivePcPorts;
}

export function ensureDrivePcRuntimeOnModule(
  configureRuntime: DrivePcRuntimeConfigurator,
): void {
  configureRuntime({
    sdkPorts: resolveImDrivePcSdkPorts() as never,
  });
  drivePcRuntimeBootstrapped = true;
}

export async function bootstrapDrivePcForIm(): Promise<void> {
  syncImSessionToDrivePc();
  const { configureDrivePcRuntime } = await import('@sdkwork/drive-pc-drive');
  ensureDrivePcRuntimeOnModule(configureDrivePcRuntime as DrivePcRuntimeConfigurator);

  if (!driveSessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToDrivePc();
    });
    driveSessionListenerRegistered = true;
  }
}

export async function rebootstrapDrivePcRuntimeForIm(): Promise<void> {
  syncImSessionToDrivePc();
  if (!imDrivePcPorts) {
    return;
  }
  const { configureDrivePcRuntime } = await import('@sdkwork/drive-pc-drive');
  configureDrivePcRuntime({
    sdkPorts: imDrivePcPorts as never,
  });
}

export function isDrivePcRuntimeBootstrapped(): boolean {
  return drivePcRuntimeBootstrapped;
}

export function resetDrivePcRuntime(): void {
  drivePcRuntimeBootstrapped = false;
  driveSessionListenerRegistered = false;
  imDrivePcPorts = null;
  resetDriveAppSdkClient();
}
