import React, { useState } from "react";
import {
  PageLayout,
  IconButton,
  cn,
  showToast,
} from "@sdkwork/im-h5-commons";
import {
  Plus,
  Search,
  Filter,
  FileText,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { ReportService, ReportItem } from "../services/ReportService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";
import { ReportHeaderStats } from "../components/ReportHeaderStats";
import { ReportTabs } from "../components/ReportTabs";
import { ReportItemCard } from "../components/ReportItemCard";

export const ReportApp = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [activeTab, setActiveTab] = useState<string>("待我查阅");
  const [reports, setReports] = useState<ReportItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  React.useEffect(() => {
    setIsLoading(true);
    ReportService.getReports().then((data) => {
      setReports(data);
      setIsLoading(false);
    });
  }, []);

  return (
    <PageLayout title={t('report.title')}>
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        <ReportHeaderStats />

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <ReportTabs activeTab={activeTab} setActiveTab={setActiveTab} />

          <div className="flex justify-between items-center mb-3 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              {t('report.all_records', { count: reports.length })}
            </h2>
            <div className="flex gap-2">
              <IconButton
                icon={<Filter className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
            </div>
          </div>

          <div className="flex flex-col gap-3 pb-20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
                <span className="text-[14px]">{t('report.loading')}</span>
              </div>
            ) : reports.length > 0 ? (
              reports.map((report) => (
                <ReportItemCard key={report.id} report={report} />
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileText className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t('report.no_records')}</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/report/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
