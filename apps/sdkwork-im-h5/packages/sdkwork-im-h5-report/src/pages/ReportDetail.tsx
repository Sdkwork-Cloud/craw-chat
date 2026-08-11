import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import {
  PageLayout,
  showToast,
} from "@sdkwork/im-h5-commons";
import { ThumbsUp } from "lucide-react";
import { useParams } from "react-router";
import { ReportService, ReportItem } from "../services/ReportService";
import { ReportDetailHeader } from "../components/ReportDetailHeader";
import { ReportDetailContent } from "../components/ReportDetailContent";
import { ReportCommentsSection } from "../components/ReportCommentsSection";

export const ReportDetail = () => {
  const { t } = useTranslation();
  const { id } = useParams();
  const [report, setReport] = useState<ReportItem | null>(null);

  useEffect(() => {
    ReportService.getReports().then((data) => {
      setReport(data.find((r) => r.id === id) || data[0]);
    });
  }, [id]);

  if (!report)
    return (
      <PageLayout title={t('report.auto_prop_32bbf95d', 'Report details')}>
        <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">{t('report.auto_7f6f37e', 'Loading...')}</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title={t('report.auto_prop_32bbf95d', 'Report details')}>
      <div className="p-4 space-y-4">
        {/* Header Content */}
        <ReportDetailHeader report={report} />

        {/* Report Content Sections */}
        <ReportDetailContent report={report} />

        {/* Comments */}
        <ReportCommentsSection />

        {/* Interaction Bar */}
        <div className="flex gap-3">
          <button
            className="flex-1 bg-chat-other-bg border border-border-color text-text-main py-3 rounded-xl font-medium shadow-sm flex justify-center items-center gap-2 hover:bg-bg-color active:scale-95 transition-all"
            onClick={() => showToast(t('report.auto_fn_bedac', 'Liked'))}
          >
            <ThumbsUp className="w-4 h-4 text-text-sub" />{t('report.auto_8d5e', 'Like')}
          </button>
        </div>
      </div>
    </PageLayout>
  );
};
