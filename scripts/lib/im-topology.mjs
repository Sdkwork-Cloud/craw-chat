import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const REPO_ROOT = path.resolve(__dirname, '..', '..');
export const SPEC_PATH = path.join(REPO_ROOT, 'specs/topology.spec.json');
export const IAM_REPO_ROOT = path.resolve(REPO_ROOT, '..', 'sdkwork-iam');

export const IAM_APPLICATION_BOOTSTRAP_ENV = {
  SDKWORK_APP_ROOT: REPO_ROOT,
  SDKWORK_IM_APP_ROOT: REPO_ROOT,
  SDKWORK_IAM_APP_ROOT: IAM_REPO_ROOT,
};

const APP_TOPOLOGY_ENTRY_RELATIVE = path.join('tools', 'topology', 'lib', 'index.mjs');

function resolveAppTopologyEntryPath() {
  const installedEntry = path.join(
    REPO_ROOT,
    'node_modules',
    '@sdkwork',
    'app-topology',
    APP_TOPOLOGY_ENTRY_RELATIVE,
  );
  if (fs.existsSync(installedEntry)) {
    return installedEntry;
  }

  const siblingEntry = path.join(REPO_ROOT, '..', 'sdkwork-app-topology', APP_TOPOLOGY_ENTRY_RELATIVE);
  if (fs.existsSync(siblingEntry)) {
    return siblingEntry;
  }

  throw new Error(
    'Missing @sdkwork/app-topology. Clone ../sdkwork-app-topology next to sdkwork-im or run: pnpm install -w',
  );
}

const {
  buildProfileId,
  createTopologyRuntime,
  isTcpPortReachable,
  loadTopologySpec,
  normalizeText,
  waitForHttpHealthy,
} = await import(pathToFileURL(resolveAppTopologyEntryPath()).href);

const spec = loadTopologySpec(SPEC_PATH);
const runtime = createTopologyRuntime(spec, REPO_ROOT);

export const VALID_DEPLOYMENT_PROFILES = runtime.deploymentProfileValues;
export const VALID_SERVICE_LAYOUTS = runtime.serviceLayoutValues;
export const VALID_ENVIRONMENTS = runtime.environmentValues;
export const DEFAULT_DEV_PROFILE_ID = runtime.defaults.developmentProfileId;
export const DEFAULT_BUILD_PROFILE_ID = runtime.defaults.productionProfileId;
export const DEFAULT_STANDALONE_BUILD_PROFILE_ID = 'standalone.unified-process.production';
export const DEFAULT_GATEWAY_BIND = runtime.defaults.gatewayBind ?? '127.0.0.1:18079';
export const POSTGRES_REACHABILITY_TIMEOUT_MS = 2000;

export const APPLICATION_PUBLIC_INGRESS_PACKAGE_PROFILE = 'standalone';
export const PLATFORM_CONFIG_BUNDLE_PROFILE = 'platform-config-bundle';
export const GATEWAY_PACKAGE_TARGETS = runtime.listPackageTargets?.() ?? spec.packaging?.targets ?? [];
export const APPLICATION_PUBLIC_INGRESS_PACKAGE_TARGETS = GATEWAY_PACKAGE_TARGETS.filter(
  (target) => target.profile === APPLICATION_PUBLIC_INGRESS_PACKAGE_PROFILE
    || target.surface === 'application.public-ingress',
);
export const IM_CLOUD_GATEWAY_CONFIGS = spec.packaging?.cloudConfigFiles ?? [];

export function resolveDevProfileId(deploymentProfile, serviceLayout = 'unified-process') {
  runtime.assertDeploymentProfile(deploymentProfile);
  runtime.assertServiceLayout(serviceLayout);
  return buildProfileId(deploymentProfile, serviceLayout, 'development');
}

export function resolveBuildProfileId(deploymentProfile) {
  runtime.assertDeploymentProfile(deploymentProfile);
  if (deploymentProfile === 'standalone') {
    return DEFAULT_STANDALONE_BUILD_PROFILE_ID;
  }
  return DEFAULT_BUILD_PROFILE_ID;
}

export const loadProfile = runtime.loadProfile;
export const applyProfileEnv = runtime.applyProfileEnv;
export const mergeRuntimeEnv = runtime.mergeRuntimeEnv;
export const loadEnvFile = runtime.loadEnvFile;
export const assertDeploymentProfile = runtime.assertDeploymentProfile;
export const assertServiceLayout = runtime.assertServiceLayout;
export const resolveSurfaceHttpUrl = runtime.resolveSurfaceHttpUrl.bind(runtime);
export const resolveSurfaceWebsocketOrigin = runtime.resolveSurfaceWebsocketOrigin.bind(runtime);
export const resolveSurfaceBind = runtime.resolveSurfaceBind.bind(runtime);
export const shouldAutostartGateway = runtime.shouldAutostartGateway;
export const resolveGatewayBind = runtime.resolveGatewayBind;
export const resolveGatewayBaseUrl = runtime.resolveGatewayBaseUrl;
export const resolveIamDevEnv = runtime.resolveIamDevEnv;
export const assertPostgresReachableForIam = runtime.assertPostgresReachableForIam;
export const describeIamDatabaseTarget = runtime.describeIamDatabaseTarget;
export const listOrchestrationProcesses = runtime.listOrchestrationProcesses;
export const listHealthSurfaces = runtime.listHealthSurfaces;
export const resolveStandaloneGatewayConfigPath = runtime.resolveStandaloneGatewayConfigPath;
export const resolveCloudGatewayConfigPath = runtime.resolveCloudGatewayConfigPath;

export function findGatewayPackageTarget(targetId) {
  return runtime.findPackageTarget?.(targetId);
}

export function listGatewayPackageTargets(profile) {
  return runtime.listPackageTargetsByProfile?.(profile) ?? [];
}

export { buildProfileId, normalizeText, isTcpPortReachable, waitForHttpHealthy, spec, runtime };
