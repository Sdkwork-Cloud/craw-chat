import { useTranslation } from "react-i18next";
import React from "react";
import { ChevronRight } from "lucide-react";
import { cn } from "../utils/cn";

export interface ListItemProps {
  icon?: React.ElementType;
  label: React.ReactNode;
  rightElement?: React.ReactNode;
  rightText?: React.ReactNode;
  onClick?: () => void;
  danger?: boolean;
  hideBorder?: boolean;
}

export const ListItem: React.FC<ListItemProps> = ({
  icon: Icon,
  label,
  rightElement,
  rightText,
  onClick,
  danger,
  hideBorder,
}) => {
  const { t } = useTranslation();
  return (
  <div
    onClick={onClick}
    className={cn(
      "flex items-center justify-between px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer",
      !hideBorder && "border-b border-border-color/60",
      "last:border-none",
    )}
  >
    <div className="flex items-center gap-3">
      {Icon && (
        <Icon
          className={cn("w-5 h-5", danger ? "text-red-500" : "text-text-main")}
        />
      )}
      <span
        className={cn(
          "text-[16px]",
          danger ? "text-red-500" : "text-text-main",
        )}
      >
        {label}
      </span>
    </div>
    <div className="flex items-center gap-1.5 text-text-sub">
      {rightText && <span className="text-[15px]">{rightText}</span>}
      {rightElement ||
        (!danger && <ChevronRight className="w-5 h-5 opacity-40" />)}
    </div>
  </div>
);
};

