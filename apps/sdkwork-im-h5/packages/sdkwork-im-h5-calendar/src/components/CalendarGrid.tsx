import React from 'react';
import { cn, IconButton } from '@sdkwork/im-h5-commons';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { format } from 'date-fns';
import { useTranslation } from 'react-i18next';

interface CalendarGridProps {
  currentDate: Date;
  setCurrentDate: (date: Date) => void;
  indicators: string[];
}

export const CalendarGrid: React.FC<CalendarGridProps> = ({
  currentDate,
  setCurrentDate,
  indicators,
}) => {
  const { t } = useTranslation();
const year = currentDate.getFullYear();
  const month = currentDate.getMonth();

  const getDaysInMonth = (year: number, month: number) =>
    new Date(year, month + 1, 0).getDate();
  const getFirstDayOfMonth = (year: number, month: number) =>
    new Date(year, month, 1).getDay();

  const daysInMonth = getDaysInMonth(year, month);
  const firstDay = getFirstDayOfMonth(year, month);

  const days = [];
  const prevMonthDays = getDaysInMonth(year, month - 1);
  for (let i = 0; i < firstDay; i++) {
    days.push({
      day: prevMonthDays - firstDay + i + 1,
      currentMonth: false,
      dateStr: "",
    });
  }
  for (let i = 1; i <= daysInMonth; i++) {
    days.push({
      day: i,
      currentMonth: true,
      isToday:
        i === new Date().getDate() &&
        month === new Date().getMonth() &&
        year === new Date().getFullYear(),
      dateStr: format(new Date(year, month, i), "yyyy-MM-dd"),
    });
  }
  const remainingSlots = 42 - days.length;
  for (let i = 1; i <= remainingSlots; i++) {
    days.push({ day: i, currentMonth: false, dateStr: "" });
  }

  const prevMonth = () =>
    setCurrentDate(new Date(year, month - 1, currentDate.getDate()));
  const nextMonth = () =>
    setCurrentDate(new Date(year, month + 1, currentDate.getDate()));
  const selectDay = (day: number) => setCurrentDate(new Date(year, month, day));

  const weekDays = [
    t('calendar.week_days.sun'),
    t('calendar.week_days.mon'),
    t('calendar.week_days.tue'),
    t('calendar.week_days.wed'),
    t('calendar.week_days.thu'),
    t('calendar.week_days.fri'),
    t('calendar.week_days.sat')
  ];

  return (
    <div className="bg-bg-color px-4 pb-4 border-b border-border-color shadow-sm z-10">
      <div className="flex items-center justify-between py-2">
        <IconButton
          icon={<ChevronLeft className="w-5 h-5 text-text-sub" />}
          onClick={prevMonth}
        />
        <div className="text-[15px] font-bold tracking-wide">
          {currentDate.toLocaleDateString("zh-CN", {
            month: "long",
            year: "numeric",
          })}
        </div>
        <IconButton
          icon={<ChevronRight className="w-5 h-5 text-text-sub" />}
          onClick={nextMonth}
        />
      </div>

      <div className="grid grid-cols-7 mb-2">
        {weekDays.map((day, ix) => (
          <div
            key={ix}
            className="text-center text-[12px] text-text-sub font-medium py-2"
          >
            {day}
          </div>
        ))}
      </div>

      <div className="grid grid-cols-7 gap-y-2">
        {days.map((item, idx) => (
          <div
            key={idx}
            className="flex flex-col items-center justify-center h-10 w-full relative cursor-pointer"
            onClick={() => item.currentMonth && selectDay(item.day)}
          >
            <div
              className={cn(
                "w-8 h-8 rounded-full flex items-center justify-center text-[15px] transition-colors",
                item.isToday
                  ? "bg-primary-blue text-white font-bold"
                  : item.currentMonth && item.day === currentDate.getDate()
                    ? "border border-primary-blue text-primary-blue"
                    : "",
                !item.currentMonth
                  ? "text-text-sub/40"
                  : !item.isToday &&
                      item.day !== currentDate.getDate() &&
                      "text-text-main font-medium",
              )}
            >
              {item.day}
            </div>
            {item.currentMonth &&
              indicators.includes(item.dateStr) &&
              !item.isToday && (
                <div className="w-1 h-1 rounded-full bg-blue-500 absolute bottom-0"></div>
              )}
            {item.currentMonth &&
              indicators.includes(item.dateStr) &&
              item.isToday && (
                <div className="w-1 h-1 rounded-full bg-white absolute bottom-1"></div>
              )}
          </div>
        ))}
      </div>
    </div>
  );
};
