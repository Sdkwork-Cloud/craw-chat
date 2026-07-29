import React from "react";
import { FileStack, MessageCircle } from "lucide-react";
import { useLocation, useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

import { cn } from "@sdkwork/im-h5-commons";

const TABS = [
  { id: "chat", icon: MessageCircle, path: "/", labelKey: "common.tabs.chat" },
  { id: "workspace", icon: FileStack, path: "/workspace", labelKey: "common.tabs.workspace" },
] as const;

export const TabBar: React.FC = () => {
  const { t } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();
  return (
    <nav className="glass-tab-bar z-40 flex shrink-0 items-start justify-around border-t border-border-color pb-safe pt-2">
      {TABS.map((tab) => {
        const active = location.pathname === tab.path;
        const Icon = tab.icon;
        return (
          <button
            key={tab.id}
            type="button"
            className={cn(
              "flex min-w-20 flex-col items-center gap-1 text-[10px]",
              active ? "text-primary-blue" : "text-text-sub",
            )}
            onClick={() => navigate(tab.path)}
          >
            <Icon className="h-6 w-6" strokeWidth={active ? 2.5 : 1.75} />
            <span>{t(tab.labelKey)}</span>
          </button>
        );
      })}
    </nav>
  );
};
