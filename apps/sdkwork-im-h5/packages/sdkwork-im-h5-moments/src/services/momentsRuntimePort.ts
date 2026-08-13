import type { SdkworkCommunityAppSdkPort } from "@sdkwork/community-sdk-ports";
import type { SdkworkFeedsOpenClient } from "@sdkwork/feeds-sdk";

/**
 * Moments runtime port binding.
 *
 * The moments feature consumes two injected ports:
 * - the generated Community App SDK port for content operations (publish,
 *   like, comment, delete — content stays owned by the community module);
 * - the standard feeds stream open client for the moments feed read path
 *   (`streams.items.list` on the `moments-global` stream, cursor paging).
 *
 * The feeds port is optional for migration: while it is not bound, the feed
 * read path degrades to the legacy community `feed.list` surface. Without
 * either binding every service call fails closed with a typed error.
 */
export class MomentCapabilityUnavailableError extends Error {
  constructor() {
    super(
      "Moments capability is not bound. Call configureMomentsRuntimePort from the host bootstrap first.",
    );
    this.name = "MomentCapabilityUnavailableError";
  }
}

let communityPort: SdkworkCommunityAppSdkPort | null = null;
let feedsPort: SdkworkFeedsOpenClient | null = null;
let bootstrapped = false;

export function configureMomentsRuntimePort(port: SdkworkCommunityAppSdkPort): void {
  communityPort = port;
  bootstrapped = true;
}

/** Binds the standard feeds stream client (optional; fallback path uses community feed). */
export function configureMomentsFeedsPort(port: SdkworkFeedsOpenClient): void {
  feedsPort = port;
  bootstrapped = true;
}

export function resetMomentsRuntimePort(): void {
  communityPort = null;
  feedsPort = null;
  bootstrapped = false;
}

export function isMomentsRuntimePortConfigured(): boolean {
  return bootstrapped && communityPort !== null;
}

export function isMomentsFeedsPortConfigured(): boolean {
  return bootstrapped && feedsPort !== null;
}

export function getMomentsRuntimePort(): SdkworkCommunityAppSdkPort {
  if (!bootstrapped || !communityPort) {
    throw new MomentCapabilityUnavailableError();
  }
  return communityPort;
}

export function getMomentsFeedsPort(): SdkworkFeedsOpenClient {
  if (!bootstrapped || !feedsPort) {
    throw new MomentCapabilityUnavailableError();
  }
  return feedsPort;
}
