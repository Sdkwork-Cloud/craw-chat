import type { ImH5CapabilityModule, ImH5ModuleId } from "./contracts";

export function requireImH5ShellModule(
  moduleId: ImH5ModuleId,
  registry: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>>,
): ImH5CapabilityModule {
  const module = registry[moduleId];
  if (!module) {
    throw new Error(`H5 module ${moduleId} is not composed in the selected registry.`);
  }
  return module;
}

export function validateImH5ShellModules(modules: readonly ImH5CapabilityModule[]): void {
  if (modules.length === 0) {
    throw new Error("H5 module composition must contain at least one module.");
  }

  const moduleIds = new Set<ImH5ModuleId>();
  const routeIds = new Set<string>();
  const routePaths = new Set<string>();
  const navigationIds = new Set<string>();

  const visitRoute = (module: ImH5CapabilityModule, route: ImH5CapabilityModule["routes"][number]) => {
    if (route.moduleId !== module.id) {
      throw new Error(`Route ${route.id} belongs to ${route.moduleId}, not ${module.id}.`);
    }
    if (routeIds.has(route.id)) {
      throw new Error(`Duplicate H5 route id: ${route.id}.`);
    }
    routeIds.add(route.id);

    if (!route.index) {
      if (routePaths.has(route.path)) {
        throw new Error(`Duplicate H5 route path: ${route.path}.`);
      }
      routePaths.add(route.path);
    }
    route.children?.forEach((child) => visitRoute(module, child));
  };

  for (const module of modules) {
    if (moduleIds.has(module.id)) {
      throw new Error(`Duplicate H5 module id: ${module.id}.`);
    }
    moduleIds.add(module.id);
    module.routes.forEach((route) => visitRoute(module, route));

    for (const navigation of module.navigation ?? []) {
      if (navigation.moduleId !== module.id) {
        throw new Error(`Navigation ${navigation.id} belongs to ${navigation.moduleId}, not ${module.id}.`);
      }
      if (navigationIds.has(navigation.id)) {
        throw new Error(`Duplicate H5 navigation id: ${navigation.id}.`);
      }
      navigationIds.add(navigation.id);
    }
  }
}
