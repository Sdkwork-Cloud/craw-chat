import { configureShopPcHost } from '@sdkwork/shop-pc-core/host';
import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';
import { readAppSdkSessionTokens } from '@sdkwork/im-pc-core/sdk/session';

export interface BootstrapImShopPcHostOptions {
  toast(message: string, variant?: 'success' | 'error' | 'info'): void;
  sendAssistantMessage?(
    recipientId: string,
    text: string,
    messageType?: string,
  ): Promise<void>;
}

let shopPcHostBootstrapped = false;

export function bootstrapImShopPcHost(options: BootstrapImShopPcHostOptions): void {
  const languageBridge = createImPcHostLanguageBridge();
  configureShopPcHost({
    toast: options.toast,
    sendAssistantMessage: options.sendAssistantMessage,
    readSessionUser: () => readAppSdkSessionTokens()?.user ?? null,
    languageBridge: {
      resolveInitialLanguage: languageBridge.resolveInitialLanguage,
      onLanguageChange: languageBridge.onLanguageChange,
    },
  });
  shopPcHostBootstrapped = true;
}

export function isImShopPcHostBootstrapped(): boolean {
  return shopPcHostBootstrapped;
}

export function resetImShopPcHostBootstrap(): void {
  shopPcHostBootstrapped = false;
}
