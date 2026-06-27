import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { Navigate, useLocation, useNavigate } from 'react-router-dom';
import { MessageSquare, Moon, Sun, X } from 'lucide-react';
import { LoadingBlock } from '@sdkwork/ui-pc-react';
import { SDKWORK_IM_PC_SETTINGS_STORAGE_KEY, applyHostAppearanceTheme, readPersistedSettingsRecord } from '@sdkwork/im-pc-commons';
import {
  SdkworkIamAuthRoutes,
} from '@sdkwork/auth-pc-react';
import {
  appAuthService,
  bootstrapLocalDevGatewayDiscovery,
  getSdkworkChatIamRuntime,
  hydrateAppSdkSessionFromSecureStorage,
  isSdkworkChatDesktopRuntime,
  isAppSdkSessionAuthenticated,
  readAppSdkSessionTokens,
  resetSdkworkChatAuthenticatedSdkClients,
  resolveSdkworkChatAuthAppearance,
  resolveSdkworkChatAuthRuntimeConfig,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
  type SdkworkChatSessionChangedDetail,
} from '@sdkwork/im-pc-core';

interface AuthGateProps {
  children: ReactNode;
}

const AUTH_BASE_PATH = '/auth';

function isAuthenticatedSession(session: SdkworkChatSession | null): boolean {
  return isAppSdkSessionAuthenticated(session);
}

function isAuthRoute(pathname: string): boolean {
  return pathname === AUTH_BASE_PATH || pathname.startsWith(`${AUTH_BASE_PATH}/`);
}

function resolveRedirectTarget(locationPathname: string, locationSearch: string, locationHash: string): string {
  const target = `${locationPathname}${locationSearch}${locationHash}`;
  if (isAuthRoute(locationPathname)) {
    return '/';
  }

  return target || '/';
}

function buildAuthLoginPath(redirectTarget: string): string {
  const params = new URLSearchParams();
  params.set('redirect', redirectTarget || '/');
  return `${AUTH_BASE_PATH}/login?${params.toString()}`;
}

function resolveAuthLocale(): string | null {
  const persistedLanguage = readPersistedSettingsRecord()?.lang;
  if (typeof persistedLanguage === 'string' && persistedLanguage.trim()) {
    return persistedLanguage.trim();
  }

  if (typeof navigator === 'undefined') {
    return null;
  }

  const language = navigator.language.trim();
  return language || null;
}

type WindowControlAction = 'close' | 'minimize' | 'toggleMaximize';
type NativeWindowControlAction = 'closeToTray' | 'minimize' | 'show' | 'startDragging' | 'toggleMaximize';
type AuthThemeMode = 'dark' | 'light';
type DesktopWindowLike = {
  isMaximized?: () => Promise<boolean> | boolean;
  maximize?: () => Promise<void> | void;
  minimize?: () => Promise<void> | void;
  startDragging?: () => Promise<void> | void;
  toggleMaximize?: () => Promise<void> | void;
  unmaximize?: () => Promise<void> | void;
};
type TauriInvoke = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

type StoredChatSettings = {
  theme?: 'system' | AuthThemeMode;
  [key: string]: unknown;
};

function readStoredChatSettings(): StoredChatSettings {
  const parsed = readPersistedSettingsRecord();
  return parsed && typeof parsed === 'object' ? parsed as StoredChatSettings : {};
}

function persistAuthThemeMode(theme: AuthThemeMode): void {
  if (typeof localStorage === 'undefined') {
    return;
  }

  try {
    localStorage.setItem(SDKWORK_IM_PC_SETTINGS_STORAGE_KEY, JSON.stringify({
      ...readStoredChatSettings(),
      theme,
    }));
  } catch {
  }
}

function resolveSystemAuthThemeMode(): AuthThemeMode {
  if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') {
    return 'dark';
  }

  return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
}

function resolveInitialAuthThemeMode(): AuthThemeMode {
  const storedTheme = readStoredChatSettings().theme;
  if (storedTheme === 'light' || storedTheme === 'dark') {
    return storedTheme;
  }

  return resolveSystemAuthThemeMode();
}

function applyAuthThemeMode(theme: AuthThemeMode): void {
  applyHostAppearanceTheme(theme);
}

