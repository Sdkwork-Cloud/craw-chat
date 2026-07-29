import React from "react";
import {
  Bell,
  FileStack,
  FolderOpen,
  Plus,
  UserRound,
  X,
} from "lucide-react";
import { Outlet, useLocation, useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

import { cn } from "@sdkwork/im-h5-commons";

const LEFT_NAV_ITEMS = [
  { id: "records", icon: FileStack, path: "/notary", labelKey: "notary.tab_records" },
  { id: "files", icon: FolderOpen, path: "/notary/files", labelKey: "notary.tab_files" },
] as const;

const RIGHT_NAV_ITEMS = [
  { id: "messages", icon: Bell, path: "/notary/messages", labelKey: "notary.tab_messages" },
  { id: "me", icon: UserRound, path: "/notary/me", labelKey: "notary.tab_me" },
] as const;

export const NotaryLayout: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();
  return (
    <div className="relative flex h-full flex-col overflow-hidden bg-bg-color">
      <button
        type="button"
        className="absolute right-3 top-[calc(env(safe-area-inset-top)+10px)] z-[60] flex h-8 w-8 items-center justify-center rounded-full bg-black/5 text-text-main"
        onClick={() => navigate("/workspace")}
        aria-label={t("notary.close", "Close notary")}
      >
        <X className="h-4 w-4" />
      </button>
      <div className="relative min-h-0 flex-1 overflow-hidden">
        <Outlet />
      </div>
      <nav className="glass-tab-bar z-50 flex shrink-0 items-start justify-around border-t border-border-color pb-safe pt-2">
        {LEFT_NAV_ITEMS.map((item) => (
          <NotaryNavItem
            key={item.id}
            active={location.pathname === item.path}
            icon={item.icon}
            label={t(item.labelKey)}
            onClick={() => navigate(item.path)}
          />
        ))}
        <button
          type="button"
          className="-mt-6 flex w-16 flex-col items-center text-text-main"
          onClick={() => navigate("/notary/create")}
        >
          <span className="flex h-14 w-14 items-center justify-center rounded-full border-4 border-bg-color bg-primary-blue text-white shadow-lg">
            <Plus className="h-7 w-7" />
          </span>
          <span className="mt-1 text-[10px] font-medium">
            {t("notary.tab_add", "Create")}
          </span>
        </button>
        {RIGHT_NAV_ITEMS.map((item) => (
          <NotaryNavItem
            key={item.id}
            active={location.pathname === item.path}
            icon={item.icon}
            label={t(item.labelKey)}
            onClick={() => navigate(item.path)}
          />
        ))}
      </nav>
    </div>
  );
};

function NotaryNavItem({
  active,
  icon: Icon,
  label,
  onClick,
}: {
  active: boolean;
  icon: React.ElementType;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={cn(
        "flex w-14 flex-col items-center gap-1 text-[10px]",
        active ? "text-primary-blue" : "text-text-sub",
      )}
      onClick={onClick}
    >
      <Icon className="h-6 w-6" strokeWidth={active ? 2.5 : 1.75} />
      <span className="max-w-full truncate">{label}</span>
    </button>
  );
}
