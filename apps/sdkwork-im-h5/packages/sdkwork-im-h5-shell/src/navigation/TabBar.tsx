import type { ImH5NavigationContribution } from "../contracts";
import { useLocation, useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

import { cn } from "@sdkwork/im-h5-commons";

export interface TabBarProps {
  items: readonly ImH5NavigationContribution[];
}

export function TabBar({ items }: TabBarProps) {
  const { t } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();
  return (
    <nav className="glass-tab-bar z-40 flex shrink-0 items-start justify-around border-t border-border-color pb-safe pt-2">
      {items.map((item) => {
        const active = location.pathname === item.path;
        const Icon = item.icon;
        return (
          <button
            key={item.id}
            type="button"
            className={cn(
              "flex min-w-20 flex-col items-center gap-1 text-[10px]",
              active ? "text-primary-blue" : "text-text-sub",
            )}
            onClick={() => navigate(item.path)}
          >
            <Icon className="h-6 w-6" strokeWidth={active ? 2.5 : 1.75} />
            <span>{t(item.labelKey)}</span>
          </button>
        );
      })}
    </nav>
  );
}
