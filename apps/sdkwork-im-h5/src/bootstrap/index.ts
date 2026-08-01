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
import { initSdkClients, resetSdkClients } from './sdkClients';
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
  configureOrderMobileRuntime({ client: sdkClients.orderAppSdkClient });

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
