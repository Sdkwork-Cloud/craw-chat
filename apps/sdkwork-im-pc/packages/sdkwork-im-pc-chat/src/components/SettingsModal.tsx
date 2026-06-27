import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import { useTranslation } from "react-i18next";
import {
  X,
  Bell,
  BellRing,
  Shield,
  Paintbrush,
  MonitorSmartphone,
  Volume2,
  Globe,
  LogOut,
  LayoutGrid,
  MessageSquare,
  ReceiptText,
  Store,
  Calendar,
  ShieldCheck,
  BookOpen,
  Sparkles,
  Users,
  Star,
  ArrowUp,
  ArrowDown,
  Cpu,
  GraduationCap,
  Building2,
  Mic,
  Cloud,
  Eye,
  EyeOff,
} from "lucide-react";
import { cn } from "@sdkwork/im-pc-commons";
import { toast } from "./Toast";
import {
  settingsService,
  AppSettings,
  DeviceInfo,
  ALL_APP_MODULES,
  DEFAULT_SIDEBAR_MODULES,
  ALWAYS_CONFIGURABLE_MODULES,
} from "../services/SettingsService";
import {
  getSystemNotificationPermission,
  querySystemNotificationPermission,
  requestSystemNotificationPermission,
  type SystemNotificationPermission,
} from "../services/NotificationService";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  onLogout: () => void | Promise<void>;
}

type SettingsTab =
  | "general"
  | "modules"
  | "notifications"
  | "privacy"
  | "appearance"
  | "devices";

const LANGUAGE_CHANGED_EVENT = "sdkwork-im-pc:language-changed";
const SUPPORTED_LANGUAGES = new Set(["zh-CN", "en-US"]);

function normalizeLanguage(lang: string | undefined) {
  return lang && SUPPORTED_LANGUAGES.has(lang) ? lang : "zh-CN";
}

function notifyLanguageChanged(lang: string) {
  if (typeof window === "undefined") return;
  window.dispatchEvent(new CustomEvent(LANGUAGE_CHANGED_EVENT, {
    detail: { lang },
  }));
}

