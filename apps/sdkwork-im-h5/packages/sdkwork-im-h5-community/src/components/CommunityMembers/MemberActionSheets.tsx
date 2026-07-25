import React from "react";
import { useTranslation } from "react-i18next";
import { CommunityMember } from "../../types";

interface MemberActionSheetsProps {
  selectedMember: CommunityMember | null;
  isActionSheetOpen: boolean;
  isBanDurationSheetOpen: boolean;
  onCloseActionSheet: () => void;
  onCloseBanSheet: () => void;
  onOpenBanSheet: () => void;
  onAction: (action: string) => void;
  onBan: (durationText: string) => void;
}

export const MemberActionSheets: React.FC<MemberActionSheetsProps> = ({
  selectedMember,
  isActionSheetOpen,
  isBanDurationSheetOpen,
  onCloseActionSheet,
  onCloseBanSheet,
  onOpenBanSheet,
  onAction,
  onBan,
}) => {
  const { t } = useTranslation();

  if (!selectedMember) return null;

  return (
    <>
      {/* Action Sheet overlay */}
      {isActionSheetOpen && (
        <div className="fixed inset-0 z-50 flex flex-col justify-end pointer-events-auto">
          <div
            className="absolute inset-0 bg-black/40 transition-opacity"
            onClick={onCloseActionSheet}
          />
          <div className="bg-[#F2F2F7] dark:bg-[#1C1C1E] rounded-t-2xl w-full max-w-md mx-auto relative z-10 overflow-hidden pb-safe animate-in slide-in-from-bottom duration-300">
            <div className="p-4 flex items-center justify-center border-b border-black/5 dark:border-white/5 bg-white dark:bg-[#2C2C2E]">
              <span className="text-[13px] text-text-sub">
                对 {selectedMember.name} 的管理操作
              </span>
            </div>

            <div className="flex flex-col">
              {selectedMember.role !== "owner" && (
                <>
                  {selectedMember.role === "member" && (
                    <button
                      onClick={() => onAction("setAdmin")}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
                    >
                      {t("community.auto_n2959bb49", "设为管理员")}
                    </button>
                  )}
                  {selectedMember.role === "admin" && (
                    <button
                      onClick={() => onAction("removeAdmin")}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-orange-500 border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
                    >
                      {t("community.auto_n2ef0c93f", "取消管理员")}
                    </button>
                  )}
                  {selectedMember.status === "active" ? (
                    <button
                      onClick={onOpenBanSheet}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-orange-500 border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
                    >
                      {t("community.auto_f409f", "禁言")}
                    </button>
                  ) : (
                    <button
                      onClick={() => onAction("unban")}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-blue-500 border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
                    >
                      {t("community.auto_40f1d540", "解除禁言")}
                    </button>
                  )}
                  <button
                    onClick={() => onAction("remove")}
                    className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-red-500 active:bg-black/5 dark:active:bg-white/5 transition-colors"
                  >
                    {t("community.auto_38b1a0e7", "移出圈子")}
                  </button>
                </>
              )}
              {selectedMember.role === "owner" && (
                <div className="bg-white dark:bg-[#2C2C2E] py-4 text-[15px] text-text-sub text-center">
                  {t("community.auto_n76ed04e9", "无法对圈主进行操作")}
                </div>
              )}
            </div>

            <div className="mt-2">
              <button
                onClick={onCloseActionSheet}
                className="w-full bg-white dark:bg-[#2C2C2E] py-4 text-[16px] font-medium text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors"
              >
                {t("community.auto_a9472", "取消")}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Ban Duration Sheet overlay */}
      {isBanDurationSheetOpen && (
        <div className="fixed inset-0 z-50 flex flex-col justify-end pointer-events-auto">
          <div
            className="absolute inset-0 bg-black/40 transition-opacity"
            onClick={onCloseBanSheet}
          />
          <div className="bg-[#F2F2F7] dark:bg-[#1C1C1E] rounded-t-2xl w-full max-w-md mx-auto relative z-10 overflow-hidden pb-safe animate-in slide-in-from-bottom duration-300">
            <div className="p-4 flex items-center justify-center border-b border-black/5 dark:border-white/5 bg-white dark:bg-[#2C2C2E]">
              <span className="text-[13px] text-text-sub">
                设置 {selectedMember.name} 的禁言时间
              </span>
            </div>

            <div className="flex flex-col">
              <button
                onClick={() => onBan("1天")}
                className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
              >
                {t("community.auto_5f18", "1天")}
              </button>
              <button
                onClick={() => onBan("1周")}
                className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
              >
                {t("community.auto_5a57", "1周")}
              </button>
              <button
                onClick={() => onBan("1个月")}
                className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
              >
                {t("community.auto_a960f", "1个月")}
              </button>
              <button
                onClick={() => onBan("永久")}
                className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-orange-500 active:bg-black/5 dark:active:bg-white/5 transition-colors"
              >
                {t("community.auto_d690d", "永久")}
              </button>
            </div>

            <div className="mt-2">
              <button
                onClick={onCloseBanSheet}
                className="w-full bg-white dark:bg-[#2C2C2E] py-4 text-[16px] font-medium text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors"
              >
                {t("community.auto_a9472", "取消")}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};
