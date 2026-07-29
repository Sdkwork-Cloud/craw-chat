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
  {
    routeId: 'im-h5-workspace',
    path: '/workspace',
    title: 'Workspace',
    mobilePresentation: 'tab',
    layoutGroup: 'app',
  },
  {
    routeId: 'im-h5-workspace-notary',
    path: '/workspace/notary',
    title: 'Notary Workspace',
    mobilePresentation: 'stack',
    layoutGroup: 'app',
  },
  {
    routeId: 'im-h5-notary-records',
    path: '/notary',
    title: 'Notary Records',
    mobilePresentation: 'tab',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-files',
    path: '/notary/files',
    title: 'Notary Files',
    mobilePresentation: 'tab',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-messages',
    path: '/notary/messages',
    title: 'Notary Messages',
    mobilePresentation: 'tab',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-me',
    path: '/notary/me',
    title: 'Notary Account',
    mobilePresentation: 'tab',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-create',
    path: '/notary/create',
    title: 'Create Notary Case',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-search',
    path: '/notary/search',
    title: 'Search Notary Cases',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-add-party',
    path: '/notary/add-party',
    title: 'Add Notary Party',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-detail',
    path: '/notary/detail/:id',
    title: 'Notary Case',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-message-detail',
    path: '/notary/messages/:messageId',
    title: 'Notary Message',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-session-chat',
    path: '/notary/chat/:caseId',
    title: 'Notary Session',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-party-signature',
    path: '/notary/cases/:caseId/parties/:partyId/signature',
    title: 'Party Signature',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-party-video',
    path: '/notary/cases/:caseId/parties/:partyId/video',
    title: 'Party Video',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
  },
  {
    routeId: 'im-h5-notary-party-video-qr',
    path: '/notary/cases/:caseId/parties/:partyId/video-qr',
    title: 'Party Video QR',
    mobilePresentation: 'stack',
    layoutGroup: 'notary',
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