export const SettingsModal: React.FC<SettingsModalProps> = ({
  isOpen,
  onClose,
  onLogout,
}) => {
  const { i18n, t } = useTranslation();
  const [activeTab, setActiveTab] = useState<SettingsTab>("general");
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const [serverModules, setServerModules] = useState<string[]>([...ALL_APP_MODULES]);
  const [systemNotificationPermission, setSystemNotificationPermission] =
    useState<SystemNotificationPermission>(() => getSystemNotificationPermission());

  useEffect(() => {
    if (isOpen) {
      settingsService.getSettings().then(setSettings);
      settingsService.getDevices().then(setDevices);
      settingsService.getServerModules().then(setServerModules);
      setSystemNotificationPermission(getSystemNotificationPermission());
      querySystemNotificationPermission()
        .then(setSystemNotificationPermission)
        .catch(() => setSystemNotificationPermission("unsupported"));
    }
  }, [isOpen]);

  const updateSetting = async (update: Partial<AppSettings>) => {
    if (!settings) return;
    const nextUpdate = { ...update };
    if (typeof update.lang === "string") {
      nextUpdate.lang = normalizeLanguage(update.lang);
    }
    const newSettings = await settingsService.updateSettings(nextUpdate);
    setSettings(newSettings);
    if (typeof nextUpdate.lang === "string") {
      await i18n.changeLanguage(nextUpdate.lang);
      notifyLanguageChanged(nextUpdate.lang);
    }
  };

  const toggleSystemNotifications = async () => {
    if (!settings) return;
    if (settings.notifySystem) {
      await updateSetting({ notifySystem: false });
      return;
    }

    const permission = await requestSystemNotificationPermission();
    setSystemNotificationPermission(permission);
    if (permission === "granted") {
      await updateSetting({ notifySystem: true });
      toast(t("chat.notification.settings.toast.systemEnabled"), "success");
      return;
    }
    await updateSetting({ notifySystem: false });
    toast(
      permission === "denied"
        ? t("chat.notification.settings.toast.systemDenied")
        : t("chat.notification.settings.toast.systemUnsupported"),
      "error",
      { placement: "bottom-right" },
    );
  };

  const renderNotificationSwitch = (
    checked: boolean,
    onToggle: () => void,
    disabled = false,
  ) => (
    <button
      type="button"
      disabled={disabled}
      aria-pressed={checked}
      className={cn(
        "relative h-6 w-11 rounded-full transition-colors disabled:cursor-not-allowed disabled:opacity-50",
        checked ? "bg-indigo-500" : "bg-gray-600",
      )}
      onClick={(event) => {
        event.stopPropagation();
        onToggle();
      }}
    >
      <span
        className={cn(
          "absolute left-1 top-1 h-4 w-4 rounded-full bg-white transition-transform",
          checked ? "translate-x-5" : "",
        )}
      />
    </button>
  );

  return (
    <AnimatePresence>
      {isOpen && settings && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 bg-black/60 flex items-center justify-center z-[100] backdrop-blur-sm p-4"
        >
          <motion.div
            initial={{ scale: 0.95, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.95, opacity: 0 }}
            transition={{ type: "spring", damping: 25, stiffness: 300 }}
            className="w-[900px] h-[75vh] max-h-[850px] min-h-[500px] bg-[#181818] rounded-2xl shadow-2xl flex overflow-hidden border border-white/10"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Sidebar */}
            <div className="w-[200px] bg-[#1e1e1e] flex flex-col shrink-0 relative z-20 border-r border-white/5">
              <div className="h-[72px] px-6 flex items-center shrink-0">
                <span className="text-lg font-semibold text-gray-200 tracking-wide">
                  {t("settingsModal.title")}
                </span>
              </div>
              <div className="flex-1 overflow-y-auto px-3 py-2 space-y-1 custom-scrollbar">
                <button
                  onClick={() => setActiveTab("general")}
                  className={cn(
                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-all font-medium",
                    activeTab === "general"
                      ? "bg-indigo-500 text-white shadow-md shadow-indigo-500/20"
                      : "text-gray-400 hover:text-gray-200 hover:bg-white/5",
                  )}
                >
                  <Volume2 size={16} /> {t("settingsModal.tabs.general")}
                </button>
                <button
                  onClick={() => setActiveTab("modules")}
                  className={cn(
                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-all font-medium",
                    activeTab === "modules"
                      ? "bg-indigo-500 text-white shadow-md shadow-indigo-500/20"
                      : "text-gray-400 hover:text-gray-200 hover:bg-white/5",
                  )}
                >
                  <LayoutGrid size={16} /> {t("settingsModal.tabs.modules")}
                </button>
                <button
                  onClick={() => setActiveTab("notifications")}
                  className={cn(
                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-all font-medium",
                    activeTab === "notifications"
                      ? "bg-indigo-500 text-white shadow-md shadow-indigo-500/20"
                      : "text-gray-400 hover:text-gray-200 hover:bg-white/5",
                  )}
                >
                  <Bell size={16} /> {t("settingsModal.tabs.notifications")}
                </button>
                <button
                  onClick={() => setActiveTab("privacy")}
                  className={cn(
                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-all font-medium",
                    activeTab === "privacy"
                      ? "bg-indigo-500 text-white shadow-md shadow-indigo-500/20"
                      : "text-gray-400 hover:text-gray-200 hover:bg-white/5",
                  )}
                >
                  <Shield size={16} /> {t("settingsModal.tabs.privacy")}
                </button>
                <button
                  onClick={() => setActiveTab("appearance")}
                  className={cn(
                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-all font-medium",
                    activeTab === "appearance"
                      ? "bg-indigo-500 text-white shadow-md shadow-indigo-500/20"
                      : "text-gray-400 hover:text-gray-200 hover:bg-white/5",
                  )}
                >
                  <Paintbrush size={16} /> {t("settingsModal.tabs.appearance")}
                </button>
                <button
                  onClick={() => setActiveTab("devices")}
                  className={cn(
                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-all font-medium",
                    activeTab === "devices"
                      ? "bg-indigo-500 text-white shadow-md shadow-indigo-500/20"
                      : "text-gray-400 hover:text-gray-200 hover:bg-white/5",
                  )}
                >
                  <Cpu size={16} /> {t("settingsModal.tabs.devices")}
                </button>
              </div>
              <div className="p-3 border-t border-white/5">
                <button
                  onClick={() => {
                    onClose();
                    void onLogout();
                  }}
                  className="w-full flex items-center justify-center gap-2 px-3 py-2.5 rounded-xl text-sm text-red-400 hover:text-red-300 hover:bg-red-500/10 transition-colors font-medium"
                >
                  <LogOut size={16} /> {t("settingsModal.logout")}
                </button>
              </div>
            </div>

            {/* Content */}
            <div className="flex-1 flex flex-col min-w-0 bg-[#181818] relative z-10">
              {/* Header Sticky */}
              <div className="h-[72px] px-8 flex items-center justify-between shrink-0 border-b border-white/5 bg-[#181818]/90 backdrop-blur-md sticky top-0 z-30">
                <h2 className="text-lg font-medium text-gray-100 tracking-wide">
                  {t(`settingsModal.tabs.${activeTab}`)}
                </h2>
                <button
                  onClick={onClose}
                  className="w-8 h-8 flex items-center justify-center text-gray-400 hover:text-gray-100 hover:bg-white/10 rounded-full transition-all focus:outline-none focus:ring-2 focus:ring-indigo-500/50"
                >
                  <X size={18} />
                </button>
              </div>

              <div className="flex-1 overflow-y-auto custom-scrollbar">
                <div className="p-8 pb-16 w-full">
                  <AnimatePresence mode="wait">
                    {activeTab === "general" && (
                      <motion.div
                        key="general"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                        className="space-y-6"
                      >
                        <div className="space-y-6">
                          <div className="space-y-2">
                            <h3 className="text-sm font-medium text-gray-300">
                              {t("settingsModal.general.language")}
                            </h3>
                            <div className="flex items-center gap-3 p-3 bg-[#2b2b2d] rounded-xl border border-white/5 flex-wrap">
                              <Globe
                                size={18}
                                className="text-gray-400 shrink-0"
                              />
                              <select
                                className="flex-1 bg-transparent border-none text-sm text-gray-200 outline-none hover:cursor-pointer min-w-[120px]"
                                value={settings.lang}
                                onChange={(e) =>
                                  updateSetting({ lang: e.target.value })
                                }
                              >
                                <option value="zh-CN">{t("settingsModal.general.languageZh")}</option>
                                <option value="en-US">{t("settingsModal.general.languageEn")}</option>
                              </select>
                            </div>
                          </div>
                          <div className="space-y-2">
                            <h3 className="text-sm font-medium text-gray-300">
                              {t("settingsModal.general.autoStartTitle")}
                            </h3>
                            <div className="flex items-center justify-between p-4 bg-[#2b2b2d] rounded-xl border border-white/5">
                              <span className="text-sm text-gray-300">
                                {t("settingsModal.general.autoStartDescription")}
                              </span>
                              <label className="relative inline-flex items-center cursor-pointer">
                                <input
                                  type="checkbox"
                                  className="sr-only peer"
                                  checked={settings.autoStart}
                                  onChange={(e) =>
                                    updateSetting({
                                      autoStart: e.target.checked,
                                    })
                                  }
                                />
                                <div className="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500"></div>
                              </label>
                            </div>
                          </div>
                        </div>
                      </motion.div>
                    )}

                    {activeTab === "modules" && (
                      <motion.div
                        key="modules"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                        className="space-y-6"
                      >
                        <div className="space-y-4">
                          <p className="text-sm text-gray-400 mb-4">
                            {t("settingsModal.modules.description")}
                          </p>
                          <div className="flex flex-col gap-3">
                            {(() => {
                              const configuredAvailable = [
                                { id: "chat", icon: MessageSquare },
                                { id: "workspace", icon: LayoutGrid },
                                { id: "orders", icon: ReceiptText },
                                { id: "shop", icon: Store },
                                { id: "calendar", icon: Calendar },
                                { id: "notary", icon: ShieldCheck },
                                { id: "knowledge", icon: BookOpen },
                                { id: "drive", icon: Cloud },
                                { id: "devices", icon: Cpu },
                                { id: "community", icon: Users },
                                { id: "course", icon: GraduationCap },
                                { id: "agent", icon: Sparkles },
                                { id: "voice", icon: Mic },
                                { id: "enterprise", icon: Building2 },
                                { id: "contacts", icon: Users },
                                { id: "favorites", icon: Star },
                              ].map((mod) => ({
                                ...mod,
                                name: t(`sidebar.${mod.id}`),
                              }));

                              const allAvailable = configuredAvailable.filter(
                                (mod) =>
                                  mod.id === "chat" ||
                                  (DEFAULT_SIDEBAR_MODULES as readonly string[]).includes(mod.id) ||
                                  (ALWAYS_CONFIGURABLE_MODULES as ReadonlySet<string>).has(mod.id) ||
                                  serverModules.includes(mod.id),
                              );

                              const currentModules =
                                settings.sidebarModules?.length
                                  ? settings.sidebarModules
                                  : DEFAULT_SIDEBAR_MODULES;
                              const currentOrder = currentModules.filter((id) =>
                                allAvailable.some((m) => m.id === id),
                              );

                              const sorted = [
                                ...currentOrder
                                  .map((id) =>
                                    allAvailable.find((m) => m.id === id),
                                  )
                                  .filter(Boolean),
                                ...allAvailable.filter(
                                  (m) => !currentOrder.includes(m.id),
                                ),
                              ] as typeof allAvailable;

                              return sorted.map((mod, index) => {
                                const isChecked =
                                  mod.id === "chat"
                                    ? true
                                    : currentOrder.includes(mod.id);

                                const handleMoveUp = () => {
                                  if (index === 0) return;
                                  const newOrder = [...sorted];
                                  const temp = newOrder[index - 1];
                                  newOrder[index - 1] = newOrder[index];
                                  newOrder[index] = temp;

                                  const newSidebarModules = newOrder
                                    .filter(
                                      (m) =>
                                        m.id === "chat" ||
                                        currentOrder.includes(m.id),
                                    )
                                    .map((m) => m.id);
                                  updateSetting({
                                    sidebarModules: newSidebarModules,
                                  });
                                };

                                const handleMoveDown = () => {
                                  if (index >= currentOrder.length - 1) return; // Prevent moving down past the last enabled item
                                  const newOrder = [...sorted];
                                  const temp = newOrder[index + 1];
                                  newOrder[index + 1] = newOrder[index];
                                  newOrder[index] = temp;

                                  const newSidebarModules = newOrder
                                    .filter(
                                      (m) =>
                                        m.id === "chat" ||
                                        currentOrder.includes(m.id),
                                    )
                                    .map((m) => m.id);
                                  updateSetting({
                                    sidebarModules: newSidebarModules,
                                  });
                                };

                                return (
                                  <div
                                    key={mod.id}
                                    className="flex items-center justify-between p-4 bg-[#2b2b2d] rounded-xl border border-white/5 transition-colors hover:bg-[#343438]"
                                  >
                                    <div className="flex items-center gap-3">
                                      <mod.icon
                                        size={18}
                                        className="text-gray-400"
                                      />
                                      <span className="text-sm font-medium text-gray-200">
                                        {mod.name}
                                      </span>
                                    </div>
                                    <div className="flex items-center gap-6">
                                      <div className="flex items-center gap-2">
                                        <button
                                          disabled={!isChecked || index === 0}
                                          onClick={handleMoveUp}
                                          className="p-1 px-2 text-gray-500 hover:text-gray-200 hover:bg-white/10 rounded transition-colors disabled:opacity-0 disabled:pointer-events-none"
                                          title={t("settingsModal.modules.moveUp")}
                                        >
                                          <ArrowUp size={16} />
                                        </button>
                                        <button
                                          disabled={
                                            !isChecked ||
                                            index >= currentOrder.length - 1
                                          }
                                          onClick={handleMoveDown}
                                          className="p-1 px-2 text-gray-500 hover:text-gray-200 hover:bg-white/10 rounded transition-colors disabled:opacity-0 disabled:pointer-events-none"
                                          title={t("settingsModal.modules.moveDown")}
                                        >
                                          <ArrowDown size={16} />
                                        </button>
                                      </div>
                                      <label
                                        className={cn(
                                          "relative inline-flex items-center",
                                          mod.id === "chat"
                                            ? "cursor-not-allowed opacity-50"
                                            : "cursor-pointer",
                                        )}
                                      >
                                        <input
                                          type="checkbox"
                                          className="sr-only peer"
                                          checked={isChecked}
                                          disabled={mod.id === "chat"}
                                          onChange={(e) => {
                                            if (mod.id === "chat") return;
                                            const checked = e.target.checked;
                                            let newModules;
                                            if (checked) {
                                              // When enabling, keep it in the current array, just add it to 'currentOrder' effectively
                                              // Since it's appended or kept at its current index?
                                              // Actually it makes sense to keep it at its visual index. So the new array of checked modules should match the visual array.
                                              newModules = sorted
                                                .filter(
                                                  (m) =>
                                                    m.id === mod.id ||
                                                    currentOrder.includes(m.id),
                                                )
                                                .map((m) => m.id);
                                            } else {
                                              newModules = currentOrder.filter(
                                                (id) => id !== mod.id,
                                              );
                                            }
                                            updateSetting({
                                              sidebarModules: newModules,
                                            });
                                          }}
                                        />
                                        <div className="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500"></div>
                                      </label>
                                    </div>
                                  </div>
                                );
                              });
                            })()}
                          </div>
                        </div>
                      </motion.div>
                    )}

                    {activeTab === "notifications" && (
                      <motion.div
                        key="notifications"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                        className="space-y-6"
                      >
                        <div className="space-y-6">
                          <div className="rounded-xl border border-indigo-500/20 bg-indigo-500/10 p-4">
                            <div className="flex items-start gap-3">
                              <BellRing size={18} className="mt-0.5 shrink-0 text-indigo-300" />
                              <div>
                                <div className="text-sm font-medium text-gray-100">
                                  {t("chat.notification.settings.title")}
                                </div>
                                <div className="mt-1 text-xs leading-5 text-gray-400">
                                  {t("chat.notification.settings.description")}
                                </div>
                              </div>
                            </div>
                          </div>

                          <div className="space-y-3">
                            <div
                              className="flex cursor-pointer items-center justify-between gap-4 rounded-xl border border-white/5 bg-[#2b2b2d] p-4 transition-colors hover:border-indigo-500/30"
                              onClick={() => updateSetting({ notifyDesktop: !settings.notifyDesktop })}
                            >
                              <div className="flex min-w-0 items-start gap-3">
                                <Bell size={18} className="mt-0.5 shrink-0 text-gray-400" />
                                <div>
                                  <div className="text-sm font-medium text-gray-200">
                                    {t("chat.notification.settings.appPopup.title")}
                                  </div>
                                  <div className="mt-1 text-xs leading-5 text-gray-500">
                                    {t("chat.notification.settings.appPopup.description")}
                                  </div>
                                </div>
                              </div>
                              {renderNotificationSwitch(
                                settings.notifyDesktop,
                                () => updateSetting({ notifyDesktop: !settings.notifyDesktop }),
                              )}
                            </div>

                            <div
                              className="flex cursor-pointer items-center justify-between gap-4 rounded-xl border border-white/5 bg-[#2b2b2d] p-4 transition-colors hover:border-indigo-500/30"
                              onClick={() => updateSetting({ notifySound: !settings.notifySound })}
                            >
                              <div className="flex min-w-0 items-start gap-3">
                                <Volume2 size={18} className="mt-0.5 shrink-0 text-gray-400" />
                                <div>
                                  <div className="text-sm font-medium text-gray-200">
                                    {t("chat.notification.settings.sound.title")}
                                  </div>
                                  <div className="mt-1 text-xs leading-5 text-gray-500">
                                    {t("chat.notification.settings.sound.description")}
                                  </div>
                                </div>
                              </div>
                              {renderNotificationSwitch(
                                settings.notifySound,
                                () => updateSetting({ notifySound: !settings.notifySound }),
                              )}
                            </div>

                            <div
                              className="flex cursor-pointer items-center justify-between gap-4 rounded-xl border border-white/5 bg-[#2b2b2d] p-4 transition-colors hover:border-indigo-500/30"
                              onClick={() => {
                                void toggleSystemNotifications();
                              }}
                            >
                              <div className="flex min-w-0 items-start gap-3">
                                <MonitorSmartphone size={18} className="mt-0.5 shrink-0 text-gray-400" />
                                <div>
                                  <div className="text-sm font-medium text-gray-200">
                                    {t("chat.notification.settings.system.title")}
                                  </div>
                                  <div className="mt-1 text-xs leading-5 text-gray-500">
                                    {t("chat.notification.settings.system.description", {
                                      permission: t(`chat.notification.settings.permission.${systemNotificationPermission}`),
                                    })}
                                  </div>
                                </div>
                              </div>
                              {renderNotificationSwitch(
                                settings.notifySystem && systemNotificationPermission === "granted",
                                () => {
                                  void toggleSystemNotifications();
                                },
                                systemNotificationPermission === "unsupported",
                              )}
                            </div>

                            <div
                              className="flex cursor-pointer items-center justify-between gap-4 rounded-xl border border-white/5 bg-[#2b2b2d] p-4 transition-colors hover:border-indigo-500/30"
                              onClick={() => updateSetting({
                                notificationWhenFocused: !settings.notificationWhenFocused,
                              })}
                            >
                              <div className="flex min-w-0 items-start gap-3">
                                <MessageSquare size={18} className="mt-0.5 shrink-0 text-gray-400" />
                                <div>
                                  <div className="text-sm font-medium text-gray-200">
                                    {t("chat.notification.settings.focused.title")}
                                  </div>
                                  <div className="mt-1 text-xs leading-5 text-gray-500">
                                    {t("chat.notification.settings.focused.description")}
                                  </div>
                                </div>
                              </div>
                              {renderNotificationSwitch(
                                settings.notificationWhenFocused,
                                () => updateSetting({
                                  notificationWhenFocused: !settings.notificationWhenFocused,
                                }),
                              )}
                            </div>
                          </div>

                          <div className="space-y-3 rounded-xl border border-white/5 bg-[#2b2b2d] p-4">
                            <div className="flex items-center gap-2 text-sm font-medium text-gray-200">
                              {settings.notificationPreview === "hidden" ? (
                                <EyeOff size={16} className="text-gray-400" />
                              ) : (
                                <Eye size={16} className="text-gray-400" />
                              )}
                              {t("chat.notification.settings.preview.title")}
                            </div>
                            <div className="grid gap-2 sm:grid-cols-3">
                              {[
                                {
                                  description: t("chat.notification.settings.preview.senderAndPreview.description"),
                                  label: t("chat.notification.settings.preview.senderAndPreview.label"),
                                  value: "sender-and-preview",
                                },
                                {
                                  description: t("chat.notification.settings.preview.senderOnly.description"),
                                  label: t("chat.notification.settings.preview.senderOnly.label"),
                                  value: "sender-only",
                                },
                                {
                                  description: t("chat.notification.settings.preview.hidden.description"),
                                  label: t("chat.notification.settings.preview.hidden.label"),
                                  value: "hidden",
                                },
                              ].map((option) => (
                                <button
                                  key={option.value}
                                  type="button"
                                  className={cn(
                                    "rounded-lg border p-3 text-left transition-colors",
                                    settings.notificationPreview === option.value
                                      ? "border-indigo-500/60 bg-indigo-500/15 text-gray-100"
                                      : "border-white/5 bg-[#202022] text-gray-400 hover:border-white/15 hover:text-gray-200",
                                  )}
                                  onClick={() => updateSetting({
                                    notificationPreview: option.value as AppSettings["notificationPreview"],
                                  })}
                                >
                                  <div className="text-xs font-medium">
                                    {option.label}
                                  </div>
                                  <div className="mt-1 text-[11px] leading-4 text-gray-500">
                                    {option.description}
                                  </div>
                                </button>
                              ))}
                            </div>
                          </div>

                          <button
                            onClick={async () => {
                              await updateSetting({
                                notifyDesktop: true,
                                notifySound: true,
                                notifySystem: false,
                                notificationPreview: "sender-and-preview",
                                notificationWhenFocused: false,
                              });
                              toast(t("chat.notification.settings.toast.reset"), "success", { placement: "bottom-right" });
                            }}
                            className="text-sm font-medium text-indigo-400 hover:text-indigo-300"
                          >
                            {t("chat.notification.settings.reset")}
                          </button>
                        </div>
                      </motion.div>
                    )}

                    {activeTab === "privacy" && (
                      <motion.div
                        key="privacy"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                        className="space-y-6"
                      >
                        <div className="space-y-4">
                          <div className="p-4 bg-[#2b2b2d] rounded-xl border border-white/5 flex flex-col gap-3">
                            <div className="flex items-center justify-between">
                              <span className="text-sm text-gray-200">
                                {t("settingsModal.privacy.showOnline")}
                              </span>
                              <label className="relative inline-flex items-center cursor-pointer">
                                <input
                                  type="checkbox"
                                  className="sr-only peer"
                                  checked={settings.privacyShowOnline}
                                  onChange={(e) =>
                                    updateSetting({
                                      privacyShowOnline: e.target.checked,
                                    })
                                  }
                                />
                                <div className="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500"></div>
                              </label>
                            </div>
                          </div>
                          <button
                            onClick={async () => {
                              await settingsService.clearCache();
                              toast(t("settingsModal.privacy.cacheCleared"), "success");
                            }}
                            className="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-500 text-sm font-medium rounded-lg transition-colors border border-red-500/20"
                          >
                            {t("settingsModal.privacy.clearCache")}
                          </button>
                        </div>
                      </motion.div>
                    )}

                    {activeTab === "appearance" && (
                      <motion.div
                        key="appearance"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                        className="space-y-6"
                      >
                        <div className="space-y-6">
                          <div>
                            <h3 className="text-sm font-medium text-gray-300 mb-4">
                              {t("settingsModal.appearance.themeTitle")}
                            </h3>
                            <div className="flex items-center gap-4">
                              <button
                                onClick={() =>
                                  updateSetting({ theme: "light" })
                                }
                                className={cn(
                                  "flex flex-col gap-2 items-center",
                                  settings.theme === "light"
                                    ? "opacity-100"
                                    : "opacity-50 hover:opacity-80 transition-opacity",
                                )}
                              >
                                <div
                                  className={cn(
                                    "w-24 h-16 rounded-xl border-2 flex items-center justify-center font-medium shadow-sm transition-all relative overflow-hidden",
                                    settings.theme === "light"
                                      ? "border-indigo-500"
                                      : "border-transparent",
                                  )}
                                  style={{ backgroundColor: "#ffffff" }}
                                >
                                  <div
                                    className="absolute inset-x-0 top-0 h-4 border-b border-[#00000010]"
                                    style={{ backgroundColor: "#f3f4f6" }}
                                  ></div>
                                </div>
                                <span className="text-xs text-gray-300">
                                  {t("settingsModal.appearance.light")}
                                </span>
                              </button>
                              <button
                                onClick={() => updateSetting({ theme: "dark" })}
                                className={cn(
                                  "flex flex-col gap-2 items-center",
                                  settings.theme === "dark"
                                    ? "opacity-100"
                                    : "opacity-50 hover:opacity-80 transition-opacity",
                                )}
                              >
                                <div
                                  className={cn(
                                    "w-24 h-16 rounded-xl border-2 flex items-center justify-center font-medium shadow-sm transition-all relative overflow-hidden",
                                    settings.theme === "dark"
                                      ? "border-indigo-500"
                                      : "border-transparent",
                                  )}
                                  style={{ backgroundColor: "#1e1e20" }}
                                >
                                  <div
                                    className="absolute inset-x-0 top-0 h-4 border-b border-[#ffffff10]"
                                    style={{ backgroundColor: "#2b2b2d" }}
                                  ></div>
                                </div>
                                <span className="text-xs text-gray-300">
                                  {t("settingsModal.appearance.dark")}
                                </span>
                              </button>
                              <button
                                onClick={() =>
                                  updateSetting({ theme: "system" })
                                }
                                className={cn(
                                  "flex flex-col gap-2 items-center",
                                  settings.theme === "system"
                                    ? "opacity-100"
                                    : "opacity-50 hover:opacity-80 transition-opacity",
                                )}
                              >
                                <div
                                  className={cn(
                                    "w-24 h-16 rounded-xl border-2 flex items-center justify-center font-medium shadow-sm transition-all overflow-hidden",
                                    settings.theme === "system"
                                      ? "border-indigo-500"
                                      : "border-transparent",
                                  )}
                                  style={{
                                    background:
                                      "linear-gradient(to right, #f3f4f6 50%, #1e1e20 50%)",
                                  }}
                                >
                                  <div
                                    className="absolute inset-x-0 top-0 h-4 border-b border-black/5"
                                    style={{
                                      background:
                                        "linear-gradient(to right, #e5e7eb 50%, #2b2b2d 50%)",
                                    }}
                                  ></div>
                                </div>
                                <span className="text-xs text-gray-300">
                                  {t("settingsModal.appearance.system")}
                                </span>
                              </button>
                            </div>
                          </div>
                        </div>
                      </motion.div>
                    )}

                    {activeTab === "devices" && (
                      <motion.div
                        key="devices"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                        className="space-y-6"
                      >
                        <div className="space-y-4">
                          <div className="p-4 bg-[#2b2b2d] rounded-xl border border-indigo-500/30 flex items-center justify-between">
                            <div className="flex items-center gap-4">
                              <MonitorSmartphone
                                size={24}
                                className="text-indigo-400"
                              />
                              <div>
                                <div className="text-sm font-medium text-gray-200">
                                  {t("settingsModal.devices.currentDevice", { name: devices[0]?.name ?? "MacBook Pro" })}
                                </div>
                                <div className="text-xs text-indigo-400 mt-1">
                                  {t("settingsModal.devices.online")}
                                </div>
                              </div>
                            </div>
                          </div>
                          {devices.map((device) => (
                            <div
                              key={device.id}
                              className="p-4 bg-[#2b2b2d] rounded-xl border border-white/5 flex items-center justify-between"
                            >
                              <div className="flex items-center gap-4">
                                <MonitorSmartphone
                                  size={24}
                                  className="text-gray-500"
                                />
                                <div>
                                  <div className="text-sm font-medium text-gray-200">
                                    {device.name}
                                  </div>
                                  <div className="text-xs text-gray-500 mt-1">
                                    {device.time}
                                  </div>
                                </div>
                              </div>
                              <button
                                onClick={async () => {
                                  await settingsService.removeDevice(device.id);
                                  setDevices(
                                    devices.filter((d) => d.id !== device.id),
                                  );
                                  toast(t("settingsModal.devices.signedOut", { name: device.name }), "success");
                                }}
                                className="text-xs text-red-400 hover:text-red-300 font-medium px-3 py-1.5 rounded-lg border border-red-500/20 bg-red-500/10 transition-colors"
                              >
                                {t("settingsModal.devices.signOut")}
                              </button>
                            </div>
                          ))}
                        </div>
                      </motion.div>
                    )}
                  </AnimatePresence>
                </div>
              </div>
            </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
