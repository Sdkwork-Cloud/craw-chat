import React from "react";
import { useTranslation } from "react-i18next";
import { showToast } from "@sdkwork/im-h5-commons";

export const VipFooter: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="absolute bottom-0 inset-x-0 p-4 bg-white dark:bg-[#1A1A1A] border-t border-[#EBEBEB] dark:border-[#333] pb-safe z-20">
      <button 
        className="w-full h-[50px] bg-gradient-to-r from-[#EAD196] to-[#C89B3C] text-[#8A5A19] font-bold text-[16px] rounded-full active:opacity-80 transition-opacity flex items-center justify-center"
        onClick={() => showToast(t('vip.auto_fn_171eb670', '订阅支付即将开发'))}
      >
        {t('vip.auto_3916e122', '立即开通')}
      </button>
    </div>
  );
};
