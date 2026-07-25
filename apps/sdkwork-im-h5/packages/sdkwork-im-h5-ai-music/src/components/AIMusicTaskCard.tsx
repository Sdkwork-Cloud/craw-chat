import React from "react";
import { Music, Sparkles, Loader2, Play, Pause } from "lucide-react";
import { MusicTask } from "../pages/AIMusicPage";

interface AIMusicTaskCardProps {
  task: MusicTask;
  t: any;
  playingId: string | null;
  handlePlay: (id: string) => void;
  vocalType: string;
  tempo: string;
}

export const AIMusicTaskCard: React.FC<AIMusicTaskCardProps> = ({
  task,
  t,
  playingId,
  handlePlay,
  vocalType,
  tempo,
}) => {
  return (
    <div className="bg-[#222] border border-white/10 rounded-2xl p-4 flex flex-col relative overflow-hidden">
      {task.status === "processing" ? (
        <div>
          <div className="flex items-center gap-4 mb-4">
            <div className="w-16 h-16 rounded-xl bg-[#333] flex items-center justify-center relative shadow-inner">
              <Loader2 className="w-6 h-6 animate-spin text-white/50" />
            </div>
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1.5">
                <Sparkles className="w-4 h-4 text-purple-400" />
                <span className="text-[14px] font-medium text-white/90">
                  {t("library.rendering")}
                </span>
              </div>
              <p className="text-[12px] text-white/50 line-clamp-1">
                {task.prompt}
              </p>
            </div>
          </div>
          <div className="w-full h-1.5 bg-[#111] rounded-full overflow-hidden">
            <div
              className="h-full bg-gradient-to-r from-purple-500 to-pink-500 rounded-full transition-all duration-200"
              style={{ width: `${task.progress}%` }}
            ></div>
          </div>
        </div>
      ) : (
        <div className="flex gap-4">
          <div className="w-20 h-20 rounded-xl relative overflow-hidden shrink-0 group shadow-md bg-[#333]">
            {task.coverUrl ? (
              <img src={task.coverUrl} className="w-full h-full object-cover" alt={task.title} />
            ) : (
              <div className="w-full h-full flex items-center justify-center">
                <Music className="w-8 h-8 text-white/20" />
              </div>
            )}
            <div
              className="absolute inset-0 bg-black/40 flex items-center justify-center cursor-pointer opacity-100 transition-opacity"
              onClick={() => handlePlay(task.id)}
            >
              {playingId === task.id ? (
                <Pause className="w-8 h-8 text-white drop-shadow-md fill-white" />
              ) : (
                <Play className="w-8 h-8 text-white drop-shadow-md fill-white ml-1" />
              )}
            </div>
          </div>
          <div className="flex-1 flex flex-col justify-between">
            <div>
              <h3 className="text-[16px] font-bold text-white mb-1 line-clamp-1">
                {task.title}
              </h3>
              <div className="text-[12px] text-white/50 space-x-2">
                <span className="text-purple-400">
                  vocal: {t(`vocal_styles.${vocalType}`, { defaultValue: vocalType })}
                </span>
                <span>tempo: {tempo}</span>
              </div>
            </div>
            <div className="flex items-center gap-2 mt-2">
              <span className="px-2 py-0.5 rounded bg-[#333] text-[10px] text-white/70 border border-white/5">
                {t(`styles.${task.style}`, { defaultValue: task.style })}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
