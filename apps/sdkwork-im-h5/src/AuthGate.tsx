import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { Navigate, useLocation, useNavigate } from 'react-router-dom';
import { SdkworkIamH5AuthRoutes } from '@sdkwork/iam-h5-auth';
import { useAppStore, type ImH5SessionUser } from '@sdkwork/im-h5-core';
import {
  getImAppAuthRuntime,
} from './bootstrap/iamRuntime';
import { createImH5AuthController } from './bootstrap/imH5AuthController';
import { bindImH5SessionLogoutHandler } from './bootstrap/imAuthService';
import {
  IM_H5_IAM_SESSION_CHANGED_EVENT,
  readImH5PersistedSession,
  restoreAndValidateImH5Session,
  type ImH5PersistedSession,
} from './bootstrap/session';
import { getSdkClients } from './bootstrap/sdkClients';
import { ensureChatWelcomeMessage } from '@sdkwork/im-h5-chat';

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

export function AuthGate({ children }: AuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [isBootstrapped, setIsBootstrapped] = useState(false);
  const [session, setSession] = useState<ImH5PersistedSession | null>(null);
  const setCurrentUser = useAppStore((state) => state.setCurrentUser);

  const redirectTarget = useMemo(
    () => resolveRedirectTarget(location.pathname, location.search, location.hash),
    [location.hash, location.pathname, location.search],
  );

  const authenticated = Boolean(session?.accessToken && session.authToken);
  const isAuthPath = isAuthRoute(location.pathname);
  const controller = useMemo(() => createImH5AuthController(), []);

  useEffect(() => {
    if (!authenticated) {
      setCurrentUser(null);
      return;
    }
    let disposed = false;
    const loadCurrentUser = async () => {
      try {
        const profile = await getSdkClients().iamAppSdkClient.iam.users.current.retrieve();
        if (disposed) {
          return;
        }
        const user = resolveIamProfileUser(profile);
        setCurrentUser(user ?? resolveSessionUser(session?.user));
      } catch {
        if (!disposed) {
          setCurrentUser(resolveSessionUser(session?.user));
        }
      }
    };
    void loadCurrentUser();
    return () => {
      disposed = true;
    };
  }, [authenticated, session, setCurrentUser]);

  useEffect(() => {
    // 注册/登录建立会话后，幂等触发系统智能体 Welcome 检查：
    // 服务端在用户未收到过 Welcome 且没有过对话时发送系统消息，否则跳过，
    // 因此重复调用（刷新/重登）不会重复发送。
    if (!authenticated) {
      return;
    }
    void ensureChatWelcomeMessage().catch(() => {
      // fire-and-forget：欢迎消息缺失不影响主流程（下次会话再触发）。
    });
  }, [authenticated]);

  useEffect(() => {
    let disposed = false;

    const bootstrap = async () => {
      const runtime = getImAppAuthRuntime().runtime;
      await restoreAndValidateImH5Session({
        clearSession: () => runtime.clearSession(),
        hydrateTokenManager: () => runtime.hydrateTokenManager(),
        retrieveCurrentSession: () => runtime.service.auth.sessions.current.retrieve(),
      });
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
    // Feature surfaces (e.g. the settings page) request logout through the
    // shared session port; the app-owned executor lives here in AuthGate so
    // it is registered for the lifetime of the gate.
    return bindImH5SessionLogoutHandler();
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
    <SdkworkIamH5AuthRoutes
      controller={controller}
      basePath={AUTH_BASE_PATH}
      locale={resolveAuthLocale()}
    />
  );
}

export default AuthGate;

function resolveIamProfileUser(value: unknown): ImH5SessionUser | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  const id = readString(record.id) ?? readString(record.userId);
  if (!id) return null;
  return {
    id,
    name: readString(record.displayName)
      ?? readString(record.nickname)
      ?? readString(record.name)
      ?? readString(record.username)
      ?? id,
    ...(readString(record.avatarUrl) ? { avatar: readString(record.avatarUrl) } : {}),
  };
}

function resolveSessionUser(value: unknown) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  const id = readString(record.id) ?? readString(record.userId);
  if (!id) return null;
  return {
    id,
    name: readString(record.displayName) ?? readString(record.name) ?? readString(record.username) ?? id,
    ...(readString(record.avatarUrl) ? { avatar: readString(record.avatarUrl) } : {}),
  };
}

function readString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim() ? value.trim() : undefined;
}
