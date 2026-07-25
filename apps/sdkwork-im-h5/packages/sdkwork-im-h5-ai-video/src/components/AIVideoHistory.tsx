import { useTranslation } from "react-i18next";
import React from "react";
import { PlayCircle, Trash2 } from "lucide-react";
import { VideoTask } from "../services/AIVideoService";

interface AIVideoHistoryProps {
  t: any;
  history: VideoTask[];
  onSelect: (item: VideoTask) => void;
  onDelete: (e: React.MouseEvent, id: string) => void;
  onSuggestionClick: (suggestion: string) => void;
  currentTask: VideoTask | null;
}

export const AIVideoHistory: React.FC<AIVideoHistoryProps> = ({
  t,
  history,
  onSelect,
  onDelete,
  onSuggestionClick,
  currentTask,
}) => {
  

return (
    <div className="flex flex-col gap-3">
      {history.length > 0 ? (
        <>
          <h3
            id="history-section"
            className="text-[16px] font-bold text-text-main"
          >
            {t('history')}
          </h3>
          <div className="grid grid-cols-2 gap-3">
            {history.map((item) => (
              <div
                key={item.id}
                className="flex flex-col gap-2 group cursor-pointer"
                onClick={() => onSelect(item)}
              >
                <div
                  className={`rounded-xl overflow-hidden relative border border-border-color bg-black ${item.options.aspectRatio === "9:16" ? "aspect-[9/16]" : item.options.aspectRatio === "1:1" ? "aspect-square" : "aspect-video"}`}
                >
                  {item.thumbnailUrl?.includes("video") ||
                  item.thumbnailUrl?.endsWith(".mp4") ? (
                    <video
                      src={item.thumbnailUrl}
                      className="w-full h-full object-cover opacity-80 group-active:opacity-100 transition-opacity"
                      muted
                      playsInline
                    />
                  ) : (
                    <img
                      src={item.thumbnailUrl}
                      className="w-full h-full object-cover opacity-80 group-active:opacity-100 transition-opacity"
                    />
                  )}
                  <div className="absolute inset-0 flex items-center justify-center">
                    <PlayCircle className="w-8 h-8 text-white/60 drop-shadow-md" />
                  </div>
                  <button
                    onClick={(e) => onDelete(e, item.id)}
                    className="absolute top-1.5 right-1.5 bg-black/40 p-1.5 rounded-full text-white/80 hover:bg-black/80 hover:text-red-400 active:text-red-400 active:bg-black/80 backdrop-blur z-10 opacity-0 group-hover:opacity-100 md:opacity-100 transition-opacity"
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                  </button>
                </div>
                <p className="text-xs text-text-main line-clamp-2 px-1 font-medium">
                  {item.options.prompt}
                </p>
              </div>
            ))}
          </div>
        </>
      ) : !currentTask ? (
        <div className="pt-6 flex flex-col items-center justify-center opacity-70">
          <PlayCircle className="w-12 h-12 text-text-sub mb-3 opacity-50" />
          <h3 className="text-sm font-medium text-text-sub mb-4">
            {t('history_section.empty_title')}
          </h3>
          <div className="flex flex-wrap gap-2 justify-center px-4">
            {(Array.isArray(t('history_section.suggestions', { returnObjects: true })) 
              ? t('history_section.suggestions', { returnObjects: true }) 
              : []).map((suggestion: string, i: number) => (
              <button
                key={i}
                onClick={() => onSuggestionClick(suggestion)}
                className="bg-active-bg border border-border-color px-3 py-1.5 rounded-full text-xs text-text-main hover:border-primary-blue transition-colors active:scale-95"
              >
                {suggestion}
              </button>
            ))}
          </div>
        </div>
      ) : null}
    </div>
  );
};
