import React from 'react';
import { useTranslation } from 'react-i18next';

export const ApprovalHeader: React.FC<{
  pendingCount: number;
  initiatedCount: number;
  ccCount: number;
}> = ({ pendingCount, initiatedCount, ccCount }) => {
  const { t } = useTranslation();
return (
    <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
      <div>
        <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
          {pendingCount}
        </div>
        <div className="text-[13px] opacity-80">{t('approval.stats.pending')}</div>
      </div>
      <div className="flex gap-4">
        <div className="flex flex-col items-center">
          <div className="text-[20px] font-medium leading-none mb-1">{initiatedCount}</div>
          <div className="text-[12px] opacity-80">{t('approval.stats.initiated')}</div>
        </div>
        <div className="flex flex-col items-center">
          <div className="text-[20px] font-medium leading-none mb-1">{ccCount}</div>
          <div className="text-[12px] opacity-80">{t('approval.stats.cc')}</div>
        </div>
      </div>
    </div>
  );
};
