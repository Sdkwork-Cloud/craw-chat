import {
  getImH5RouteMetadata,
  listImH5RouteMetadata,
  registerImH5RouteMetadata,
  resetImH5RouteMetadata,
  type ImH5RouteMetadata,
} from "@sdkwork/im-h5-core/routes";
import { IM_H5_APP_ROUTE_METADATA } from "@sdkwork/im-h5-shell/routeCatalog";

export type H5RouteMeta = ImH5RouteMetadata;

export const IM_H5_APP_ROUTE_PREFIX = "/app";
export const IM_H5_CONSOLE_ROUTE_PREFIX = "/console";
export const IM_H5_ADMIN_ROUTE_PREFIX = "/admin";

export const IM_H5_ROUTE_REGISTRY = IM_H5_APP_ROUTE_METADATA;

export function registerRoute(meta: H5RouteMeta): void {
  registerImH5RouteMetadata(meta);
}

export function getRoute(routeId: string): H5RouteMeta | undefined {
  return getImH5RouteMetadata(routeId)
    ?? IM_H5_ROUTE_REGISTRY.find(
      (route) => route.id === routeId || route.legacyRouteId === routeId,
    );
}

export function listRoutes(): H5RouteMeta[] {
  const merged = new Map(IM_H5_ROUTE_REGISTRY.map((route) => [route.id, route]));
  for (const route of listImH5RouteMetadata()) {
    merged.set(route.id, route);
  }
  return Array.from(merged.values());
}

export function resetRoutes(): void {
  resetImH5RouteMetadata();
}
