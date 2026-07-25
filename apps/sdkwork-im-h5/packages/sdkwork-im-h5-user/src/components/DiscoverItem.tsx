import React from "react";
import { ChevronRight } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

export interface DiscoverItemProps {
  icon: React.ElementType;
  label: string;
  rightElement?: React.ReactNode;
  colorClass?: string;
  hasBorder?: boolean;
  onClick?: () => void;
}

export const DiscoverItem: React.FC<DiscoverItemProps> = ({
  icon: Icon,
  label,
  rightElement,
  colorClass = "text-text-main",
  hasBorder = true,
  onClick,
}) => {
  return (
    <>
      <div
        onClick={onClick}
        className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer"
      >
        <div className="flex items-center gap-3">
          <Icon className={cn("w-6 h-6", colorClass)} strokeWidth={2} />
          <span className="text-[16px] text-text-main">{label}</span>
        </div>
        <div className="flex items-center gap-2 text-text-sub">
          {rightElement}
          <ChevronRight className="w-5 h-5 opacity-40" />
        </div>
      </div>
      {hasBorder && <div className="h-[0.5px] bg-border-color ml-[52px]" />}
    </>
  );
};
