import React from "react";
import { Search } from "lucide-react";
import { useTranslation } from "react-i18next";

export const ShoppingSearchBarAndBanner: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="px-4 py-2">
      <div className="bg-chat-other-bg rounded-lg h-10 flex items-center px-4 gap-2 mb-4">
        <Search className="w-5 h-5 text-text-sub" />
        <input
          type="text"
          placeholder={t('shopping.auto_prop_612e559c', '搜索好物...')}
          className="bg-transparent flex-1 text-[15px] text-text-main outline-none"
        />
      </div>
      <div className="w-full h-32 rounded-xl bg-gradient-to-r from-blue-500 to-indigo-600 relative overflow-hidden mb-6 flex items-center px-6 shadow-sm">
        <img
          src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/salebanner/800x300.png"
          alt="Sale Banner"
          className="absolute inset-0 w-full h-full object-cover opacity-60 mix-blend-overlay"
        />
        <div className="relative z-10 text-white">
          <h2 className="text-2xl font-bold italic drop-shadow-md">
            {t('shopping.auto_2fd0d5da', '春季大促')}
          </h2>
          <p className="text-sm opacity-90 drop-shadow-md">
            {t('shopping.auto_7a4a9486', '满199减50 / 限时包邮')}
          </p>
        </div>
      </div>
    </div>
  );
};
