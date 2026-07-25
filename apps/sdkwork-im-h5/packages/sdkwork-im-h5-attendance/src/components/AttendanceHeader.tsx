import React from 'react';
import { MapPin, Calendar } from 'lucide-react';
import { useTranslation } from 'react-i18next';

export const AttendanceHeader: React.FC<{ time: Date }> = ({ time }) => {
  const { t } = useTranslation();
return (
    <div className="bg-primary-blue text-white p-6 pb-8 rounded-b-[2rem] shadow-sm">
      <div className="flex justify-between items-center mb-6">
        <div className="flex items-center gap-2">
          <Calendar className="w-5 h-5 opacity-90" />
          <span className="font-medium">
            {time.toLocaleDateString(t('common.locale', { defaultValue: 'zh-CN' }), {
              month: "long",
              day: "numeric",
              weekday: "long",
            })}
          </span>
        </div>
      </div>

      <div className="flex flex-col items-center justify-center pt-2">
        <div className="text-[48px] font-mono font-medium tracking-tight">
          {time.toLocaleTimeString(t('common.locale', { defaultValue: 'zh-CN' }), {
            hour12: false,
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
          })}
        </div>
        <div className="flex items-center gap-1.5 mt-2 text-white/80 text-[14px] bg-white/10 px-3 py-1 rounded-full">
          <MapPin className="w-4 h-4" />
          {t('attendance.location')}
        </div>
      </div>
    </div>
  );
};
