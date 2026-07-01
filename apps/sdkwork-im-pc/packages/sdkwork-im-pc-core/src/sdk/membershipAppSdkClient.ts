import {
  createHttpClient,
  createMembershipsApi,
  type MembershipsApi,
  type SdkworkAppConfig,
} from '@sdkwork/membership-app-sdk';
import type { MembershipAppSdkClient } from '@sdkwork/membership-sdk-ports';
import type { Interceptors } from '@sdkwork/sdk-common';

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
