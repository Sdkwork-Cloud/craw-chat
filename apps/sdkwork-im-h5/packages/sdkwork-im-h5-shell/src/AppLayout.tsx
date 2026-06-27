import type { ReactNode } from "react";

import { createImH5AppRoutes } from "./imRoutes";

interface AppLayoutProps {
  children: ReactNode;
  activePath: string;
}

export function AppLayout({ children, activePath }: AppLayoutProps) {
  const routes = createImH5AppRoutes();

  return (
    <div className="im-h5-app-layout">
      <header className="im-h5-app-header">
        <div className="im-h5-app-brand">
          <strong>SDKWork IM</strong>
        </div>
        <nav className="im-h5-app-nav" aria-label="Application surfaces">
          {routes.map((route) => {
            const normalized = route.path.replace(/^#/, "");
            const active = activePath.startsWith(normalized);
            return (
              <a
                key={route.path}
                href={route.path}
                className={active ? "active" : undefined}
                aria-current={active ? "page" : undefined}
              >
                {route.label}
              </a>
            );
          })}
        </nav>
      </header>
      <main className="im-h5-app-content">{children}</main>
    </div>
  );
}
