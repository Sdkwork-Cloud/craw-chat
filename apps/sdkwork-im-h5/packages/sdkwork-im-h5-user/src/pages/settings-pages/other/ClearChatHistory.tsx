import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const ClearChatHistory: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <PageLayout title={t("user:other_settings.clear_history", "清空聊天记录")}>
      <div className="flex flex-col items-center py-10 px-4">
        <p className="text-[15px] text-text-main text-center mb-8">
          {t('user.auto_60cf3dc4', `将清空所有个人和群聊的聊天记录，此操作不可恢复。`)}
        </p>
        <button
          className="w-full h-12 bg-[#FA5151] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => {
            showToast("已清空");
            navigate(-1);
          }}
        >
          清空全部聊天记录
        </button>
      </div>
    </PageLayout>
  );
};
