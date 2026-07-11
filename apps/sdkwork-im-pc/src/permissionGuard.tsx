import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { Navigate } from 'react-router-dom';
import {
  hasAppSdkPermission,
  readAppSdkSessionTokens,
  resolveAppSdkPermissionScope,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core';

/**
 * Route-level RBAC primitives for the PC application.
 *
 * These guards close the commercial-readiness gap flagged in
 * `REVIEW-2026-0710-im-commercial-readiness.md`: `/admin/*` and `/console/*`
 * previously relied on authentication alone, so any authenticated user could
 * mount privileged operator/admin surfaces. The guards now verify a
 * permission claim from the session token before mounting those routes.
 *
 * Permission matching mirrors the backend `AppContext::has_permission`
 * semantics (`crates/im-app-context/src/lib.rs`): `*`, `tenant.admin`, exact
 * code, and `<prefix>.*` wildcards all grant access.
 */

export interface PermissionGuard {
  /** Permission scope codes resolved from the current session token. */
  permissionScope: string[];
  /** True when the current session grants `permission`. */
  hasPermission: (permission: string) => boolean;
  /** True when the current session grants at least one of `permissions`. */
  hasAnyPermission: (permissions: string[]) => boolean;
}

function readSessionSnapshot(): SdkworkChatSession | null {
  return readAppSdkSessionTokens();
}

/**
 * Reactive permission guard. Reads the persisted session and re-resolves the
 * permission scope whenever the session is persisted or cleared (login,
 * refresh, logout), so route guards stay in sync with the live token.
 */
export function usePermissionGuard(): PermissionGuard {
  const [session, setSession] = useState<SdkworkChatSession | null>(readSessionSnapshot);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return undefined;
    }

    const handleSessionChanged = () => {
      setSession(readSessionSnapshot());
    };

    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => {
      window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleSessionChanged);
    };
  }, []);

  const permissionScope = useMemo(() => resolveAppSdkPermissionScope(session), [session]);

  const hasPermission = useCallback(
    (permission: string) => hasAppSdkPermission(session, permission),
    [session],
  );

  const hasAnyPermission = useCallback(
    (permissions: string[]) => permissions.some((permission) => hasAppSdkPermission(session, permission)),
    [session],
  );

  return { permissionScope, hasPermission, hasAnyPermission };
}

export interface RequirePermissionProps {
  /** The route mounts only when the session grants at least one listed code. */
  anyOf: string[];
  children: ReactNode;
  /** Destination when no permission is granted. Defaults to the chat surface. */
  fallbackPath?: string;
}

/**
 * Route guard: mounts `children` only when the current session grants at least
 * one of the declared permission codes. Otherwise redirects to `fallbackPath`.
 *
 * Used to gate `/admin/*` (requires `admin.read` or `admin.write`) and
 * `/console/*` (requires `control.read` or `control.write`) so privileged
 * surfaces are never mounted for tokens that lack the authorization claim.
 */
export function RequirePermission({
  anyOf,
  children,
  fallbackPath = '/',
}: RequirePermissionProps) {
  const { hasAnyPermission } = usePermissionGuard();

  if (!hasAnyPermission(anyOf)) {
    return <Navigate to={fallbackPath} replace />;
  }

  return <>{children}</>;
}
