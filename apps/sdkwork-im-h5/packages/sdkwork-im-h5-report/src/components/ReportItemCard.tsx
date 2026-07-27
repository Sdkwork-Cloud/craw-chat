import { useTranslation } from "react-i18next";
import React from 'react';
import { cn } from '@sdkwork/im-h5-commons';
import { LayoutTemplate, Briefcase, FileSignature, FileText, ChevronRight } from 'lucide-react';
import { ReportItem } from '../services/ReportService';
import { motion } from 'motion/react';
import { useNavigate } from 'react-router';

interface ReportItemCardProps {
  report: ReportItem;
}

export const ReportItemCard: React.FC<ReportItemCardProps> = ({ report }) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  const getTypeIcon = (type: string) => {
  switch (type) {
      case "日报":
        return <LayoutTemplate className="w-5 h-5 text-indigo-500" />;
      case "周报":
        return <Briefcase className="w-5 h-5 text-blue-500" />;
      case "月报":
        return <FileSignature className="w-5 h-5 text-orange-500" />;
      default:
        return <FileText className="w-5 h-5 text-primary-blue" />;
    }
  };

  return (
    <motion.div
      whileTap={{ scale: 0.98 }}
      onClick={() => navigate(`/workspace/report/${report.id}`)}
      className={cn(
        "bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border",
        report.isRead
          ? "border-border-color/30"
          : "border-primary-blue/20"
      )}
    >
      <div className="flex justify-between items-start mb-3">
        <div className="flex items-center gap-3">
          <div className="relative">
            <div className="w-10 h-10 rounded-xl bg-gray-100 dark:bg-[#3a3b3c] flex items-center justify-center">
              {getTypeIcon(report.type)}
            </div>
            {!report.isRead && (
              <div className="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full border-2 border-white dark:border-[#2c2d2e]" />
            )}
          </div>
          <div>
            <div className="text-[16px] font-medium text-text-main leading-tight mb-1 flex items-center gap-2">{report.reporter} 的 {report.type}</div>
            <div className="text-[13px] text-text-sub font-mono">
              {report.date}
            </div>
          </div>
        </div>
      </div>
      <div className="text-[14px] text-text-main bg-[#f8f9fa] dark:bg-[#202122] p-3 rounded-lg flex items-center justify-between">
        <span className="truncate pr-4 line-clamp-1">
          {report.summary}
        </span>
        <ChevronRight className="w-4 h-4 text-text-sub shrink-0" />
      </div>
    </motion.div>
  );
};
