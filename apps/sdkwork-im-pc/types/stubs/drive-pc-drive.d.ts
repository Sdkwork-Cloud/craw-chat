declare module '@sdkwork/drive-pc-drive' {
  import type { ComponentType } from 'react';

  export interface DrivePcSdkPorts {
    getDriveClient: () => unknown;
    readHostSession: () => unknown;
    subscribeHostSession?: (listener: () => void) => () => void;
    resolveHostLanguage?: () => string;
    subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
  }

  export interface DriveOpenRequest {
    requestId: string;
    section: 'recent';
    nodeId: string;
    spaceId?: string;
    intent: 'preview';
  }

  export interface DriveViewProps {
    openRequest?: DriveOpenRequest;
    onOpenRequestHandled?: (requestId: string) => void;
  }

  export const DriveView: ComponentType<DriveViewProps>;
  export function configureDrivePcRuntime(options: { sdkPorts: DrivePcSdkPorts }): void;
}
