import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { Navigate, useLocation, useNavigate } from 'react-router-dom';
import {
  SdkworkIamAuthRoutes,
  type SdkworkAuthAppearanceConfig,
  type SdkworkAuthRuntimeConfig,
  type SdkworkIamRuntimeAuthRuntimeLike,
} from '@sdkwork/auth-pc-react';
import {
  getImAppAuthRuntime,
  resolveImAuthRuntimeConfig,
} from './bootstrap/iamRuntime';
import {
  IM_H5_IAM_SESSION_CHANGED_EVENT,
  readImH5PersistedSession,
  type ImH5PersistedSession,
} from './bootstrap/session';

const AUTH_BASE_PATH = '/auth';
const AUTH_LOGIN_PATH = '/auth/login';
const AUTH_HOME_PATH = '/';

interface AuthGateProps {
  children: ReactNode;
}

function isAuthRoute(pathname: string): boolean {
  return pathname === AUTH_BASE_PATH || pathname.startsWith(`${AUTH_BASE_PATH}/`);
}

function resolveRedirectTarget(pathname: string, search: string, hash: string): string {
  if (isAuthRoute(pathname)) {
    return AUTH_HOME_PATH;
  }
  const target = `${pathname}${search}${hash}`;
  return target || AUTH_HOME_PATH;
}

function buildAuthLoginPath(redirectTarget: string): string {
  const params = new URLSearchParams();
  params.set('redirect', redirectTarget || AUTH_HOME_PATH);
  return `${AUTH_LOGIN_PATH}?${params.toString()}`;
}

function resolveAuthLocale(): string | null {
  if (typeof navigator === 'undefined') {
    return null;
  }
  const language = navigator.language.trim();
  return language || null;
}

function resolveAuthAppearance(): SdkworkAuthAppearanceConfig {
  return {
    asidePanelClassName: 'sdkwork-im-h5-auth-aside-panel',
    bodyClassName: 'sdkwork-im-h5-auth-body',
    contentContainerClassName: 'sdkwork-im-h5-auth-content',
    pageClassName: 'sdkwork-im-h5-auth-page',
    qrFrameClassName: 'sdkwork-im-h5-auth-qr-frame',
    shellClassName: 'sdkwork-im-h5-auth-card-shell',
    theme: {},
  };
}

export function AuthGate({ children }: AuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [isBootstrapped, setIsBootstrapped] = useState(false);
  const [session, setSession] = useState<ImH5PersistedSession | null>(null);

  const redirectTarget = useMemo(
    () => resolveRedirectTarget(location.pathname, location.search, location.hash),
    [location.hash, location.pathname, location.search],
  );

  const authenticated = Boolean(session?.accessToken && session.authToken);
  const isAuthPath = isAuthRoute(location.pathname);

  const authAppearance = useMemo(() => resolveAuthAppearance(), []);
  const authRuntimeConfig = useMemo(() => resolveImAuthRuntimeConfig() as SdkworkAuthRuntimeConfig, []);

  const getRuntime = useCallback((): SdkworkIamRuntimeAuthRuntimeLike => {
    return getImAppAuthRuntime().runtime as unknown as SdkworkIamRuntimeAuthRuntimeLike;
  }, []);

  useEffect(() => {
    let disposed = false;

    const bootstrap = async () => {
      const runtime = getImAppAuthRuntime().runtime;
      try {
        const tokens = await runtime.hydrateTokenManager();
        if (!tokens.accessToken || !tokens.authToken) {
          await runtime.clearSession();
        } else {
          await runtime.service.auth.sessions.current.retrieve();
        }
      } catch {
        await runtime.clearSession();
      }
      if (disposed) {
        return;
      }
      setSession(readImH5PersistedSession());
      setIsBootstrapped(true);
    };

    void bootstrap();

    return () => {
      disposed = true;
    };
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return undefined;
    }

    const handleSessionChanged = (event: Event) => {
      const detail = (event as CustomEvent<{ session?: ImH5PersistedSession | null }>).detail;
      setSession(detail?.session ?? readImH5PersistedSession());
    };

    window.addEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => {
      window.removeEventListener(IM_H5_IAM_SESSION_CHANGED_EVENT, handleSessionChanged);
    };
  }, []);

  useEffect(() => {
    if (!isBootstrapped || authenticated || isAuthPath) {
      return;
    }
    navigate(buildAuthLoginPath(redirectTarget), { replace: true });
  }, [isAuthPath, authenticated, isBootstrapped, navigate, redirectTarget]);

  if (!isBootstrapped) {
    return <div className="sdkwork-im-h5-auth-loading">Loading authentication...</div>;
  }

  if (authenticated && isAuthPath) {
    return <Navigate replace to={redirectTarget} />;
  }

  if (authenticated) {
    return <>{children}</>;
  }

  return (
    <SdkworkIamAuthRoutes
      appearance={authAppearance}
      basePath={AUTH_BASE_PATH}
      getRuntime={getRuntime}
      homePath={AUTH_HOME_PATH}
      locale={resolveAuthLocale()}
      runtimeConfig={authRuntimeConfig}
      viewportMode="flow"
    />
  );
}

export default AuthGate;
