import React from "react";
import { FileText, Target, AlertCircle } from "lucide-react";
import { useTranslation } from "react-i18next";
import { ReportItem } from "../services/ReportService";

interface ReportDetailContentProps {
  report: ReportItem;
}

export const ReportDetailContent: React.FC<ReportDetailContentProps> = ({ report }) => {
  const { t } = useTranslation();
  return (
    <div className="bg-chat-other-bg rounded-xl shadow-sm border border-border-color/30 overflow-hidden">
      {/* Completed Work */}
      <div className="p-4 border-b border-border-color/30">
        <div className="flex items-center gap-2 mb-3">
          <div className="w-6 h-6 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center">
            <FileText className="w-3.5 h-3.5 text-primary-blue" />
          </div>
          <h3 className="text-[15px] font-bold text-text-main">{t('report.auto_56f54a0d', '已完成工作')}</h3>
        </div>
        <div className="text-[15px] text-text-main leading-relaxed pl-2 whitespace-pre-wrap">
          {report.summary}
        </div>
      </div>

      {/* Plan */}
      <div className="p-4 border-b border-border-color/30">
        <div className="flex items-center gap-2 mb-3">
          <div className="w-6 h-6 rounded-full bg-orange-50 dark:bg-orange-900/30 flex items-center justify-center">
            <Target className="w-3.5 h-3.5 text-orange-500" />
          </div>
          <h3 className="text-[15px] font-bold text-text-main">{t('report.auto_2be9bee8', '工作计划')}</h3>
        </div>
        <div className="text-[15px] text-text-sub leading-relaxed pl-2">
          {t('report.auto_n595d4a6b', '按计划推进下一步开发，重点关注性能优化。')}
        </div>
      </div>

      {/* Issues */}
      <div className="p-4 bg-bg-color/30">
        <div className="flex items-center gap-2 mb-3">
          <div className="w-6 h-6 rounded-full bg-rose-50 dark:bg-rose-900/30 flex items-center justify-center">
            <AlertCircle className="w-3.5 h-3.5 text-rose-500" />
          </div>
          <h3 className="text-[15px] font-bold text-text-main">{t('report.auto_77dc24fe', '需协调问题')}</h3>
        </div>
        <div className="text-[15px] text-text-sub leading-relaxed pl-2">
          {t('report.auto_n2e900039', '暂无需要协调的问题。')}
        </div>
      </div>
    </div>
  );
};
