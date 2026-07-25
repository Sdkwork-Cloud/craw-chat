import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router";
import { ChevronLeft, Play, Square, Mic, Settings2 } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { motion } from "motion/react";
import { VoiceService, type VoiceInfo } from "@sdkwork/im-h5-commons";
import { cn } from "@sdkwork/im-h5-commons";

export const MyVoiceDetail: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const navigate = useNavigate();
  const [voice, setVoice] = useState<VoiceInfo | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);

  useEffect(() => {
    if (!id) return;
    VoiceService.getVoiceCategories().then(cats => {
        let found: VoiceInfo | undefined;
        for (const cat of cats) {
            found = cat.voices.find(v => v.id === id);
            if (found) break;
        }
        if (found) setVoice(found);
    });
  }, [id]);

  const togglePlay = () => {
  if (isPlaying) {
      setIsPlaying(false);
    } else {
      setIsPlaying(true);
      setTimeout(() => setIsPlaying(false), 3000);
    }
  };

  if (!voice) return null;

  return (
    <div className="flex flex-col h-full bg-[#f2f2f2] dark:bg-[#121212]">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" strokeWidth={2.5} />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">声音详情</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-2">
           <IconButton
            icon={<Settings2 className="w-5 h-5 text-text-main" />}
            onClick={() => {}}
          />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto w-full flex flex-col items-center pb-8">
        <div className="w-full bg-white dark:bg-[#1A1A1A] p-6 border-b border-border-color">
            <h2 className="text-[20px] font-bold text-text-main mb-2 text-center">{voice.label}</h2>
            <p className="text-[14px] text-text-sub text-center mb-6">{voice.desc}</p>
            
            <div className="w-full bg-chat-other-bg rounded-3xl p-8 shadow-sm border border-border-color flex flex-col items-center justify-center relative">
                <h3 className="text-[16px] font-medium text-text-sub mb-6 flex items-center gap-2">试听一下</h3>
                <p className="text-[22px] leading-relaxed text-text-main/90 font-serif tracking-wide text-center mt-2 px-4">“您好，我是您的专属AI智能语音伴侣。”</p>
                <div className="mt-8 flex justify-center w-full">
                    <button
                        onClick={togglePlay}
                        className="flex flex-col items-center gap-3 active:scale-95 transition-transform"
                    >
                        <div className="w-16 h-16 bg-primary-blue rounded-full flex items-center justify-center shadow-md relative">
                            {isPlaying && (
                                <motion.div
                                    className="absolute inset-0 border-[2px] border-primary-blue rounded-full"
                                    animate={{
                                        scale: [1, 1.25, 1],
                                        opacity: [0.6, 0, 0.6],
                                    }}
                                    transition={{ repeat: Infinity, duration: 1.5 }}
                                />
                            )}
                            {isPlaying ? (
                                <Square className="w-6 h-6 text-white fill-current relative z-10" />
                            ) : (
                                <Play className="w-6 h-6 text-white fill-current ml-1 relative z-10" />
                            )}
                        </div>
                    </button>
                </div>
            </div>
        </div>

        {/* Stats / Info */}
        <div className="w-full mt-2 bg-white dark:bg-[#1A1A1A] py-2">
            <div className="flex items-center px-4 py-4 border-b border-border-color last:border-0 hover:bg-chat-active-bg transition-colors">
                 <span className="text-[16px] text-text-main flex-1">声音类型</span>
                 <span className="text-[15px] text-text-sub">{voice.id.startsWith('custom_') ? '克隆声音' : '预设声音'}</span>
            </div>
             <div className="flex items-center px-4 py-4 border-b border-border-color last:border-0 hover:bg-chat-active-bg transition-colors">
                 <span className="text-[16px] text-text-main flex-1">创建时间</span>
                 <span className="text-[15px] text-text-sub">2026-05-24</span>
            </div>
             <div className="flex items-center px-4 py-4 border-b border-border-color last:border-0 hover:bg-chat-active-bg transition-colors">
                 <span className="text-[16px] text-text-main flex-1">使用限制</span>
                 <span className="text-[15px] text-text-sub">仅限本人使用</span>
            </div>
        </div>

        <div className="w-full mt-6 px-4 pb-8">
            <button className="w-full py-3.5 bg-white dark:bg-[#1A1A1A] border border-border-color text-red-500 rounded-full font-medium active:bg-chat-active-bg transition-colors">删除声音</button>
        </div>
      </div>
    </div>
  );
};
