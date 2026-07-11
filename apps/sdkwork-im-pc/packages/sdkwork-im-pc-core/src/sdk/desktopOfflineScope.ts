import {
  readAppSdkSessionTokens,
  resolveAppSdkActorId,
  resolveAppSdkActorKind,
  resolveAppSdkOrganizationId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from './session';

export type DesktopOfflinePrincipalScope = {
  tenantId: string;
  organizationId: string;
  principalKind: 'user' | 'agent' | 'system' | 'service';
  principalId: string;
};

const SUPPORTED_PRINCIPAL_KINDS = new Set<DesktopOfflinePrincipalScope['principalKind']>([
  'user',
  'agent',
  'system',
  'service',
]);

export function resolveDesktopOfflinePrincipalScope(
  session: SdkworkChatSession | null = readAppSdkSessionTokens(),
): DesktopOfflinePrincipalScope | undefined {
  const tenantId = resolveAppSdkTenantId(session);
  const organizationId = resolveAppSdkOrganizationId(session) ?? '0';
  const principalId = resolveAppSdkActorId(session) ?? resolveAppSdkUserId(session);
  const rawPrincipalKind = (resolveAppSdkActorKind(session) ?? 'user').trim().toLowerCase();
  if (
    !tenantId
    || !organizationId
    || !principalId
    || !SUPPORTED_PRINCIPAL_KINDS.has(rawPrincipalKind as DesktopOfflinePrincipalScope['principalKind'])
  ) {
    return undefined;
  }
  return {
    tenantId,
    organizationId,
    principalKind: rawPrincipalKind as DesktopOfflinePrincipalScope['principalKind'],
    principalId,
  };
}

export function desktopOfflineScopesEqual(
  left: DesktopOfflinePrincipalScope | undefined,
  right: DesktopOfflinePrincipalScope | undefined,
): boolean {
  return left?.tenantId === right?.tenantId
    && left?.organizationId === right?.organizationId
    && left?.principalKind === right?.principalKind
    && left?.principalId === right?.principalId;
}
