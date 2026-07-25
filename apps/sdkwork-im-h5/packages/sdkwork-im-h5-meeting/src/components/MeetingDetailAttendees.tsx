import React from 'react';
import { useTranslation } from 'react-i18next';
import { MeetingRecord } from '../services/MeetingService';

export const MeetingDetailAttendees: React.FC<{ meeting: MeetingRecord }> = ({ meeting }) => {
  const { t } = useTranslation();
return (
    <div className="bg-white dark:bg-[#1a1b1c] p-4 mb-2">
      <h3 className="text-[15px] font-medium text-text-main mb-4">
        {t('meeting.detail.attendees')} ({meeting.attendees.length})
      </h3>
      <div className="flex gap-4 overflow-x-auto pb-2">
        {meeting.attendees.map((a) => (
          <div
            key={a.id}
            className="flex flex-col items-center gap-1 shrink-0"
          >
            <img
              src={a.avatar}
              className="w-12 h-12 rounded-full object-cover bg-gray-100"
            />
            <span className="text-[12px] text-text-sub truncate w-14 text-center">
              {a.name}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
};
