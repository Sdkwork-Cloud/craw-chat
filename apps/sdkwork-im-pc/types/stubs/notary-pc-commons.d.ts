declare module '@sdkwork/notary-pc-commons' {
  import type { ComponentType } from 'react';

  export interface NotaryDocument {
    id: string;
  }

  export interface NotaryTask {
    id: string;
  }

  export interface Party {
    id: string;
  }

  export interface TimelineEvent {
    id: string;
  }

  export interface NotaryCallOverlayProps {
    [key: string]: unknown;
  }

  export interface NotaryMediaPreviewProps {
    [key: string]: unknown;
  }

  export interface NotaryPcHostAdapter {
    toast(message: string, variant?: string): void;
    openExternalUrl(url: string): void;
    createDefaultAvatar(seed: string): string;
    CallOverlay: ComponentType<NotaryCallOverlayProps>;
    MediaViewer: ComponentType<NotaryMediaPreviewProps>;
    sanitizeLinkHref(url: string): string;
    cn: (...values: unknown[]) => string;
    resolveInitialLanguage?: () => string;
    onLanguageChange?: (listener: (language: string) => void) => () => void;
  }
}
