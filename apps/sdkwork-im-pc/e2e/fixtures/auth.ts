export const SESSION_STORAGE_KEY = 'sdkwork-im-pc:session:v1';

const SDKWORK_TOKEN_VERSION = 1;

function createPlaywrightTestJwt(claims: Record<string, unknown>): string {
  const payload = {
    token_version: SDKWORK_TOKEN_VERSION,
    ...claims,
  };
  const header = Buffer.from(JSON.stringify({ alg: 'none', typ: 'JWT' })).toString('base64url');
  const encodedPayload = Buffer.from(JSON.stringify(payload)).toString('base64url');
  return `${header}.${encodedPayload}.signature`;
}

export interface PlaywrightSessionFixture {
  accessToken: string;
  authToken: string;
  refreshToken?: string;
  expiresAt: number;
  sessionId: string;
  context: {
    appId: string;
    authLevel: string;
    deploymentMode: string;
    environment: string;
    organizationId: string;
    sessionId: string;
    tenantId: string;
    userId: string;
    dataScope: string[];
    permissionScope: string[];
  };
  user: {
    id: string;
    userId: string;
    displayName: string;
  };
}

export function buildPlaywrightSessionFixture(
  overrides: Partial<PlaywrightSessionFixture> = {},
): PlaywrightSessionFixture {
  const expiresAt = Date.now() + 60 * 60 * 1000;
  const tenantId = '100001';
  const userId = '1';
  const appId = 'sdkwork-im-pc';
  return {
    accessToken: createPlaywrightTestJwt({
      tenant_id: tenantId,
      user_id: userId,
      app_id: appId,
      marker: 'access',
    }),
    authToken: createPlaywrightTestJwt({
      tenant_id: tenantId,
      user_id: userId,
      app_id: appId,
      auth_level: 'password',
      marker: 'auth',
    }),
    refreshToken: createPlaywrightTestJwt({
      tenant_id: tenantId,
      user_id: userId,
      app_id: appId,
      marker: 'refresh',
    }),
    expiresAt,
    sessionId: 'session.playwright.1',
    context: {
      appId,
      authLevel: 'password',
      deploymentMode: 'saas',
      environment: 'dev',
      organizationId: '0',
      sessionId: 'session.playwright.1',
      tenantId,
      userId,
      dataScope: [`tenant:${tenantId}`],
      permissionScope: ['*'],
    },
    user: {
      id: userId,
      userId,
      displayName: 'Playwright User',
    },
    ...overrides,
  };
}

export function buildPlaywrightSessionResponse(session: PlaywrightSessionFixture) {
  return {
    code: '0',
    message: 'ok',
    requestId: 'playwright.request.1',
    data: {
      accessToken: session.accessToken,
      authToken: session.authToken,
      refreshToken: session.refreshToken,
      expiresAt: session.expiresAt,
      sessionId: session.sessionId,
      context: session.context,
      user: session.user,
    },
  };
}
