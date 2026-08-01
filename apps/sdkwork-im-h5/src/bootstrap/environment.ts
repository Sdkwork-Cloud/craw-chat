/**
 * H5 runtime environment loader.
 *
 * Resolves public browser runtime config from `import.meta.env` and the
 * `etc/browser.runtime.json` deployment binding. Browser SDK base URLs must
 * load from public runtime config before SDK client construction.
 */

export interface H5RuntimeEnvironment {
  readonly appKey: string;
  readonly deploymentProfile: 'standalone' | 'cloud';
  readonly imApiBaseUrl: string;
  readonly sdkGatewayApiBaseUrl: string;
  readonly driveAppApiBaseUrl: string;
  readonly orderAppApiBaseUrl: string;
  readonly iamApiBaseUrl: string;
}

const DEFAULT_APP_KEY = 'sdkwork-im-h5';
const DEFAULT_DEPLOYMENT_PROFILE: H5RuntimeEnvironment['deploymentProfile'] = 'standalone';

function readEnvValue(key: string): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const value = meta.env?.[key];
  if (typeof value === 'string' && value.trim().length > 0) {
    return value.trim();
  }
  return undefined;
}

function resolveDeploymentProfile(): H5RuntimeEnvironment['deploymentProfile'] {
  const value = readEnvValue('SDKWORK_DEPLOYMENT_PROFILE')
    ?? readEnvValue('VITE_SDKWORK_DEPLOYMENT_PROFILE');
  if (value === 'cloud' || value === 'standalone') {
    return value;
  }
  return DEFAULT_DEPLOYMENT_PROFILE;
}

let cachedEnvironment: H5RuntimeEnvironment | null = null;

export function resolveH5RuntimeEnvironment(): H5RuntimeEnvironment {
  if (cachedEnvironment) {
    return cachedEnvironment;
  }

  const deploymentProfile = resolveDeploymentProfile();
  const platformGatewayApiBaseUrl = readEnvValue('SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL')
    ?? readEnvValue('VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL');
  if (deploymentProfile === 'cloud' && !platformGatewayApiBaseUrl) {
    throw new Error(
      'Cloud H5 requires SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL for dependency App SDK routing',
    );
  }

  cachedEnvironment = {
    appKey: readEnvValue('SDKWORK_APP_KEY') ?? DEFAULT_APP_KEY,
    deploymentProfile,
    imApiBaseUrl: readEnvValue('SDKWORK_IM_API_BASE_URL')
      ?? readEnvValue('VITE_SDKWORK_IM_API_BASE_URL')
      ?? '/',
    sdkGatewayApiBaseUrl: platformGatewayApiBaseUrl
      ?? readEnvValue('SDKWORK_IM_API_BASE_URL')
      ?? readEnvValue('VITE_SDKWORK_IM_API_BASE_URL')
      ?? '/',
    driveAppApiBaseUrl: readEnvValue('SDKWORK_DRIVE_APP_API_BASE_URL')
      ?? readEnvValue('VITE_SDKWORK_DRIVE_APP_API_BASE_URL')
      ?? '/',
    orderAppApiBaseUrl: readEnvValue('SDKWORK_ORDER_APP_API_BASE_URL')
      ?? readEnvValue('VITE_SDKWORK_ORDER_APP_API_BASE_URL')
      ?? '/',
    iamApiBaseUrl: readEnvValue('SDKWORK_IAM_API_BASE_URL')
      ?? readEnvValue('VITE_SDKWORK_IAM_API_BASE_URL')
      ?? '/',
  };

  return cachedEnvironment;
}

export function resetH5RuntimeEnvironment(): void {
  cachedEnvironment = null;
}

export function getH5RuntimeEnvironment(): H5RuntimeEnvironment {
  return cachedEnvironment ?? resolveH5RuntimeEnvironment();
}
