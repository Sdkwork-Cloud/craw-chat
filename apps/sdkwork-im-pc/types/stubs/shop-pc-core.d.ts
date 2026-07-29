declare module '@sdkwork/shop-pc-core' {
  import type { CatalogAppSdkClient } from '@sdkwork/catalog-app-sdk';
  import type { OrderAppSdkClient } from '@sdkwork/order-app-sdk';
  import type { ShopAppSdkClient } from '@sdkwork/shop-app-sdk';

  export type { CatalogAppSdkClient, OrderAppSdkClient, ShopAppSdkClient };

  export function getCatalogAppSdkClient(): CatalogAppSdkClient;
  export function getOrderAppSdkClient(): OrderAppSdkClient;
  export function getShopAppSdkClient(): ShopAppSdkClient;
  export function getShopPcTokenManager(): unknown;
  export function resetCatalogAppSdkClient(): void;
  export function resetOrderAppSdkClient(): void;
  export function resetShopAppSdkClient(): void;
  export function resetShopPcTokenManager(): void;
  export function syncShopPcTokenManagerFromRuntimeSession(tokenManager: unknown): void;
  export function configureShopPcHost(options: {
    toast(message: string, variant?: 'success' | 'error' | 'info'): void;
    sendAssistantMessage?: (
      recipientId: string,
      text: string,
      messageType?: string,
    ) => Promise<void>;
    readSessionUser(): unknown;
    languageBridge: {
      resolveInitialLanguage(): string;
      onLanguageChange(listener: (language: string) => void): () => void;
    };
  }): void;
}
