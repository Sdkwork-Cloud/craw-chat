import type { ImH5NavigationContribution } from "../contracts";
import { useLocation, useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

import { cn } from "@sdkwork/im-h5-commons";

export interface TabBarProps {
  items: readonly ImH5NavigationContribution[];
}

/**
 * Bottom tab bar restored to the original sdkwork-im-h5 UI: filled icon
 * variant with scale/opacity animation on the active tab, outline icon at
 * reduced opacity otherwise, rendered as an absolute glass bar over the
 * page bottom and hidden outside the main tab paths.
 */
export function TabBar({ items }: TabBarProps) {
  const { t } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();

  const tabPaths = items.map((item) => item.path);
  // Only show the tab bar on main root pages.
  if (!tabPaths.includes(location.pathname)) return null;

  return (
    <nav className="w-full pb-safe pt-2 flex justify-around items-start glass-tab-bar z-40 shrink-0 absolute bottom-0 left-0">
      {items.map((item) => {
        const active = location.pathname === item.path;
        const Icon = active && item.activeIcon ? item.activeIcon : item.icon;
        return (
          <div
            key={item.id}
            onClick={() => navigate(item.path)}
            className={cn(
              "flex flex-col items-center gap-1 text-[10px] cursor-pointer transition-colors mb-1",
              active ? "text-primary-blue" : "text-text-sub"
            )}
          >
            <Icon
              className={cn(
                "w-6 h-6 transition-all",
                active ? "opacity-100 scale-110" : "opacity-50 scale-100"
              )}
              strokeWidth={active ? undefined : 1.5}
            />
            <span>{t(item.labelKey)}</span>
          </div>
        );
      })}
    </nav>
  );
}
