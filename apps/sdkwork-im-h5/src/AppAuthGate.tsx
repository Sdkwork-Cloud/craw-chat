import { useEffect, useState, type ReactNode } from 'react';
import { SdkworkIamAuthRoutes } from '@sdkwork/auth-pc-react';
import {
  IM_H5_IAM_SESSION_CHANGED_EVENT,
  isAppSdkSessionAuthenticated,
  readAppSdkSessionTokens,
} from '@sdkwork/im-h5-core';
import { getIamRuntime } from './bootstrap/iamRuntime';
import { resolveImAuthAppearance, resolveImAuthRuntimeConfig } from './bootstrap/imAuthConfig';
import { IM_APP_HOME_PATH } from './ImApp';

export interface AppAuthGateProps {
  children: ReactNode;
}

export function AppAuthGate({ children }: AppAuthGateProps) {
  const [isAuthenticated, setIsAuthenticated] = useState(() =>
    isAppSdkSessionAuthenticated(readAppSdkSessionTokens()),
  );

  useEffect(() => {
    const handleSessionChanged = () => {
      setIsAuthenticated(isAppSdkSessionAuthenticated(readAppSdkSessionTokens()));
    };
    window.addEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => {
      window.removeEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, handleSessionChanged);
    };
  }, []);

  if (!isAuthenticated) {
    return (
      <SdkworkIamAuthRoutes
        basePath="/auth"
        getRuntime={getIamRuntime}
        homePath={IM_APP_HOME_PATH}
        runtimeConfig={resolveImAuthRuntimeConfig()}
        appearance={resolveImAuthAppearance()}
        viewportMode="flow"
      />
    );
  }

  return <>{children}</>;
}

export default AppAuthGate;
