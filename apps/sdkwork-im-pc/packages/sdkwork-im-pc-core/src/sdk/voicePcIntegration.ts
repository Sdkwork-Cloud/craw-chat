import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import type { HostCapabilitySessionSnapshot, VoiceCapabilitySdkPorts } from './hostCapabilitySession';
import { getVoiceAppSdkClient } from './voiceAppSdkClient';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let voicePcRuntimeBootstrapped = false;
let imVoicePcPorts: VoiceCapabilitySdkPorts | null = null;

function mapImSessionToVoiceSnapshot(
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

function createImVoicePcSdkPorts(): VoiceCapabilitySdkPorts {
  const hostLanguageBridge = createImPcHostLanguageBridge();
  return {
    getVoiceClient: getVoiceAppSdkClient,
    readHostSession: () => mapImSessionToVoiceSnapshot(readAppSdkSessionTokens()),
    subscribeHostSession(listener) {
      const handler = () => listener();
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
    },
    resolveHostLanguage: hostLanguageBridge.resolveInitialLanguage,
    subscribeHostLanguage: hostLanguageBridge.onLanguageChange,
  };
}

export type VoicePcRuntimeConfigurator = (options: {
  sdkPorts: VoiceCapabilitySdkPorts;
}) => void;

function resolveImVoicePcSdkPorts(): VoiceCapabilitySdkPorts {
  imVoicePcPorts ??= createImVoicePcSdkPorts();
  return imVoicePcPorts;
}

export function ensureVoicePcRuntimeOnModule(
  configureRuntime: VoicePcRuntimeConfigurator,
): void {
  configureRuntime({
    sdkPorts: resolveImVoicePcSdkPorts() as never,
  });
  voicePcRuntimeBootstrapped = true;
}

export async function bootstrapVoicePcForIm(): Promise<void> {
  const { configureVoicePcRuntime } = await import('@sdkwork/voice-pc-market');
  ensureVoicePcRuntimeOnModule(configureVoicePcRuntime as VoicePcRuntimeConfigurator);
}

export async function rebootstrapVoicePcRuntimeForIm(
  configureRuntime?: VoicePcRuntimeConfigurator,
): Promise<void> {
  if (!imVoicePcPorts) {
    return;
  }
  if (configureRuntime) {
    configureRuntime({
      sdkPorts: imVoicePcPorts as never,
    });
    return;
  }
  const { configureVoicePcRuntime } = await import('@sdkwork/voice-pc-market');
  configureVoicePcRuntime({
    sdkPorts: imVoicePcPorts as never,
  });
}

export function isVoicePcRuntimeBootstrapped(): boolean {
  return voicePcRuntimeBootstrapped;
}

export function resetVoicePcRuntime(): void {
  voicePcRuntimeBootstrapped = false;
  imVoicePcPorts = null;
}
