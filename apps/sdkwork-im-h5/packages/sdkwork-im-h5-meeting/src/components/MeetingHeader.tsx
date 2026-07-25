import React from 'react';
import { Video } from 'lucide-react';
import { useTranslation } from 'react-i18next';

export const MeetingHeader: React.FC<{ count: number }> = ({ count }) => {
  const { t } = useTranslation();
return (
    <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
      <div>
        <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
          {count}
        </div>
        <div className="text-[13px] opacity-80">{t('meeting.stats.upcoming')}</div>
      </div>
      <div className="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center">
        <Video className="w-6 h-6 text-white" />
      </div>
    </div>
  );
};
