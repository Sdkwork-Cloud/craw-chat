declare module '@sdkwork/aiot-pc-core' {
  import type { SdkworkAiotAppClient } from '@sdkwork/aiot-app-sdk';

  export function getAiotAppSdkClient(): SdkworkAiotAppClient;
  export function getAiotPcTokenManager(): unknown;
  export function resetAiotAppSdkClient(): void;
  export function syncPcTokenManagerFromRuntimeSession(tokenManager: unknown): void;
}
