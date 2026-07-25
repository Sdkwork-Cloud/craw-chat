import React from "react";
import { useTranslation } from "react-i18next";
import { Crown } from "lucide-react";

export const VipBanner: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="absolute top-0 inset-x-0 h-[250px] bg-gradient-to-br from-[#EAD196] to-[#C89B3C] px-6 pt-6 rounded-b-[40px] shadow-sm">
      <div className="flex items-center gap-4 text-[#8A5A19]">
        <Crown className="w-10 h-10 drop-shadow-md" strokeWidth={1.5} />
        <div>
          <h2 className="text-2xl font-bold">{t('vip.auto_70e3199d', '超级会员 VIP')}</h2>
          <p className="text-sm opacity-90 mt-1">{t('vip.auto_4ecfdde3', '尊享特权，体验飞升')}</p>
        </div>
      </div>
    </div>
  );
};
