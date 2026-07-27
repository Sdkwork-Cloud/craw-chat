import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
} from '@sdkwork/auth-runtime-pc-react';
import { disposeChatLiveConnection } from '@sdkwork/im-h5-chat';
import { resolveImAuthRuntimeConfig } from './imAuthConfig';

export interface ImAppAuthRuntimeOptions {
  appId?: string;
  deploymentMode?: 'local' | 'private' | 'saas';
  environment?: 'dev' | 'prod' | 'test';
}

let imAppAuthRuntimeComposition: SdkworkAppbasePcAuthRuntimeComposition | null = null;

function resolveAppId(): string {
  return 'sdkwork-im-h5';
}

function resolveAppbaseAppApiBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  return meta.env?.SDKWORK_IM_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_IM_API_BASE_URL
    ?? '/';
}

export function createImAppAuthRuntime(
  options: ImAppAuthRuntimeOptions = {},
): SdkworkAppbasePcAuthRuntimeComposition {
  if (imAppAuthRuntimeComposition) {
    return imAppAuthRuntimeComposition;
  }

  const runtimeConfig = resolveImAuthRuntimeConfig();

  imAppAuthRuntimeComposition = createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: options.appId ?? resolveAppId(),
      deploymentMode: options.deploymentMode ?? 'saas',
      environment: options.environment ?? 'dev',
      platform: "h5",
    },
    baseUrls: {
      appbaseAppApiBaseUrl: resolveAppbaseAppApiBaseUrl(),
    },
    hooks: {
      onSessionChanged: () => {
        try {
          disposeChatLiveConnection();
        } catch {
          // ignore teardown errors during session refresh
        }
      },
    },
    sessionAuth: true,
  });

  void runtimeConfig;

  return imAppAuthRuntimeComposition;
}

export function resetImAppAuthRuntime(): void {
  imAppAuthRuntimeComposition = null;
}

export function getImAppAuthRuntime(): SdkworkAppbasePcAuthRuntimeComposition {
  return imAppAuthRuntimeComposition ?? createImAppAuthRuntime();
}
