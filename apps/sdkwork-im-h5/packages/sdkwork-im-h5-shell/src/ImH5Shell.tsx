import React, { type ReactElement, type ReactNode } from "react";
import { Navigate, Route, Routes } from "react-router";

import type {
  ImH5CapabilityModule,
  ImH5ModuleId,
  ImH5RouteContribution,
} from "./contracts";
import {
  DEFAULT_IM_H5_MODULES,
  resolveImH5ShellModules,
  validateImH5ShellModules,
} from "./moduleRegistry";
import { IM_APP_HOME_PATH } from "./modules/chatModule";
import { resolveImH5ShellHomePath } from "./moduleNavigation";
import { TabBar } from "./navigation/TabBar";

export interface ImH5ShellProps {
  children?: ReactNode;
  moduleIds?: readonly ImH5ModuleId[];
  modules?: readonly ImH5CapabilityModule[];
}

function MainShell({ children, modules }: { children: ReactNode; modules: readonly ImH5CapabilityModule[] }) {
  const navigation = modules.flatMap((module) => module.navigation ?? []);
  return (
    <div className="relative flex h-full flex-col overflow-hidden bg-bg-color">
      <div className="min-h-0 flex-1">{children}</div>
      <TabBar items={navigation} />
    </div>
  );
}

function renderRoute(
  route: ImH5RouteContribution,
  modules: readonly ImH5CapabilityModule[],
): ReactElement {
  const content = route.layoutGroup === "main"
    ? <MainShell modules={modules}>{route.render()}</MainShell>
    : route.render();

  if (route.index) {
    return <Route key={route.id} index element={content} />;
  }

  return (
    <Route key={route.id} path={route.relativePath ?? route.path} element={content}>
      {route.children?.map((child) => renderRoute(child, modules))}
    </Route>
  );
}

export function listImH5ShellRouteMetadata(modules: readonly ImH5CapabilityModule[]) {
  const flatten = (routes: readonly ImH5RouteContribution[]): ImH5RouteContribution[] =>
    routes.flatMap((route) => [route, ...flatten(route.children ?? [])]);
  return modules.flatMap((module) => flatten(module.routes));
}

export function ImH5Shell({ children, moduleIds = DEFAULT_IM_H5_MODULES, modules }: ImH5ShellProps) {
  const resolvedModules = modules ? [...modules] : resolveImH5ShellModules(moduleIds);
  if (modules) {
    validateImH5ShellModules(resolvedModules);
  }
  const homePath = resolveImH5ShellHomePath(resolvedModules, IM_APP_HOME_PATH);
  return (
    <>
      {resolvedModules.map((module) => {
        const Lifecycle = module.lifecycle;
        return Lifecycle ? <Lifecycle key={module.id} /> : null;
      })}
      <React.Suspense fallback={null}>
        <Routes>
          {resolvedModules.flatMap((module) =>
            module.routes.map((route) => renderRoute(route, resolvedModules))
          )}
          {children}
          <Route path="*" element={<Navigate to={homePath} replace />} />
        </Routes>
      </React.Suspense>
    </>
  );
}

export default ImH5Shell;
