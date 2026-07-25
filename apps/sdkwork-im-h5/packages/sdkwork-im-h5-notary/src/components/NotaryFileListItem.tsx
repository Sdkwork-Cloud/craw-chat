import React from "react";
import { MoreVertical } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";

interface NotaryFileListItemProps {
  item: any;
  icon: React.ComponentType<any>;
  onTouchStart: (item: any) => void;
  clearTouchTimeout: () => void;
  onItemClick: (item: any) => void;
  onMoreClick: (item: any, e: React.MouseEvent) => void;
}

export const NotaryFileListItem: React.FC<NotaryFileListItemProps> = ({
  item,
  icon: Icon,
  onTouchStart,
  clearTouchTimeout,
  onItemClick,
  onMoreClick,
}) => {
  return (
    <div
      onTouchStart={() => onTouchStart(item)}
      onTouchEnd={clearTouchTimeout}
      onTouchMove={clearTouchTimeout}
      onMouseDown={() => onTouchStart(item)}
      onMouseUp={clearTouchTimeout}
      onMouseLeave={clearTouchTimeout}
      onClick={() => onItemClick(item)}
      className={cn(
        "flex items-center p-3 mb-2 bg-white dark:bg-[#1c1c1e] rounded-2xl shadow-sm border border-border-color/30 active:scale-[0.99] transition-transform select-none"
      )}
    >
      <div
        className={cn(
          "w-12 h-12 rounded-[14px] flex items-center justify-center shrink-0 mr-4",
          item.bg ||
            (item.type === "folder"
              ? "bg-yellow-50 dark:bg-yellow-500/10"
              : "bg-gray-100 dark:bg-white/5")
        )}
      >
        <Icon
          className={cn("w-7 h-7", item.iconColor, item.fill)}
          strokeWidth={item.type === "folder" ? 1.5 : 2}
        />
      </div>
      <div className="flex-1 min-w-0 flex items-center justify-between">
        <div className="flex flex-col min-w-0 pr-2">
          <span className="text-[16px] text-text-main font-medium truncate tracking-wide">
            {item.name}
          </span>
          <div className="flex items-center gap-2 mt-1.5">
            <span className="text-[12px] text-text-sub/70">{item.date}</span>
            {item.size !== "-" && (
              <span className="text-[12px] text-text-sub/70">{item.size}</span>
            )}
          </div>
        </div>
        <IconButton
          icon={
            <MoreVertical className="w-5 h-5 text-text-sub opacity-50 relative z-10" />
          }
          onClick={(e) => onMoreClick(item, e)}
        />
      </div>
    </div>
  );
};
