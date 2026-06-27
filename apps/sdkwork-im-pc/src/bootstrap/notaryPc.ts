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
    toast(message, variant = 'info') {
      imToast(message, variant);
    },
    openExternalUrl(url) {
      const safeUrl = sanitizeMessageLinkHref(url);
      if (safeUrl) {
        window.open(safeUrl, '_blank', 'noopener,noreferrer');
      }
    },
    createDefaultAvatar(seed: string) {
      return createDefaultAvatar(seed === 'user' ? 'user' : 'user');
    },
    CallOverlay: CallOverlay as ComponentType<NotaryCallOverlayProps>,
    MediaViewer: MediaViewer as ComponentType<NotaryMediaPreviewProps>,
    sanitizeLinkHref(url) {
      return sanitizeMessageLinkHref(url) ?? '';
    },
    cn,
    resolveInitialLanguage: hostLanguageBridge.resolveInitialLanguage,
    onLanguageChange: hostLanguageBridge.onLanguageChange,
  };
}

export function bootstrapImNotaryPcIntegration(): void {
  bootstrapNotaryPcForIm(createImNotaryPcHostAdapter());
}
