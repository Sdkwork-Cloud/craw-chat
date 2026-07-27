import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { Sparkles, Square, Play, RotateCcw, CheckCircle2 } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

interface Props {
  previewLang: string;
  setPreviewLang: (v: string) => void;
  isPreviewPlaying: boolean;
  togglePreview: () => void;
  onRetake: () => void;
  onConfirm: () => void;
}

export const CreateVoicePreviewStep: React.FC<Props> = ({ previewLang, setPreviewLang, isPreviewPlaying, togglePreview, onRetake, onConfirm }) => {
  const { t } = useTranslation();
return (
  <motion.div
    key="recorded-mode"
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="w-full flex-1 flex flex-col min-h-0 pt-4"
  >
    <div className="flex-1 bg-chat-other-bg rounded-3xl p-8 shadow-sm border border-border-color flex flex-col items-center justify-center w-full relative">
      <div className="absolute top-4 right-4 flex items-center gap-1 bg-black/5 dark:bg-white/5 rounded-full p-1">{['中文', 'English'].map(lang => (<button
            key={lang}
            onClick={() => setPreviewLang(lang)}
            className={cn("px-4 py-1.5 rounded-full text-[13px] font-medium transition-colors", previewLang === lang ? "bg-white dark:bg-[#333] shadow-sm text-text-main" : "text-text-sub")}
          >
            {lang}
          </button>
        ))}
      </div>
      
      <h3 className="text-[16px] font-medium text-text-sub mb-6 flex items-center gap-2">
        <Sparkles className="w-5 h-5 text-primary-blue" />{t('user.auto_3dcae669', `克隆成功，快来试听一下吧`)}</h3>
      <p className="text-[22px] leading-relaxed text-text-main/90 font-serif tracking-wide text-center mt-2 px-4">{previewLang === '中文' ? '“您好，我是您的专属AI智能语音伴侣。”' : '"Hello, I am your personal AI voice companion."'}</p>
    </div>

    <div className="shrink-0 flex flex-col items-center w-full mt-8 mb-6 gap-6">
      <div className="flex flex-col items-center w-full max-w-[300px] mx-auto">
        <button
          onClick={togglePreview}
          className="flex flex-col items-center gap-3 active:scale-95 transition-transform mb-6"
        >
          <div className="w-16 h-16 bg-primary-blue rounded-full flex items-center justify-center shadow-md relative">
            {isPreviewPlaying && (
              <motion.div
                className="absolute inset-0 border-[2px] border-primary-blue rounded-full"
                animate={{ scale: [1, 1.25, 1], opacity: [0.6, 0, 0.6] }}
                transition={{ repeat: Infinity, duration: 1.5 }}
              />
            )}
            {isPreviewPlaying ? (
              <Square className="w-6 h-6 text-white fill-current relative z-10" />
            ) : (
              <Play className="w-7 h-7 text-white fill-current ml-1 relative z-10" />
            )}
          </div>
          <span className="text-[14px] font-medium text-text-main">{isPreviewPlaying ? "停止试听" : "播放试听"}</span>
        </button>

        <div className="flex items-center justify-between w-full mt-2 px-4">
          <button
            onClick={onRetake}
            className="flex flex-col items-center gap-2 active:opacity-70 transition-opacity"
          >
            <div className="w-12 h-12 bg-chat-other-bg rounded-full flex items-center justify-center border border-border-color">
              <RotateCcw className="w-5 h-5 text-text-sub" />
            </div>
            <span className="text-[13px] text-text-sub">{t('user.auto_43d09644', `重新录制`)}</span>
          </button>
          
          <button
            onClick={onConfirm}
            className="flex flex-col items-center gap-2 active:opacity-70 transition-opacity"
          >
            <div className="w-12 h-12 bg-primary-blue/10 rounded-full flex items-center justify-center border border-primary-blue/20">
              <CheckCircle2 className="w-6 h-6 text-primary-blue" />
            </div>
            <span className="text-[13px] font-medium text-primary-blue">{t('user.auto_38d4d0ff', `确认使用`)}</span>
          </button>
        </div>
      </div>
    </div>
  </motion.div>
  );
};
