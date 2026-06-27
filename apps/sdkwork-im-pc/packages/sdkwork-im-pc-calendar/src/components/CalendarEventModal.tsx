import React from 'react';
import { motion } from 'motion/react';
import { Settings, Trash2, X, Clock, MapPin, AlignLeft, UserPlus } from 'lucide-react';
import { CalendarEvent } from '../services/CalendarService';

interface CalendarEventModalProps {
  selectedEvent: CalendarEvent;
  setSelectedEvent: (event: CalendarEvent | null) => void;
  onEditEvent?: (event: CalendarEvent) => void;
  onDeleteEvent?: (id: string) => void;
}

export const CalendarEventModal: React.FC<CalendarEventModalProps> = ({
  selectedEvent,
  setSelectedEvent,
  onEditEvent,
  onDeleteEvent,
}) => {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.15 }}
      className="absolute inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
      onClick={() => setSelectedEvent(null)}
    >
      <motion.div
        initial={{ scale: 0.95, opacity: 0, y: 10 }}
        animate={{ scale: 1, opacity: 1, y: 0 }}
        exit={{ scale: 0.95, opacity: 0, y: 10 }}
        transition={{ type: "spring", damping: 25, stiffness: 300 }}
        className="w-full max-w-[440px] bg-[#1a1a1a] rounded-2xl shadow-2xl border border-white/10 overflow-hidden flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Modal Header */}
        <div className="h-16 px-6 flex items-center justify-between border-b border-white/5 relative bg-white/5">
          <div
            className="absolute top-0 left-0 right-0 h-1"
            style={{ backgroundColor: selectedEvent.color }}
          />
          <h2 className="text-[16px] font-bold text-white tracking-wide">
            日程详情
          </h2>
          <div className="flex items-center gap-2">
            <button
              className="p-1.5 rounded-lg hover:bg-white/10 transition-colors text-gray-400 hover:text-white"
              title="编辑"
              onClick={() => {
                if (onEditEvent && selectedEvent) {
                  onEditEvent(selectedEvent);
                }
              }}
            >
              <Settings size={16} />
            </button>
            <button
              className="p-1.5 rounded-lg hover:bg-red-500/20 text-red-400 transition-colors"
              title="删除"
              onClick={() => {
                if (onDeleteEvent && selectedEvent) {
                  onDeleteEvent(selectedEvent.id);
                  setSelectedEvent(null);
                }
              }}
            >
              <Trash2 size={16} />
            </button>
            <button
              className="p-1.5 rounded-lg hover:bg-white/10 transition-colors text-gray-400 hover:text-white ml-2"
              onClick={() => setSelectedEvent(null)}
            >
              <X size={20} />
            </button>
          </div>
        </div>

        {/* Modal Body */}
        <div className="p-6 flex flex-col gap-6">
          {/* Title */}
          <div className="flex gap-4">
            <div className="w-5 h-5 rounded mt-1 shrink-0 bg-white/10 flex items-center justify-center border border-white/5">
              <div
                className="w-2.5 h-2.5 rounded-full shadow-sm"
                style={{ backgroundColor: selectedEvent.color }}
              />
            </div>
            <div className="flex flex-col gap-1 min-w-0">
              <h3 className="text-xl font-black text-white leading-tight break-words">
                {selectedEvent.title}
              </h3>
              <p className="text-[13px] text-gray-400 font-medium">
                所属日历: 默认日历
              </p>
            </div>
          </div>

          {/* Details List */}
          <div className="flex flex-col gap-4 pl-9">
            <div className="flex items-center gap-3 text-[14px]">
              <Clock size={16} className="text-gray-400 shrink-0" />
              <span className="text-gray-200 font-medium">
                {selectedEvent.date} · {selectedEvent.time}
              </span>
            </div>
            {selectedEvent.location && (
              <div className="flex items-center gap-3 text-[14px]">
                <MapPin size={16} className="text-gray-400 shrink-0" />
                <span className="text-gray-200">{selectedEvent.location}</span>
              </div>
            )}
            {selectedEvent.description && (
              <div className="flex items-start gap-3 text-[14px]">
                <AlignLeft
                  size={16}
                  className="text-gray-400 shrink-0 mt-1"
                />
                <p className="text-gray-300 leading-relaxed max-h-32 overflow-y-auto custom-scrollbar">
                  {selectedEvent.description}
                </p>
              </div>
            )}
            {selectedEvent.guests && selectedEvent.guests.length > 0 && (
              <div className="flex items-center gap-3 text-[14px] mt-2">
                <UserPlus size={16} className="text-gray-400 shrink-0" />
                <div className="flex -space-x-2">
                  {selectedEvent.guests.slice(0, 5).map((guest, i) => (
                    <div
                      key={i}
                      className="w-7 h-7 rounded-full border-2 border-[#1a1a1a] bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-[10px] font-bold text-white relative group"
                    >
                      {guest.charAt(0).toUpperCase()}
                      <div className="absolute -bottom-8 left-1/2 -translate-x-1/2 bg-black text-white text-[10px] px-2 py-1 rounded opacity-0 group-hover:opacity-100 whitespace-nowrap pointer-events-none transition-opacity z-10">
                        {guest}
                      </div>
                    </div>
                  ))}
                  {selectedEvent.guests.length > 5 && (
                    <div className="w-7 h-7 rounded-full border-2 border-[#1a1a1a] bg-white/10 flex items-center justify-center text-[10px] text-gray-300">
                      +{selectedEvent.guests.length - 5}
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>
        </div>
      </motion.div>
    </motion.div>
  );
};
