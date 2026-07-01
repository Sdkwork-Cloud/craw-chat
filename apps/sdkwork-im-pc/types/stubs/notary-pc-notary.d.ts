declare module '@sdkwork/notary-pc-notary' {
  import type { ComponentType } from 'react';
  import type { NotaryPcHostAdapter } from '@sdkwork/notary-pc-commons';

  export const NotaryView: ComponentType<unknown>;
  export function configureNotaryPcRuntime(options: {
    host: NotaryPcHostAdapter;
    sdkPorts: Record<string, unknown>;
  }): void;
  export function resetNotaryService(): void;
}
