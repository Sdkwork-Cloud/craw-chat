import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
} from '@sdkwork/auth-runtime-pc-react';
import { notifyImH5SessionChanged } from '@sdkwork/im-h5-core/session';
import { setImLiveSessionActiveProvider } from '@sdkwork/im-h5-core/realtime';
import { resolveH5RuntimeEnvironment } from './environment';
import { resolveImAuthRuntimeConfig } from './imAuthConfig';
import { initSdkClients } from './sdkClients';
import {
  createImH5SessionBridge,
  emitImH5SessionChanged,
} from './session';
import { getTokenManagerBinding } from './tokenManager';

export interface ImAppAuthRuntimeOptions {
  appId?: string;
  deploymentMode?: 'local' | 'private' | 'saas';
  environment?: 'dev' | 'prod' | 'test';
}

let imAppAuthRuntimeComposition: SdkworkAppbasePcAuthRuntimeComposition | null = null;

function resolveAppId(): string {
  return resolveH5RuntimeEnvironment().appKey;
}

export function createImAppAuthRuntime(
  options: ImAppAuthRuntimeOptions = {},
): SdkworkAppbasePcAuthRuntimeComposition {
  if (imAppAuthRuntimeComposition) {
    return imAppAuthRuntimeComposition;
  }

  const runtimeConfig = resolveImAuthRuntimeConfig();
  const environment = resolveH5RuntimeEnvironment();
  const tokenManager = getTokenManagerBinding();
  const sdkClients = initSdkClients(tokenManager);
  // Bind the realtime manager's session gate to the shared TokenManager so
  // reconnects/invalidations follow the actual authenticated session.
  setImLiveSessionActiveProvider(() => tokenManager.hasToken());
  const sessionBridge = createImH5SessionBridge({
    notifySessionChanged: (session) => {
      emitImH5SessionChanged(session);
      if (!session) {
        notifyImH5SessionChanged();
      }
    },
  });

  imAppAuthRuntimeComposition = createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: options.appId ?? resolveAppId(),
      deploymentMode: options.deploymentMode ?? 'saas',
      environment: options.environment ?? 'dev',
      platform: "h5",
    },
    baseUrls: {
      appbaseAppApiBaseUrl: environment.iamApiBaseUrl,
    },
    hooks: {
      onSessionChanged: (session) => {
        emitImH5SessionChanged(session);
        if (session) {
          notifyImH5SessionChanged();
        }
      },
    },
    sdkClients: [
      sdkClients.driveAppSdkClient,
      sdkClients.imSdkClient,
      sdkClients.orderAppSdkClient,
      sdkClients.notaryAppSdkClient,
    ],
    sessionBridge,
    sessionAuth: {
      // 401 处理不启用 raw `window.location.replace` 跳转，避免在任意
      // 深层路由（history 模式）下丢失登录回跳目标。AuthGate owns the
      // login redirect through the session-changed event instead.
      shouldRedirectOnUnauthorized: () => false,
    },
    tokenManager,
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