function resolveTauriWindowApi(): DesktopWindowLike | null {
  const tauri = (globalThis as {
    __TAURI__?: {
      core?: {
        invoke?: TauriInvoke;
      };
      window?: {
        getCurrentWindow?: () => {
          isMaximized?: () => Promise<boolean> | boolean;
          maximize?: () => Promise<void> | void;
          minimize?: () => Promise<void> | void;
          startDragging?: () => Promise<void> | void;
          toggleMaximize?: () => Promise<void> | void;
          unmaximize?: () => Promise<void> | void;
        };
        appWindow?: {
          isMaximized?: () => Promise<boolean> | boolean;
          maximize?: () => Promise<void> | void;
          minimize?: () => Promise<void> | void;
          startDragging?: () => Promise<void> | void;
          toggleMaximize?: () => Promise<void> | void;
          unmaximize?: () => Promise<void> | void;
        };
      };
    };
  }).__TAURI__?.window;
  return tauri?.getCurrentWindow?.() ?? tauri?.appWindow ?? null;
}

function resolveTauriInvoke(): TauriInvoke | null {
  return (globalThis as {
    __TAURI__?: {
      core?: {
        invoke?: TauriInvoke;
      };
    };
  }).__TAURI__?.core?.invoke ?? null;
}

async function invokeDesktopWindowControl(action: NativeWindowControlAction): Promise<boolean> {
  const invoke = resolveTauriInvoke();

  if (!invoke) {
    return false;
  }

  await invoke('sdkwork_chat_pc_window_control', { action });
  return true;
}

async function handleWindowControl(action: WindowControlAction): Promise<void> {
  const nativeAction = action === 'close' ? 'closeToTray' : action;
  if (await invokeDesktopWindowControl(nativeAction)) {
    return;
  }

  if (action === 'close') {
    return;
  }

  const appWindow = resolveTauriWindowApi();
  if (!appWindow) {
    return;
  }
  if (action === 'minimize') {
    await appWindow.minimize?.();
    return;
  }
  if (action === 'toggleMaximize') {
    if (appWindow.toggleMaximize) {
      await appWindow.toggleMaximize();
      return;
    }

    if (appWindow.isMaximized && appWindow.unmaximize && await appWindow.isMaximized()) {
      await appWindow.unmaximize();
      return;
    }

    await appWindow.maximize?.();
    return;
  }
}

async function startAuthTitleBarDragging(): Promise<void> {
  if (await invokeDesktopWindowControl('startDragging')) {
    return;
  }

  const appWindow = resolveTauriWindowApi();
  await appWindow?.startDragging?.();
}

function isAuthHeaderNoDragTarget(target: EventTarget | null): boolean {
  if (!target || typeof target !== 'object') {
    return false;
  }

  const maybeElement = target as {
    closest?: (selector: string) => Element | null | unknown;
  };
  return typeof maybeElement.closest === 'function'
    ? Boolean(maybeElement.closest('[data-no-drag="true"]'))
    : false;
}

function WindowControlMinimizeIcon() {
  return (
    <svg
      aria-hidden="true"
      className="h-3.5 w-3.5"
      fill="none"
      shapeRendering="crispEdges"
      viewBox="0 0 10 10"
    >
      <path
        d="M2 7H8"
        stroke="currentColor"
        strokeLinecap="square"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
    </svg>
  );
}

function WindowControlMaximizeIcon() {
  return (
    <svg
      aria-hidden="true"
      className="h-3.5 w-3.5"
      fill="none"
      shapeRendering="crispEdges"
      viewBox="0 0 10 10"
    >
      <path
        d="M2 2.5H8V8H2V2.5Z"
        stroke="currentColor"
        strokeLinejoin="miter"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
      <path
        d="M2 3.5H8"
        stroke="currentColor"
        strokeLinecap="square"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
    </svg>
  );
}

