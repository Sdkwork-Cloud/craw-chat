import React from "react";
import { Download } from "lucide-react";
import { motion } from "motion/react";
import type { VideoTask } from "../services/AIVideoService";

interface AIVideoPreviewCardProps {
  t: (key: string, params?: any) => string;
  currentTask: VideoTask;
  isGenerating: boolean;
  currentProgress: number;
  onDownload: (url?: string) => void;
}

export const AIVideoPreviewCard: React.FC<AIVideoPreviewCardProps> = ({
  t,
  currentTask,
  isGenerating,
  currentProgress,
  onDownload,
}) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="flex flex-col gap-3 mb-6"
    >
      <div className="flex items-center justify-between">
        <h3 className="text-[16px] font-bold text-text-main">
          {t("result.title")}
        </h3>
        {isGenerating ? (
          <span className="text-xs text-primary-blue font-medium bg-primary-blue/10 px-2 py-0.5 rounded-full animate-pulse">
            {t("result.processing", {
              progress: Math.round(currentProgress),
            })}
          </span>
        ) : (
          <button
            onClick={() => onDownload(currentTask.videoUrl)}
            className="text-[#576B95] text-sm font-medium flex items-center gap-1.5 bg-[#576B95]/10 border border-[#576B95]/20 px-3 py-1.5 rounded-lg shadow-sm"
          >
            <Download className="w-3.5 h-3.5" />
            {t("result.save")}
          </button>
        )}
      </div>

      <div
        className={`bg-black rounded-2xl overflow-hidden relative border border-border-color flex items-center justify-center shadow-md mx-auto ${
          currentTask.options.aspectRatio === "9:16"
            ? "w-[70%] aspect-[9/16]"
            : currentTask.options.aspectRatio === "1:1"
              ? "w-full aspect-square"
              : "w-full aspect-video"
        }`}
      >
        {currentTask.status === "generating" ? (
          <div className="flex flex-col items-center justify-center absolute inset-0 bg-black/80 backdrop-blur-sm z-10 text-white gap-3">
            <div className="w-12 h-12 relative flex items-center justify-center">
              <div className="absolute inset-0 border-4 border-white/20 rounded-full" />
              <div className="absolute inset-0 border-4 border-primary-blue rounded-full border-t-transparent animate-spin" />
            </div>
            <div className="text-sm tracking-widest font-mono">RENDERING</div>
            <div className="w-3/4 max-w-[200px] h-1.5 bg-white/20 rounded-full overflow-hidden mt-1">
              <div
                className="h-full bg-primary-blue transition-all duration-300"
                style={{ width: `${currentProgress}%` }}
              />
            </div>
          </div>
        ) : (
          <>
            <video
              src={currentTask.videoUrl}
              poster={currentTask.thumbnailUrl}
              controls
              autoPlay
              loop
              className="w-full h-full object-contain bg-black"
            />
            <div className="absolute top-3 left-3 bg-black/60 backdrop-blur text-white/90 text-[11px] font-medium px-2 py-1 rounded">
              {currentTask.options.style}
            </div>
          </>
        )}
      </div>
    </motion.div>
  );
};
