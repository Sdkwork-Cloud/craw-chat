import React from "react";
import { Trash2 } from "lucide-react";
import { WritingTask } from "../services/AIWritingService";

interface AIWritingHistoryItemProps {
  item: WritingTask;
  t: any;
  onSelect: (item: WritingTask) => void;
  onDelete: (e: React.MouseEvent, id: string) => void;
}

export const AIWritingHistoryItem: React.FC<AIWritingHistoryItemProps> = ({
  item,
  t,
  onSelect,
  onDelete,
}) => {
  return (
    <div
      className="bg-bg-color border border-border-color rounded-xl p-3 shadow-sm cursor-pointer active:bg-active-bg transition-colors"
      onClick={() => onSelect(item)}
    >
      <div className="flex justify-between items-start mb-2 pr-6 relative group">
        <span className="text-[14px] font-bold text-text-main line-clamp-1 flex-1">
          {item.options.topic}
        </span>
        <span className="text-[10px] bg-active-bg text-text-sub px-2 py-0.5 rounded ml-2 whitespace-nowrap shrink-0">
          {t(`styles.${item.options.style}`, { defaultValue: item.options.style })}
        </span>
        <button
          onClick={(e) => onDelete(e, item.id)}
          className="absolute top-0 right-0 text-text-sub opacity-50 hover:opacity-100 transition-opacity active:text-red-500 hover:text-red-500 z-10 p-1"
        >
          <Trash2 className="w-4 h-4" />
        </button>
      </div>
      <p className="text-[12px] text-text-sub line-clamp-2 leading-relaxed">
        {item.content}
      </p>
    </div>
  );
};
