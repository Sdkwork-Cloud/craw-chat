import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import { getDriveAppSdkClient } from './driveAppSdkClient';
import { getKnowledgebaseAppSdkClient } from './knowledgebaseAppSdkClient';
import type { HostCapabilitySessionSnapshot, KnowledgebaseCapabilitySdkPorts } from './hostCapabilitySession';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let knowledgebasePcRuntimeBootstrapped = false;
let imKnowledgebasePcPorts: KnowledgebaseCapabilitySdkPorts | null = null;

function mapImSessionToKnowledgebaseSnapshot(
  session: SdkworkChatSession | null,
): HostCapabilitySessionSnapshot | null {
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

function createImKnowledgebasePcSdkPorts(): KnowledgebaseCapabilitySdkPorts {
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

export type KnowledgebasePcRuntimeConfigurator = (options: {
  sdkPorts: KnowledgebaseCapabilitySdkPorts;
}) => void;

function resolveImKnowledgebasePcSdkPorts(): KnowledgebaseCapabilitySdkPorts {
  imKnowledgebasePcPorts ??= createImKnowledgebasePcSdkPorts();
  return imKnowledgebasePcPorts;
}

export function ensureKnowledgebasePcRuntimeOnModule(
  configureRuntime: KnowledgebasePcRuntimeConfigurator,
): void {
  configureRuntime({
    sdkPorts: resolveImKnowledgebasePcSdkPorts() as never,
  });
  knowledgebasePcRuntimeBootstrapped = true;
}

export async function bootstrapKnowledgebasePcForIm(): Promise<void> {
  const { configureKnowledgebasePcRuntime } = await import('@sdkwork/knowledgebase-pc-knowledge');
  ensureKnowledgebasePcRuntimeOnModule(configureKnowledgebasePcRuntime as KnowledgebasePcRuntimeConfigurator);
}

export async function rebootstrapKnowledgebasePcRuntimeForIm(
  configureRuntime?: KnowledgebasePcRuntimeConfigurator,
): Promise<void> {
  if (!imKnowledgebasePcPorts) {
    return;
  }
  if (configureRuntime) {
    configureRuntime({
      sdkPorts: imKnowledgebasePcPorts as never,
    });
    return;
  }
  const { configureKnowledgebasePcRuntime } = await import('@sdkwork/knowledgebase-pc-knowledge');
  configureKnowledgebasePcRuntime({
    sdkPorts: imKnowledgebasePcPorts as never,
  });
}

export function isKnowledgebasePcRuntimeBootstrapped(): boolean {
  return knowledgebasePcRuntimeBootstrapped;
}

export function resetKnowledgebasePcRuntime(): void {
  knowledgebasePcRuntimeBootstrapped = false;
  imKnowledgebasePcPorts = null;
}