function WindowControlRestoreIcon() {
  return (
    <svg
      aria-hidden="true"
      className="h-3.5 w-3.5"
      fill="none"
      shapeRendering="crispEdges"
      viewBox="0 0 10 10"
    >
      <path
        d="M3.5 2H8V6.5H6.5"
        stroke="currentColor"
        strokeLinejoin="miter"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
      <path
        d="M3.5 3H8"
        stroke="currentColor"
        strokeLinecap="square"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
      <path
        d="M2 3.5H6.5V8H2V3.5Z"
        stroke="currentColor"
        strokeLinejoin="miter"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
      <path
        d="M2 4.5H6.5"
        stroke="currentColor"
        strokeLinecap="square"
        strokeWidth="1"
        vectorEffect="non-scaling-stroke"
      />
    </svg>
  );
}

function SdkworkChatAuthShell({ children }: { children: ReactNode }) {
  const [authThemeMode, setAuthThemeMode] = useState<AuthThemeMode>(() => resolveInitialAuthThemeMode());
  const [isWindowMaximized, setIsWindowMaximized] = useState(false);
  const isLightMode = authThemeMode === 'light';
  const shouldRenderDesktopAppHeader = isSdkworkChatDesktopRuntime();

  useEffect(() => {
    applyAuthThemeMode(authThemeMode);
  }, [authThemeMode]);

  const refreshWindowState = useCallback(async () => {
    const appWindow = resolveTauriWindowApi();
    if (!appWindow?.isMaximized) {
      setIsWindowMaximized(false);
      return;
    }

    try {
      setIsWindowMaximized(Boolean(await appWindow.isMaximized()));
    } catch {
      setIsWindowMaximized(false);
    }
  }, []);

  useEffect(() => {
    void refreshWindowState();
  }, [refreshWindowState]);

  const toggleAuthTheme = useCallback(() => {
    setAuthThemeMode((currentTheme) => {
      const nextTheme = currentTheme === 'light' ? 'dark' : 'light';
      persistAuthThemeMode(nextTheme);
      return nextTheme;
    });
  }, []);

  const onWindowControl = useCallback((action: WindowControlAction) => {
    void handleWindowControl(action)
      .then(() => {
        if (action === 'toggleMaximize') {
          void refreshWindowState();
        }
      });
  }, [refreshWindowState]);

  const handleTitleBarPointerDown = useCallback((event: React.PointerEvent<HTMLElement>) => {
    if (event.button !== 0 || event.isPrimary === false || isAuthHeaderNoDragTarget(event.target)) {
      return;
    }

    event.preventDefault();
    void startAuthTitleBarDragging();
  }, []);

  const handleTitleBarDoubleClick = useCallback((event: React.MouseEvent<HTMLElement>) => {
    if (event.button !== 0 || isAuthHeaderNoDragTarget(event.target)) {
      return;
    }

    void handleWindowControl('toggleMaximize').then(refreshWindowState);
  }, [refreshWindowState]);

  const handleTitleBarContextMenu = useCallback((event: React.MouseEvent<HTMLElement>) => {
    if (isAuthHeaderNoDragTarget(event.target)) {
      return;
    }

    event.preventDefault();
  }, []);

  const handleTitleBarDragStart = useCallback((event: React.DragEvent<HTMLElement>) => {
    if (isAuthHeaderNoDragTarget(event.target)) {
      return;
    }

    event.preventDefault();
  }, []);

  return (
    <div className="sdkwork-chat-auth-shell">
      {shouldRenderDesktopAppHeader && (
        <header
          className="sdkwork-chat-auth-header drag-region"
          data-tauri-drag-region
          onContextMenu={handleTitleBarContextMenu}
          onDoubleClick={handleTitleBarDoubleClick}
          onDragStart={handleTitleBarDragStart}
          onPointerDown={handleTitleBarPointerDown}
        >
          <div className="sdkwork-chat-auth-header-brand" data-tauri-drag-region>
            <span className="sdkwork-chat-auth-header-mark" aria-hidden="true">
              <MessageSquare size={12} />
            </span>
            <span>Sdkwork IM</span>
          </div>
          <div className="sdkwork-chat-auth-header-center" data-tauri-drag-region />
          <div className="sdkwork-chat-auth-header-actions no-drag" data-no-drag="true">
            <button
              aria-label={isLightMode ? 'Switch to dark mode' : 'Switch to light mode'}
              className="sdkwork-chat-auth-theme-button"
              onClick={toggleAuthTheme}
              title={isLightMode ? 'Switch to dark mode' : 'Switch to light mode'}
              type="button"
            >
              {isLightMode ? <Moon size={14} /> : <Sun size={14} />}
            </button>
            <div className="sdkwork-chat-auth-window-controls">
              <button
                aria-label="Minimize window"
                className="sdkwork-chat-auth-window-button"
                onClick={() => onWindowControl('minimize')}
                title="Minimize"
                type="button"
              >
                <WindowControlMinimizeIcon />
              </button>
              <button
                aria-label={isWindowMaximized ? 'Restore window' : 'Maximize window'}
                className="sdkwork-chat-auth-window-button"
                onClick={() => onWindowControl('toggleMaximize')}
                title={isWindowMaximized ? 'Restore' : 'Maximize'}
                type="button"
              >
                {isWindowMaximized ? <WindowControlRestoreIcon /> : <WindowControlMaximizeIcon />}
              </button>
              <button
                aria-label="Close window"
                className="sdkwork-chat-auth-window-button sdkwork-chat-auth-window-button-danger"
                onClick={() => onWindowControl('close')}
                title="Close"
                type="button"
              >
                <X size={14} />
              </button>
            </div>
          </div>
        </header>
      )}
      <main className="sdkwork-chat-auth-main">{children}</main>
    </div>
  );
}

