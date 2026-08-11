/**
 * IM H5 moments (朋友圈) runtime port wiring.
 *
 * Binds the moments feature package to the generated Community App SDK port
 * constructed from the IM gateway base URL and the shared H5 token manager
 * (same port the 圈子 capability consumes). Without this binding the moments
 * pages fail closed with `MomentCapabilityUnavailableError`.
 */

import {
  configureMomentsRuntimePort,
  isMomentsRuntimePortConfigured,
} from '@sdkwork/im-h5-moments';
import { getSdkClients } from './sdkClients';

let bootstrapped = false;

export function bootstrapImMomentsH5Port(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;

  configureMomentsRuntimePort(getSdkClients().communityAppSdkPort);
}

export function isImMomentsH5PortBootstrapped(): boolean {
  return bootstrapped && isMomentsRuntimePortConfigured();
}
