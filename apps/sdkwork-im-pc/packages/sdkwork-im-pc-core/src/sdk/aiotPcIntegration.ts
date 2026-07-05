import { persistPcReactRuntimeSession } from '@sdkwork/core-pc-react';
import {
  getAiotAppSdkClient,
  getAiotPcTokenManager,
  resetAiotAppSdkClient,
  syncPcTokenManagerFromRuntimeSession,
} from '@sdkwork/aiot-pc-core';
import type { SdkworkAiotAppClient } from '@sdkwork/aiot-app-sdk';

import {
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
} from './session';

let aiotPcRuntimeBootstrapped = false;
let aiotSessionListenerRegistered = false;

export function syncImSessionToAiotPc(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetAiotAppSdkClient();
    return;
  }

  persistPcReactRuntimeSession({
    authToken: resolveAppSdkAuthToken(session),
    accessToken: resolveAppSdkAccessToken(session),
    refreshToken: session.refreshToken,
  });
  syncPcTokenManagerFromRuntimeSession(getAiotPcTokenManager());
  resetAiotAppSdkClient();
}

export function getImHostedAiotAppSdkClient(): SdkworkAiotAppClient {
  syncImSessionToAiotPc();
  return getAiotAppSdkClient();
}

export function bootstrapAiotPcForIm(): void {
  syncImSessionToAiotPc();
  aiotPcRuntimeBootstrapped = true;

  if (!aiotSessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToAiotPc();
    });
    aiotSessionListenerRegistered = true;
  }
}

export function isAiotPcRuntimeBootstrapped(): boolean {
  return aiotPcRuntimeBootstrapped;
}

export function resetAiotPcIntegration(): void {
  aiotPcRuntimeBootstrapped = false;
  resetAiotAppSdkClient();
}