export function AuthGate({ children }: AuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [isBootstrapped, setIsBootstrapped] = useState(false);
  const [session, setSession] = useState<SdkworkChatSession | null>(null);

  const redirectTarget = useMemo(
    () => resolveRedirectTarget(location.pathname, location.search, location.hash),
    [location.hash, location.pathname, location.search],
  );
  const isAuthenticated = isAuthenticatedSession(readAppSdkSessionTokens());
  const isAuthPath = isAuthRoute(location.pathname);

  useEffect(() => {
    let disposed = false;

    const bootstrapAuthSession = async () => {
      await bootstrapLocalDevGatewayDiscovery();
      resetSdkworkChatAuthenticatedSdkClients();
      await hydrateAppSdkSessionFromSecureStorage();
      if (disposed) {
        return;
      }

      const storedSession = readAppSdkSessionTokens();

      if (!isAuthenticatedSession(storedSession)) {
        setSession(null);
        setIsBootstrapped(true);
        return;
      }

      setIsBootstrapped(false);

      try {
        const nextSession = await appAuthService.getCurrentSession();
        if (disposed) {
          return;
        }

        setSession(nextSession);
        setIsBootstrapped(true);
      } catch {
        if (disposed) {
          return;
        }

        const fallbackSession = readAppSdkSessionTokens();
        setSession(isAuthenticatedSession(fallbackSession) ? fallbackSession : null);
        setIsBootstrapped(true);
      }
    };

    void bootstrapAuthSession();

    return () => {
      disposed = true;
    };
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return undefined;
    }

    const handleSessionChanged = (event: Event) => {
      const detail = (event as CustomEvent<SdkworkChatSessionChangedDetail>).detail;
      setSession(detail?.session ?? readAppSdkSessionTokens());
      setIsBootstrapped(true);
    };

    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => {
      window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleSessionChanged);
    };
  }, []);

  useEffect(() => {
    if (!isBootstrapped || isAuthenticated || isAuthPath) {
      return;
    }

    navigate(buildAuthLoginPath(redirectTarget), { replace: true });
  }, [isAuthPath, isAuthenticated, isBootstrapped, navigate, redirectTarget]);

  if (!isBootstrapped) {
    return (
      <div className="sdkwork-chat-auth-loading">
        <LoadingBlock className="w-full max-w-[420px]" label="Loading authentication..." />
      </div>
    );
  }

  if (isAuthenticated && isAuthPath) {
    return <Navigate replace to={redirectTarget} />;
  }

  if (isAuthenticated) {
    return children;
  }

  return (
    <SdkworkChatAuthShell>
      <SdkworkIamAuthRoutes
        appearance={resolveSdkworkChatAuthAppearance()}
        basePath="/auth"
        getRuntime={getSdkworkChatIamRuntime}
        homePath="/"
        locale={resolveAuthLocale()}
        runtimeConfig={resolveSdkworkChatAuthRuntimeConfig()}
        viewportMode="fixed"
      />
    </SdkworkChatAuthShell>
  );
}
