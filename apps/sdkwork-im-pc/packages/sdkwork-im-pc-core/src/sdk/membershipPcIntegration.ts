import {
  createHttpClient,
  createMembershipsApi,
  type MembershipsApi,
  type SdkworkAppConfig,
} from '@sdkwork/membership-app-sdk';
import type { MembershipAppSdkClient } from '@sdkwork/membership-sdk-ports';
import {
  bootstrapSdkworkOrderAppService,
  configureSdkworkOrderAppServiceProvider,
  configureSdkworkOrderSessionTokenProvider,
  createSdkworkMembershipCheckoutService,
  type SdkworkMembershipCheckoutService,
} from '@sdkwork/order-service';
import type { Interceptors } from '@sdkwork/sdk-common';
import {
  configureSdkworkMembershipAppServiceProvider,
  configureSdkworkMembershipSessionTokenProvider,
  createSdkworkMembershipAppService,
  type SdkworkMembershipAppService,
} from '@sdkwork/membership-service';

import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';

export type MembershipAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

export type MembershipAppSdkFacade = MembershipAppSdkClient & {
  membershipsApi: MembershipsApi;
};

let membershipAppSdkFacade: MembershipAppSdkFacade | null = null;
let membershipCheckoutService: SdkworkMembershipCheckoutService | null = null;
let membershipServiceBootstrapped = false;

function requireMembershipCheckoutService(): SdkworkMembershipCheckoutService {
  if (!membershipCheckoutService) {
    throw new Error('IM membership checkout composition is unavailable.');
  }
  return membershipCheckoutService;
}

const imHostedMembershipCheckoutService: SdkworkMembershipCheckoutService = {
  createCheckout(input) {
    return requireMembershipCheckoutService().createCheckout(input);
  },
  getCheckoutStatus(orderId) {
    return requireMembershipCheckoutService().getCheckoutStatus(orderId);
  },
};

export function createMembershipAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): MembershipAppSdkClientConfig {
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

export function initMembershipAppSdkClient(
  config: MembershipAppSdkClientConfig = createMembershipAppSdkClientConfig(),
): MembershipAppSdkFacade {
  const httpClient = createHttpClient(config);
  const membershipsApi = createMembershipsApi(httpClient);
  membershipAppSdkFacade = {
    commerce: {
      memberships: membershipsApi,
    },
    membershipsApi,
  };
  return membershipAppSdkFacade;
}

export function getMembershipAppSdkClient(): MembershipAppSdkFacade {
  return membershipAppSdkFacade ?? initMembershipAppSdkClient();
}

export function getMembershipAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): MembershipAppSdkFacade {
  return initMembershipAppSdkClient(createMembershipAppSdkClientConfig(session));
}

export function resetMembershipAppSdkClient(): void {
  membershipAppSdkFacade = null;
}

export function bootstrapMembershipPcIntegrationForIm(): SdkworkMembershipAppService {
  const tokenManager = getSdkworkChatGlobalTokenManager();
  const orderAppService = bootstrapSdkworkOrderAppService({
    baseUrl: resolveAppSdkBaseUrl(),
    platform: 'pc',
    tokenManager,
  });
  membershipCheckoutService = createSdkworkMembershipCheckoutService({
    appService: orderAppService,
  });
  configureSdkworkMembershipSessionTokenProvider(() => readAppSdkSessionTokens() ?? {});
  configureSdkworkMembershipAppServiceProvider(() => createSdkworkMembershipAppService({
    appClient: getMembershipAppSdkClient(),
  }));
  membershipServiceBootstrapped = true;
  return createSdkworkMembershipAppService({
    appClient: getMembershipAppSdkClient(),
  });
}

export function rebootstrapMembershipPcIntegrationForIm(): SdkworkMembershipAppService {
  resetMembershipAppSdkClient();
  return bootstrapMembershipPcIntegrationForIm();
}

export function getImHostedMembershipCheckoutService(): SdkworkMembershipCheckoutService {
  return imHostedMembershipCheckoutService;
}

export function isMembershipPcIntegrationBootstrapped(): boolean {
  return membershipServiceBootstrapped;
}

export function resetMembershipPcIntegration(): void {
  configureSdkworkOrderAppServiceProvider(null);
  configureSdkworkOrderSessionTokenProvider(null);
  configureSdkworkMembershipAppServiceProvider(null);
  configureSdkworkMembershipSessionTokenProvider(null);
  membershipCheckoutService = null;
  resetMembershipAppSdkClient();
  membershipServiceBootstrapped = false;
}

export { hasSdkworkMembershipSession } from '@sdkwork/membership-service';
