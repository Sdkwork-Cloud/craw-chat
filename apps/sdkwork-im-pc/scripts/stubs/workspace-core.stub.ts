export interface SdkworkImAppClient {
  portal: {
    home: {
      retrieve(): Promise<unknown>;
    };
  };
}

export interface SdkworkDriveAppClient {
  drive: {
    recent: {
      list(params: { pageSize: string }): Promise<unknown>;
    };
  };
}

export function getAppSdkClientWithSession(): SdkworkImAppClient {
  throw new Error('workspace service test must inject the app SDK client');
}

export function getDriveAppSdkClientWithSession(): SdkworkDriveAppClient {
  throw new Error('workspace service test must inject the Drive SDK client');
}

export async function retrievePortalHome<TClient extends SdkworkImAppClient>(client: TClient): Promise<unknown> {
  return client.portal.home.retrieve();
}

interface WorkspaceTestSessionScope {
  tenantId?: string;
  userId?: string;
}

function readWorkspaceTestSessionScope(): WorkspaceTestSessionScope | undefined {
  return (globalThis as typeof globalThis & {
    __workspaceTestSessionScope?: WorkspaceTestSessionScope;
  }).__workspaceTestSessionScope;
}

export function readAppSdkSessionTokens(): { context: WorkspaceTestSessionScope } | undefined {
  const scope = readWorkspaceTestSessionScope();
  return scope ? { context: scope } : undefined;
}

export function resolveAppSdkTenantId(
  session = readAppSdkSessionTokens(),
): string | undefined {
  return session?.context.tenantId;
}

export function resolveAppSdkUserId(
  session = readAppSdkSessionTokens(),
): string | undefined {
  return session?.context.userId;
}
