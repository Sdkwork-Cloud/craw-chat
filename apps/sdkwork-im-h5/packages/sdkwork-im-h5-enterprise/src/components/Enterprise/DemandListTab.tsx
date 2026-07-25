import React from "react";
import { Building2, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface DemandItem {
  title: string;
  company: string;
  type: string;
  budget: string;
}

export interface DemandListTabProps {
  demands: DemandItem[];
}

export const DemandListTab: React.FC<DemandListTabProps> = ({ demands }) => {
  const { t } = useTranslation();

  return (
    <>
      {demands.map((dem, i) => (
        <div
          key={i}
          className="px-4 py-3 border-b border-border-color/50 flex flex-col gap-2 active:bg-chat-active-bg transition-colors cursor-pointer group hover:bg-[#fcfdff] dark:hover:bg-[#333538] relative overflow-hidden"
        >
          <div className="absolute inset-0 bg-primary-blue/5 opacity-0 group-active:opacity-100 transition-opacity" />
          <div className="flex items-center justify-between mb-1">
            <div className="flex items-center gap-2">
              <span className="bg-red-500/10 text-red-500 text-[10px] px-1.5 py-0.5 rounded-sm font-bold border border-red-500/20">
                {t('enterprise.auto_2705bd70', '加急求购')}
              </span>
              <span className="text-[12px] text-text-sub font-medium bg-[#f5f6f8] dark:bg-[#1a1b1c] px-1.5 py-0.5 rounded-sm">
                {dem.type}
              </span>
            </div>
            <span className="text-[13px] font-bold text-primary-blue">
              {t('enterprise.auto_69caab14', `预算: ${dem.budget}`)}
            </span>
          </div>
          <h3 className="text-[15px] font-bold text-text-main group-hover:text-primary-blue transition-colors">
            {dem.title}
          </h3>
          <div className="flex justify-between items-center mt-1 pt-2 border-t border-border-color/30">
            <div className="flex items-center gap-1.5 flex-1 overflow-hidden opacity-90">
              <Building2 className="w-4 h-4 text-text-sub shrink-0" />
              <span className="text-[12px] text-text-sub truncate font-medium">{dem.company}</span>
            </div>
            <ChevronRight className="w-4 h-4 text-text-sub opacity-50" />
          </div>
        </div>
      ))}
    </>
  );
};
