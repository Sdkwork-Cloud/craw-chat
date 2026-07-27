import React from 'react';
import { motion } from 'motion/react';
import { CheckCircle2 } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { useTranslation } from 'react-i18next';

export const PunchButton: React.FC<{
  handleClockIn: () => void;
  isDoneToday: boolean;
  hasPunchedIn: boolean;
}> = ({ handleClockIn, isDoneToday, hasPunchedIn }) => {
  const { t } = useTranslation();

  return (
    <motion.div whileTap={{ scale: 0.95 }} className="mb-12">
      <button
        onClick={handleClockIn}
        disabled={isDoneToday}
        className={cn(
          "w-40 h-40 rounded-full flex flex-col items-center justify-center text-white shadow-[0_8px_30px_rgb(0,0,0,0.12)] transition-colors",
          isDoneToday
            ? "bg-slate-400 shadow-slate-400/30"
            : hasPunchedIn
              ? "bg-orange-500 shadow-orange-500/30"
              : "bg-gradient-to-tr from-blue-600 to-primary-blue shadow-blue-500/30",
        )}
      >
        {isDoneToday ? (
          <>
            <CheckCircle2 className="w-10 h-10 mb-2" />
            <span className="text-[18px] font-medium">{t('attendance.status.done')}</span>
          </>
        ) : hasPunchedIn ? (
          <>
            <span className="text-[20px] font-medium mb-1">{t('attendance.status.clockOut')}</span>
            <span className="text-[13px] opacity-80">18:00</span>
          </>
        ) : (
          <>
            <span className="text-[20px] font-medium mb-1">{t('attendance.status.clockIn')}</span>
            <span className="text-[13px] opacity-80">09:00</span>
          </>
        )}
      </button>
    </motion.div>
  );
};
