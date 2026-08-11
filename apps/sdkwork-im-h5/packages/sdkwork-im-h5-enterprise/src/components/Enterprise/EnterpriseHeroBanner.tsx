import React from "react";
import { Building2 } from "lucide-react";
import { useTranslation } from "react-i18next";

export const EnterpriseHeroBanner: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="mx-3 mt-3 mb-2 relative overflow-hidden rounded-2xl bg-gradient-to-br from-blue-600 via-indigo-600 to-purple-600 p-5 shadow-sm shrink-0">
      <div className="absolute top-0 right-0 w-[150px] h-[150px] bg-[url('https://api.dicom.cn/1')] opacity-10 mix-blend-overlay"></div>
      <div className="absolute -bottom-10 -right-10 w-32 h-32 bg-white/10 rounded-full blur-xl"></div>
      <div className="absolute -top-10 -left-10 w-32 h-32 bg-white/10 rounded-full blur-xl"></div>
      
      <div className="relative z-10 flex items-center justify-between">
        <div className="flex flex-col">
          <h2 className="text-[18px] font-extrabold text-white mb-1 tracking-wide">{t('enterprise.auto_n30585810', 'Geek Business Center')}</h2>
          <p className="text-[13px] text-white/80 font-medium tracking-wide">{t('enterprise.auto_n3c3c50e6', 'Discover great opportunities and build reliable connections')}</p>
        </div>
        <div className="w-12 h-12 bg-white/20 backdrop-blur-md rounded-2xl flex items-center justify-center border border-white/30 shadow-inner rotate-3">
          <Building2 className="w-6 h-6 text-white" />
        </div>
      </div>
    </div>
  );
};
