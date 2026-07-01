import type {
  SdkworkAuthAppearanceConfig,
  SdkworkAuthRuntimeConfig,
  SdkworkIamRuntimeAuthRuntimeLike,
} from '@sdkwork/auth-pc-react';
import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
  type SdkworkAppbasePcAuthRuntimeSdkClient,
} from '@sdkwork/auth-runtime-pc-react';
import { resetAiotAppSdkClient, getAiotAppSdkClient } from './aiotAppSdkClient';
import { resetAppSdkClient, getAppSdkClient, resolveAppSdkBaseUrl } from './appSdkClient';
import { resetAgentAppSdkClient, getAgentAppSdkClient } from './agentAppSdkClient';
import { resetAppbaseAppSdkClient } from './appbaseAppSdkClient';
import {
  getCatalogAppSdkClient,
  resetCatalogAppSdkClient,
} from './catalogAppSdkClient';
import {
  getMembershipAppSdkClient,
  resetMembershipAppSdkClient,
} from './membershipAppSdkClient';
import {
  getOrderAppSdkClient,
  resetOrderAppSdkClient,
} from './orderAppSdkClient';
import {
  getShopAppSdkClient,
  resetShopAppSdkClient,
} from './shopAppSdkClient';
import {
  getCommunityAppSdkClient,
  resetCommunityAppSdkClient,
} from './communityAppSdkClient';
import { getCourseAppSdkClient, resetCourseAppSdkClient } from './courseAppSdkClient';
import { resetCourseBackendSdkClient } from './courseBackendSdkClient';
import { resetDriveAppSdkClient, getDriveAppSdkClient } from './driveAppSdkClient';
import { rebootstrapDrivePcRuntimeForIm, resetDrivePcRuntime } from './drivePcIntegration';
import { rebootstrapCoursePcRuntimeForIm, resetCoursePcRuntime } from './coursePcIntegration';
import { resetImSdkClient, getImSdkClient } from './imSdkClient';
import { getKnowledgebaseAppSdkClient, resetKnowledgebaseAppSdkClient } from './knowledgebaseAppSdkClient';
import { rebootstrapKnowledgebasePcRuntimeForIm, resetKnowledgebasePcRuntime } from './knowledgebasePcIntegration';
import { getVoiceAppSdkClient, resetVoiceAppSdkClient } from './voiceAppSdkClient';
import { rebootstrapVoicePcRuntimeForIm, resetVoicePcRuntime } from './voicePcIntegration';
import { getMailAppSdkClient, resetMailAppSdkClient } from './mailAppSdkClient';
import { resetNotaryAppSdkClient, getNotaryAppSdkClient } from './notaryAppSdkClient';
import { rebootstrapNotaryPcRuntimeForIm, resetNotaryPcRuntime } from './notaryPcIntegration';
import { rebootstrapMembershipPcIntegrationForIm, resetMembershipPcIntegration, bootstrapMembershipPcIntegrationForIm } from './membershipPcIntegration';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  type SdkworkChatSession,
} from './session';

type IamEnvironment = 'dev' | 'prod' | 'test';
type IamDeploymentMode = 'local' | 'private' | 'saas';

const SDKWORK_IM_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: false,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: false,
  phoneRegistrationVerificationRequired: false,
} as const;

let sdkworkChatIamRuntimeComposition: SdkworkAppbasePcAuthRuntimeComposition | null = null;

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

