export interface SdkworkImH5CapacitorConfig {
  appId: string;
  appName: string;
  webDir: string;
  android: {
    path: string;
  };
  ios: {
    path: string;
  };
  server: {
    androidScheme: string;
  };
}

export function createSdkworkImH5CapacitorConfig(): SdkworkImH5CapacitorConfig {
  return {
    appId: 'com.sdkwork.im.h5',
    appName: 'SDKWork IM',
    webDir: '../../dist',
    android: {
      path: 'android',
    },
    ios: {
      path: 'ios',
    },
    server: {
      androidScheme: 'https',
    },
  };
}

export default createSdkworkImH5CapacitorConfig();
