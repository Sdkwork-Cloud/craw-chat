import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";
import { ChatService } from "@sdkwork/im-h5-chat";

/**
 * Clear chat history — fail-closed (PRD).
 *
 * `ChatService.clearChatHistory` throws `ChatCapabilityUnavailableError`
 * because the generated IM SDK does not expose a clear-history operation.
 * The page invokes the service and surfaces the typed error; it never toasts
 * a fake "已清空" success.
 */
export const ClearChatHistory: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const handleClear = async () => {
    try {
      await ChatService.clearChatHistory("");
      showToast(t("user:other_settings.cleared", "Cleared"));
      navigate(-1);
    } catch (error) {
      console.error("Clear chat history unavailable", error);
      showToast(
        t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."),
      );
    }
  };

  return (
    <PageLayout title={t("user:other_settings.clear_history", "Clear chat history")}>
      <div className="flex flex-col items-center py-10 px-4">
        <p className="text-[15px] text-text-main text-center mb-8">
          {t('user.auto_60cf3dc4', `将清空所有个人和群聊的聊天记录，此操作不可恢复。`)}
        </p>
        <button
          className="w-full h-12 bg-accent-red text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => void handleClear()}
        >
          清空全部聊天记录
        </button>
      </div>
    </PageLayout>
  );
};
