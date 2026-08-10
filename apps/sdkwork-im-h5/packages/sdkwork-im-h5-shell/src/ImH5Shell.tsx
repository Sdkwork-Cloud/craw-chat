import React, { useEffect, type ReactElement, type ReactNode } from "react";
import { Navigate, Route, Routes } from "react-router";
import { GlobalMiniPlayer } from "@sdkwork/music-mobile-react-playback";
import { SettingsService } from "@sdkwork/im-h5-user";

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

/**
 * Reads the persisted dark-mode preference; `null` when the user never made
 * an explicit choice (then the system color scheme applies).
 */
function readExplicitDarkPreference(): boolean | null {
  try {
    const stored = window.localStorage.getItem("clawchat_app_settings");
    if (stored) {
      const parsed = JSON.parse(stored) as { darkMode?: boolean };
      if (typeof parsed.darkMode === "boolean") {
        return parsed.darkMode;
      }
    }
  } catch {
    // malformed storage: treated as no explicit preference
  }
  return null;
}

/**
 * Applies the persisted dark-mode preference on startup; falls back to the
 * system color scheme when the user never chose. Keeps the `.dark` class in
 * sync so both the theme variables and the `dark:` variants switch together.
 */
function applyInitialTheme() {
  const root = document.documentElement;
  const explicit = readExplicitDarkPreference();
  if (explicit !== null) {
    root.classList.toggle("dark", explicit);
    return;
  }
  root.classList.toggle("dark", window.matchMedia("(prefers-color-scheme: dark)").matches);
}

function ThemeInitializer() {
  useEffect(() => {
    applyInitialTheme();
    // 实时跟随系统/微信开发者工具的主题切换：开发者工具模拟器在页面加载后
    // 切换深色模式时 matchMedia 事件会触发，若不监听则页面一直停留在启动时
    // 的配色。用户已在设置中显式选择深/浅色时不覆盖其偏好。
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = (event: MediaQueryListEvent) => {
      if (readExplicitDarkPreference() !== null) {
        return;
      }
      document.documentElement.classList.toggle("dark", event.matches);
    };
    media.addEventListener("change", onChange);
    return () => media.removeEventListener("change", onChange);
  }, []);
  return null;
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
      <ThemeInitializer />
      {resolvedModules.map((module) => {
        const Lifecycle = module.lifecycle;
        return Lifecycle ? <Lifecycle key={module.id} /> : null;
      })}
      <GlobalMiniPlayer />
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
