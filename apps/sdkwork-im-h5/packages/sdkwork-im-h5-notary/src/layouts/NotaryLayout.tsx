import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { useNavigate, useLocation, Outlet } from "react-router";
import {
  FileStack,
  FolderOpen,
  Plus,
  Bell,
  UserRound,
  MoreHorizontal,
  X,
} from "lucide-react";
import { cn, showToast, ActionSheet } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";

// Solid SVG Icons for TabBar
const TabSolidFileStack = ({ className }: any) => {
  const { t } = useTranslation();
  return (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <path
      d="M4 6h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z"
      fill="currentColor"
    />
    <path
      d="M6 3h12"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
    />
  </svg>
);
};


const TabSolidFolder = ({ className }: any) => {
  const { t } = useTranslation();
  
  return (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <path
      d="M3 5.5C3 4.119 4.119 3 5.5 3h4.636a1.5 1.5 0 0 1 1.06.44l1.364 1.36A1.5 1.5 0 0 0 13.62 5.24H18.5C19.881 5.24 21 6.359 21 7.74v10.518c0 1.38-1.119 2.5-2.5 2.5H5.5c-1.38 0-2.5-1.12-2.5-2.5V5.5z"
      fill="currentColor"
    />
  </svg>
);
};


const TabSolidBell = ({ className }: any) => {
  const { t } = useTranslation();
  
  return (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <path
      d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.07-1.63-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.64 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2z"
      fill="currentColor"
    />
  </svg>
);
};


const TabSolidUser = ({ className }: any) => {
  const { t } = useTranslation();
  
  return (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="7" r="5" fill="currentColor" />
    <path
      d="M20 21a8 8 0 0 0-16 0"
      fill="currentColor"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
    />
  </svg>
);
};


const NotaryTabBar: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const location = useLocation();

  const tabs = [
    {
      id: "records",
      outline: FileStack,
      solid: TabSolidFileStack,
      label: t('notary.tab_records', "公证记录"),
      path: "/notary",
    },
    {
      id: "files",
      outline: FolderOpen,
      solid: TabSolidFolder,
      label: t('notary.tab_files', "文件"),
      path: "/notary/files",
    },
    { id: "add", isAdd: true, label: t('notary.tab_add', "公证"), path: "#" },
    {
      id: "messages",
      outline: Bell,
      solid: TabSolidBell,
      label: t('notary.tab_messages', "消息"),
      path: "/notary/messages",
    },
    {
      id: "me",
      outline: UserRound,
      solid: TabSolidUser,
      label: t('notary.tab_me', "我的"),
      path: "/notary/me",
    },
  ];

  const handleAddClick = () => {
  navigate("/notary/create");
  };

  return (
    <nav className="pb-safe pt-2 flex justify-around items-start glass-tab-bar z-50 shrink-0 absolute bottom-0 left-0 right-0 border-t border-border-color">
      {tabs.map((tab) => {
        if (tab.isAdd) {
          return (
            <div
              key="add"
              className="relative flex flex-col items-center -mt-6 cursor-pointer"
              onClick={handleAddClick}
            >
              <div className="w-14 h-14 bg-primary-blue rounded-full shadow-lg shadow-primary-blue/30 flex items-center justify-center active:scale-95 transition-transform border-[4px] border-bg-color border-opacity-50">
                <Plus className="w-8 h-8 text-white stroke-[2.5]" />
              </div>
              <span className="text-[10px] text-text-main font-bold mt-1.5">
                {tab.label}
              </span>
            </div>
          );
        }

        const isActive =
          location.pathname === tab.path ||
          (tab.path === "/notary" && location.pathname === "/notary/");
        const Icon = isActive ? tab.solid : tab.outline;

        return (
          <div
            key={tab.id}
            onClick={() => navigate(tab.path)}
            className={cn(
              "flex flex-col items-center gap-1 text-[10px] cursor-pointer transition-colors mb-1 w-12",
              isActive ? "text-primary-blue" : "text-text-sub",
            )}
          >
            <Icon
              className={cn(
                "w-6 h-6 transition-all",
                isActive ? "opacity-100 scale-110" : "opacity-50 scale-100",
              )}
              strokeWidth={isActive ? undefined : 2}
            />
            <span className="font-medium whitespace-nowrap">{tab.label}</span>
          </div>
        );
      })}
    </nav>
  );
};

export const NotaryLayout: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);

  return (
    <div className="w-full h-full flex flex-col relative bg-bg-color overflow-hidden">
      {/* Mini-program style capsule button */}
      <div className="absolute top-safe right-3 mt-2 z-[60] flex items-center bg-black/10 dark:bg-white/10 backdrop-blur-md rounded-full border border-black/5 dark:border-white/5 shadow-sm overflow-hidden h-8">
        <div
          className="px-2.5 h-full flex items-center justify-center active:bg-black/10 dark:active:bg-white/10"
          onClick={() => setIsActionSheetOpen(true)}
        >
          <MoreHorizontal className="w-4 h-4 text-text-main" />
        </div>
        <div className="h-4 w-[1px] bg-black/10 dark:bg-white/20" />
        <div
          className="px-2.5 h-full flex items-center justify-center active:bg-black/10 dark:active:bg-white/10"
          onClick={() => navigate("/workspace")}
        >
          <X className="w-4 h-4 text-text-main" />
        </div>
      </div>

      <div className="flex-1 overflow-hidden relative">
        <Outlet />
      </div>
      <NotaryTabBar />

      <ActionSheet
        isOpen={isActionSheetOpen}
        onClose={() => setIsActionSheetOpen(false)}
        title={t('notary.auto_prop_116b70', '设置')}
        options={[
          { label: t('notary.action_help', "帮助"), onClick: () => navigate("/settings/help") },
          { label: t('notary.action_about', "关于"), onClick: () => navigate("/settings/about") },
          { label: t('notary.action_settings', "系统设置"), onClick: () => navigate("/settings") },
        ]}
      />
    </div>
  );
};
