declare module '@sdkwork/aiot-pc-console-device' {
  import type { ComponentType } from 'react';

  export const SdkworkDevicePage: ComponentType<{
    onNavigate(route: string): void;
  }>;
}
