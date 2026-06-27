import React, { useState, useEffect } from 'react';
import { ChevronLeft, ChevronRight, Plus, Search, Calendar as CalendarIcon, Check } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { calendarService, CalendarList } from './services/CalendarService';
import { toast } from '@sdkwork/im-pc-chat';

interface SidebarProps {
  currentDate: Date;
  setCurrentDate: (date: Date) => void;
  onNewEvent: () => void;
}

export const CalendarSidebar: React.FC<SidebarProps> = ({ currentDate, setCurrentDate, onNewEvent }) => {
  const [miniYear, setMiniYear] = useState(currentDate.getFullYear());
  const [miniMonth, setMiniMonth] = useState(currentDate.getMonth());
  const [calendars, setCalendars] = useState<CalendarList[]>([]);
  const [showSubMenu, setShowSubMenu] = useState(false);
  const [showUrlModal, setShowUrlModal] = useState(false);
  const [urlInput, setUrlInput] = useState('');

  useEffect(() => {
    calendarService.getCalendars().then(setCalendars);
  }, []);

  const handlePrevMonth = () => {
    if (miniMonth === 0) {
      setMiniMonth(11); setMiniYear(prev => prev - 1);
    } else {
      setMiniMonth(prev => prev - 1);
    }
  };

  const handleNextMonth = () => {
    if (miniMonth === 11) {
      setMiniMonth(0); setMiniYear(prev => prev + 1);
    } else {
      setMiniMonth(prev => prev + 1);
    }
  };

  // Generate mini calendar dates
  const firstDay = new Date(miniYear, miniMonth, 1).getDay();
  const daysInMonth = new Date(miniYear, miniMonth + 1, 0).getDate();
  const prevMonthDays = new Date(miniYear, miniMonth, 0).getDate();
  
  const days = [];
  // previous month trailing days
  for (let i = firstDay - 1; i >= 0; i--) {
     days.push({ day: prevMonthDays - i, isCurrentMonth: false, date: new Date(miniYear, miniMonth - 1, prevMonthDays - i) });
  }
  // current month days
  for (let i = 1; i <= daysInMonth; i++) {
     days.push({ day: i, isCurrentMonth: true, date: new Date(miniYear, miniMonth, i) });
  }
  // next month trailing days
  const remainingCells = 42 - days.length;
  for (let i = 1; i <= remainingCells; i++) {
     days.push({ day: i, isCurrentMonth: false, date: new Date(miniYear, miniMonth + 1, i) });
  }

  const isSameDay = (d1: Date, d2: Date) => d1.getFullYear() === d2.getFullYear() && d1.getMonth() === d2.getMonth() && d1.getDate() === d2.getDate();
  const isToday = (d: Date) => isSameDay(d, new Date());

  return (
    <div className="w-[300px] h-full flex flex-col shrink-0 bg-[#121212] font-sans">
      {/* Top Creation Buttons */}
      <div className="p-6 pb-2 border-b border-transparent">
        <button 
          className="w-full flex items-center justify-center gap-2 bg-[#ea4335] hover:bg-[#d93025] text-white font-bold py-3.5 rounded-xl transition-all shadow-[0_4px_14px_0_rgba(234,67,53,0.39)]"
          onClick={onNewEvent}
        >
          <Plus size={20} />
          <span className="tracking-wide">新建日程</span>
        </button>
      </div>
      
      {/* Mini Calendar */}
      <div className="px-6 py-4">
        <div className="flex items-center justify-between mb-4 text-[15px] font-bold tracking-wide">
          <span>{miniYear}年 {miniMonth + 1}月</span>
          <div className="flex items-center gap-0.5">
            <button className="p-1 hover:bg-white/10 rounded-md transition-colors" onClick={handlePrevMonth}><ChevronLeft size={18}/></button>
            <button className="p-1 hover:bg-white/10 rounded-md transition-colors" onClick={handleNextMonth}><ChevronRight size={18}/></button>
          </div>
        </div>
        <div className="grid grid-cols-7 text-center text-[11px] font-bold tracking-widest text-gray-500 mb-2 uppercase">
          {['日', '一', '二', '三', '四', '五', '六'].map(d => <div key={d}>{d}</div>)}
        </div>
        <div className="grid grid-cols-7 gap-y-1.5 text-center text-sm font-medium">
          {days.map((item, idx) => {
             const selected = isSameDay(item.date, currentDate);
             const today = isToday(item.date);
             return (
               <div key={idx} className="flex flex-col items-center justify-center h-8 text-[13px] relative">
                 <button 
                   onClick={() => {
                     setCurrentDate(item.date);
                     setMiniYear(item.date.getFullYear());
                     setMiniMonth(item.date.getMonth());
                   }}
                   className={cn(
                     "w-7 h-7 rounded-full flex items-center justify-center transition-all relative z-10",
                     selected ? "bg-white text-black font-black shadow-md scale-110" : (
                       today ? "text-[#ea4335] bg-[#ea4335]/10 font-black shadow-inner" : (
                         item.isCurrentMonth ? "text-gray-300 hover:bg-white/10 hover:scale-110" : "text-gray-600 hover:text-gray-400"
                       )
                     )
                   )}
                 >
                   {item.day}
                 </button>
                 {item.isCurrentMonth && (item.day === new Date().getDate() || item.day === new Date().getDate() + 1) && !selected && (
                    <div className="absolute bottom-[1px] w-1 h-1 rounded-full bg-[#ea4335] opacity-80" />
                 )}
               </div>
             )
          })}
        </div>
      </div>

      <div className="px-6 py-4 border-t border-white/5 flex-1 overflow-y-auto custom-scrollbar">
        <h3 className="text-[11px] font-bold text-gray-500 uppercase tracking-widest mb-3 px-1">我的日历</h3>
        <div className="space-y-1">
          {calendars.map(cal => (
            <label key={cal.id} className="flex items-center gap-3 px-2 py-1.5 rounded-lg hover:bg-white/5 cursor-pointer group">
              <input 
                type="checkbox" 
                className="hidden" 
                checked={cal.checked} 
                onChange={() => {
                  const newCalendars = calendars.map(c => c.id === cal.id ? { ...c, checked: !c.checked } : c);
                  setCalendars(newCalendars);
                }}
              />
              <div 
                className="w-4 h-4 rounded-[4px] border-2 flex items-center justify-center transition-colors shadow-sm shrink-0"
                style={{ 
                  borderColor: cal.color, 
                  backgroundColor: cal.checked ? cal.color : 'transparent' 
                }}
              >
                {cal.checked && <Check size={10} strokeWidth={3} className="text-[#121212]" />}
              </div>
              <span className="text-[13px] text-gray-300 group-hover:text-white transition-colors truncate">{cal.name}</span>
            </label>
          ))}
        </div>
        
        <h3 className="text-[12px] font-bold text-gray-400 uppercase tracking-wider mb-3 px-1 mt-6">其他订阅</h3>
        <div className="relative">
          <button className="flex items-center gap-2 px-2 py-1.5 text-[13px] text-gray-400 hover:text-white transition-colors" onClick={() => setShowSubMenu(!showSubMenu)}>
            <Plus size={16} /> 订阅公共日历
          </button>
          {showSubMenu && (
            <>
              <div className="fixed inset-0 z-40" onClick={() => setShowSubMenu(false)} />
              <div className="absolute top-8 left-2 w-48 bg-[#282828] border border-white/10 rounded-xl shadow-xl z-50 p-1.5 animate-in fade-in zoom-in-95">
                <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={async () => {
                   setShowSubMenu(false); 
                   const newCal = await calendarService.createCalendar({name: '中国法定节假日', color: '#ea4335', checked: true});
                   setCalendars([...calendars, newCal]);
                   toast('已成功订阅中国节假日日历', 'success'); 
                }}>中国法定节假日</button>
                <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { setShowSubMenu(false); setShowUrlModal(true); setUrlInput(''); }}>从 URL 订阅</button>
                <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { setShowSubMenu(false); toast('已进入发现联系人日历界面', 'success'); }}>浏览同事公开日历</button>
              </div>
            </>
          )}
          {showUrlModal && (
             <div className="fixed inset-0 z-50 flex items-center justify-center">
                <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={() => setShowUrlModal(false)}></div>
                <div className="relative bg-[#282828] border border-white/10 rounded-2xl w-full max-w-sm shadow-xl p-6">
                   <h3 className="text-lg font-medium text-white mb-4">从 URL 订阅日历</h3>
                   <input
                     type="text"
                     placeholder="https://..."
                     className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 mb-6"
                     value={urlInput}
                     onChange={(e) => setUrlInput(e.target.value)}
                   />
                   <div className="flex justify-end gap-3">
                      <button className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors" onClick={() => setShowUrlModal(false)}>取消</button>
                      <button className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium disabled:opacity-50" disabled={!urlInput.trim()} onClick={async () => { 
                         setShowUrlModal(false); 
                         const newCal = await calendarService.createCalendar({name: urlInput, color: '#1a73e8', checked: true});
                         setCalendars([...calendars, newCal]);
                         toast('日历订阅成功', 'success'); 
                      }}>订阅</button>
                   </div>
                </div>
             </div>
          )}
        </div>
      </div>
    </div>
  );
}
