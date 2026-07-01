declare module '@sdkwork/drive-pc-drive' {
  import type { ComponentType } from 'react';

  export interface DrivePcSdkPorts {
    getDriveClient: () => unknown;
    readHostSession: () => unknown;
    subscribeHostSession?: (listener: () => void) => () => void;
    resolveHostLanguage?: () => string;
    subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
  }

  export const DriveView: ComponentType<unknown>;
  export function configureDrivePcRuntime(options: { sdkPorts: DrivePcSdkPorts }): void;
}
