/**
 * H5 route assembly.
 *
 * App routes belong to `h5-shell` and app capability route contributions.
 * Route metadata may declare title, icon, route id, required permission hint,
 * layout group, mobile presentation, and lazy import. It MUST NOT declare
 * API path constants.
 */

export interface H5RouteMeta {
  readonly routeId: string;
  readonly path: string;
  readonly title?: string;
  readonly icon?: string;
  readonly layoutGroup?: string;
  readonly mobilePresentation?: 'stack' | 'tab' | 'sheet' | 'modal' | 'drawer';
  readonly requiredPermissionHint?: string;
  readonly lazyImport?: () => Promise<{ default: unknown }>;
}

export const IM_H5_APP_ROUTE_PREFIX = '/app';
export const IM_H5_CONSOLE_ROUTE_PREFIX = '/console';
export const IM_H5_ADMIN_ROUTE_PREFIX = '/admin';

export const IM_H5_ROUTE_REGISTRY: readonly H5RouteMeta[] = [
  {
    routeId: 'im-h5-chat-inbox',
    path: '/',
    title: 'Chat',
    mobilePresentation: 'tab',
    layoutGroup: 'app',
  },
  {
    routeId: 'im-h5-chat-conversation',
    path: '/chat/:conversationId',
    title: 'Conversation',
    mobilePresentation: 'stack',
    layoutGroup: 'app',
  },
] as const;

const registeredRoutes = new Map<string, H5RouteMeta>();

export function registerRoute(meta: H5RouteMeta): void {
  registeredRoutes.set(meta.routeId, meta);
}

export function getRoute(routeId: string): H5RouteMeta | undefined {
  return registeredRoutes.get(routeId) ?? IM_H5_ROUTE_REGISTRY.find((r) => r.routeId === routeId);
}

export function listRoutes(): H5RouteMeta[] {
  const merged = new Map<string, H5RouteMeta>();
  for (const meta of IM_H5_ROUTE_REGISTRY) {
    merged.set(meta.routeId, meta);
  }
  for (const [id, meta] of registeredRoutes) {
    merged.set(id, meta);
  }
  return Array.from(merged.values());
}

export function resetRoutes(): void {
  registeredRoutes.clear();
}
