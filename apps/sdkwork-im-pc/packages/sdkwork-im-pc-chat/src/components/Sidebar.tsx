import React, { useCallback, useState, useEffect } from "react";
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from "motion/react";
import { Avatar, IconButton } from "@sdkwork/im-pc-commons";
import {
  MessageSquare,
  LayoutGrid,
  Sparkles,
  Users,
  Orbit,
  Star,
  Settings,
  ShieldCheck,
  Calendar,
  ReceiptText,
  Store,
  Smartphone,
  BookOpen,
  Cpu,
  GraduationCap,
  Mic,
  Building2,
  Cloud,
} from "lucide-react";
import { toast } from "./Toast";
import { SDKWORK_IM_SESSION_CHANGED_EVENT } from "@sdkwork/im-pc-core";
import { contactService } from "../services/ContactService";
import {
  settingsService,
  DEFAULT_SIDEBAR_MODULES,
  ALWAYS_CONFIGURABLE_MODULES,
} from "../services/SettingsService";
import { MobileLinkModal } from "./MobileLinkModal";
import { ProfileMenuModal } from "./ProfileMenuModal";

interface SidebarProps {
  activeTab: string;
  onLogout: () => void | Promise<void>;
  onTabChange: (tab: string) => void;
  onOpenSettings?: () => void;
  chatUnreadCount?: number;
  friendRequestUnreadCount?: number;
}

