import React from 'react';
import { Clock, ChevronRight } from 'lucide-react';
import { useTranslation } from 'react-i18next';

export const CandidateProgress: React.FC<{ candidate: any }> = ({ candidate }) => {
  const { t } = useTranslation();
return (
    <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 mb-4 shadow-sm border border-border-color/30">
      <h3 className="text-[15px] font-bold text-text-main mb-4 border-l-4 border-primary-blue pl-2 leading-tight">
        {t('recruitment.detail.currentStage')}
      </h3>
      <div className="flex items-center gap-4">
        <div className="w-10 h-10 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center shrink-0">
          <Clock className="w-5 h-5 text-primary-blue" />
        </div>
        <div className="flex-1">
          <div className="text-[15px] text-text-main font-medium">
            {candidate.stage}
          </div>
          <div className="text-[13px] text-text-sub mt-0.5">
            {t('recruitment.detail.updatedAt')}: {candidate.date}
          </div>
        </div>
        <ChevronRight className="w-5 h-5 text-border-color" />
      </div>
    </div>
  );
};
