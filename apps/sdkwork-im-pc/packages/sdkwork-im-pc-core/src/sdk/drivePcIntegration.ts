import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import { getDriveAppSdkClient } from './driveAppSdkClient';
import type { DriveCapabilitySdkPorts, HostCapabilitySessionSnapshot } from './hostCapabilitySession';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let drivePcRuntimeBootstrapped = false;
let imDrivePcPorts: DriveCapabilitySdkPorts | null = null;

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
  const { configureDrivePcRuntime } = await import('@sdkwork/drive-pc-drive');
  ensureDrivePcRuntimeOnModule(configureDrivePcRuntime as DrivePcRuntimeConfigurator);
}

export async function rebootstrapDrivePcRuntimeForIm(): Promise<void> {
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
  imDrivePcPorts = null;
}
