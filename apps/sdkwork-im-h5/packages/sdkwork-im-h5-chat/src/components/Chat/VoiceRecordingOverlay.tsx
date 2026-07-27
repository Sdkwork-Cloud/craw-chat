import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { useTranslation } from "react-i18next";

interface VoiceRecordingOverlayProps {
  isRecording: boolean;
  recordingTime: number;
}

export const VoiceRecordingOverlay: React.FC<VoiceRecordingOverlayProps> = ({
  isRecording,
  recordingTime,
}) => {
  const { t } = useTranslation();
const formatTime = (secs: number) => {
  const m = Math.floor(secs / 60);
    const s = secs % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <AnimatePresence>
      {isRecording && (
        <motion.div
          initial={{ opacity: 0, y: 20, scale: 0.9 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, y: 10, scale: 0.95 }}
          transition={{ type: "spring", stiffness: 400, damping: 25 }}
          className="absolute bottom-[130px] left-1/2 -translate-x-1/2 z-50 flex flex-col items-center pointer-events-none"
        >
          <div className="bg-bg-color/95 dark:bg-[#1A1A1A]/95 backdrop-blur-2xl rounded-3xl px-6 py-5 min-w-[220px] flex flex-col items-center gap-3 shadow-2xl border border-border-color">
            <div className="text-text-main font-mono text-2xl tracking-wider font-semibold">
              {formatTime(recordingTime)}
            </div>
            <div className="flex items-center justify-center gap-1.5 h-12 w-32">
              {[...Array(9)].map((_, i) => (
                <motion.div
                  key={i}
                  className="w-1.5 rounded-full bg-gradient-to-t from-indigo-500 via-purple-500 to-pink-500"
                  animate={{ height: ["20%", "100%", "30%", "80%", "20%"] }}
                  transition={{
                    duration: 0.5 + (i % 3) * 0.2,
                    repeat: Infinity,
                    ease: "easeInOut",
                    delay: i * 0.05,
                  }}
                />
              ))}
            </div>
            <div className="flex items-center justify-center gap-2 text-text-sub text-[13px] mt-1 font-medium whitespace-nowrap">
              <span className="w-1.5 h-1.5 rounded-full bg-accent-red animate-pulse shrink-0" />
              {t('chat.date.release_to_send')} · {t('chat.date.slide_to_cancel')}
            </div>
          </div>
          {/* Tooltip Arrow */}
          <div className="w-4 h-4 bg-bg-color/95 dark:bg-[#1A1A1A]/95 rotate-45 -mt-2 border-r border-b border-border-color backdrop-blur-2xl" />
        </motion.div>
      )}
    </AnimatePresence>
  );
};