function parseBoolean(value: string | undefined): boolean | undefined {
  const normalized = (value ?? '').trim().toLowerCase();
  if (!normalized) {
    return undefined;
  }

  if (['1', 'on', 'true', 'yes'].includes(normalized)) {
    return true;
  }

  if (['0', 'false', 'no', 'off'].includes(normalized)) {
    return false;
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

export function resetSdkworkChatAuthenticatedSdkClients(): void {
  resetAppbaseAppSdkClient();
  resetAiotAppSdkClient();
  resetAppSdkClient();
  resetAgentAppSdkClient();
  resetCatalogAppSdkClient();
  resetMembershipAppSdkClient();
  resetOrderAppSdkClient();
  resetShopAppSdkClient();
  resetCommunityAppSdkClient();
  resetCourseAppSdkClient();
  resetCourseBackendSdkClient();
  resetDriveAppSdkClient();
  resetImSdkClient();
  resetKnowledgebaseAppSdkClient();
  resetVoiceAppSdkClient();
  resetMailAppSdkClient();
  resetNotaryAppSdkClient();
  void resetNotaryPcRuntime();
  resetMembershipPcIntegration();
  resetDrivePcRuntime();
  resetKnowledgebasePcRuntime();
  resetVoicePcRuntime();
  resetCoursePcRuntime();
}

export function clearSdkworkChatIamRuntimeSession(): void {
  clearAppSdkSessionTokens();
  resetSdkworkChatAuthenticatedSdkClients();
}

function getAuthenticatedSdkClients(): SdkworkAppbasePcAuthRuntimeSdkClient[] {
  return [
    getAiotAppSdkClient(),
    getAppSdkClient(),
    getAgentAppSdkClient(),
    getCatalogAppSdkClient(),
    getOrderAppSdkClient(),
    getShopAppSdkClient(),
    getCommunityAppSdkClient(),
    getCourseAppSdkClient(),
    getDriveAppSdkClient(),
    getImSdkClient(),
    getKnowledgebaseAppSdkClient(),
    getVoiceAppSdkClient(),
    getMailAppSdkClient(),
    getNotaryAppSdkClient(),
  ] as SdkworkAppbasePcAuthRuntimeSdkClient[];
}

function createSdkworkChatIamRuntime(): SdkworkAppbasePcAuthRuntimeComposition {
  bootstrapMembershipPcIntegrationForIm();
  return createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: 'sdkwork-im-pc',
      deploymentMode: resolveIamDeploymentMode(),
      environment: resolveIamEnvironment(),
      platform: 'pc',
    },
    baseUrls: {
      appbaseAppApiBaseUrl: resolveAppSdkBaseUrl(),
    },
    hooks: {
      onSessionChanged: () => {
        resetSdkworkChatAuthenticatedSdkClients();
        void rebootstrapNotaryPcRuntimeForIm();
        rebootstrapMembershipPcIntegrationForIm();
        void rebootstrapDrivePcRuntimeForIm();
        void rebootstrapKnowledgebasePcRuntimeForIm();
        void rebootstrapVoicePcRuntimeForIm();
        void rebootstrapCoursePcRuntimeForIm();
      },
    },
    sdkClients: getAuthenticatedSdkClients(),
    sessionBridge: {
      clearSession: clearSdkworkChatIamRuntimeSession,
      commitSession: (session) => applyAppSdkSessionTokens(session as SdkworkChatSession),
      readSession: readAppSdkSessionTokens,
    },
    tokenManager: getSdkworkChatGlobalTokenManager(),
  });
}

function resolveDevelopmentPrefill(): SdkworkAuthRuntimeConfig['developmentPrefill'] {
  const account = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_ACCOUNT',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_ACCOUNT',
  );
  const email = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_EMAIL',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_EMAIL',
  );
  const phone = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_PHONE',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_PHONE',
  );
  const password = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_DEFAULT_PASSWORD',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_PASSWORD',
  );
  const verificationCode = readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_VERIFICATION_CODE',
    'VITE_SDKWORK_AUTH_DEV_VERIFICATION_CODE',
  );
  const verificationCodePrefillEnabled = parseBoolean(readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_VERIFICATION_CODE_PREFILL_ENABLED',
    'VITE_SDKWORK_AUTH_DEV_VERIFICATION_CODE_PREFILL_ENABLED',
    'VITE_SDKWORK_IM_AUTH_DEV_VERIFICATION_CODE_ENABLED',
    'VITE_SDKWORK_AUTH_DEV_VERIFICATION_CODE_ENABLED',
  ));
  const enabled = parseBoolean(readEnvValue(
    'VITE_SDKWORK_IM_AUTH_DEV_PREFILL_ENABLED',
    'VITE_SDKWORK_AUTH_DEV_PREFILL_ENABLED',
  ));
  const shouldEnable = enabled ?? Boolean(account || email || phone || password || verificationCode);

  if (!shouldEnable) {
    return undefined;
  }

  return {
    account: account || email || phone,
    email,
    enabled: true,
    loginMethod: 'password',
    password,
    phone,
    verificationCode,
    ...(typeof verificationCodePrefillEnabled === 'boolean'
      ? { verificationCodePrefillEnabled }
      : {}),
  };
}

