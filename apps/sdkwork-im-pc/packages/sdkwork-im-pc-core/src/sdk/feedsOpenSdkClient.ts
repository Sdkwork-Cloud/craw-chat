import {
  createClient as createFeedsOpenClient,
  type SdkworkCustomClient as SdkworkFeedsOpenClient,
} from '@sdkwork/feeds-sdk';

import { resolveAppSdkBaseUrl } from './appSdkClient';

let feedsOpenSdkClient: SdkworkFeedsOpenClient | null = null;

/**
 * Standard feeds stream client (anonymous open-surface reads). Circle feeds
 * and moments read through the feeds stream system (`community-{circleId}` /
 * `moments-global` streams) served by the IM gateway origin; content write
 * operations keep using the community App SDK port.
 */
export function createFeedsOpenSdkClient(): SdkworkFeedsOpenClient {
  feedsOpenSdkClient = createFeedsOpenClient({
    baseUrl: resolveAppSdkBaseUrl(),
    platform: 'pc',
  });
  return feedsOpenSdkClient;
}

export function getFeedsOpenSdkClient(): SdkworkFeedsOpenClient {
  return feedsOpenSdkClient ?? createFeedsOpenSdkClient();
}

export function resetFeedsOpenSdkClient(): void {
  feedsOpenSdkClient = null;
}
