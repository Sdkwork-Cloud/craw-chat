import React from "react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { Avatar } from "@sdkwork/im-pc-commons";
import { Settings, Star } from "lucide-react";
import { toast } from "./Toast";
import type { User } from "@sdkwork/im-pc-types";

interface ProfileMenuModalProps {
  currentUser: User;
  onClose: () => void;
  onLogout: () => void | Promise<void>;
  onTabChange: (tab: string) => void;
  onOpenSettings?: () => void;
}

export const ProfileMenuModal: React.FC<ProfileMenuModalProps> = ({
  currentUser,
  onClose,
  onLogout,
  onTabChange,
  onOpenSettings,
}) => {
  const { t } = useTranslation();
  const currentUserChatId = currentUser.chatId ?? "";

  const copyCurrentUserId = async () => {
    if (!currentUserChatId) {
      toast(t("profile.toast.chatIdNotReady"), "error");
      return;
    }

    try {
      await navigator.clipboard.writeText(currentUserChatId);
      toast(t("profile.toast.chatIdCopied"), "success");
    } catch {
      toast(t("profile.toast.copyChatIdFailed"), "error");
    }
  };

  return (
    <>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="fixed inset-0 z-40"
        onClick={onClose}
      />
      <motion.div
        initial={{ opacity: 0, x: -20 }}
        animate={{ opacity: 1, x: 0 }}
        exit={{ opacity: 0, x: -20 }}
        transition={{ type: "spring", damping: 25, stiffness: 300 }}
        className="absolute left-16 top-12 z-50 w-80 overflow-hidden rounded-2xl border border-white/10 bg-[#1e1e1e] shadow-2xl"
      >
        <div className="flex items-center gap-4 border-b border-white/5 p-5">
          <Avatar
            src={currentUser.avatar}
            alt={currentUser.name}
            className="h-16 w-16 rounded-xl bg-[#2b2b2d]"
          />
          <div className="flex min-w-0 flex-1 flex-col">
            <h3 className="truncate text-xl font-bold text-gray-100">
              {currentUser.name}
            </h3>
            <button
              type="button"
              title={t("profile.copyChatId")}
              onClick={copyCurrentUserId}
              disabled={!currentUserChatId}
              className="mt-1 flex min-w-0 items-center gap-1 text-left text-xs text-gray-500 transition-colors hover:text-gray-300 disabled:cursor-not-allowed disabled:hover:text-gray-500"
            >
              <span className="shrink-0">{t("profile.chatIdLabel")}:</span>
              <span className="truncate font-mono">{currentUserChatId}</span>
            </button>
            <div className="mt-1 flex items-center gap-2 text-sm text-gray-400">
              <div className="h-2 w-2 rounded-full bg-[#00b42a]" />
              {t("profile.status.online")}
            </div>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-1 border-b border-white/5 p-2 text-center">
          <div
            className="flex cursor-pointer flex-col items-center rounded-xl p-3 transition-colors hover:bg-white/5"
            onClick={() => {
              onClose();
              onTabChange("favorites");
            }}
          >
            <Star size={20} className="mb-1 text-gray-400" />
            <span className="text-xs text-gray-400">{t("profile.actions.favorites")}</span>
          </div>
          <div
            className="flex cursor-pointer flex-col items-center rounded-xl p-3 transition-colors hover:bg-white/5"
            onClick={() => {
              onClose();
              onOpenSettings?.();
            }}
          >
            <Settings size={20} className="mb-1 text-gray-400" />
            <span className="text-xs text-gray-400">{t("profile.actions.settings")}</span>
          </div>
        </div>

        <div className="p-2">
          <button className="flex w-full items-center gap-3 rounded-lg px-4 py-2.5 text-left text-sm text-gray-300 transition-colors hover:bg-white/5 hover:text-white">
            <span className="flex w-5 justify-center">
              <div className="h-2.5 w-2.5 rounded-full border-2 border-[#1e1e1e] bg-red-500" />
            </span>
            {t("profile.status.busy")}
          </button>
          <button className="flex w-full items-center gap-3 rounded-lg px-4 py-2.5 text-left text-sm text-gray-300 transition-colors hover:bg-white/5 hover:text-white">
            <span className="flex w-5 justify-center">
              <div className="h-2.5 w-2.5 rounded-full border-2 border-[#1e1e1e] bg-yellow-500" />
            </span>
            {t("profile.status.away")}
          </button>
          <div className="mx-2 my-2 h-px bg-white/5" />
          <button
            onClick={() => {
              onClose();
              void onLogout();
            }}
            className="flex w-full items-center gap-3 rounded-lg px-4 py-2 text-left text-sm text-red-400 transition-colors hover:bg-red-500/10"
          >
            {t("profile.actions.logout")}
          </button>
        </div>
      </motion.div>
    </>
  );
};
