import assert from 'node:assert/strict';
import {
  createSdkworkChatRequestContext,
  createSdkworkChatRequestContextInterceptors,
  resolveAppSdkOrganizationId,
  resolveAppSdkSessionId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts';

function base64UrlEncode(value: string): string {
  return Buffer.from(value, 'utf8')
    .toString('base64')
    .replace(/\+/gu, '-')
    .replace(/\//gu, '_')
    .replace(/=+$/u, '');
}

function createJwt(payload: Record<string, unknown>): string {
  const header = base64UrlEncode(JSON.stringify({ alg: 'none', typ: 'JWT' }));
  const body = base64UrlEncode(JSON.stringify(payload));
  return `${header}.${body}.local`;
}

async function main(): Promise<void> {
  const session: SdkworkChatSession = {
    accessToken: createJwt({
      app_id: 'sdkwork-im-pc',
      tenant_id: 'tenant-from-access-token',
      organization_id: 'org-from-access-token',
      user_id: 'user-from-access-token',
      sid: 'session-from-access-token',
      environment: 'prod',
      deployment_mode: 'private',
      auth_level: 'mfa',
      data_scope: ['iam.organization.read', 'iam.department.read'],
      permission_scope: ['iam.member.manage'],
      actor_id: 'actor-from-access-token',
      actor_kind: 'user',
      device_id: 'device-from-access-token',
    }),
    authToken: createJwt({
      appId: 'sdkwork-im-pc-auth',
      tenantId: 'tenant-from-auth-token',
      organizationId: 'org-from-auth-token',
      userId: 'user-from-auth-token',
      sessionId: 'session-from-auth-token',
    }),
    context: {
      appId: 'stale-session-context',
      tenantId: 'stale-tenant',
      organizationId: 'stale-org',
      userId: 'stale-user',
      sessionId: 'stale-session',
      environment: 'dev',
      deploymentMode: 'saas',
      authLevel: 'password',
      dataScope: [],
      permissionScope: [],
    },
  };

  assert.equal(
    resolveAppSdkTenantId(session),
    'tenant-from-access-token',
    'request context tenant id must come from token claims instead of stale session params',
  );
  assert.equal(
    resolveAppSdkOrganizationId(session),
    'org-from-access-token',
    'request context organization id must come from token claims instead of stale session params',
  );
  assert.equal(
    resolveAppSdkUserId(session),
    'user-from-access-token',
    'request context user id must come from token claims when present',
  );
  assert.equal(
    resolveAppSdkSessionId(session),
    'session-from-access-token',
    'request context session id must come from token claims when present',
  );

  assert.deepEqual(
    createSdkworkChatRequestContext(session),
    {
      appId: 'sdkwork-im-pc',
      tenantId: 'tenant-from-access-token',
      organizationId: 'org-from-access-token',
      userId: 'user-from-access-token',
      sessionId: 'session-from-access-token',
      environment: 'prod',
      deploymentMode: 'private',
      authLevel: 'mfa',
      dataScope: ['iam.organization.read', 'iam.department.read'],
      permissionScope: ['iam.member.manage'],
      actorId: 'actor-from-access-token',
      actorKind: 'user',
      deviceId: 'device-from-access-token',
    },
    'shared SDK wrapper must expose token-derived request context for UI state only',
  );

  const [contextInterceptor] = createSdkworkChatRequestContextInterceptors(session).request;
  assert.deepEqual(
    await contextInterceptor({
      url: '/app/v3/api/iam/departments',
      method: 'GET',
      headers: {
        'X-Feature-Header': 'keep-me',
      },
    }),
    {
      url: '/app/v3/api/iam/departments',
      method: 'GET',
      headers: {
        'X-Feature-Header': 'keep-me',
      },
    },
    'request interceptor must not synthesize app context headers; auth is owned by the shared TokenManager',
  );

  assert.deepEqual(
    createSdkworkChatRequestContext({
      authToken: createJwt({
        appId: 'sdkwork-im-pc',
        tenantId: 'tenant-from-auth-token',
        organizationId: 'org-from-auth-token',
        sub: 'user-from-subject',
        sessionId: 'session-from-auth-token',
      }),
    }),
    {
      appId: 'sdkwork-im-pc',
      tenantId: 'tenant-from-auth-token',
      organizationId: 'org-from-auth-token',
      userId: 'user-from-subject',
      sessionId: 'session-from-auth-token',
    },
    'auth token claims remain available to local UI context readers when access token claims are absent',
  );

  console.log('sdkwork-im-pc request context contract passed');
}

void main();
