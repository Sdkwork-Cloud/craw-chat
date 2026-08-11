/**
 * H5 capability bootstrap entrypoint.
 *
 * Orchestrates environment resolution, SDK client construction, token manager
 * binding, host adapter registration, and route assembly before the React app
 * renders. UI packages must not call bootstrap modules directly; they consume
 * the bound composition through hooks or context.
 */

import { resolveH5RuntimeEnvironment } from './environment';
import {
  configureCloudDriveRuntime,
  resetCloudDriveRuntime,
} from '@sdkwork/drive-mobile-react-drive';
import {
  configureOrderMobileRuntime,
  resetOrderMobileRuntime,
} from '@sdkwork/order-mobile-react-orders';
import {
  configureKnowledgeBaseRuntime,
  resetKnowledgeBaseRuntime,
} from '@sdkwork/knowledgebase-mobile-react-knowledge';
import {
  configureAgentService,
  configureAgentChatService,
  configureKnowledgeSelectionAdapter,
  createKnowledgebaseSelectionAdapter,
} from '@sdkwork/agents-h5-agents';
import { useAppStore } from '@sdkwork/im-h5-core';
import { initSdkClients, resetSdkClients } from './sdkClients';
import { bootstrapImCommunityH5Port } from './communityPort';
import { bootstrapImCourseH5Port } from './coursePort';
import { bootstrapImMomentsH5Port } from './momentsPort';
import { createWechatPaymentOAuthChannel } from './wechatPaymentOAuth';
import { configureVoiceMyVoicesRuntime } from '@sdkwork/im-h5-ai-voice';
import { resolveTokenManagerBinding, resetTokenManagerBinding } from './tokenManager';
import { registerHostAdapter, resetHostAdapters } from './hostAdapters';
import { registerRoute, resetRoutes, IM_H5_ROUTE_REGISTRY } from './routes';

export interface H5BootstrapResult {
  readonly environment: ReturnType<typeof resolveH5RuntimeEnvironment>;
  readonly sdkClients: ReturnType<typeof initSdkClients>;
  readonly tokenManager: ReturnType<typeof resolveTokenManagerBinding>;
  readonly hostAdapters: ReturnType<typeof registerHostAdapter>[];
  readonly routes: typeof IM_H5_ROUTE_REGISTRY;
}

let bootstrapResult: H5BootstrapResult | null = null;

export async function bootstrapImH5CapabilityIntegrations(): Promise<H5BootstrapResult> {
  if (bootstrapResult) {
    return bootstrapResult;
  }

  const environment = resolveH5RuntimeEnvironment();
  const sdkClients = initSdkClients();
  const tokenManager = resolveTokenManagerBinding();
  configureCloudDriveRuntime({ client: sdkClients.driveAppSdkClient });
  configureOrderMobileRuntime({
    client: sdkClients.orderAppSdkClient,
    wechatPaymentOAuth: createWechatPaymentOAuthChannel(sdkClients.iamAppSdkClient),
    paymentRegion: environment.paymentRegion,
  });
  configureKnowledgeBaseRuntime({
    client: sdkClients.knowledgebaseAppSdkClient,
    // Scope the local knowledge base registry to the signed-in user.
    resolveScopeKey: () => useAppStore.getState().currentUser?.id,
  });
  // Agents capability: inject the IM-constructed agents app SDK client so the
  // agents H5 views never construct raw HTTP or their own transport. The chat
  // transport shares the same injected client (sessions/turns).
  configureAgentService(() => sdkClients.agentsAppSdkClient as never);
  configureAgentChatService(() => sdkClients.agentsAppSdkClient as never);
  configureKnowledgeSelectionAdapter(
    createKnowledgebaseSelectionAdapter(sdkClients.knowledgebaseAppSdkClient),
  );
  // My voices capability: inject the voice app SDK client + Drive media ports.
  configureVoiceMyVoicesRuntime();
  // Community (圈子) capability: bind the mobile React package to the
  // generated Community App SDK port (auth session port side effect included).
  bootstrapImCommunityH5Port();
  // Course (课程) capability: bind the canonical course mobile React package
  // to the generated Course App SDK port. Without this binding the course
  // pages fail closed with CourseCapabilityUnavailableError.
  bootstrapImCourseH5Port();
  // Moments (朋友圈) capability: bind the moments feature package to the
  // generated Community App SDK port (feed / publish / reactions / comments).
  // Without this binding the moments pages fail closed with
  // MomentCapabilityUnavailableError.
  bootstrapImMomentsH5Port();

  const hostAdapters: H5BootstrapResult['hostAdapters'] = [];
  for (const meta of IM_H5_ROUTE_REGISTRY) {
    registerRoute(meta);
  }

  bootstrapResult = {
    environment,
    sdkClients,
    tokenManager,
    hostAdapters,
    routes: IM_H5_ROUTE_REGISTRY,
  };

  return bootstrapResult;
}

export function getH5BootstrapResult(): H5BootstrapResult | null {
  return bootstrapResult;
}

export function resetH5Bootstrap(): void {
  resetCloudDriveRuntime();
  resetOrderMobileRuntime();
  resetKnowledgeBaseRuntime();
  resetSdkClients();
  resetTokenManagerBinding();
  resetHostAdapters();
  resetRoutes();
  bootstrapResult = null;
}

export {
  resolveH5RuntimeEnvironment,
} from './environment';
export {
  initSdkClients,
  getSdkClients,
} from './sdkClients';
export {
  resolveTokenManagerBinding,
  getTokenManagerBinding,
} from './tokenManager';
export {
  registerHostAdapter,
  getHostAdapter,
} from './hostAdapters';
export {
  registerRoute,
  listRoutes,
} from './routes';
export { getImAppAuthRuntime } from './iamRuntime';
export { resolveImAuthRuntimeConfig } from './imAuthConfig';
