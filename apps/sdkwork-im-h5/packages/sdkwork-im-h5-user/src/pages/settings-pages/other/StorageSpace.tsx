import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

/**
 * Storage space — fail-closed (PRD).
 *
 * No real storage-usage API is composed: the previously hard-coded GB figures
 * and the fake "缓存已清理" success were fabricated and are removed. The page
 * surfaces the typed unavailable state instead.
 */
export const StorageSpace: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const unavailable = () =>
    showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."));

  return (
    <PageLayout title={t("user:other_settings.storage_space", "Storage")}>
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-32 h-32 rounded-full border-[12px] border-accent-green flex items-center justify-center mb-6">
          <div className="text-center">
            <div className="text-[24px] font-bold text-text-main">—</div>
            <div className="text-[12px] text-text-sub">GB</div>
          </div>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">
          {t('user.auto_5b27a947', `存储空间统计暂不可用`)}
        </h3>
        <p className="text-[14px] text-text-sub text-center mb-8">
          {t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated.")}
        </p>
        <button
          className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity mb-4"
          onClick={unavailable}
        >
          {t("user:other_settings.clear_cache", "Clear cache")}
        </button>
        <button
          onClick={() => navigate("/settings/general/storage/chat")}
          className="w-full h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
        >
          {t("user:other_settings.manage_chat_history", "Manage chat history")}
        </button>
      </div>
    </PageLayout>
  );
};
