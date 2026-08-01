import type { ImH5CapabilityModule } from "./contracts";

export function resolveImH5ShellHomePath(
  modules: readonly ImH5CapabilityModule[],
  fallbackPath = "/",
): string {
  const navigationPath = modules.flatMap((module) => module.navigation ?? [])[0]?.path;
  if (navigationPath) {
    return navigationPath;
  }

  for (const module of modules) {
    const routePath = module.routes.find((route) => !route.index)?.path;
    if (routePath) {
      return routePath;
    }
  }

  return fallbackPath;
}
