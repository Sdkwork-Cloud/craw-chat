import { persistPcReactRuntimeSession } from '@sdkwork/core-pc-react';
import {
  getCatalogAppSdkClient,
  getOrderAppSdkClient,
  getShopAppSdkClient,
  getShopPcTokenManager,
  resetCatalogAppSdkClient,
  resetOrderAppSdkClient,
  resetShopAppSdkClient,
  resetShopPcTokenManager,
  syncShopPcTokenManagerFromRuntimeSession,
} from '@sdkwork/shop-pc-core';
import type { CatalogAppSdkClient, OrderAppSdkClient, ShopAppSdkClient } from '@sdkwork/shop-pc-core/sdk';

import {
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
} from './session';

let commercePcRuntimeBootstrapped = false;
let commerceSessionListenerRegistered = false;

export function syncImSessionToCommercePc(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetCatalogAppSdkClient();
    resetOrderAppSdkClient();
    resetShopAppSdkClient();
    resetShopPcTokenManager();
    return;
  }

  persistPcReactRuntimeSession({
    authToken: resolveAppSdkAuthToken(session),
    accessToken: resolveAppSdkAccessToken(session),
    refreshToken: session.refreshToken,
  });
  syncShopPcTokenManagerFromRuntimeSession(getShopPcTokenManager());
  resetCatalogAppSdkClient();
  resetOrderAppSdkClient();
  resetShopAppSdkClient();
}

export function getImHostedCatalogAppSdkClient(): CatalogAppSdkClient {
  syncImSessionToCommercePc();
  return getCatalogAppSdkClient();
}

export function getImHostedOrderAppSdkClient(): OrderAppSdkClient {
  syncImSessionToCommercePc();
  return getOrderAppSdkClient();
}

export function getImHostedShopAppSdkClient(): ShopAppSdkClient {
  syncImSessionToCommercePc();
  return getShopAppSdkClient();
}

export function bootstrapCommercePcForIm(): void {
  syncImSessionToCommercePc();
  commercePcRuntimeBootstrapped = true;

  if (!commerceSessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToCommercePc();
    });
    commerceSessionListenerRegistered = true;
  }
}

export function isCommercePcRuntimeBootstrapped(): boolean {
  return commercePcRuntimeBootstrapped;
}

export function resetCommercePcIntegration(): void {
  commercePcRuntimeBootstrapped = false;
  resetCatalogAppSdkClient();
  resetOrderAppSdkClient();
  resetShopAppSdkClient();
  resetShopPcTokenManager();
}
