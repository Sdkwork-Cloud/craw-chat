import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { X } from 'lucide-react';
import { format } from 'date-fns';
import { IconButton } from '@sdkwork/im-h5-commons';

interface CalendarAddModalProps {
  isAdding: boolean;
  setIsAdding: (val: boolean) => void;
  newTitle: string;
  setNewTitle: (val: string) => void;
  newTime: string;
  setNewTime: (val: string) => void;
  currentDate: Date;
  handleAddSchedule: () => void;
}

export const CalendarAddModal: React.FC<CalendarAddModalProps> = ({
  isAdding,
  setIsAdding,
  newTitle,
  setNewTitle,
  newTime,
  setNewTime,
  currentDate,
  handleAddSchedule,
}) => {
  const { t } = useTranslation();
return (
    <AnimatePresence>
      {isAdding && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4"
          onClick={() => setIsAdding(false)}
        >
          <motion.div
            initial={{ scale: 0.95 }}
            animate={{ scale: 1 }}
            exit={{ scale: 0.95 }}
            onClick={(e) => e.stopPropagation()}
            className="bg-bg-color w-full max-w-sm rounded-2xl p-5 shadow-xl flex flex-col"
          >
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-bold">{t('calendar.create_schedule')}</h3>
              <IconButton
                icon={<X className="w-5 h-5" />}
                onClick={() => setIsAdding(false)}
              />
            </div>
            <div className="flex flex-col gap-4">
              <input
                type="text"
                placeholder={t('calendar.title_placeholder')}
                className="bg-chat-other-bg rounded-lg px-4 py-3 text-[15px] outline-none"
                value={newTitle}
                onChange={(e) => setNewTitle(e.target.value)}
                autoFocus
              />
              <input
                type="text"
                placeholder={t('calendar.time_placeholder')}
                className="bg-chat-other-bg rounded-lg px-4 py-3 text-[15px] outline-none"
                value={newTime}
                onChange={(e) => setNewTime(e.target.value)}
              />
              <div className="text-sm text-text-sub mt-2 mb-4">{t('calendar.added_to', { date: format(currentDate, "yyyy年MM月dd日") })}</div>
              <button
                className="bg-primary-blue text-white w-full rounded-full py-3 font-semibold disabled:opacity-50"
                onClick={handleAddSchedule}
                disabled={!newTitle.trim()}
              >
                {t('calendar.save')}
              </button>
            </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
