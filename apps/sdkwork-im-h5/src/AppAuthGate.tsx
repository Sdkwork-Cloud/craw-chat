import { useEffect, useMemo, useState, type ReactNode } from "react";
import { Navigate, useLocation, useNavigate } from "react-router-dom";
import {
  SdkworkIamAuthRoutes,
  type SdkworkIamAuthRoutesProps,
} from "@sdkwork/auth-pc-react";
import {
  IM_H5_IAM_SESSION_CHANGED_EVENT,
  isImH5IamSessionAuthenticated,
  readImH5IamSessionTokens,
  type ImH5IamSession,
} from "@sdkwork/im-h5-core";

import { getImIamRuntimeForAuth } from "./bootstrap/imAppAuthRuntime";
import {
  resolveImAuthAppearance,
  resolveImAuthLocale,
  resolveImAuthRuntimeConfig,
} from "./bootstrap/imAuthConfig";
import { IM_APP_HOME_PATH } from "./constants/appRoutes";

export { IM_APP_HOME_PATH };

const AUTH_BASE_PATH = "/auth";

interface AppAuthGateProps {
  children: ReactNode;
  homePath?: string;
}

function isAuthRoute(pathname: string): boolean {
  return pathname === AUTH_BASE_PATH || pathname.startsWith(`${AUTH_BASE_PATH}/`);
}

function resolveRedirectTarget(pathname: string, search: string, hash: string, homePath: string): string {
  const target = `${pathname}${search}${hash}`;
  if (isAuthRoute(pathname)) {
    return homePath;
  }
  return target || homePath;
}

function buildAuthLoginPath(redirectTarget: string): string {
  const params = new URLSearchParams();
  params.set("redirect", redirectTarget || IM_APP_HOME_PATH);
  return `${AUTH_BASE_PATH}/login?${params.toString()}`;
}

export function AppAuthGate({ children, homePath = IM_APP_HOME_PATH }: AppAuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [session, setSession] = useState<ImH5IamSession | null>(() => readImH5IamSessionTokens());

  const redirectTarget = useMemo(
    () => resolveRedirectTarget(location.pathname, location.search, location.hash, homePath),
    [homePath, location.hash, location.pathname, location.search],
  );
  const isAuthenticated = isImH5IamSessionAuthenticated(session);
  const isAuthPath = isAuthRoute(location.pathname);

  useEffect(() => {
    setSession(readImH5IamSessionTokens());
  }, [location.hash, location.pathname, location.search]);

  useEffect(() => {
    if (typeof window === "undefined") {
      return undefined;
    }

    const handleSessionChanged = (event: Event) => {
      const detail = (event as CustomEvent<{ session?: ImH5IamSession | null }>).detail;
      setSession(detail?.session ?? readImH5IamSessionTokens());
    };

    window.addEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => window.removeEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, handleSessionChanged);
  }, []);

  useEffect(() => {
    if (isAuthenticated || isAuthPath) {
      return;
    }
    navigate(buildAuthLoginPath(redirectTarget), { replace: true });
  }, [isAuthPath, isAuthenticated, navigate, redirectTarget]);

  if (isAuthenticated && isAuthPath) {
    return <Navigate replace to={redirectTarget} />;
  }

  if (isAuthenticated) {
    return <>{children}</>;
  }

  return (
    <SdkworkIamAuthRoutes
      appearance={resolveImAuthAppearance()}
      basePath={AUTH_BASE_PATH}
      getRuntime={
        getImIamRuntimeForAuth as unknown as SdkworkIamAuthRoutesProps["getRuntime"]
      }
      homePath={homePath}
      locale={resolveImAuthLocale()}
      routerContextMode="external"
      runtimeConfig={resolveImAuthRuntimeConfig()}
      viewportMode="flow"
    />
  );
}
