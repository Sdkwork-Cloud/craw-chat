import { useTranslation } from "react-i18next";
import React from "react";
import { Eye, Heart, MessageSquare, MoreHorizontal, Play, FileText, Headphones, Image as ImageIcon } from "lucide-react";
import { Work } from "../services/WorkService";

interface WorkCardProps {
  work: Work;
  onClick: () => void;
  onMoreClick: (e: React.MouseEvent) => void;
  onLongPressProps?: any;
}

export const WorkCard: React.FC<WorkCardProps> = ({ work, onClick, onMoreClick, onLongPressProps }) => {
  const { t } = useTranslation();
const getWorkIcon = (type: Work["type"]) => {
  switch (type) {
      case "video":
        return <Play className="w-3.5 h-3.5 text-white fill-current" />;
      case "article":
        return <FileText className="w-3.5 h-3.5 text-white" />;
      case "audio":
        return <Headphones className="w-3.5 h-3.5 text-white" />;
      case "ai_image":
        return <ImageIcon className="w-3.5 h-3.5 text-white" />;
    }
  };

  const formatNumber = (num: number) => {
  if (num >= 10000) return (num / 10000).toFixed(1) + "w";
    return num.toString();
  };

  return (
    <div
      className="bg-bg-color overflow-hidden flex flex-col active:opacity-80 transition-opacity cursor-pointer select-none touch-callout-none"
      onClick={onClick}
      {...onLongPressProps}
    >
      {/* Cover Area */}
      <div
        className="w-full aspect-[3/4] relative bg-cover bg-center pointer-events-none"
        style={{ backgroundImage: `url(${work.coverUrl})` }}
      >
        {/* Overlay Gradient for Type Icon */}
        <div className="absolute inset-0 bg-gradient-to-b from-black/20 via-transparent to-black/60 pointer-events-none" />

        {/* Type Icon Badge */}
        <div className="absolute top-2 right-2 w-6 h-6 rounded-full bg-black/40 backdrop-blur-md flex items-center justify-center pointer-events-none">
          {getWorkIcon(work.type)}
        </div>

        {/* Stats overlay */}
        <div className="absolute bottom-2 left-2 right-2 flex items-center gap-2 text-white/90 pointer-events-none">
          <div className="flex items-center gap-1">
            <Eye className="w-3 h-3" />
            <span className="text-[11px] font-medium">
              {formatNumber(work.views)}
            </span>
          </div>
        </div>
      </div>

      {/* Content Area */}
      <div className="p-2.5 flex flex-col flex-1 pointer-events-none">
        <span className="text-[13px] font-medium text-text-main line-clamp-2 leading-snug mb-2 flex-1">
          {work.title}
        </span>

        <div className="flex items-center justify-between mt-auto pt-2 border-t border-border-color/30">
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1 text-text-sub">
              <Heart className="w-3.5 h-3.5" />
              <span className="text-[11px]">
                {formatNumber(work.likes)}
              </span>
            </div>
            <div className="flex items-center gap-1 text-text-sub">
              <MessageSquare className="w-3.5 h-3.5" />
              <span className="text-[11px]">
                {formatNumber(work.comments)}
              </span>
            </div>
          </div>

          <div
            className="p-1 -mr-1 rounded-full active:bg-chat-active-bg transition-colors pointer-events-auto"
            onClick={onMoreClick}
          >
            <MoreHorizontal className="w-4 h-4 text-text-sub" />
          </div>
        </div>
      </div>
    </div>
  );
};
