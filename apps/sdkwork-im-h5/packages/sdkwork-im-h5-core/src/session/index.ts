import type { AuthTokenManager } from '@sdkwork/sdk-common';

export interface ImH5SessionUser {
  id: string;
  name: string;
  avatar?: string;
  status?: "online" | "offline" | "busy";
}

export interface ImH5SessionPort {
  readonly tokenManager: AuthTokenManager;
}
