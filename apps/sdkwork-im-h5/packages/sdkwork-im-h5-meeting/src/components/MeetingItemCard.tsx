import React from 'react';
import { motion } from 'motion/react';
import { Clock, MapPin, Users, Play } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { MeetingRecord } from '../services/MeetingService';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';

export const MeetingItemCard: React.FC<{
  meeting: MeetingRecord;
}> = ({ meeting }) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <motion.div
      whileTap={{ scale: 0.98 }}
      onClick={() => navigate(`/workspace/meeting/${meeting.id}`)}
      className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border border-border-color/30"
    >
      <div className="flex justify-between items-start mb-3">
        <div>
          <h3 className="text-[17px] font-medium text-text-main mb-1.5">
            {meeting.title}
          </h3>
          <div className="flex items-center gap-2 text-[13px] text-text-sub font-mono bg-gray-50 dark:bg-[#202122] px-2 py-1 rounded inline-flex">
            <Clock className="w-3.5 h-3.5" />
            {meeting.date} {meeting.time}
          </div>
        </div>
        <div
          className={cn(
            "text-[12px] px-2 py-1 rounded shrink-0",
            meeting.status === "upcoming" &&
              "bg-blue-50 text-blue-600 dark:bg-blue-500/10",
            meeting.status === "ongoing" &&
              "bg-green-50 text-green-600 dark:bg-green-500/10",
            meeting.status === "finished" &&
              "bg-gray-100 text-gray-500 dark:bg-gray-800",
          )}
        >
          {meeting.status === "upcoming"
            ? t('meeting.status.upcoming')
            : meeting.status === "ongoing"
              ? t('meeting.status.ongoing')
              : t('meeting.status.finished')}
        </div>
      </div>

      <div className="flex items-center gap-1.5 text-[13px] text-text-sub mb-3">
        <MapPin className="w-4 h-4 shrink-0" />
        <span className="truncate">{meeting.room}</span>
      </div>

      <div className="flex justify-between items-center pt-3 border-t border-border-color">
        <div className="flex items-center gap-2 text-[13px] text-text-sub">
          <Users className="w-4 h-4" />
          <span>{t('meeting.participants', { count: meeting.attendees.length })}</span>
        </div>
        {meeting.status === "upcoming" && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              navigate(`/call/video/${meeting.id}`);
            }}
            className="bg-primary-blue text-white px-4 py-1.5 rounded-full text-[13px] font-medium active:scale-95 transition-transform flex items-center gap-1.5"
          >
            <Play className="w-3.5 h-3.5" /> {t('meeting.join')}
          </button>
        )}
      </div>
    </motion.div>
  );
};