const SETTINGS_CHANGED_EVENT = "sdkwork-im-pc:settings-changed";
const MODULE_REFRESH_DEDUPE_MS = 800;
const PINNED_SIDEBAR_MODULES = new Set<string>(DEFAULT_SIDEBAR_MODULES as readonly string[]);

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  onLogout,
  onTabChange,
  onOpenSettings,
  chatUnreadCount = 0,
  friendRequestUnreadCount = 0,
}) => {
  const { t } = useTranslation();
  const [currentUser, setCurrentUser] = useState(() => contactService.getCurrentUser());
  const [showLinkMobile, setShowLinkMobile] = useState(false);
  const [showProfileMenu, setShowProfileMenu] = useState(false);
  const [sidebarModules, setSidebarModules] = useState<string[]>(DEFAULT_SIDEBAR_MODULES);

  const refreshCurrentUser = useCallback(async () => {
    const sessionUser = contactService.getCurrentUser();
    setCurrentUser(contactService.getCurrentUser());
    try {
      const hydratedUser = await contactService.getUserById(sessionUser.id);
      if (hydratedUser?.chatId) {
        setCurrentUser({
          ...sessionUser,
          ...hydratedUser,
          id: sessionUser.id,
          chatId: hydratedUser.chatId,
          name: hydratedUser.name || sessionUser.name,
          avatar: hydratedUser.avatar ?? sessionUser.avatar,
          status: sessionUser.status ?? hydratedUser.status,
        });
        return hydratedUser;
      }
      return sessionUser;
    } catch {
      setCurrentUser(sessionUser);
      return sessionUser;
    }
  }, []);

  const openProfileMenu = useCallback(async () => {
    await refreshCurrentUser();
    setShowProfileMenu(true);
  }, [refreshCurrentUser]);

  useEffect(() => {
    let disposed = false;

    const handleCurrentUserRefresh = () => {
      void refreshCurrentUser().catch(() => {
        if (!disposed) {
          setCurrentUser(contactService.getCurrentUser());
        }
      });
    };

    handleCurrentUserRefresh();
    window.addEventListener("focus", handleCurrentUserRefresh);
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleCurrentUserRefresh);

    return () => {
      disposed = true;
      window.removeEventListener("focus", handleCurrentUserRefresh);
      window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handleCurrentUserRefresh);
    };
  }, [refreshCurrentUser]);

  useEffect(() => {
    let disposed = false;
    let inFlightRefresh: Promise<void> | null = null;
    let lastRefreshStartedAt = 0;

    const refreshSidebarModules = async (force = false) => {
      const now = Date.now();
      if (!force && inFlightRefresh) {
        return inFlightRefresh;
      }
      if (!force && now - lastRefreshStartedAt < MODULE_REFRESH_DEDUPE_MS) {
        return inFlightRefresh ?? Promise.resolve();
      }
      lastRefreshStartedAt = now;

      inFlightRefresh = (async () => {
        const s = await settingsService.getSettings();
        const serverModules = await settingsService.getServerModules();

        const configuredModules = s.sidebarModules?.length
          ? s.sidebarModules
          : DEFAULT_SIDEBAR_MODULES;

        // Ensure 'chat' is always included, and intersect the rest with the server-allowed list.
        const actualModules = configuredModules.filter(
          (m) =>
            m === "chat" ||
            PINNED_SIDEBAR_MODULES.has(m) ||
            (ALWAYS_CONFIGURABLE_MODULES as ReadonlySet<string>).has(m) ||
            serverModules.includes(m),
        );
        if (!disposed) {
          setSidebarModules(Array.from(new Set(["chat", ...actualModules])));
        }
      })().finally(() => {
        inFlightRefresh = null;
      });

      return inFlightRefresh;
    };

    const refreshWhenVisible = () => {
      if (typeof document === "undefined" || document.visibilityState === "visible") {
        void refreshSidebarModules();
      }
    };

    const refreshAfterSettingsChange = () => {
      void refreshSidebarModules(true);
    };

    void refreshSidebarModules(true);
    window.addEventListener("focus", refreshWhenVisible);
    document.addEventListener("visibilitychange", refreshWhenVisible);
    window.addEventListener(SETTINGS_CHANGED_EVENT, refreshAfterSettingsChange);

    return () => {
      disposed = true;
      window.removeEventListener("focus", refreshWhenVisible);
      document.removeEventListener("visibilitychange", refreshWhenVisible);
      window.removeEventListener(SETTINGS_CHANGED_EVENT, refreshAfterSettingsChange);
    };
  }, []);

  return (
    <div className="flex w-[60px] h-full shrink-0 flex-col items-center justify-between bg-[#181818] border-r border-white/5 min-h-0">
      <div className="flex flex-col items-center gap-2 overflow-y-auto custom-scrollbar w-full relative h-full">
        {/* Avatar at Top */}
        <div className="h-[64px] w-full flex items-center justify-center shrink-0">
          <div
            className="relative group cursor-pointer"
            onClick={() => {
              void openProfileMenu();
            }}
          >
            <Avatar
              src={currentUser.avatar}
              alt={currentUser.name}
              className="w-[40px] h-[40px] rounded-lg bg-[#2b2b2d] group-hover:opacity-80 transition-opacity"
            />
            <div className="absolute -bottom-0.5 -right-0.5 w-[10px] h-[10px] bg-[#00b42a] border-2 border-[#181818] rounded-full"></div>
          </div>
        </div>

        {/* Action icons */}
        <div className="flex flex-col items-center gap-2 w-full pt-2">
          {sidebarModules.map((modId) => {
            if (modId === "chat") {
              return (
                <div key="chat" className="relative">
                  <IconButton
                    active={activeTab === "chat"}
                    onClick={() => onTabChange("chat")}
                    title={t('sidebar.chat')}
                  >
                    <MessageSquare
                      size={22}
                      className={
                        activeTab === "chat"
                          ? "fill-blue-500 text-blue-500"
                          : ""
                      }
                    />
                  </IconButton>
                  {chatUnreadCount > 0 && (
                    <div className="absolute top-0 right-0 w-4 h-4 bg-red-500 rounded-full border-2 border-[#181818] text-[9px] text-white flex items-center justify-center font-bold pointer-events-none">
                      {chatUnreadCount > 99 ? "99+" : chatUnreadCount}
                    </div>
                  )}
                </div>
              );
            }
            if (modId === "workspace")
              return (
                <IconButton
                  key="workspace"
                  active={activeTab === "workspace"}
                  onClick={() => onTabChange("workspace")}
                  title={t('sidebar.workspace')}
                >
                  <LayoutGrid
                    size={22}
                    className={
                      activeTab === "workspace"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "orders")
              return (
                <IconButton
                  key="orders"
                  active={activeTab === "orders"}
                  onClick={() => onTabChange("orders")}
                  title={t('sidebar.orders')}
                >
                  <ReceiptText
                    size={22}
                    className={
                      activeTab === "orders"
                        ? "stroke-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "shop")
              return (
                <IconButton
                  key="shop"
                  active={activeTab === "shop"}
                  onClick={() => onTabChange("shop")}
                  title={t('sidebar.shop')}
                >
                  <Store
                    size={22}
                    className={
                      activeTab === "shop"
                        ? "stroke-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "calendar")
              return (
                <IconButton
                  key="calendar"
                  active={activeTab === "calendar"}
                  onClick={() => onTabChange("calendar")}
                  title={t('sidebar.calendar')}
                >
                  <Calendar
                    size={22}
                    className={
                      activeTab === "calendar"
                        ? "stroke-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "notary")
              return (
                <IconButton
                  key="notary"
                  active={activeTab === "notary"}
                  onClick={() => onTabChange("notary")}
                  title={t('sidebar.notary')}
                >
                  <ShieldCheck
                    size={22}
                    className={
                      activeTab === "notary"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "knowledge")
              return (
                <IconButton
                  key="knowledge"
                  active={activeTab === "knowledge"}
                  onClick={() => onTabChange("knowledge")}
                  title={t('sidebar.knowledge')}
                >
                  <BookOpen
                    size={22}
                    className={
                      activeTab === "knowledge"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "drive")
              return (
                <IconButton
                  key="drive"
                  active={activeTab === "drive"}
                  onClick={() => onTabChange("drive")}
                  title={t('sidebar.drive')}
                >
                  <Cloud
                    size={22}
                    className={
                      activeTab === "drive"
                        ? "text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "enterprise")
              return (
                <IconButton
                  key="enterprise"
                  active={activeTab === "enterprise"}
                  onClick={() => onTabChange("enterprise")}
                  title={t('sidebar.enterprise')}
                >
                  <Building2
                    size={22}
                    className={
                      activeTab === "enterprise"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "devices")
              return (
                <IconButton
                  key="devices"
                  active={activeTab === "devices"}
                  onClick={() => onTabChange("devices")}
                  title={t('sidebar.devices')}
                >
                  <Cpu
                    size={22}
                    className={
                      activeTab === "devices"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "community")
              return (
                <IconButton
                  key="community"
                  active={activeTab === "community"}
                  onClick={() => onTabChange("community")}
                  title={t('sidebar.community')}
                >
                  <Orbit
                    size={22}
                    className={
                      activeTab === "community"
                        ? "text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "agent")
              return (
                <IconButton
                  key="agent"
                  active={activeTab === "agent"}
                  onClick={() => onTabChange("agent")}
                  title={t('sidebar.agent')}
                >
                  <Sparkles
                    size={22}
                    className={
                      activeTab === "agent" ? "fill-blue-500 text-blue-500" : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "voice")
              return (
                <IconButton
                  key="voice"
                  active={activeTab === "voice"}
                  onClick={() => onTabChange("voice")}
                  title={t('sidebar.voice')}
                >
                  <Mic
                    size={22}
                    className={
                      activeTab === "voice" ? "text-purple-500" : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "course")
              return (
                <IconButton
                  key="course"
                  active={activeTab === "course"}
                  onClick={() => onTabChange("course")}
                  title={t('sidebar.course')}
                >
                  <GraduationCap
                    size={22}
                    className={
                      activeTab === "course"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            if (modId === "contacts")
              return (
                <div key="contacts" className="relative">
                  <IconButton
                    active={activeTab === "contacts"}
                    onClick={() => onTabChange("contacts")}
                    title={t('sidebar.contacts')}
                  >
                    <Users
                      size={22}
                      className={
                        activeTab === "contacts"
                          ? "fill-blue-500 text-blue-500"
                          : ""
                      }
                    />
                  </IconButton>
                  {friendRequestUnreadCount > 0 && (
                    <div className="absolute top-0 right-0 w-4 h-4 bg-red-500 rounded-full border-2 border-[#181818] text-[9px] text-white flex items-center justify-center font-bold pointer-events-none">
                      {friendRequestUnreadCount > 99 ? "99+" : friendRequestUnreadCount}
                    </div>
                  )}
                </div>
              );
            if (modId === "favorites")
              return (
                <IconButton
                  key="favorites"
                  active={activeTab === "favorites"}
                  onClick={() => onTabChange("favorites")}
                  title={t('sidebar.favorites')}
                >
                  <Star
                    size={22}
                    className={
                      activeTab === "favorites"
                        ? "fill-blue-500 text-blue-500"
                        : ""
                    }
                  />
                </IconButton>
              );
            return null;
          })}
        </div>
      </div>

      <div className="flex flex-col items-center gap-4 py-4 shrink-0">
        <IconButton title={t('sidebar.mobile')} onClick={() => setShowLinkMobile(true)}>
          <Smartphone size={22} />
        </IconButton>
        <IconButton
          title={t('sidebar.settings')}
          onClick={() =>
            onOpenSettings
              ? onOpenSettings()
              : toast(t('sidebar.settingsLocked'), "success")
          }
        >
          <Settings size={22} />
        </IconButton>
      </div>

      <AnimatePresence>
        {showLinkMobile && (
          <MobileLinkModal onClose={() => setShowLinkMobile(false)} />
        )}
      </AnimatePresence>

      <AnimatePresence>
        {showProfileMenu && (
          <ProfileMenuModal
            currentUser={currentUser}
            onClose={() => setShowProfileMenu(false)}
            onLogout={onLogout}
            onTabChange={onTabChange}
            onOpenSettings={onOpenSettings}
          />
        )}
      </AnimatePresence>
    </div>
  );
};
