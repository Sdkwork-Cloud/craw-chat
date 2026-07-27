import {
  createDriveAppClient,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core';

let sessionAwareDriveClient: SdkworkDriveAppClient | null = null;

function resolveSessionTokens(): {
  accessToken?: string;
  authToken?: string;
} {
  if (typeof localStorage === 'undefined') {
    return {};
  }

  try {
    const raw = localStorage.getItem('sdkwork-im-h5-session');
    if (!raw) {
      return {};
    }
    const parsed = JSON.parse(raw) as {
      accessToken?: string;
      authToken?: string;
    };
    return {
      accessToken: parsed.accessToken,
      authToken: parsed.authToken,
    };
  } catch {
    return {};
  }
}

export function getDriveAppSdkClientWithSession(): SdkworkDriveAppClient {
  if (sessionAwareDriveClient) {
    return sessionAwareDriveClient;
  }

  const session = resolveSessionTokens();
  sessionAwareDriveClient = createDriveAppClient({
    accessToken: session.accessToken,
    authToken: session.authToken,
    platform: 'h5',
  });

  return sessionAwareDriveClient;
}

export function resetDriveAppSdkClientWithSession(): void {
  sessionAwareDriveClient = null;
}
