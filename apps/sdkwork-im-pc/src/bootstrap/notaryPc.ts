import type { NotaryPcHostAdapter, NotaryCallOverlayProps, NotaryMediaPreviewProps } from '@sdkwork/notary-pc-commons';
import type { ComponentType } from 'react';
import { CallOverlay, createDefaultAvatar, toast as imToast } from '@sdkwork/im-pc-chat';
import {
  cn,
  createImPcHostLanguageBridge,
  MediaViewer,
  sanitizeMessageLinkHref,
} from '@sdkwork/im-pc-commons';
import { bootstrapNotaryPcForIm } from '@sdkwork/im-pc-core';

const hostLanguageBridge = createImPcHostLanguageBridge();

function createImNotaryPcHostAdapter(): NotaryPcHostAdapter {
  return {
    toast(message: string, variant = 'info') {
      imToast(message, variant as 'info');
    },
    openExternalUrl(url: string) {
      const safeUrl = sanitizeMessageLinkHref(url);
      if (safeUrl) {
        window.open(safeUrl, '_blank', 'noopener,noreferrer');
      }
    },
    createDefaultAvatar(seed: string) {
      return createDefaultAvatar(seed === 'user' ? 'user' : 'user');
    },
    CallOverlay: CallOverlay as unknown as ComponentType<NotaryCallOverlayProps>,
    MediaViewer: MediaViewer as unknown as ComponentType<NotaryMediaPreviewProps>,
    sanitizeLinkHref(url: string) {
      return sanitizeMessageLinkHref(url) ?? '';
    },
    cn: (...values: unknown[]) => cn(...(values as Parameters<typeof cn>)),
    resolveInitialLanguage: hostLanguageBridge.resolveInitialLanguage,
    onLanguageChange: hostLanguageBridge.onLanguageChange,
  };
}

export function bootstrapImNotaryPcIntegration(): Promise<void> {
  return bootstrapNotaryPcForIm(createImNotaryPcHostAdapter());
}
