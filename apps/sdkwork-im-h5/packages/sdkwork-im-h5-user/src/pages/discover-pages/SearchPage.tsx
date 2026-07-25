import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { Search } from "lucide-react";

export const SearchPage = () => {
  const { t } = useTranslation();
return (
  <PageLayout title={t('user.auto_prop_181a338', '搜一搜')}>
    <div className="p-4">
      <div className="bg-chat-other-bg rounded-lg h-10 flex items-center px-3 gap-2 mb-6">
        <Search className="w-5 h-5 text-text-sub" />
        <input
          type="text"
          placeholder={t('user.auto_prop_n15ee1ae8', '搜索文章、小程序等')}
          className="bg-transparent flex-1 text-[15px] text-text-main outline-none"
        />
      </div>
      <h3 className="text-[14px] text-text-sub mb-4">{t('user.auto_n19db12f3', '搜索指定内容')}</h3>
      <div className="grid grid-cols-3 gap-4 text-center">
        <span className="text-[#2B5CE7] text-[15px]">{t('user.auto_18d4ce8', '朋友圈')}</span>
        <span className="text-[#2B5CE7] text-[15px]">{t('user.auto_cc639', '文章')}</span>
        <span className="text-[#2B5CE7] text-[15px]">{t('user.auto_13b8e2c', '公众号')}</span>
        <span className="text-[#2B5CE7] text-[15px]">{t('user.auto_168ba33', '小程序')}</span>
        <span className="text-[#2B5CE7] text-[15px]">{t('user.auto_12b4bd', '音乐')}</span>
        <span className="text-[#2B5CE7] text-[15px]">{t('user.auto_10e55d', '表情')}</span>
      </div>
    </div>
  </PageLayout>
  );
};
