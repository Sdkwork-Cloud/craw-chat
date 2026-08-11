import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";

export const EmojiManagement: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <PageLayout title={t("user:other_settings.emoji_mgr", "Sticker management")}>
      <div className="flex flex-col items-center py-20 bg-bg-color h-full">
        <div className="flex gap-2 mb-8">
          <div className="w-16 h-16 rounded-xl bg-chat-other-bg flex items-center justify-center text-3xl shadow-sm border border-border-color">
            😀
          </div>
          <div className="w-16 h-16 rounded-xl bg-chat-other-bg flex items-center justify-center text-3xl shadow-sm border border-border-color">
            🤣
          </div>
          <div className="w-16 h-16 rounded-xl bg-chat-other-bg flex items-center justify-center text-3xl shadow-sm border border-border-color">
            😎
          </div>
        </div>
        <p className="text-[15px] text-text-sub mb-8">{t('user.auto_1e3cb2cb', `管理已有表情或添加新表情`)}</p>
        <button
          onClick={() => navigate("/me/emoji")}
          className="w-[200px] h-12 bg-primary-blue text-white rounded-full font-medium active:scale-95 transition-transform"
        >
          去表情商店发现更多
        </button>
      </div>
    </PageLayout>
  );
};
