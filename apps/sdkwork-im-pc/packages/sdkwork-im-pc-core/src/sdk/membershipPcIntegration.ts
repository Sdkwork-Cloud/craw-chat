import {
  configureSdkworkMembershipAppServiceProvider,
  configureSdkworkMembershipSessionTokenProvider,
  createSdkworkMembershipAppService,
  type SdkworkMembershipAppService,
} from '@sdkwork/membership-service';

import {
  getMembershipAppSdkClient,
  resetMembershipAppSdkClient,
} from './membershipAppSdkClient';
import { readAppSdkSessionTokens } from './session';

let membershipServiceBootstrapped = false;

export function bootstrapMembershipPcIntegrationForIm(): SdkworkMembershipAppService {
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

export function isMembershipPcIntegrationBootstrapped(): boolean {
  return membershipServiceBootstrapped;
}

export function resetMembershipPcIntegration(): void {
  configureSdkworkMembershipAppServiceProvider(null);
  configureSdkworkMembershipSessionTokenProvider(null);
  resetMembershipAppSdkClient();
  membershipServiceBootstrapped = false;
}
