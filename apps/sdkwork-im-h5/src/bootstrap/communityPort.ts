/**
 * IM H5 community (圈子) runtime port wiring.
 *
 * Binds the shared `@sdkwork/community-mobile-react-community` package to the
 * IM host:
 *
 * - importing `@sdkwork/im-h5-community` runs its side effect that binds the
 *   IM auth session port (current-user lookup for the payment sheet);
 * - `configureCommunityRuntimePort` switches the package to the generated
 *   Community App SDK port constructed from the IM gateway base URL and the
 *   shared H5 token manager.
 */

import '@sdkwork/im-h5-community';
import { configureCommunityRuntimePort } from '@sdkwork/community-mobile-react-community';
import { getSdkClients } from './sdkClients';

let bootstrapped = false;

export function bootstrapImCommunityH5Port(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;
  configureCommunityRuntimePort(getSdkClients().communityAppSdkPort);
}

export function isImCommunityH5PortBootstrapped(): boolean {
  return bootstrapped;
}
