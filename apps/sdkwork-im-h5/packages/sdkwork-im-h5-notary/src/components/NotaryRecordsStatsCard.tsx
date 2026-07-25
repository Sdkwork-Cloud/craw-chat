import React from "react";
import { useTranslation } from "react-i18next";

export const NotaryRecordsStatsCard: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="p-4 flex flex-col gap-3">
      <div className="grid grid-cols-2 gap-3">
        <div className="bg-gradient-to-br from-primary-blue/10 to-indigo-500/10 border border-primary-blue/20 rounded-2xl p-4 flex flex-col justify-between h-[104px]">
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-4 bg-primary-blue rounded-full" />
            <span className="text-[14px] text-primary-blue font-bold">
              {t("notary.records.total_count")}
            </span>
          </div>
          <span className="text-[32px] font-bold text-primary-blue font-mono tracking-tight">
            128
          </span>
        </div>
        <div className="bg-chat-other-bg border border-border-color rounded-2xl p-4 flex flex-col justify-between h-[104px]">
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-4 bg-orange-500 rounded-full" />
            <span className="text-[14px] text-text-main font-bold">
              {t("notary.records.processing_queue")}
            </span>
          </div>
          <span className="text-[32px] font-bold text-text-main font-mono tracking-tight">
            3
          </span>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-3">
        <div className="bg-chat-other-bg border border-border-color/50 rounded-xl p-3.5 flex flex-col justify-between h-[84px]">
          <span className="text-[12px] text-text-sub font-medium">
            {t("notary.records.new_today")}
          </span>
          <div className="flex items-end gap-1">
            <span className="text-[22px] font-bold text-text-main font-mono leading-none">
              2
            </span>
            <span className="text-[10px] text-green-500 font-bold mb-0.5">
              +2
            </span>
          </div>
        </div>
        <div className="bg-chat-other-bg border border-border-color/50 rounded-xl p-3.5 flex flex-col justify-between h-[84px]">
          <span className="text-[12px] text-text-sub font-medium">
            {t("notary.records.new_this_week")}
          </span>
          <div className="flex items-end gap-1">
            <span className="text-[22px] font-bold text-text-main font-mono leading-none">
              15
            </span>
            <span className="text-[10px] text-green-500 font-bold mb-0.5">
              +5
            </span>
          </div>
        </div>
        <div className="bg-chat-other-bg border border-border-color/50 rounded-xl p-3.5 flex flex-col justify-between h-[84px]">
          <span className="text-[12px] text-text-sub font-medium">
            {t("notary.records.total_this_month")}
          </span>
          <div className="flex items-end gap-1">
            <span className="text-[22px] font-bold text-text-main font-mono leading-none">
              48
            </span>
            <span className="text-[10px] text-green-500 font-bold mb-0.5">
              +12
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};
