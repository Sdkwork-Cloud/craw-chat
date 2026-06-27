import React, { useState } from 'react';
import { cn } from '@sdkwork/im-pc-commons';
import { CalendarEvent } from './services/CalendarService';
import { AnimatePresence } from 'motion/react';
import { CalendarTopHeader } from './components/CalendarTopHeader';
import { CalendarEventModal } from './components/CalendarEventModal';

interface ViewProps {
  currentDate: Date;
  setCurrentDate: (date: Date) => void;
  events: CalendarEvent[];
  onDeleteEvent?: (id: string) => void;
  onEditEvent?: (event: CalendarEvent) => void;
}

export const CalendarContent: React.FC<ViewProps> = ({ currentDate, setCurrentDate, events, onDeleteEvent, onEditEvent }) => {
  const [viewMode, setViewMode] = useState<'month'|'week'|'day'>('month');
  const [selectedEvent, setSelectedEvent] = useState<CalendarEvent | null>(null);

  const year = currentDate.getFullYear();
  const month = currentDate.getMonth();

  const handlePrevMonth = () => setCurrentDate(new Date(year, month - 1, 1));
  const handleNextMonth = () => setCurrentDate(new Date(year, month + 1, 1));
  const handleToday = () => setCurrentDate(new Date());

  const firstDay = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const prevMonthDays = new Date(year, month, 0).getDate();
  
  const calendarCells = [];
  for (let i = firstDay - 1; i >= 0; i--) {
     calendarCells.push({ date: new Date(year, month - 1, prevMonthDays - i), isCurrentMonth: false });
  }
  for (let i = 1; i <= daysInMonth; i++) {
     calendarCells.push({ date: new Date(year, month, i), isCurrentMonth: true });
  }
  const remainingCells = 42 - calendarCells.length;
  for (let i = 1; i <= remainingCells; i++) {
     calendarCells.push({ date: new Date(year, month + 1, i), isCurrentMonth: false });
  }

  const isSameDay = (d1: Date, d2: Date) => d1.getFullYear() === d2.getFullYear() && d1.getMonth() === d2.getMonth() && d1.getDate() === d2.getDate();
  const isToday = (d: Date) => isSameDay(d, new Date());

  return (
    <div className="flex flex-col h-full bg-[#0A0A0A] font-sans relative">
      <CalendarTopHeader
        year={year}
        month={month}
        viewMode={viewMode}
        setViewMode={setViewMode}
        handleToday={handleToday}
        handlePrevMonth={handlePrevMonth}
        handleNextMonth={handleNextMonth}
      />
      
      {/* Calendar Grid Header */}
      <div className="grid grid-cols-7 border-b border-white/5 bg-[#121212] shrink-0 sticky top-0 z-10">
        {['周日', '周一', '周二', '周三', '周四', '周五', '周六'].map((d, i) => (
          <div key={d} className={cn("py-3 text-center text-[12px] font-bold tracking-widest uppercase", i === 0 || i === 6 ? "text-gray-500" : "text-gray-400")}>
            {d}
          </div>
        ))}
      </div>
      
      {/* Calendar Cells */}
      <div className="flex-1 grid grid-cols-7 grid-rows-6 auto-rows-fr bg-[#0A0A0A] overflow-hidden custom-scrollbar border-b border-white/5">
        {calendarCells.map((cell, idx) => {
          const dateStr = cell.date.toISOString().split('T')[0];
          const dayEvents = events.filter(e => e.date === dateStr);
          const isSelected = isSameDay(cell.date, currentDate);
          const isTD = isToday(cell.date);

          return (
            <div 
              key={idx} 
              className={cn(
                "relative border-r border-b border-white/5 p-1.5 flex flex-col transition-colors min-h-0 group cursor-pointer",
                cell.isCurrentMonth ? "bg-transparent hover:bg-white/[0.02]" : "bg-white/[0.01]",
                idx % 7 === 6 ? "border-r-0" : "" // Remove right border for last col
              )}
              onClick={() => setCurrentDate(cell.date)}
            >
              <div className="flex items-center justify-between px-1 mb-1.5">
                <span className={cn(
                  "text-[13px] w-7 h-7 flex items-center justify-center rounded-full font-bold transition-all",
                  isTD ? "bg-[#ea4335] text-white shadow-[0_0_15px_rgba(234,67,53,0.4)]" : (
                    cell.isCurrentMonth ? (isSelected ? "bg-white/20 text-white" : "text-gray-300 group-hover:text-white") : "text-gray-600"
                  )
                )}>
                  {cell.date.getDate()} {cell.date.getDate() === 1 && !isTD && <span className="text-[10px] ml-0.5 opacity-60 font-medium">{cell.date.getMonth() + 1}月</span>}
                </span>
              </div>
              
              <div className="flex-1 overflow-y-auto custom-scrollbar px-0.5 space-y-1">
                {dayEvents.map(evt => (
                  <div 
                    key={evt.id} 
                    className="px-2 py-1 text-[11px] font-bold rounded truncate hover:brightness-125 shadow-sm border border-transparent transition-all flex items-center gap-1.5 cursor-pointer relative overflow-hidden group/event"
                    style={{ 
                      backgroundColor: evt.color + '20', 
                      color: evt.color, 
                    }}
                    title={evt.title}
                    onClick={(e) => { e.stopPropagation(); setSelectedEvent(evt); }}
                  >
                    <div className="absolute left-0 top-0 bottom-0 w-1 opacity-80 group-hover/event:opacity-100 transition-opacity" style={{ backgroundColor: evt.color }} />
                    <span className="truncate ml-1">{evt.time} {evt.title}</span>
                  </div>
                ))}
              </div>
            </div>
          )
        })}
      </div>

      <AnimatePresence>
        {selectedEvent && (
          <CalendarEventModal
            selectedEvent={selectedEvent}
            setSelectedEvent={setSelectedEvent}
            onEditEvent={onEditEvent}
            onDeleteEvent={onDeleteEvent}
          />
        )}
      </AnimatePresence>
    </div>
  );
};
