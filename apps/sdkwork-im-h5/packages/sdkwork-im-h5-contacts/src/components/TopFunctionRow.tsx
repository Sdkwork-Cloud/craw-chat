import React from 'react';
import { cn } from '@sdkwork/im-h5-commons';

export const TopFunctionRow = ({
  icon: Icon,
  title,
  bgColor,
  onClick,
  badge,
}: {
  icon: React.ElementType;
  title: string;
  bgColor: string;
  onClick: () => void;
  badge?: number;
}) => (
  <div
    className="flex items-center pl-4 pr-3 py-2.5 bg-bg-color active:bg-active-bg transition-colors cursor-pointer"
    onClick={onClick}
  >
    <div
      className={cn(
        "w-10 h-10 rounded-[10px] flex items-center justify-center shrink-0 mr-3.5",
        bgColor,
      )}
    >
      <Icon className="w-5 h-5 text-white" />
    </div>
    <div className="flex-1 border-b border-border-color/50 min-h-[44px] flex items-center gap-2">
      <span className="text-[16px] text-text-main">{title}</span>
      {badge !== undefined && badge > 0 && (
        <span className="h-[18px] min-w-[18px] px-1.5 rounded-full bg-[#FF3B30] text-white text-[11px] font-medium flex items-center justify-center leading-none shrink-0">
          {badge > 99 ? "99+" : badge}
        </span>
      )}
    </div>
  </div>
);
