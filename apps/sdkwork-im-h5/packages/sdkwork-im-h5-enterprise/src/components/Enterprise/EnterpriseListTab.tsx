import React from "react";
import { ShieldCheck, MapPin } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

export interface EnterpriseItem {
  name: string;
  industry: string;
  location: string;
  tags: string[];
  logo: string;
  isAuth: boolean;
}

export interface EnterpriseListTabProps {
  enterprises: EnterpriseItem[];
}

export const EnterpriseListTab: React.FC<EnterpriseListTabProps> = ({ enterprises }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <>
      {enterprises.map((ent, i) => (
        <div
          key={i}
          onClick={() => navigate("/enterprise/site")}
          className="px-4 py-3 border-b border-border-color/50 flex gap-3 active:bg-chat-active-bg transition-colors cursor-pointer group relative overflow-hidden"
        >
          <div className="absolute inset-0 bg-primary-blue/5 opacity-0 group-active:opacity-100 transition-opacity" />
          <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-black/5 to-black/10 dark:from-white/5 dark:to-white/10 flex items-center justify-center text-2xl shrink-0 shadow-inner">
            {ent.logo}
          </div>
          <div className="flex flex-col flex-1 overflow-hidden">
            <div className="flex justify-between items-start">
              <div className="flex items-center gap-1.5 overflow-hidden">
                <h3 className="text-[16px] font-bold text-text-main truncate group-hover:text-primary-blue transition-colors">
                  {ent.name}
                </h3>
                {ent.isAuth ? (
                  <span className="text-[10px] font-bold text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/30 px-1 py-0.5 rounded-sm shrink-0 border border-blue-200 dark:border-blue-800/50 flex items-center gap-0.5">
                    <ShieldCheck className="w-3 h-3" />
                    {t('enterprise.auto_1721e0f', '已认证')}
                  </span>
                ) : (
                  <span className="text-[10px] font-medium text-text-sub bg-bg-color px-1 py-0.5 rounded-sm shrink-0 border border-border-color/50">
                    {t('enterprise.auto_194b947', '未认证')}
                  </span>
                )}
              </div>
            </div>
            <div className="flex items-center gap-1.5 mt-1.5">
              <span className="text-[10px] font-medium bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 px-1.5 py-0.5 rounded-sm border border-blue-100 dark:border-blue-800/50">
                {ent.industry}
              </span>
              {ent.tags.map((tag) => (
                <span
                  key={tag}
                  className="text-[10px] font-medium bg-bg-color text-text-sub px-1.5 py-0.5 rounded-sm border border-border-color/50"
                >
                  {tag}
                </span>
              ))}
            </div>
            <div className="flex items-center text-[12px] text-text-sub mt-2 opacity-80">
              <MapPin className="w-3.5 h-3.5 mr-1" /> {ent.location}
            </div>
          </div>
        </div>
      ))}
    </>
  );
};
