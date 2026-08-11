import type { SdkworkCommunityAppSdkPort } from "@sdkwork/community-sdk-ports";

/**
 * Moments runtime port binding.
 *
 * The moments feature consumes the generated Community App SDK port injected
 * by the host bootstrap (`configureMomentsRuntimePort`). The package never
 * constructs SDK clients or raw HTTP; without a bound port every service call
 * fails closed with a typed error.
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
let bootstrapped = false;

export function configureMomentsRuntimePort(port: SdkworkCommunityAppSdkPort): void {
  communityPort = port;
  bootstrapped = true;
}

export function resetMomentsRuntimePort(): void {
  communityPort = null;
  bootstrapped = false;
}

export function isMomentsRuntimePortConfigured(): boolean {
  return bootstrapped && communityPort !== null;
}

export function getMomentsRuntimePort(): SdkworkCommunityAppSdkPort {
  if (!bootstrapped || !communityPort) {
    throw new MomentCapabilityUnavailableError();
  }
  return communityPort;
}
