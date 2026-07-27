import {
  createHttpClient,
  createMembershipsApi,
  type MembershipsApi,
  type SdkworkAppConfig,
} from '@sdkwork/membership-app-sdk';
import {
  configureSdkworkAccountAppServiceProvider,
  configureSdkworkAccountSessionTokenProvider,
  createAccountAppSdkClientFromTransport,
  createAccountAppTransportClient,
  createSdkworkAccountAppService,
} from '@sdkwork/account-service';
import type { MembershipAppSdkClient } from '@sdkwork/membership-sdk-ports';
import {
  bootstrapSdkworkOrderAppService,
  configureSdkworkOrderAppServiceProvider,
  configureSdkworkOrderSessionTokenProvider,
  createSdkworkCouponRechargeService,
  createSdkworkMembershipCheckoutService,
  createSdkworkPointsRechargeService,
  type SdkworkCouponRechargeService,
  type SdkworkMembershipCheckoutService,
  type SdkworkPointsRechargeService,
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

type ImAccountAppSdkClient = ReturnType<typeof createAccountAppTransportClient>;

let accountAppSdkClient: ImAccountAppSdkClient | null = null;
let couponRechargeService: SdkworkCouponRechargeService | null = null;
let membershipAppSdkFacade: MembershipAppSdkFacade | null = null;
let membershipCheckoutService: SdkworkMembershipCheckoutService | null = null;
let membershipServiceBootstrapped = false;
let pointsRechargeService: SdkworkPointsRechargeService | null = null;

function requireCouponRechargeService(): SdkworkCouponRechargeService {
  if (!couponRechargeService) {
    throw new Error('IM coupon redemption composition is unavailable.');
  }
  return couponRechargeService;
}

function requireMembershipCheckoutService(): SdkworkMembershipCheckoutService {
  if (!membershipCheckoutService) {
    throw new Error('IM membership checkout composition is unavailable.');
  }
  return membershipCheckoutService;
}

function requirePointsRechargeService(): SdkworkPointsRechargeService {
  if (!pointsRechargeService) {
    throw new Error('IM Token Bank recharge composition is unavailable.');
  }
  return pointsRechargeService;
}

const imHostedCouponRechargeService: SdkworkCouponRechargeService = {
  redeem(code) {
    return requireCouponRechargeService().redeem(code);
  },
};

const imHostedMembershipCheckoutService: SdkworkMembershipCheckoutService = {
  createCheckout(input) {
    return requireMembershipCheckoutService().createCheckout(input);
  },
  getCheckoutStatus(orderId) {
    return requireMembershipCheckoutService().getCheckoutStatus(orderId);
  },
};

const imHostedPointsRechargeService: SdkworkPointsRechargeService = {
  createOrder(input) {
    return requirePointsRechargeService().createOrder(input);
  },
  getOrderStatus(orderId) {
    return requirePointsRechargeService().getOrderStatus(orderId);
  },
  listPackages() {
    return requirePointsRechargeService().listPackages();
  },
};

function initAccountPcIntegrationForIm(): ImAccountAppSdkClient {
  const currentSession = readAppSdkSessionTokens();
  const transportClient = createAccountAppTransportClient({
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    baseUrl: resolveAppSdkBaseUrl(),
    platform: 'pc',
    tokenManager: getSdkworkChatGlobalTokenManager(),
  });
  const accountAppService = createSdkworkAccountAppService({
    appClient: createAccountAppSdkClientFromTransport(transportClient),
  });
  configureSdkworkAccountAppServiceProvider(() => accountAppService);
  configureSdkworkAccountSessionTokenProvider(() => readAppSdkSessionTokens() ?? {});
  accountAppSdkClient = transportClient;
  return transportClient;
}

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
  initAccountPcIntegrationForIm();
  const orderAppService = bootstrapSdkworkOrderAppService({
    baseUrl: resolveAppSdkBaseUrl(),
    platform: 'pc',
    tokenManager,
  });
  configureSdkworkOrderSessionTokenProvider(() => readAppSdkSessionTokens() ?? {});
  membershipCheckoutService = createSdkworkMembershipCheckoutService({
    appService: orderAppService,
  });
  pointsRechargeService = createSdkworkPointsRechargeService({
    appService: orderAppService,
  });
  couponRechargeService = createSdkworkCouponRechargeService({
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

export function getImHostedAccountAppSdkClient(): ImAccountAppSdkClient {
  return accountAppSdkClient ?? initAccountPcIntegrationForIm();
}

export function getImHostedCouponRechargeService(): SdkworkCouponRechargeService {
  return imHostedCouponRechargeService;
}

export function getImHostedPointsRechargeService(): SdkworkPointsRechargeService {
  return imHostedPointsRechargeService;
}

export function isMembershipPcIntegrationBootstrapped(): boolean {
  return membershipServiceBootstrapped;
}

export function resetMembershipPcIntegration(): void {
  configureSdkworkAccountAppServiceProvider(null);
  configureSdkworkAccountSessionTokenProvider(null);
  configureSdkworkOrderAppServiceProvider(null);
  configureSdkworkOrderSessionTokenProvider(null);
  configureSdkworkMembershipAppServiceProvider(null);
  configureSdkworkMembershipSessionTokenProvider(null);
  accountAppSdkClient = null;
  couponRechargeService = null;
  membershipCheckoutService = null;
  pointsRechargeService = null;
  resetMembershipAppSdkClient();
  membershipServiceBootstrapped = false;
}

export { hasSdkworkMembershipSession } from '@sdkwork/membership-service';
