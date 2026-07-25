import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { Mic, Square } from "lucide-react";

interface Props {
  recordingState: string;
  timer: number;
  formatTime: (s: number) => string;
  startRecording: () => void;
  stopRecording: () => void;
}

export const CreateVoiceRecordStep: React.FC<Props> = ({ recordingState, timer, formatTime, startRecording, stopRecording }) => {
  const { t } = useTranslation();
return (
  <motion.div
    key="record-mode"
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="w-full flex-1 flex flex-col h-full min-h-0"
  >
    <div className="flex-1 bg-chat-other-bg rounded-3xl p-6 shadow-sm border border-border-color flex flex-col justify-center min-h-[200px]">
      <h3 className="text-[16px] font-bold text-text-main mb-4 text-center">{t('user.auto_n514e0400', '请使用普通话朗读以下文本')}</h3>
      <div className="relative">
        <p className="text-[22px] leading-relaxed text-text-main/90 font-serif tracking-wide text-center">{t('user.auto_n23f04a34', '"清晨的阳光透过树叶的缝隙，洒在林间小路上。微风拂过，带来阵阵花香，这是美好的一天开始。"')}</p>
      </div>
      <div className="mt-8 flex justify-center">
        <span className="text-[32px] font-mono font-bold tracking-wider text-text-main tabular-nums">
          {formatTime(timer)}
        </span>
      </div>
    </div>

    <div className="shrink-0 flex justify-center w-full mt-10 mb-6 min-h-[100px]">
      {recordingState === "idle" && (
        <button
          onClick={startRecording}
          className="flex flex-col items-center gap-3 active:opacity-70 transition-opacity pb-2"
        >
          <div className="w-20 h-20 bg-red-500 rounded-full flex items-center justify-center shadow-lg shadow-red-500/20">
            <Mic className="w-8 h-8 text-white" />
          </div>
          <span className="text-[14px] font-medium text-text-sub">{t('user.auto_a436eab', '点击开始录音')}</span>
        </button>
      )}

      {recordingState === "recording" && (
        <div className="flex flex-col items-center gap-3 pb-2">
          <button
            onClick={stopRecording}
            className="relative w-20 h-20 bg-red-500 rounded-full flex items-center justify-center active:scale-95 transition-transform shadow-lg shadow-red-500/20"
          >
            <motion.div
              className="absolute inset-0 bg-red-500 rounded-full"
              animate={{ scale: [1, 1.4, 1], opacity: [0.5, 0, 0.5] }}
              transition={{ repeat: Infinity, duration: 1.5 }}
            />
            <Square className="w-8 h-8 text-white fill-current relative z-10" />
          </button>
          <span className="text-[14px] font-bold text-red-500 tracking-wide">{t('user.auto_2475d9db', '录音中...，点击结束')}</span>
        </div>
      )}
    </div>
  </motion.div>
  );
};
