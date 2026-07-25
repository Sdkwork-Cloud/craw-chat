import React from 'react';
import { useTranslation } from 'react-i18next';

export const ReportHeaderStats: React.FC = () => {
  const { t } = useTranslation();
return (
    <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
      <div>
        <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
          5
        </div>
        <div className="text-[13px] opacity-80">{t('report.unread')}</div>
      </div>
      <div className="flex gap-4">
        <div className="flex flex-col items-center">
          <div className="text-[20px] font-medium leading-none mb-1">2</div>
          <div className="text-[12px] opacity-80">{t('report.sent_by_me')}</div>
        </div>
        <div className="flex flex-col items-center">
          <div className="text-[20px] font-medium leading-none mb-1">
            10+
          </div>
          <div className="text-[12px] opacity-80">{t('report.received')}</div>
        </div>
      </div>
    </div>
  );
};
