export type ImH5Surface = "app" | "console" | "admin";

export type ImH5MobilePresentation = "stack" | "tab" | "modal" | "sheet";

export interface ImH5RouteMetadata {
  readonly id: string;
  readonly legacyRouteId?: string;
  readonly moduleId: string;
  readonly surface: ImH5Surface;
  readonly domain: string;
  readonly capability: string;
  readonly screen: string;
  readonly path: string;
  readonly titleKey: string;
  readonly auth: "public" | "required";
  readonly permissionHint?: string;
  readonly layoutGroup?: string;
  readonly presentation: {
    readonly h5Mobile: ImH5MobilePresentation;
  };
}

const registeredRoutes = new Map<string, ImH5RouteMetadata>();

function routeKeys(route: ImH5RouteMetadata): string[] {
  return route.legacyRouteId ? [route.id, route.legacyRouteId] : [route.id];
}

export function registerImH5RouteMetadata(route: ImH5RouteMetadata): void {
  for (const key of routeKeys(route)) {
    registeredRoutes.set(key, route);
  }
}

export function getImH5RouteMetadata(routeId: string): ImH5RouteMetadata | undefined {
  return registeredRoutes.get(routeId);
}

export function listImH5RouteMetadata(): ImH5RouteMetadata[] {
  return Array.from(new Map(
    Array.from(registeredRoutes.values(), (route) => [route.id, route]),
  ).values());
}

export function resetImH5RouteMetadata(): void {
  registeredRoutes.clear();
}