export function getSdkworkChatIamRuntime(): SdkworkIamRuntimeAuthRuntimeLike {
  if (!sdkworkChatIamRuntimeComposition) {
    sdkworkChatIamRuntimeComposition = createSdkworkChatIamRuntime();
  }

  return sdkworkChatIamRuntimeComposition.runtime as unknown as SdkworkIamRuntimeAuthRuntimeLike;
}

export function resetSdkworkChatIamRuntime(): void {
  sdkworkChatIamRuntimeComposition = null;
}

export function resolveSdkworkChatAuthRuntimeConfig(): SdkworkAuthRuntimeConfig {
  const developmentPrefill = resolveDevelopmentPrefill();
  return {
    leftRailMode: 'qr-only',
    loginMethods: ['password'],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: [],
    registerMethods: ['email', 'phone'],
    verificationPolicy: SDKWORK_IM_VERIFICATION_POLICY,
    ...(developmentPrefill ? { developmentPrefill } : {}),
  };
}

export function resolveSdkworkChatAuthAppearance(): SdkworkAuthAppearanceConfig {
  return {
    asidePanelClassName: 'sdkwork-chat-auth-aside-panel',
    bodyClassName: 'sdkwork-chat-auth-body',
    contentContainerClassName: 'sdkwork-chat-auth-content',
    pageClassName: 'sdkwork-chat-auth-page',
    qrFrameClassName: 'sdkwork-chat-auth-qr-frame',
    shellClassName: 'sdkwork-chat-auth-card-shell',
    slotProps: {
      background: {
        className: 'sdkwork-chat-auth-background',
      },
      page: {
        className: 'sdkwork-chat-auth-page',
      },
      shell: {
        className: 'sdkwork-chat-auth-card-shell',
      },
    },
    theme: {
      asideCardBackgroundColor: 'var(--sdkwork-chat-auth-aside-card-bg)',
      asideCardBorderColor: 'var(--sdkwork-chat-auth-aside-card-border)',
      asidePanelBackgroundColor: 'var(--sdkwork-chat-auth-aside-bg)',
      asidePanelBorderColor: 'var(--sdkwork-chat-auth-aside-border)',
      asidePanelColor: 'var(--sdkwork-chat-auth-aside-text)',
      badgeBackgroundColor: 'var(--sdkwork-chat-auth-aside-badge-bg)',
      badgeTextColor: 'var(--sdkwork-chat-auth-aside-badge-text)',
      contentBackgroundColor: 'var(--sdkwork-chat-auth-content-bg)',
      contentBorderColor: 'transparent',
      contentTextColor: 'var(--sdkwork-chat-auth-content-text)',
      descriptionColor: 'var(--sdkwork-chat-auth-muted-text)',
      dividerColor: 'var(--sdkwork-chat-auth-divider)',
      fieldBackgroundColor: 'var(--sdkwork-chat-auth-field-bg)',
      fieldBorderColor: 'transparent',
      fieldPlaceholderColor: '#9ca3af',
      fieldTextColor: 'var(--sdkwork-chat-auth-content-text)',
      formMutedTextColor: 'var(--sdkwork-chat-auth-muted-text)',
      iconMutedColor: 'var(--sdkwork-chat-auth-muted-text)',
      labelColor: 'var(--sdkwork-chat-auth-content-text)',
      pageBackgroundColor: 'var(--sdkwork-chat-auth-bg)',
      qrFrameBackgroundColor: 'var(--sdkwork-chat-auth-qr-bg)',
      qrFrameBorderColor: 'transparent',
      shellBackgroundColor: 'var(--sdkwork-chat-auth-content-bg)',
      shellBorderColor: 'transparent',
      tabActiveBackgroundColor: 'transparent',
      tabActiveTextColor: 'var(--sdkwork-chat-auth-content-text)',
      tabBackgroundColor: 'transparent',
      tabInactiveTextColor: 'var(--sdkwork-chat-auth-muted-text)',
      titleColor: 'var(--sdkwork-chat-auth-content-text)',
    },
  };
}
