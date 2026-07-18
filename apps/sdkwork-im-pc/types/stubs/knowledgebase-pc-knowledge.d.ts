declare module '@sdkwork/knowledgebase-pc-knowledge' {
  import type { ComponentType } from 'react';

  export interface KnowledgebasePcSdkPorts {
    getKnowledgebaseClient: () => unknown;
    getDriveClient: () => unknown;
    readHostSession: () => unknown;
    subscribeHostSession?: (listener: () => void) => () => void;
    resolveHostLanguage?: () => string;
    subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
  }

  export type KnowledgebaseHostPresentationMode = 'inline' | 'detached-iframe' | 'detached-window';
  export interface KnowledgebaseHostSurfaceProps {
    presentationMode: KnowledgebaseHostPresentationMode;
    title?: string;
    context?: { groupId?: string; groupName?: string };
  }
  export const KnowledgebaseHostSurface: ComponentType<KnowledgebaseHostSurfaceProps>;
  export function resolveKnowledgebaseHostPresentationMode(): KnowledgebaseHostPresentationMode;
  export function configureKnowledgebasePcRuntime(options: { sdkPorts: KnowledgebasePcSdkPorts }): void;

  export const knowledgeSelectionService: {
    getBases(): Promise<Array<Record<string, unknown>>>;
  };
}
