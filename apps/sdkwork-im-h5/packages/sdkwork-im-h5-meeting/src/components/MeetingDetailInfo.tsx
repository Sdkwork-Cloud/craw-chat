import React from 'react';
import { useTranslation } from 'react-i18next';
import { Clock, MapPin } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { MeetingRecord } from '../services/MeetingService';

export const MeetingDetailInfo: React.FC<{ meeting: MeetingRecord }> = ({ meeting }) => {
  const { t } = useTranslation();
return (
    <div className="bg-white dark:bg-[#1a1b1c] p-5 pb-8 mb-2">
      <div className="flex items-center gap-2 mb-2">
        <span
          className={cn(
            "text-[12px] px-2 py-1 rounded shrink-0",
            meeting.status === "upcoming" && "bg-blue-50 text-blue-600 dark:bg-blue-500/10",
            meeting.status === "ongoing" && "bg-green-50 text-green-600 dark:bg-green-500/10",
            meeting.status === "finished" && "bg-gray-100 text-gray-500 dark:bg-gray-800",
            meeting.status === "cancelled" && "bg-red-50 text-red-500 dark:bg-red-500/10",
          )}
        >
          {t(`meeting.status.${meeting.status}`, { defaultValue: meeting.status })}
        </span>
      </div>
      <h1 className="text-[22px] font-medium text-text-main leading-tight mb-5">
        {meeting.title}
      </h1>

      <div className="flex flex-col gap-3">
        <div className="flex items-start gap-3 text-[14px]">
          <Clock className="w-5 h-5 text-gray-400 shrink-0" />
          <div>
            <div className="text-text-main">
              {meeting.date} {meeting.time}
            </div>
            <div className="text-text-sub mt-0.5">
              {t('meeting.detail.organizer')}: {meeting.organizerName || "Admin"}
            </div>
          </div>
        </div>

        <div className="flex items-start gap-3 text-[14px]">
          <MapPin className="w-5 h-5 text-gray-400 shrink-0" />
          <div className="text-text-main">{meeting.room}</div>
        </div>
      </div>
    </div>
  );
};
