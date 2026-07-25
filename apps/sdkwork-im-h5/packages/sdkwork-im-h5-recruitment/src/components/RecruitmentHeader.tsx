import React from 'react';
import { useTranslation } from 'react-i18next';

export const RecruitmentHeader: React.FC<{ ongoingCount: number, interviewCount: number, reviewCount: number }> = ({ ongoingCount, interviewCount, reviewCount }) => {
  const { t } = useTranslation();
return (
    <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
      <div>
        <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
          {ongoingCount}
        </div>
        <div className="text-[13px] opacity-80">{t('recruitment.stats.ongoing')}</div>
      </div>
      <div className="flex gap-4">
        <div className="flex flex-col items-center">
          <div className="text-[20px] font-medium leading-none mb-1">{interviewCount}</div>
          <div className="text-[12px] opacity-80">{t('recruitment.stats.todayInterview')}</div>
        </div>
        <div className="flex flex-col items-center">
          <div className="text-[20px] font-medium leading-none mb-1">{reviewCount}</div>
          <div className="text-[12px] opacity-80">{t('recruitment.stats.pendingReview')}</div>
        </div>
      </div>
    </div>
  );
};
