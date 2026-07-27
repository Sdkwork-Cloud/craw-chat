import React from 'react';
import { useTranslation } from 'react-i18next';
import { Calendar as CalendarIcon, Clock, Trash2, Plus } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { Schedule } from '../services/CalendarService';

interface CalendarScheduleListProps {
  currentDate: Date;
  schedules: Schedule[];
  loading: boolean;
  setIsAdding: (val: boolean) => void;
  handleDeleteSchedule: (id: number) => void;
}

const itemIsToday = (date: Date) => {
  const today = new Date();
  return (
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
  );
};

export const CalendarScheduleList: React.FC<CalendarScheduleListProps> = ({
  currentDate,
  schedules,
  loading,
  setIsAdding,
  handleDeleteSchedule,
}) => {
  const { t } = useTranslation();
const title = itemIsToday(currentDate) 
    ? t('calendar.today') 
    : `${currentDate.getMonth() + 1}月${currentDate.getDate()}日`;

  return (
    <div className="flex-1 bg-[#F5F6F8] dark:bg-black p-4 flex flex-col gap-3">
      <div className="text-[14px] text-text-sub font-medium mb-1">
        {t('calendar.schedules_of', { date: title })}
      </div>

      {loading ? (
        <div className="flex flex-col items-center justify-center py-12 text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">{t('calendar.loading')}</span>
        </div>
      ) : schedules.length > 0 ? (
        schedules.map((schedule) => (
          <div
            key={schedule.id}
            className="bg-bg-color rounded-xl p-4 shadow-sm border border-border-color flex items-stretch gap-3 cursor-pointer active:scale-[0.98] transition-all relative group"
          >
            <div
              className={cn("w-1 rounded-full shrink-0", schedule.color)}
            />
            <div className="flex flex-col flex-1">
              <span className="text-[16px] font-bold text-text-main mb-1.5">
                {schedule.title}
              </span>
              <div className="flex items-center text-[13px] text-text-sub gap-1.5">
                <Clock className="w-3.5 h-3.5" />
                <span>{schedule.time}</span>
              </div>
            </div>
            <div
              className="absolute right-3 top-1/2 -translate-y-1/2 p-2 hover:bg-black/5 dark:hover:bg-white/5 rounded-full"
              onClick={(e) => {
                e.stopPropagation();
                handleDeleteSchedule(schedule.id);
              }}
            >
              <Trash2 className="w-4 h-4 text-red-500 opacity-60 hover:opacity-100 transition-opacity" />
            </div>
          </div>
        ))
      ) : (
        <div className="flex flex-col items-center justify-center py-12 text-text-sub opacity-70">
          <CalendarIcon className="w-12 h-12 mb-3 stroke-current opacity-40" />
          <span className="text-[14px]">{t('calendar.no_schedules')}</span>
        </div>
      )}

      {/* Create Button (Floating inside list) */}
      <div className="mt-4 flex justify-center">
        <button
          className="flex items-center gap-1.5 text-primary-blue text-[14px] font-medium py-2 px-4 rounded-full bg-blue-50 dark:bg-blue-900/20 active:opacity-80 transition-opacity"
          onClick={() => setIsAdding(true)}
        >
          <Plus className="w-4 h-4" />
          {t('calendar.add_schedule')}
        </button>
      </div>
    </div>
  );
};
