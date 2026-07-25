import React from 'react';
import { Clock, MapPin } from 'lucide-react';
import { AttendanceRecord } from '../services/AttendanceService';
import { useTranslation } from 'react-i18next';

export const AttendanceHistory: React.FC<{ todayRecords: AttendanceRecord[] }> = ({ todayRecords }) => {
  const { t } = useTranslation();
return (
    <div className="w-full bg-chat-other-bg rounded-2xl p-4 shadow-sm border border-border-color/50">
      <h3 className="text-[15px] font-medium text-text-main mb-4 flex items-center gap-2">
        <Clock className="w-4 h-4 text-primary-blue" />
        {t('attendance.history.title')}
      </h3>

      <div className="flex flex-col gap-4 relative">
        {/* Timeline line */}
        <div className="absolute left-[7px] top-2 bottom-2 w-[2px] bg-border-color border-dashed"></div>

        {todayRecords.map((record) => (
          <div key={record.id} className="flex gap-4 relative z-10">
            <div className="w-4 h-4 rounded-full bg-primary-blue border-4 border-chat-other-bg shrink-0 mt-0.5" />
            <div>
              <div className="text-[15px] font-medium text-text-main mb-1">
                {record.type === "in" ? t('attendance.status.clockIn') : t('attendance.status.clockOut')}{" "}
                {record.time}
              </div>
              <div className="flex items-center gap-1 text-[13px] text-text-sub">
                <MapPin className="w-3.5 h-3.5" />
                {record.location}
              </div>
            </div>
          </div>
        ))}

        {todayRecords.length === 0 && (
          <div className="flex flex-col items-center justify-center py-6 text-text-sub bg-bg-color/50 rounded-xl border border-dashed border-border-color">
            <Clock className="w-8 h-8 mb-2 opacity-20" />
            <span className="text-[13px]">{t('attendance.history.empty')}</span>
          </div>
        )}
      </div>
    </div>
  );
};
