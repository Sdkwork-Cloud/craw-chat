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

  export const KnowledgeView: ComponentType<unknown>;
  export function configureKnowledgebasePcRuntime(options: { sdkPorts: KnowledgebasePcSdkPorts }): void;

  export const knowledgeSelectionService: {
    getBases(): Promise<Array<Record<string, unknown>>>;
  };
}
