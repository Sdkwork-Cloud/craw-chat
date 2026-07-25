import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import {
  PageLayout,
  Avatar,
  cn,
  showToast,
} from "@sdkwork/im-h5-commons";
import {
  FileText,
  Target,
  AlertCircle,
  Clock,
  Calendar,
  MessageSquare,
  ThumbsUp,
} from "lucide-react";
import { useParams, useNavigate } from "react-router";
import { ReportService, ReportItem } from "../services/ReportService";

export const ReportDetail = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const navigate = useNavigate();
  const [report, setReport] = useState<ReportItem | null>(null);

  useEffect(() => {
    ReportService.getReports().then((data) => {
      setReport(data.find((r) => r.id === id) || data[0]);
    });
  }, [id]);

  if (!report)
    return (
      <PageLayout title={t('report.auto_prop_32bbf95d', '汇报详情')}>
        <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">{t('report.auto_7f6f37e', '加载中...')}</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title={t('report.auto_prop_32bbf95d', '汇报详情')}>
      <div className="p-4 space-y-4">
        {/* Header Content */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-5 shadow-sm border border-border-color/30 text-center flex flex-col items-center relative overflow-hidden">
          <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-500 to-indigo-500" />
          <Avatar
            src={`https://picsum.photos/seed/${report.reporter}/200`}
            size="xl"
            className="mb-3 border-2 border-white shadow-sm"
            fallback={report.reporter.substring(0, 1)}
          />
          <h2 className="text-[18px] font-bold text-text-main mb-1">
            {report.reporter}
          </h2>
          <div className="text-[13px] text-text-sub flex items-center justify-center gap-1.5 bg-bg-color px-3 py-1 rounded-full">
            <Clock className="w-3.5 h-3.5" />{t('report.auto_n67688416', '提交于 {report.date}')}</div>
        </div>

        {/* Report Content Sections */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl shadow-sm border border-border-color/30 overflow-hidden">
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
            <div className="text-[15px] text-text-sub leading-relaxed pl-2">{t('report.auto_n595d4a6b', '按计划推进下一步开发，重点关注性能优化。')}</div>
          </div>

          {/* Issues */}
          <div className="p-4 bg-bg-color/30">
            <div className="flex items-center gap-2 mb-3">
              <div className="w-6 h-6 rounded-full bg-rose-50 dark:bg-rose-900/30 flex items-center justify-center">
                <AlertCircle className="w-3.5 h-3.5 text-rose-500" />
              </div>
              <h3 className="text-[15px] font-bold text-text-main">{t('report.auto_77dc24fe', '需协调问题')}</h3>
            </div>
            <div className="text-[15px] text-text-sub leading-relaxed pl-2">{t('report.auto_n2e900039', '暂无需要协调的问题。')}</div>
          </div>
        </div>

        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm border border-border-color/30">
          <h3 className="text-[15px] font-bold text-text-main mb-4">{t('report.auto_30616b26', '最新评论')}</h3>
          <div className="space-y-4 mb-4">
            <div className="flex gap-3">
              <Avatar
                src="https://picsum.photos/seed/leader/200"
                size="sm"
                fallback={t('report.auto_prop_957f', '长')}
                className="shrink-0"
              />
              <div>
                <div className="flex items-center gap-2">
                  <span className="text-[14px] font-medium text-text-main">{t('report.auto_be51b', '张总')}</span>
                  <span className="text-[12px] text-text-sub">{t('report.auto_40e8b95', '10分钟前')}</span>
                </div>
                <div className="text-[14px] text-text-main mt-1">{t('report.auto_51849a17', '辛苦了，下周的计划很清晰，继续保持！')}</div>
              </div>
            </div>
          </div>
          <div className="flex gap-2">
            <input
              type="text"
              className="flex-1 bg-bg-color border border-border-color/50 rounded-lg px-3 py-2 text-[14px] outline-none"
              placeholder={t('report.auto_prop_2eb1a43f', '写评论...')}
            />
            <button
              className="bg-primary-blue text-white px-4 rounded-lg font-medium text-[14px]"
              onClick={() => showToast(t('report.auto_fn_41a16585', '评论成功'))}
            >{t('report.auto_ab650', '发送')}</button>
          </div>
        </div>

        {/* Interaction Bar */}
        <div className="flex gap-3">
          <button
            className="flex-1 bg-white dark:bg-[#2c2d2e] border border-border-color text-text-main py-3 rounded-xl font-medium shadow-sm flex justify-center items-center gap-2 hover:bg-bg-color active:scale-95 transition-all"
            onClick={() => showToast(t('report.auto_fn_bedac', '已赞'))}
          >
            <ThumbsUp className="w-4 h-4 text-text-sub" />{t('report.auto_8d5e', '赞')}</button>
        </div>
      </div>
    </PageLayout>
  );
};
