import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X, Calendar as CalendarIcon, Clock, MapPin, AlignLeft, Users, Palette } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { CalendarEvent } from './services/CalendarService';
import { toast } from '@sdkwork/im-pc-chat';

interface CreateEventModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreate: (event: CalendarEvent) => void;
  selectedDate: Date | null;
  initialEvent?: CalendarEvent | null;
}

const colors = [
  { id: 'red', hex: '#ea4335' },
  { id: 'blue', hex: '#1a73e8' },
  { id: 'yellow', hex: '#fbbc05' },
  { id: 'green', hex: '#34a853' },
  { id: 'purple', hex: '#a142f4' },
];

export const CreateEventModal: React.FC<CreateEventModalProps> = ({ isOpen, onClose, onCreate, selectedDate, initialEvent }) => {
  const [title, setTitle] = useState('');
  const [date, setDate] = useState('');
  const [time, setTime] = useState('10:00');
  const [color, setColor] = useState(colors[0].hex);
  const [guests, setGuests] = useState<string[]>([]);
  const [guestInput, setGuestInput] = useState('');
  const [location, setLocation] = useState('');
  const [description, setDescription] = useState('');

  useEffect(() => {
    if (isOpen) {
      if (initialEvent) {
        setTitle(initialEvent.title);
        setDate(initialEvent.date);
        setTime(initialEvent.time);
        setColor(initialEvent.color);
        setGuests(initialEvent.guests || []);
        setLocation(initialEvent.location || '');
        setDescription(initialEvent.description || '');
        setGuestInput('');
      } else {
        setTitle('');
        setGuests([]);
        setGuestInput('');
        setLocation('');
        setDescription('');
        if (selectedDate) {
          const year = selectedDate.getFullYear();
          const month = String(selectedDate.getMonth() + 1).padStart(2, '0');
          const day = String(selectedDate.getDate()).padStart(2, '0');
          setDate(`${year}-${month}-${day}`);
        } else {
          const today = new Date();
          const year = today.getFullYear();
          const month = String(today.getMonth() + 1).padStart(2, '0');
          const day = String(today.getDate()).padStart(2, '0');
          setDate(`${year}-${month}-${day}`);
        }
        setTime('10:00');
        setColor(colors[0].hex);
      }
    }
  }, [isOpen, selectedDate, initialEvent]);

  if (!isOpen) return null;

  const handleCreate = () => {
    if (!title.trim() || !date || !time) return;
    
    const newEvent: CalendarEvent = {
      id: initialEvent ? initialEvent.id : Date.now().toString(),
      title,
      date,
      time,
      color,
      location,
      description,
      guests
    };
    
    onCreate(newEvent);
    toast(initialEvent ? '日程已更新' : '日程保存成功', 'success');
  };

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="absolute inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
        onClick={onClose}
      >
        <motion.div
          initial={{ scale: 0.95, opacity: 0, y: 10 }}
          animate={{ scale: 1, opacity: 1, y: 0 }}
          exit={{ scale: 0.95, opacity: 0, y: 10 }}
          transition={{ type: "spring", damping: 25, stiffness: 300 }}
          className="w-full max-w-[480px] bg-[#1a1a1a] rounded-2xl shadow-2xl border border-white/10 overflow-hidden flex flex-col"
          onClick={e => e.stopPropagation()}
        >
          {/* Header */}
          <div className="h-14 px-6 flex items-center justify-between border-b border-white/5 relative bg-white/5">
            <h2 className="text-[16px] font-bold text-white tracking-wide">{initialEvent ? '编辑日程安排' : '新建日程安排'}</h2>
            <button className="p-1.5 rounded-lg hover:bg-white/10 transition-colors text-gray-400 hover:text-white" onClick={onClose}>
              <X size={20} />
            </button>
          </div>

          {/* Form Content */}
          <div className="p-7 flex flex-col gap-6">
            {/* Title Input */}
            <div className="relative">
              <input
                type="text"
                placeholder="添加标题..."
                value={title}
                onChange={e => setTitle(e.target.value)}
                className="w-full bg-transparent text-[22px] font-black text-white outline-none placeholder:text-gray-600 border-b border-white/10 pb-2 transition-colors relative z-10"
                autoFocus
              />
              <div 
                className="absolute bottom-0 left-0 h-[2px] transition-all duration-300 z-20" 
                style={{ width: title ? '100%' : '0%', backgroundColor: color }} 
              />
            </div>

            <div className="flex flex-col gap-5 mt-2">
              {/* Date & Time */}
              <div className="flex items-center gap-4">
                <div className="flex-1 flex items-center gap-3">
                  <div className="w-[36px] h-[36px] rounded-xl flex items-center justify-center shrink-0 transition-colors shadow-sm" style={{ backgroundColor: color + '15', color: color }}>
                    <CalendarIcon size={18} strokeWidth={2.5} />
                  </div>
                  <div className="flex-1 border-b border-white/5 pb-1 focus-within:border-white/20 transition-colors">
                    <input
                      type="date"
                      value={date}
                      onChange={e => setDate(e.target.value)}
                      className="w-full bg-transparent text-[15px] font-medium text-gray-200 outline-none cursor-pointer [&::-webkit-calendar-picker-indicator]:filter [&::-webkit-calendar-picker-indicator]:invert-[0.6]"
                    />
                  </div>
                </div>

                <div className="w-3 h-[2px] rounded-full shrink-0 transition-colors" style={{ backgroundColor: color + '30' }} />

                <div className="w-[140px] flex items-center gap-3">
                  <div className="w-[36px] h-[36px] rounded-xl flex items-center justify-center shrink-0 transition-colors shadow-sm" style={{ backgroundColor: color + '15', color: color }}>
                    <Clock size={18} strokeWidth={2.5} />
                  </div>
                  <div className="flex-1 border-b border-white/5 pb-1 focus-within:border-white/20 transition-colors">
                    <input
                      type="time"
                      value={time}
                      onChange={e => setTime(e.target.value)}
                      className="w-full bg-transparent text-[15px] font-medium text-gray-200 outline-none cursor-pointer [&::-webkit-calendar-picker-indicator]:filter [&::-webkit-calendar-picker-indicator]:invert-[0.6]"
                    />
                  </div>
                </div>
              </div>

              {/* Location */}
              <div className="flex items-center gap-3">
                <div className="w-[36px] h-[36px] rounded-xl flex items-center justify-center shrink-0 transition-colors shadow-sm" style={{ backgroundColor: color + '15', color: color }}>
                  <MapPin size={18} strokeWidth={2.5} />
                </div>
                <div className="flex-1 border-b border-white/5 pb-1 focus-within:border-white/20 transition-colors">
                  <input
                    type="text"
                    placeholder="添加地点"
                    value={location}
                    onChange={e => setLocation(e.target.value)}
                    className="w-full bg-transparent text-[15px] text-gray-200 outline-none placeholder:text-gray-600"
                  />
                </div>
              </div>

              {/* Guests */}
              <div className="flex items-start gap-3">
                <div className="w-[36px] h-[36px] rounded-xl flex items-center justify-center shrink-0 transition-colors shadow-sm mt-1" style={{ backgroundColor: color + '15', color: color }}>
                  <Users size={18} strokeWidth={2.5} />
                </div>
                <div className="flex-1 flex flex-col gap-2 border-b border-white/5 pb-2 focus-within:border-white/20 transition-colors pt-1">
                  <div className="flex flex-wrap gap-2">
                    {guests.map((guest, idx) => (
                      <div key={idx} className="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[13px] font-medium text-white shadow-sm border" style={{ backgroundColor: color + '15', borderColor: color + '20' }}>
                         <div className="w-4 h-4 rounded-full flex items-center justify-center text-[9px] font-black text-white" style={{ backgroundColor: color }}>
                           {guest.charAt(0).toUpperCase()}
                         </div>
                         <span>{guest}</span>
                         <button 
                           onClick={() => setGuests(guests.filter((_, i) => i !== idx))}
                           className="hover:text-white transition-colors ml-0.5"
                           style={{ color: color + '90' }}
                         >
                           <X size={14} />
                         </button>
                      </div>
                    ))}
                    <input
                      type="text"
                      placeholder={guests.length === 0 ? "添加参与者 (输入后回车)" : "继续添加..."}
                      value={guestInput}
                      onChange={e => setGuestInput(e.target.value)}
                      onKeyDown={e => {
                        if (e.key === 'Enter' && guestInput.trim()) {
                           e.preventDefault();
                           if (!guests.includes(guestInput.trim())) {
                              setGuests([...guests, guestInput.trim()]);
                           }
                           setGuestInput('');
                        } else if (e.key === 'Backspace' && !guestInput && guests.length > 0) {
                           setGuests(guests.slice(0, -1));
                        }
                      }}
                      className="flex-1 min-w-[140px] bg-transparent text-[15px] text-gray-200 outline-none placeholder:text-gray-600 mt-1"
                    />
                  </div>
                </div>
              </div>

              {/* Description */}
              <div className="flex items-start gap-3 mt-1">
                <div className="w-[36px] h-[36px] rounded-xl flex items-center justify-center shrink-0 transition-colors shadow-sm mt-0.5" style={{ backgroundColor: color + '15', color: color }}>
                  <AlignLeft size={18} strokeWidth={2.5} />
                </div>
                <div className="flex-1">
                  <textarea
                    placeholder="添加描述..."
                    rows={3}
                    value={description}
                    onChange={e => setDescription(e.target.value)}
                    className="w-full bg-[#1e1e1e] text-[15px] text-gray-200 rounded-xl px-4 py-3 outline-none border border-white/5 focus:border-white/20 custom-scrollbar resize-none transition-colors"
                  />
                </div>
              </div>

              {/* Color Selection */}
              <div className="flex items-center gap-3 mt-1">
                <div className="w-[36px] h-[36px] rounded-xl flex items-center justify-center shrink-0 transition-colors shadow-sm" style={{ backgroundColor: color + '15', color: color }}>
                  <Palette size={18} strokeWidth={2.5} />
                </div>
                <div className="flex items-center gap-3 ml-1">
                  {colors.map(c => (
                    <button
                      key={c.id}
                      onClick={() => setColor(c.hex)}
                      className={cn(
                        "w-6 h-6 rounded-full transition-all outline-none border-[3px] shadow-sm",
                        color === c.hex ? "scale-125 border-white" : "border-transparent hover:scale-110"
                      )}
                      style={{ backgroundColor: c.hex }}
                    />
                  ))}
                </div>
              </div>
            </div>
          </div>

          {/* Footer Actions */}
          <div className="px-6 py-4 bg-white/[0.02] border-t border-white/5 flex items-center justify-end gap-3">
            <button 
              className="px-5 py-2 text-[15px] font-medium text-gray-400 hover:text-white hover:bg-white/5 rounded-xl transition-colors"
              onClick={onClose}
            >
              取消
            </button>
            <button 
              className="px-6 py-2 text-[15px] font-bold text-white rounded-xl shadow-md transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:scale-100 hover:scale-[1.02]"
              onClick={handleCreate}
              disabled={!title.trim() || !date || !time}
              style={{ backgroundColor: color, boxShadow: `0 4px 14px 0 ${color}40` }}
            >
              {initialEvent ? '保存修改' : '保存日程'}
            </button>
          </div>
        </motion.div>
      </motion.div>
    </AnimatePresence>
  );
};
