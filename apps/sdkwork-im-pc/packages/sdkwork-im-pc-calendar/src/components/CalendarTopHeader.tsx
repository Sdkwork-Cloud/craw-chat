import React, { useState } from 'react';
import { ChevronLeft, ChevronRight, ListFilter, Settings } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '@sdkwork/im-pc-chat';

interface CalendarTopHeaderProps {
  year: number;
  month: number;
  viewMode: 'month' | 'week' | 'day';
  setViewMode: (mode: 'month' | 'week' | 'day') => void;
  handleToday: () => void;
  handlePrevMonth: () => void;
  handleNextMonth: () => void;
}

export const CalendarTopHeader: React.FC<CalendarTopHeaderProps> = ({
  year,
  month,
  viewMode,
  setViewMode,
  handleToday,
  handlePrevMonth,
  handleNextMonth,
}) => {
  const [showFilters, setShowFilters] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

  const monthNames = [
    '一月', '二月', '三月', '四月', '五月', '六月',
    '七月', '八月', '九月', '十月', '十一月', '十二月',
  ];

  return (
    <div className="h-[68px] flex items-center justify-between px-6 border-b border-white/5 shrink-0 bg-[#0A0A0A] relative z-20">
      <div className="flex items-center gap-6">
        <h1 className="text-2xl font-black text-white min-w-[160px] tracking-tight">
          {year}年 {monthNames[month]}
        </h1>
        <div className="flex items-center gap-1.5 bg-white/5 px-1 py-1 rounded-lg border border-white/5 shadow-sm">
          <button
            className="px-3 py-1 text-sm font-bold rounded hover:bg-white/10 transition-colors text-white"
            onClick={handleToday}
          >
            今天
          </button>
          <div className="w-px h-4 bg-white/10 mx-1" />
          <button
            className="p-1 rounded hover:bg-white/10 transition-colors"
            onClick={handlePrevMonth}
          >
            <ChevronLeft size={18} className="text-gray-300" />
          </button>
          <button
            className="p-1 rounded hover:bg-white/10 transition-colors"
            onClick={handleNextMonth}
          >
            <ChevronRight size={18} className="text-gray-300" />
          </button>
        </div>
      </div>

      <div className="flex items-center gap-4">
        {/* View Mode Toggle */}
        <div className="flex items-center bg-white/5 rounded-lg p-1 border border-white/5 shadow-inner">
          {['month', 'week', 'day'].map((mode, idx) => (
            <button
              key={mode}
              onClick={() => {
                setViewMode(mode as any);
                toast(`已切换到${['月', '周', '日'][idx]}视图`, 'success');
              }}
              className={cn(
                'px-4 py-1.5 text-[13px] font-bold rounded-md transition-all shadow-sm',
                viewMode === mode
                  ? 'bg-[#2b2b2d] text-white border border-white/5'
                  : 'text-gray-400 hover:text-white'
              )}
            >
              {['月', '周', '日'][idx]}
            </button>
          ))}
        </div>

        <div className="w-px h-6 bg-white/10 mx-2" />

        <div className="relative">
          <button
            className={cn(
              'w-9 h-9 rounded-full flex items-center justify-center transition-colors group',
              showFilters
                ? 'bg-white/10 text-white'
                : 'hover:bg-white/10 text-gray-400 hover:text-white'
            )}
            onClick={() => {
              setShowFilters(!showFilters);
              setShowSettings(false);
            }}
          >
            <ListFilter
              size={18}
              className="group-hover:scale-110 transition-transform"
            />
          </button>
          {showFilters && (
            <div className="absolute top-12 right-0 w-48 bg-[#282828] border border-white/10 shadow-xl rounded-xl z-50 p-2 animate-in fade-in zoom-in-95">
              <div className="px-3 py-2 text-xs font-bold text-gray-400 tracking-wider">
                显示筛选
              </div>
              <label className="flex items-center gap-3 px-3 py-2 hover:bg-white/5 rounded-lg cursor-pointer">
                <input
                  type="checkbox"
                  defaultChecked
                  className="accent-[#1db954]"
                />{' '}
                <span className="text-sm text-gray-200">会议申请</span>
              </label>
              <label className="flex items-center gap-3 px-3 py-2 hover:bg-white/5 rounded-lg cursor-pointer">
                <input
                  type="checkbox"
                  defaultChecked
                  className="accent-[#1db954]"
                />{' '}
                <span className="text-sm text-gray-200">个人日程</span>
              </label>
              <label className="flex items-center gap-3 px-3 py-2 hover:bg-white/5 rounded-lg cursor-pointer">
                <input
                  type="checkbox"
                  defaultChecked
                  className="accent-[#1db954]"
                />{' '}
                <span className="text-sm text-gray-200">休假提醒</span>
              </label>
            </div>
          )}
        </div>

        <div className="relative">
          <button
            className={cn(
              'w-9 h-9 rounded-full flex items-center justify-center transition-colors group',
              showSettings
                ? 'bg-white/10 text-white'
                : 'hover:bg-white/10 text-gray-400 hover:text-white'
            )}
            onClick={() => {
              setShowSettings(!showSettings);
              setShowFilters(false);
            }}
          >
            <Settings
              size={18}
              className="group-hover:scale-110 transition-transform"
            />
          </button>
          {showSettings && (
            <div className="absolute top-12 right-0 w-64 bg-[#282828] border border-white/10 shadow-xl rounded-xl z-50 p-2 animate-in fade-in zoom-in-95">
              <div className="px-3 py-2 text-xs font-bold text-gray-400 tracking-wider">
                快速设置
              </div>
              <div className="p-3 bg-white/5 rounded-lg mt-1 mb-2">
                <div className="text-sm text-gray-200 mb-2">默认视图</div>
                <select className="w-full bg-[#1a1a1a] border border-white/10 rounded-md p-1.5 text-sm outline-none text-gray-300">
                  <option>月视图</option>
                  <option>周视图</option>
                  <option>日视图</option>
                </select>
              </div>
              <label className="flex items-center justify-between px-3 py-2 hover:bg-white/5 rounded-lg cursor-pointer">
                <span className="text-sm text-gray-200">显示拒绝的日程</span>
                <input type="checkbox" className="accent-[#1db954]" />
              </label>
              <label className="flex items-center justify-between px-3 py-2 hover:bg-white/5 rounded-lg cursor-pointer">
                <span className="text-sm text-gray-200">显示周数</span>
                <input
                  type="checkbox"
                  defaultChecked
                  className="accent-[#1db954]"
                />
              </label>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
