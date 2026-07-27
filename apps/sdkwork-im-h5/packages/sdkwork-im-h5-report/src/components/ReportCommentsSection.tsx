import React from "react";
import { Avatar, showToast } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

export const ReportCommentsSection: React.FC = () => {
  const { t } = useTranslation();
  return (
    <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm border border-border-color/30">
      <h3 className="text-[15px] font-bold text-text-main mb-4">{t('report.auto_30616b26', '最新评论')}</h3>
      <div className="space-y-4 mb-4">
        <div className="flex gap-3">
          <Avatar
            src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/leader/200.png"
            size="sm"
            fallback={t('report.auto_prop_957f', '长')}
            className="shrink-0"
          />
          <div>
            <div className="flex items-center gap-2">
              <span className="text-[14px] font-medium text-text-main">{t('report.auto_be51b', '张总')}</span>
              <span className="text-[12px] text-text-sub">{t('report.auto_40e8b95', '10分钟前')}</span>
            </div>
            <div className="text-[14px] text-text-main mt-1">
              {t('report.auto_51849a17', '辛苦了，下周的计划很清晰，继续保持！')}
            </div>
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
        >
          {t('report.auto_ab650', '发送')}
        </button>
      </div>
    </div>
  );
};
