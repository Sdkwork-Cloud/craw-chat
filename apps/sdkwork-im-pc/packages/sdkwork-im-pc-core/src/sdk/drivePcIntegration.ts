import type { SessionSnapshot } from 'sdkwork-drive-pc-core';
import { configureDrivePcRuntime } from '@sdkwork/drive-pc-drive';
import type { DrivePcSdkPorts } from '@sdkwork/drive-pc-drive';
import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import { getDriveAppSdkClient } from './driveAppSdkClient';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let drivePcRuntimeBootstrapped = false;
let imDrivePcPorts: DrivePcSdkPorts | null = null;

function mapImSessionToDriveSnapshot(session: SdkworkChatSession | null): SessionSnapshot | null {
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

function createImDrivePcSdkPorts(): DrivePcSdkPorts {
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

export function bootstrapDrivePcForIm(): void {
  imDrivePcPorts = createImDrivePcSdkPorts();
  configureDrivePcRuntime({
    sdkPorts: imDrivePcPorts,
  });
  drivePcRuntimeBootstrapped = true;
}

export function rebootstrapDrivePcRuntimeForIm(): void {
  if (!imDrivePcPorts) {
    return;
  }
  configureDrivePcRuntime({
    sdkPorts: imDrivePcPorts,
  });
}

export function isDrivePcRuntimeBootstrapped(): boolean {
  return drivePcRuntimeBootstrapped;
}

export function resetDrivePcRuntime(): void {
  drivePcRuntimeBootstrapped = false;
}
