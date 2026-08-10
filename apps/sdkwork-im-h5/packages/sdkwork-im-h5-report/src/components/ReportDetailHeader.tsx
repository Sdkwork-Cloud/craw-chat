import React from "react";
import { Avatar } from "@sdkwork/im-h5-commons";
import { Clock } from "lucide-react";
import { useTranslation } from "react-i18next";
import { ReportItem } from "../services/ReportService";

interface ReportDetailHeaderProps {
  report: ReportItem;
}

export const ReportDetailHeader: React.FC<ReportDetailHeaderProps> = ({ report }) => {
  const { t } = useTranslation();
  return (
    <div className="bg-chat-other-bg rounded-xl p-5 shadow-sm border border-border-color/30 text-center flex flex-col items-center relative overflow-hidden">
      <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-500 to-indigo-500" />
      <Avatar
        size="xl"
        className="mb-3 border-2 border-white shadow-sm"
        fallback={report.reporter.substring(0, 1)}
      />
      <h2 className="text-[18px] font-bold text-text-main mb-1">
        {report.reporter}
      </h2>
      <div className="text-[13px] text-text-sub flex items-center justify-center gap-1.5 bg-bg-color px-3 py-1 rounded-full">
        <Clock className="w-3.5 h-3.5" />
        {t('report.auto_n67688416', `提交于 ${report.date}`)}
      </div>
    </div>
  );
};
