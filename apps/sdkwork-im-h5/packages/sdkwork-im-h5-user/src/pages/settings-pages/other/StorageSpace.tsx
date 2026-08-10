import React, { useState } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const StorageSpace: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [cleared, setCleared] = useState(false);

  return (
    <PageLayout title={t("user:other_settings.storage_space", "存储空间")}>
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-32 h-32 rounded-full border-[12px] border-accent-green flex items-center justify-center mb-6">
          <div className="text-center">
            <div className="text-[24px] font-bold text-text-main">
              {cleared ? "1.2" : "2.4"}
            </div>
            <div className="text-[12px] text-text-sub">GB</div>
          </div>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">
          {t('user.auto_5b27a947', `ClawChat 已用空间`)}
        </h3>
        <p className="text-[14px] text-text-sub text-center mb-8">
          手机剩余空间 {cleared ? "129.2 GB" : "128 GB"}
        </p>
        <button
          className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity mb-4"
          disabled={cleared}
          onClick={() => {
            setCleared(true);
            showToast("缓存已清理");
          }}
        >
          {cleared ? "清理缓存 (0 B)" : "清理缓存 (1.2 GB)"}
        </button>
        <button
          onClick={() => navigate("/settings/general/storage/chat")}
          className="w-full h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
        >
          管理聊天记录 (1.2 GB)
        </button>
      </div>
    </PageLayout>
  );
};
