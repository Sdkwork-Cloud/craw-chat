import { useTranslation } from "react-i18next";
import React from "react";
import { Search, FileText, Image as ImageIcon, Mic, File, Link as LinkIcon, MessageCircle } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

export interface FavoriteItem {
  id: string;
  title: string;
  type: string;
  typeLabel: string;
  time: string;
  source: string;
  preview: string;
  icon: string;
  color: string;
}

interface FavoriteCardProps {
  item: FavoriteItem;
  onClick: () => void;
  onLongPressProps?: any;
}

export const FavoriteCard: React.FC<FavoriteCardProps> = ({ item, onClick, onLongPressProps }) => {
  const { t } = useTranslation();
const Icon = item.icon === 'FileText' ? FileText :
               item.icon === 'Image' ? ImageIcon :
               item.icon === 'Mic' ? Mic :
               item.icon === 'File' ? File :
               item.icon === 'Link' ? LinkIcon : MessageCircle;

  return (
    <div
      className="p-4 border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer select-none touch-callout-none"
      onClick={onClick}
      {...onLongPressProps}
    >
      <div className="flex items-start gap-3 pointer-events-none">
         <div className={cn("w-10 h-10 rounded-xl flex items-center justify-center shrink-0 bg-hover-bg border border-border-color", item.color)}>
            <Icon className="w-5 h-5" strokeWidth={1.5} />
         </div>
         <div className="flex-1 min-w-0">
            <div className="font-bold text-text-main text-[16px] leading-tight mb-1">
              {item.title}
            </div>
            <div className="text-[13px] text-text-sub line-clamp-2 leading-relaxed mb-3">
              {item.preview}
            </div>
            <div className="flex items-center gap-3 text-[11px] font-medium text-text-sub uppercase tracking-wider">
              <span className="bg-gray-100 dark:bg-white/10 px-2 py-0.5 rounded-md text-[10px]">
                {item.typeLabel}
              </span>
              <span>{item.source}</span>
              <span className="ml-auto">{item.time}</span>
            </div>
         </div>
      </div>
    </div>
  );
};
