declare module '@sdkwork/notary-pc-core' {
  export interface NotaryAccessState {
    [key: string]: unknown;
  }

  export interface NotaryAccessService {
    [key: string]: unknown;
  }

  export function createNotaryAccessService(
    getClient: () => unknown,
  ): NotaryAccessService;

  export function bootstrapNotaryPcForIm(hostAdapter: unknown): void;
  export function rebootstrapNotaryPcRuntimeForIm(): void;
  export function resetNotaryPcRuntime(): void;
}
