import { configureKnowledgebasePcRuntime } from '@sdkwork/knowledgebase-pc-knowledge';
import type { KnowledgebasePcSdkPorts } from '@sdkwork/knowledgebase-pc-knowledge';
import type { SessionSnapshot } from 'sdkwork-knowledgebase-pc-core';
import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import { getDriveAppSdkClient } from './driveAppSdkClient';
import { getKnowledgebaseAppSdkClient } from './knowledgebaseAppSdkClient';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let knowledgebasePcRuntimeBootstrapped = false;
let imKnowledgebasePcPorts: KnowledgebasePcSdkPorts | null = null;

function mapImSessionToKnowledgebaseSnapshot(
  session: SdkworkChatSession | null,
): SessionSnapshot | null {
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
      iamDeploymentMode: session.context.deploymentMode,
      actorId: session.context.actorId,
      actorKind: session.context.actorKind,
      deviceId: session.context.deviceId,
      dataScope: session.context.dataScope,
      permissionScope: session.context.permissionScope,
      authLevel: session.context.authLevel,
    },
  };
}

function createImKnowledgebasePcSdkPorts(): KnowledgebasePcSdkPorts {
  const hostLanguageBridge = createImPcHostLanguageBridge();
  return {
    getKnowledgebaseClient: getKnowledgebaseAppSdkClient,
    getDriveClient: getDriveAppSdkClient,
    readHostSession: () => mapImSessionToKnowledgebaseSnapshot(readAppSdkSessionTokens()),
    subscribeHostSession(listener) {
      const handler = () => listener();
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
    },
    resolveHostLanguage: hostLanguageBridge.resolveInitialLanguage,
    subscribeHostLanguage: hostLanguageBridge.onLanguageChange,
  };
}

export function bootstrapKnowledgebasePcForIm(): void {
  imKnowledgebasePcPorts = createImKnowledgebasePcSdkPorts();
  configureKnowledgebasePcRuntime({
    sdkPorts: imKnowledgebasePcPorts,
  });
  knowledgebasePcRuntimeBootstrapped = true;
}

export function rebootstrapKnowledgebasePcRuntimeForIm(): void {
  if (!imKnowledgebasePcPorts) {
    return;
  }
  configureKnowledgebasePcRuntime({
    sdkPorts: imKnowledgebasePcPorts,
  });
}

export function isKnowledgebasePcRuntimeBootstrapped(): boolean {
  return knowledgebasePcRuntimeBootstrapped;
}

export function resetKnowledgebasePcRuntime(): void {
  knowledgebasePcRuntimeBootstrapped = false;
}
