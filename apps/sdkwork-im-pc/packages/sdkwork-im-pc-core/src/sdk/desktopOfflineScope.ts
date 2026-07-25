import {
  readAppSdkSessionTokens,
  resolveAppSdkActorId,
  resolveAppSdkActorKind,
  resolveAppSdkDeploymentMode,
  resolveAppSdkEnvironment,
  resolveAppSdkOrganizationId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from './session';
import { resolveImApiBaseUrl } from './sdkBaseUrls';

export type DesktopOfflinePrincipalScope = {
  environment: 'development' | 'test' | 'staging' | 'production';
  deploymentProfile: 'standalone' | 'cloud';
  deploymentMode: 'local' | 'private' | 'saas';
  apiOrigin: string;
  tenantId: string;
  organizationId: string;
  accountId: string;
  principalKind: 'user' | 'agent' | 'system' | 'service';
  principalId: string;
};

const SUPPORTED_PRINCIPAL_KINDS = new Set<DesktopOfflinePrincipalScope['principalKind']>([
  'user',
  'agent',
  'system',
  'service',
]);

function normalizeRequired(value: string | undefined): string | undefined {
  const normalized = value?.trim();
  return normalized ? normalized : undefined;
}

function readRuntimeValue(key: string): string | undefined {
  const importMetaValue = (import.meta as ImportMeta & {
    env?: Record<string, string | boolean | undefined>;
  }).env?.[key];
  if (typeof importMetaValue === 'string' && importMetaValue.trim()) {
    return importMetaValue.trim();
  }
  const processValue = (globalThis as {
    process?: { env?: Record<string, string | undefined> };
  }).process?.env?.[key];
  return normalizeRequired(processValue);
}

function normalizeEnvironment(
  value: string | undefined,
): DesktopOfflinePrincipalScope['environment'] | undefined {
  const normalized = value?.trim().toLowerCase();
  if (normalized === 'dev') {
    return 'development';
  }
  if (normalized === 'prod') {
    return 'production';
  }
  return normalized === 'development'
    || normalized === 'test'
    || normalized === 'staging'
    || normalized === 'production'
    ? normalized
    : undefined;
}

function normalizeDeploymentProfile(
  value: string | undefined,
): DesktopOfflinePrincipalScope['deploymentProfile'] | undefined {
  const normalized = value?.trim().toLowerCase();
  return normalized === 'standalone' || normalized === 'cloud' ? normalized : undefined;
}

function normalizeDeploymentMode(
  value: string | undefined,
): DesktopOfflinePrincipalScope['deploymentMode'] | undefined {
  const normalized = value?.trim().toLowerCase();
  return normalized === 'local' || normalized === 'private' || normalized === 'saas'
    ? normalized
    : undefined;
}

function normalizeApiOrigin(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }
  try {
    const base = typeof window === 'undefined' ? undefined : window.location.origin;
    const parsed = base ? new URL(value, base) : new URL(value);
    if (
      (parsed.protocol !== 'http:' && parsed.protocol !== 'https:')
      || parsed.username
      || parsed.password
    ) {
      return undefined;
    }
    return parsed.origin;
  } catch {
    return undefined;
  }
}

export function resolveDesktopOfflinePrincipalScope(
  session: SdkworkChatSession | null = readAppSdkSessionTokens(),
): DesktopOfflinePrincipalScope | undefined {
  const environment = normalizeEnvironment(resolveAppSdkEnvironment(session));
  const deploymentProfile = normalizeDeploymentProfile(
    readRuntimeValue('VITE_SDKWORK_IM_DEPLOYMENT_PROFILE'),
  );
  const deploymentMode = normalizeDeploymentMode(resolveAppSdkDeploymentMode(session));
  const apiOrigin = normalizeApiOrigin(resolveImApiBaseUrl());
  const tenantId = normalizeRequired(resolveAppSdkTenantId(session));
  const organizationId = normalizeRequired(resolveAppSdkOrganizationId(session));
  const accountId = normalizeRequired(resolveAppSdkUserId(session));
  const actorId = normalizeRequired(resolveAppSdkActorId(session));
  const rawPrincipalKind = actorId
    ? normalizeRequired(resolveAppSdkActorKind(session))?.toLowerCase()
    : 'user';
  const principalId = actorId ?? accountId;
  if (
    !environment
    || !deploymentProfile
    || !deploymentMode
    || !apiOrigin
    || !tenantId
    || !organizationId
    || !accountId
    || !principalId
    || !rawPrincipalKind
    || !SUPPORTED_PRINCIPAL_KINDS.has(rawPrincipalKind as DesktopOfflinePrincipalScope['principalKind'])
  ) {
    return undefined;
  }
  return {
    environment,
    deploymentProfile,
    deploymentMode,
    apiOrigin,
    tenantId,
    organizationId,
    accountId,
    principalKind: rawPrincipalKind as DesktopOfflinePrincipalScope['principalKind'],
    principalId,
  };
}

export function desktopOfflineScopeKey(scope: DesktopOfflinePrincipalScope): string {
  return JSON.stringify([
    scope.environment,
    scope.deploymentProfile,
    scope.deploymentMode,
    scope.apiOrigin,
    scope.tenantId,
    scope.organizationId,
    scope.accountId,
    scope.principalKind,
    scope.principalId,
  ]);
}

export function desktopOfflineScopesEqual(
  left: DesktopOfflinePrincipalScope | undefined,
  right: DesktopOfflinePrincipalScope | undefined,
): boolean {
  if (!left || !right) {
    return left === right;
  }
  return desktopOfflineScopeKey(left) === desktopOfflineScopeKey(right);
}
